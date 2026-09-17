use super::tables::{
    DELTA, DEVIATION_MAP, DEVIATIONS_A_G, DEVIATIONS_K_ZC, GRADE_MAP, HOLE_PREFERRED, LOWER_J,
    SHAFT_PREFERRED, STANDARD_TOLERANCE_GRADES, UPPER_J,
};
use std::collections::BinaryHeap;

use wasm_bindgen::prelude::*;

// Largest nominal size covered by ISO 286, in millimetres.
const MAX_SIZE: i32 = 3150;

// Deviations are held in nanometres, so a size cannot be resolved any finer.
const MAX_DECIMALS: usize = 6;

// Indices into DEVIATION_MAP. The letters are ordered by fundamental deviation,
// so the groups that share a lookup rule are contiguous ranges.
const DEV_B: usize = 1;
const DEV_G: usize = 9;
const DEV_H: usize = 10;
const DEV_JS: usize = 11;
const DEV_J: usize = 12;
const DEV_K: usize = 13;
const DEV_M: usize = 14;
const DEV_N: usize = 15;
const DEV_P: usize = 16;
const DEV_ZC: usize = 27;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tolerance {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

impl Tolerance {
    fn from_int(upper_nm: i32, lower_nm: i32) -> Self {
        Self {
            upper: nm_to_mm(upper_nm),
            middle: nm_to_mm((upper_nm + lower_nm) / 2),
            lower: nm_to_mm(lower_nm),
        }
    }
}

/// Which side of a fit a tolerance applies to.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    /// An internal feature, taking an upper case class such as `"H7"`.
    Hole,
    /// An external feature, taking a lower case class such as `"h7"`.
    Shaft,
}

impl Feature {
    fn is_hole(self) -> bool {
        matches!(self, Feature::Hole)
    }
}

/// One tolerance class, and the nominal size it was matched at.
///
/// The deviations are held flat rather than as a [`Tolerance`], so that reading one
/// across the wasm boundary is a plain number rather than a further handle. Use
/// [`Match::tolerance`] to get them back as a [`Tolerance`].
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    /// Nominal size in millimetres. Equal to the size supplied under a strict search.
    pub size: f64,
    /// Tolerance class, e.g. `"H7"`.
    #[wasm_bindgen(getter_with_clone)]
    pub class: String,
    /// Upper deviation in millimetres, as `limits` would report it.
    pub upper: f64,
    /// Midpoint of the two deviations in millimetres.
    pub middle: f64,
    /// Lower deviation in millimetres, as `limits` would report it.
    pub lower: f64,
    /// Distance from the wanted tolerance in millimetres: the differences in upper
    /// limit, lower limit and midpoint, added together. Zero is an exact match.
    pub error: f64,
}

impl Match {
    /// The deviations as a [`Tolerance`], matching what [`limits`] returns for this
    /// class at this size.
    pub fn tolerance(&self) -> Tolerance {
        Tolerance {
            upper: self.upper,
            middle: self.middle,
            lower: self.lower,
        }
    }

    // Build from the integer limits and doubled nanometre error the search holds.
    fn from_int(size: f64, class: String, limits: (i32, i32), error: i64) -> Self {
        let tolerance = Tolerance::from_int(limits.0, limits.1);
        Self {
            size,
            class,
            upper: tolerance.upper,
            middle: tolerance.middle,
            lower: tolerance.lower,
            error: error_to_mm(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidToleranceClass(String),
    SizeOutOfRange(f64),
    UnsupportedCombination { tolerance_class: String, size: f64 },
    InvalidTolerance { upper: f64, lower: f64 },
    InvalidPrecision(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidToleranceClass(s) => write!(f, "Invalid tolerance class: {s:?}"),
            Error::SizeOutOfRange(s) => write!(f, "Nominal size {s} mm is out of range (0 < 3150)"),
            Error::UnsupportedCombination {
                tolerance_class,
                size,
            } => write!(
                f,
                "Combination not supported by ISO 286: {size} {tolerance_class}"
            ),
            Error::InvalidTolerance { upper, lower } => {
                write!(f, "Invalid tolerance limits: {upper} / {lower}")
            }
            Error::InvalidPrecision(d) => write!(
                f,
                "Decimal precision {d} is out of range (0 to {MAX_DECIMALS})"
            ),
        }
    }
}

impl std::error::Error for Error {}

// Parse a tolerance class like "H7", "g6", "js4", "ZC3" into (deviation, grade).
fn parse_tolerance_class(tolerance_class: &str) -> Result<(&str, &str), Error> {
    let split = tolerance_class
        .find(|c: char| c.is_ascii_digit())
        .filter(|&i| i > 0)
        .ok_or_else(|| Error::InvalidToleranceClass(tolerance_class.to_owned()))?;
    let (dev, grade) = tolerance_class.split_at(split);
    if grade.is_empty() || !grade.chars().all(|c| c.is_ascii_digit()) {
        return Err(Error::InvalidToleranceClass(tolerance_class.to_owned()));
    }
    Ok((dev, grade))
}

/// Available IT grade designations.
pub fn grades() -> Vec<String> {
    GRADE_MAP.iter().map(|d| d.to_string()).collect()
}

/// Available hole deviation letters (uppercase).
pub fn hole_deviations() -> Vec<String> {
    DEVIATION_MAP.iter().map(|d| d.to_uppercase()).collect()
}

/// Available shaft deviation letters (lowercase).
pub fn shaft_deviations() -> Vec<String> {
    DEVIATION_MAP.iter().map(|d| d.to_string()).collect()
}

/// Preferred hole tolerance classses
pub fn hole_preferred_tolerances() -> Vec<String> {
    HOLE_PREFERRED.iter().map(|t| t.to_string()).collect()
}

/// Preferred shaft tolerance classes
pub fn shaft_preferred_tolerances() -> Vec<String> {
    SHAFT_PREFERRED.iter().map(|t| t.to_string()).collect()
}

/// List the closest preferred tolerances in order of suitability
pub fn list_preferred(size: f64, tolerance_class: &str) -> Result<Vec<Match>, Error> {
    let int_size = checked_size(size)?;
    let (hole, dev, idx_grade) = class_indices(tolerance_class)?;
    let unsupported = || Error::UnsupportedCombination {
        tolerance_class: tolerance_class.to_owned(),
        size,
    };
    let target = widen(limits_nm(int_size, hole, dev, idx_grade).ok_or_else(unsupported)?);

    let preferred = if hole {
        &HOLE_PREFERRED[..]
    } else {
        &SHAFT_PREFERRED[..]
    };
    let mut scored = preferred
        .iter()
        .filter_map(|class| {
            let (_, dev, idx_grade) = class_indices(class).ok()?;
            let limits = limits_nm(int_size, hole, dev, idx_grade)?;
            Some((band_error(widen(limits), target), *class, limits))
        })
        .collect::<Vec<(i64, &str, (i32, i32))>>();

    // Ranked in whole nanometres, so no rounding noise, and stable so that classes
    // scoring identically keep the order they appear in the table.
    scored.sort_by_key(|&(error, _, _)| error);

    // Size may not map to a preferred tolerance class
    if scored.is_empty() {
        return Err(unsupported());
    }

    Ok(scored
        .into_iter()
        .map(|(error, class, limits)| Match::from_int(size, class.to_owned(), limits, error))
        .collect())
}

/// Find the closest preferred tolerance
pub fn find_preferred(size: f64, tolerance_class: &str) -> Result<Match, Error> {
    list_preferred(size, tolerance_class)?
        .into_iter()
        .next()
        .ok_or_else(|| Error::UnsupportedCombination {
            tolerance_class: tolerance_class.to_owned(),
            size,
        })
}

/// List the closest ISO tolerance classes to a size tolerance.
///
/// `size` is the nominal dimension in millimetres, `upper` and `lower` are the wanted
/// deviations from it, also in millimetres. They are ordered largest first if supplied
/// the other way around. `feature` selects hole or shaft classes.
///
/// With `strict` the nominal size is held exactly as given and only the tolerance class
/// varies. Otherwise the nominal size may also be moved to centre the band on the wanted
/// one, rounded to `decimals` places. Good practice is a single decimal place.
///
/// At most `results` classes are returned, closest first.
pub fn list_closest(
    size: f64,
    upper: f64,
    lower: f64,
    feature: Feature,
    strict: bool,
    results: usize,
    decimals: usize,
) -> Result<Vec<Match>, Error> {
    let int_size = checked_size(size)?;
    if !upper.is_finite() || !lower.is_finite() {
        return Err(Error::InvalidTolerance { upper, lower });
    }
    if decimals > MAX_DECIMALS {
        return Err(Error::InvalidPrecision(decimals));
    }
    let (upper, lower) = if upper < lower {
        (lower, upper)
    } else {
        (upper, lower)
    };
    if results == 0 {
        return Ok(Vec::new());
    }

    // Work from the absolute limits of the feature rather than the deviations, because a
    // candidate is free to reach them from a different nominal size.
    let hole = feature.is_hole();
    let size_nm = mm_to_nm(size);
    let target = (size_nm + mm_to_nm(upper), size_nm + mm_to_nm(lower));
    let quantum = 10i64.pow((MAX_DECIMALS - decimals) as u32);
    let preferred = preferred_indices(hole);

    // A size bracket and a grade fix the band width, whatever the deviation letter, so
    // they are worth ordering by their bound before paying for the 28 letters under each.
    let mut groups = Vec::new();
    for (start, end) in size_brackets(strict.then_some(int_size)) {
        for idx_grade in 1..=GRADE_MAP.len() {
            let Some(tolerance) = it_nm(end, idx_grade) else {
                continue;
            };
            let bound = width_bound(tolerance, target);
            groups.push((bound, start, end, idx_grade));
        }
    }
    groups.sort_unstable();

    let mut best: BinaryHeap<Candidate> = BinaryHeap::new();
    for (bound, start, end, idx_grade) in groups {
        // Nothing later in the list has a smaller bound, so once even a perfectly placed
        // band of this width would lose to the worst result held, the search is over.
        if best.len() == results && bound > best.peek().map_or(i64::MAX, |worst| worst.error) {
            break;
        }
        // Every table is constant across a bracket, so one lookup at its upper bound
        // stands for every nominal size within it.
        for dev in 0..DEVIATION_MAP.len() {
            let Some(limits) = limits_nm(end, hole, dev, idx_grade) else {
                continue;
            };
            let (found, error) = if strict {
                (size_nm, band_error(absolute(size_nm, limits), target))
            } else {
                best_fit(limits, target, (nm(start), nm(end)), quantum)
            };
            best.push(Candidate {
                error,
                shift: (found - size_nm).abs(),
                preferred: preferred.contains(&(dev, idx_grade)),
                size: found,
                limits,
                dev,
                idx_grade,
            });
            if best.len() > results {
                best.pop();
            }
        }
    }

    Ok(best
        .into_sorted_vec()
        .into_iter()
        .map(|c| {
            // Held exactly as supplied under strict, rather than round tripped.
            let size = if strict { size } else { nm_to_size(c.size) };
            let class = class_name(hole, c.dev, c.idx_grade);
            Match::from_int(size, class, c.limits, c.error)
        })
        .collect())
}

// The least error any class of this band width can reach, however its nominal size is
// chosen. Sliding the nominal size moves a band without resizing it, so the mismatch in
// width is the part of the error that no choice of size can pay off. Quantising the size
// and clamping it into a bracket only add to the error, so this stays a true bound.
fn width_bound(tolerance: i32, target: (i64, i64)) -> i64 {
    2 * (tolerance as i64 - (target.0 - target.1)).abs()
}

struct Candidate {
    error: i64,
    shift: i64,
    preferred: bool,
    size: i64,
    limits: (i32, i32),
    dev: usize,
    idx_grade: usize,
}

impl Candidate {
    // Closest first, then preferred classes, then the smallest change to the nominal
    // size, then the tightest grade. The trailing fields only exist to keep the ordering
    // total, and so deterministic.
    fn sort_key(&self) -> (i64, bool, i64, usize, usize) {
        (
            self.error,
            !self.preferred,
            self.shift,
            self.idx_grade,
            self.dev,
        )
    }
}

// Ordered worst first, so that a heap capped at the wanted number of results discards
// the candidate that is furthest away.
impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.sort_key() == other.sort_key()
    }
}

impl Eq for Candidate {}

// Place a candidate band as close to the target as the size grid allows, returning the
// nominal size chosen and the error it achieves, within a single size bracket.
//
// Moving the nominal size slides the band without resizing it, so the error is convex in
// the size and bottoms out where the two bands share a midpoint. Whatever is left there
// is the mismatch in band width, which no choice of size can fix. The best size on the
// grid is therefore whichever of the two grid points either side of that ideal scores
// better, once both are clamped into the bracket.
fn best_fit(
    limits: (i32, i32),
    target: (i64, i64),
    bracket: (i64, i64),
    quantum: i64,
) -> (i64, i64) {
    // Twice the size at which the midpoints coincide, left doubled to stay exact.
    let ideal = target.0 + target.1 - limits.0 as i64 - limits.1 as i64;
    let below = ideal.div_euclid(2 * quantum) * quantum;

    // The bracket excludes its lower bound, and both bounds are whole millimetres, so
    // they always sit on the grid.
    let (low, high) = (bracket.0 + quantum, bracket.1);
    [below, below + quantum]
        .into_iter()
        .map(|size| size.clamp(low, high))
        .map(|size| (band_error(absolute(size, limits), target), size))
        .min()
        .map(|(error, size)| (size, error))
        .expect("two candidate sizes are always evaluated")
}

// Every nominal size bracket, as exclusive lower and inclusive upper bounds in whole
// millimetres. Taking the boundaries of all the tables at once guarantees that a lookup
// anywhere inside a bracket answers for the whole of it. The 1 mm boundary is not a table
// row; it is there because a, b and N are undefined at exactly 1 mm.
//
// Passing a size narrows the result to the single bracket holding it.
fn size_brackets(containing: Option<i32>) -> Vec<(i32, i32)> {
    let mut bounds = STANDARD_TOLERANCE_GRADES
        .iter()
        .map(|row| row[0])
        .chain(DEVIATIONS_A_G.iter().map(|row| row[0]))
        .chain(DEVIATIONS_K_ZC.iter().map(|row| row[0]))
        .chain(DELTA.iter().map(|row| row[0]))
        .chain(UPPER_J.iter().map(|row| row[0]))
        .chain(LOWER_J.iter().map(|row| row[0]))
        .chain(std::iter::once(1))
        .filter(|&bound| bound <= MAX_SIZE)
        .collect::<Vec<i32>>();
    bounds.sort_unstable();
    bounds.dedup();

    let mut start = 0;
    bounds
        .into_iter()
        .map(|end| {
            let bracket = (start, end);
            start = end;
            bracket
        })
        .filter(|&(start, end)| containing.is_none_or(|size| size > start && size <= end))
        .collect()
}

// The preferred classes as table indices, so that membership can be tested inside the
// search without building a class name for every candidate.
fn preferred_indices(hole: bool) -> Vec<(usize, usize)> {
    let preferred = if hole {
        &HOLE_PREFERRED[..]
    } else {
        &SHAFT_PREFERRED[..]
    };
    preferred
        .iter()
        .filter_map(|class| class_indices(class).ok())
        .map(|(_, dev, idx_grade)| (dev, idx_grade))
        .collect()
}

// Deviations against a nominal size, as the absolute limits of the feature.
fn absolute(size: i64, limits: (i32, i32)) -> (i64, i64) {
    (size + limits.0 as i64, size + limits.1 as i64)
}

fn widen(limits: (i32, i32)) -> (i64, i64) {
    (limits.0 as i64, limits.1 as i64)
}

// How far a candidate band sits from the wanted one, in nanometres, doubled. Both limits
// are counted, plus the midpoint, which weights where the band sits against how wide it
// is. Doubling keeps the midpoint term exact rather than halving it away, and only scales
// the comparison. Translation invariant, so it reads either absolute limits or deviations
// as long as both arguments agree.
fn band_error(candidate: (i64, i64), target: (i64, i64)) -> i64 {
    let upper = candidate.0 - target.0;
    let lower = candidate.1 - target.1;
    2 * upper.abs() + 2 * lower.abs() + (upper + lower).abs()
}

/// Compute ISO 286 tolerance limits for a given nominal size and tolerance class.
///
/// `size` is the nominal dimension in millimetres.
/// `tolerance_class` is the standard notation, e.g. `"H7"`, `"g6"`, `"js4"`.
///
/// Returns upper and lower deviations in millimetres.
pub fn limits(size: f64, tolerance_class: &str) -> Result<Tolerance, Error> {
    let int_size = checked_size(size)?;
    let (hole, dev, idx_grade) = class_indices(tolerance_class)?;

    limits_nm(int_size, hole, dev, idx_grade)
        .map(|(upper, lower)| Tolerance::from_int(upper, lower))
        .ok_or_else(|| Error::UnsupportedCombination {
            tolerance_class: tolerance_class.to_owned(),
            size,
        })
}

// Resolve a tolerance class to whether it is a hole, and its two table indices.
fn class_indices(tolerance_class: &str) -> Result<(bool, usize, usize), Error> {
    let (deviation, grade) = parse_tolerance_class(tolerance_class)?;
    let hole = deviation.chars().next().unwrap().is_uppercase(); // unwraps because parsed ok
    let invalid = || Error::InvalidToleranceClass(tolerance_class.to_owned());

    let dev = dev_idx(deviation).ok_or_else(invalid)?;
    let idx_grade = GRADE_MAP
        .iter()
        .position(|&g| g == grade)
        .ok_or_else(invalid)?
        + 1;

    Ok((hole, dev, idx_grade))
}

// Validate a nominal size and reduce it to the integer used for bracket lookups.
// Brackets are upper bound inclusive, so a size of 10.1 falls in the "over 10 to 18" row.
fn checked_size(size: f64) -> Result<i32, Error> {
    if !size.is_finite() || size <= 0.0 {
        return Err(Error::SizeOutOfRange(size));
    }
    let int_size = size.ceil() as i32;
    if int_size > MAX_SIZE {
        return Err(Error::SizeOutOfRange(size));
    }
    Ok(int_size)
}

// Convert nanometre integer to millimetre float (final step)
fn nm_to_mm(d: i32) -> f64 {
    d as f64 / 1_000_000.0
}

// Convert a millimetre float to nanometres. Sizes reach 3150 mm, which overflows i32.
fn mm_to_nm(d: f64) -> i64 {
    (d * 1_000_000.0).round() as i64
}

// Convert a whole millimetre size bound to nanometres.
fn nm(d: i32) -> i64 {
    d as i64 * 1_000_000
}

// Convert a nanometre size back to millimetres.
fn nm_to_size(d: i64) -> f64 {
    d as f64 / 1_000_000.0
}

// Undo the doubling `band_error` works in, and convert to millimetres.
fn error_to_mm(d: i64) -> f64 {
    d as f64 / 2_000_000.0
}

// Convert table micrometre value to nanometres, filtering sentinel -1
fn um_to_nm(d: i32) -> Option<i32> {
    if d != -1 { Some(d * 1_000) } else { None }
}

// Index into DEVIATION_MAP for a deviation letter, case insensitive.
fn dev_idx(deviation: &str) -> Option<usize> {
    DEVIATION_MAP
        .iter()
        .position(|&d| d.eq_ignore_ascii_case(deviation))
}

// Column in DEVIATIONS_A_G for a deviation index. +1 because column 0 is the size bracket.
fn col_a_g(dev: usize) -> usize {
    dev + 1
}

// Column in DEVIATIONS_K_ZC for a deviation index. The table starts at k, which is
// DEVIATION_MAP index 13, in column 1.
fn col_k_zc(dev: usize) -> usize {
    dev - 12
}

// Standard tolerance (the IT value) in nanometres, or None where the table has no entry.
fn it_nm(int_size: i32, idx_grade: usize) -> Option<i32> {
    let row = STANDARD_TOLERANCE_GRADES
        .iter()
        .position(|&s| s[0] >= int_size)?;
    // Tolerance grades are stored in 1/10 um; convert to nanometres. -1 is the sentinel.
    let tolerance = *STANDARD_TOLERANCE_GRADES[row].get(idx_grade)?;
    if tolerance < 0 {
        None
    } else {
        Some(tolerance * 100)
    }
}

// Upper and lower deviations in nanometres, addressed by table index rather than by name.
// This is the hot path for the search functions; `limits` is the string facade over it.
fn limits_nm(int_size: i32, hole: bool, dev: usize, idx_grade: usize) -> Option<(i32, i32)> {
    let tolerance_nm = it_nm(int_size, idx_grade)?;
    if hole {
        lookup_hole(int_size, tolerance_nm, dev, idx_grade)
    } else {
        lookup_shaft(int_size, tolerance_nm, dev, idx_grade)
    }
}

// Render a tolerance class from its table indices, e.g. (true, 10, 8) -> "H7".
fn class_name(hole: bool, dev: usize, idx_grade: usize) -> String {
    let letters = DEVIATION_MAP[dev];
    let grade = GRADE_MAP[idx_grade - 1];
    if hole {
        format!("{}{grade}", letters.to_uppercase())
    } else {
        format!("{letters}{grade}")
    }
}

fn lookup_hole(size: i32, tol: i32, dev: usize, idx_grade: usize) -> Option<(i32, i32)> {
    match dev {
        ..=DEV_G => {
            let row = DEVIATIONS_A_G.iter().position(|&s| s[0] >= size)?;
            let deviation = um_to_nm(*DEVIATIONS_A_G[row].get(col_a_g(dev))?)?;
            if dev <= DEV_B && size == 1 {
                None
            } else {
                Some((deviation + tol, deviation))
            }
        }
        DEV_H => Some((tol, 0)),
        DEV_JS => Some((tol / 2, -(tol / 2))),
        DEV_J if (8..11).contains(&idx_grade) => {
            let row = UPPER_J.iter().position(|&s| s[0] >= size)?;
            let deviation = um_to_nm(*UPPER_J[row].get(idx_grade - 7)?)?;
            Some((deviation, deviation - tol))
        }
        DEV_K => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let deviation =
                -um_to_nm(*DEVIATIONS_K_ZC[row].get(col_k_zc(dev))?)? + delta(size, idx_grade);
            if idx_grade > 10 && size > 3 {
                None
            } else {
                Some((deviation, deviation - tol))
            }
        }
        DEV_M => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let mut deviation =
                -um_to_nm(*DEVIATIONS_K_ZC[row].get(col_k_zc(dev))?)? + delta(size, idx_grade);
            if idx_grade == 8 && size > 250 && size <= 315 {
                deviation += 2_000;
            }
            Some((deviation, deviation - tol))
        }
        DEV_N => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let deviation =
                -um_to_nm(*DEVIATIONS_K_ZC[row].get(col_k_zc(dev))?)? + delta(size, idx_grade);
            if idx_grade > 10 && (size > 500 || size <= 1) {
                None
            } else {
                Some((deviation, deviation - tol))
            }
        }
        DEV_P..=DEV_ZC => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let deviation = -um_to_nm(*DEVIATIONS_K_ZC[row].get(col_k_zc(dev))?)?
                + if idx_grade < 10 {
                    delta(size, idx_grade)
                } else {
                    0
                };
            Some((deviation, deviation - tol))
        }
        _ => None,
    }
}

fn lookup_shaft(size: i32, tol: i32, dev: usize, idx_grade: usize) -> Option<(i32, i32)> {
    match dev {
        ..=DEV_G => {
            let row = DEVIATIONS_A_G.iter().position(|&s| s[0] >= size)?;
            let deviation = -um_to_nm(*DEVIATIONS_A_G[row].get(col_a_g(dev))?)?;
            if dev <= DEV_B && size == 1 {
                None
            } else {
                Some((deviation, deviation - tol))
            }
        }
        DEV_H => Some((0, -tol)),
        DEV_JS => Some((tol / 2, -(tol / 2))),
        DEV_J if idx_grade > 6 && idx_grade < 11 => {
            let row = LOWER_J.iter().position(|&s| s[0] >= size)?;
            let deviation = -um_to_nm(*LOWER_J[row].get(idx_grade.max(8) - 7)?)?;
            Some((deviation + tol, deviation))
        }
        DEV_K => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let deviation = if idx_grade > 5 && idx_grade < 10 {
                um_to_nm(*DEVIATIONS_K_ZC[row].get(col_k_zc(dev))?)?
            } else {
                0
            };
            Some((deviation + tol, deviation))
        }
        DEV_M..=DEV_ZC => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let deviation = um_to_nm(*DEVIATIONS_K_ZC[row].get(col_k_zc(dev))?)?;
            Some((deviation + tol, deviation))
        }
        _ => None,
    }
}

fn delta(size: i32, grade: usize) -> i32 {
    if size > 500 || grade < 4 || grade > 9 {
        0
    } else {
        let idx = DELTA.iter().position(|&s| s[0] >= size).unwrap();
        100 * DELTA[idx][grade - 4]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delta_fn() {
        assert_eq!(delta(165, 7), 100 * DELTA[8][3]);
        assert_eq!(delta(19, 5), 100 * DELTA[4][1]);
        assert_eq!(delta(333, 8), 100 * DELTA[11][4]);
        assert_eq!(delta(38, 5), 100 * DELTA[5][1]);
    }
}
