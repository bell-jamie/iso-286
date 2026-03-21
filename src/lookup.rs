use super::tables::{
    DELTA, DEVIATION_MAP, DEVIATIONS_A_G, DEVIATIONS_K_ZC, GRADE_MAP, LOWER_J,
    STANDARD_TOLERANCE_GRADES, UPPER_J,
};
use wasm_bindgen::prelude::*;

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

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidToleranceClass(String),
    SizeOutOfRange(f64),
    UnsupportedCombination { tolerance_class: String, size: f64 },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidToleranceClass(s) => write!(f, "invalid tolerance class: {s:?}"),
            Error::SizeOutOfRange(s) => write!(f, "nominal size {s} mm is out of range"),
            Error::UnsupportedCombination {
                tolerance_class,
                size,
            } => write!(f, "unsupported combination: {tolerance_class} at {size} mm"),
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
pub fn grades() -> &'static [&'static str] {
    GRADE_MAP.as_slice()
}

/// Available hole deviation letters (uppercase).
pub fn hole_deviations() -> Vec<String> {
    DEVIATION_MAP.iter().map(|d| d.to_uppercase()).collect()
}

/// Available shaft deviation letters (lowercase).
pub fn shaft_deviations() -> &'static [&'static str] {
    DEVIATION_MAP.as_slice()
}

/// Compute ISO 286 tolerance limits for a given nominal size and tolerance class.
///
/// `size` is the nominal dimension in millimetres.
/// `tolerance_class` is the standard notation, e.g. `"H7"`, `"g6"`, `"js4"`.
///
/// Returns upper and lower deviations in millimetres.
pub fn limits(size: f64, tolerance_class: &str) -> Result<Tolerance, Error> {
    let (deviation, grade) = parse_tolerance_class(tolerance_class)?;
    let hole = deviation.chars().next().unwrap().is_uppercase();
    let int_size = size.ceil() as i32;

    let idx_grade = GRADE_MAP
        .iter()
        .position(|&g| g == grade)
        .ok_or_else(|| Error::InvalidToleranceClass(tolerance_class.to_owned()))?
        + 1;
    let idx_tol = STANDARD_TOLERANCE_GRADES
        .iter()
        .position(|&s| s[0] >= int_size)
        .ok_or(Error::SizeOutOfRange(size))?;

    // Tolerance grades are stored in 1/10 µm; convert to nanometres
    // Sentinel -1 becomes -100
    let tolerance_nm = *STANDARD_TOLERANCE_GRADES[idx_tol]
        .get(idx_grade)
        .ok_or(Error::SizeOutOfRange(size))?
        * 100;
    if tolerance_nm == -100 {
        return Err(Error::UnsupportedCombination {
            tolerance_class: tolerance_class.to_owned(),
            size,
        });
    }

    let result = if hole {
        lookup_hole(int_size, tolerance_nm, deviation, idx_grade)
    } else {
        lookup_shaft(int_size, tolerance_nm, deviation, idx_grade)
    };

    result.ok_or_else(|| Error::UnsupportedCombination {
        tolerance_class: tolerance_class.to_owned(),
        size,
    })
}

// Convert nanometre integer to millimetre float (final step)
fn nm_to_mm(d: i32) -> f64 {
    d as f64 / 1_000_000.0
}

// Convert table micrometre value to nanometres, filtering sentinel -1
fn um_to_nm(d: i32) -> Option<i32> {
    if d != -1 { Some(d * 1_000) } else { None }
}

// Column index in DEVIATIONS_A_G or DEVIATIONS_K_ZC for a given deviation letter.
// +1 because column 0 is the size bracket.
fn dev_col(deviation: &str) -> Option<usize> {
    DEVIATION_MAP
        .iter()
        .position(|&d| d.eq_ignore_ascii_case(deviation))
        .map(|i| i + 1)
}

fn lookup_hole(size: i32, tol: i32, deviation: &str, idx_grade: usize) -> Option<Tolerance> {
    let col = dev_col(deviation)?;

    match deviation {
        "A" | "B" | "C" | "CD" | "D" | "E" | "EF" | "F" | "FG" | "G" => {
            let row = DEVIATIONS_A_G.iter().position(|&s| s[0] >= size)?;
            let dev = um_to_nm(*DEVIATIONS_A_G[row].get(col)?)?;
            if matches!(deviation, "A" | "B") && size == 1 {
                None
            } else {
                Some(Tolerance::from_int(dev + tol, dev))
            }
        }
        "H" => Some(Tolerance::from_int(tol, 0)),
        "JS" => Some(Tolerance::from_int(tol / 2, -(tol / 2))),
        "J" if (8..11).contains(&idx_grade) => {
            let row = UPPER_J.iter().position(|&s| s[0] >= size)?;
            let dev = um_to_nm(*UPPER_J[row].get(idx_grade - 7)?)?;
            Some(Tolerance::from_int(dev, dev - tol))
        }
        "K" => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let dev = -um_to_nm(*DEVIATIONS_K_ZC[row].get(col - 13)?)? + delta(size, idx_grade);
            if idx_grade > 10 && size > 3 {
                None
            } else {
                Some(Tolerance::from_int(dev, dev - tol))
            }
        }
        "M" => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let mut dev = -um_to_nm(*DEVIATIONS_K_ZC[row].get(col - 13)?)? + delta(size, idx_grade);
            if idx_grade == 8 && size > 250 && size <= 315 {
                dev += 2_000;
            }
            Some(Tolerance::from_int(dev, dev - tol))
        }
        "N" => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let dev = -um_to_nm(*DEVIATIONS_K_ZC[row].get(col - 13)?)? + delta(size, idx_grade);
            if (idx_grade > 10 && size > 500) || (idx_grade > 10 && size <= 1) {
                None
            } else {
                Some(Tolerance::from_int(dev, dev - tol))
            }
        }
        "P" | "R" | "S" | "T" | "U" | "V" | "X" | "Y" | "Z" | "ZA" | "ZB" | "ZC" => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let dev = -um_to_nm(*DEVIATIONS_K_ZC[row].get(col - 13)?)?
                + if idx_grade < 10 {
                    delta(size, idx_grade)
                } else {
                    0
                };
            Some(Tolerance::from_int(dev, dev - tol))
        }
        _ => None,
    }
}

fn lookup_shaft(size: i32, tol: i32, deviation: &str, idx_grade: usize) -> Option<Tolerance> {
    let col = dev_col(deviation)?;

    match deviation {
        "a" | "b" | "c" | "cd" | "d" | "e" | "ef" | "f" | "fg" | "g" => {
            let row = DEVIATIONS_A_G.iter().position(|&s| s[0] >= size)?;
            let dev = -um_to_nm(*DEVIATIONS_A_G[row].get(col)?)?;
            if matches!(deviation, "a" | "b") && size == 1 {
                None
            } else {
                Some(Tolerance::from_int(dev, dev - tol))
            }
        }
        "h" => Some(Tolerance::from_int(0, -tol)),
        "js" => Some(Tolerance::from_int(tol / 2, -(tol / 2))),
        "j" if idx_grade > 6 && idx_grade < 11 => {
            let row = LOWER_J.iter().position(|&s| s[0] >= size)?;
            let dev = -um_to_nm(*LOWER_J[row].get(idx_grade.max(8) - 7)?)?;
            Some(Tolerance::from_int(dev + tol, dev))
        }
        "k" => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let dev = if idx_grade > 5 && idx_grade < 10 {
                um_to_nm(*DEVIATIONS_K_ZC[row].get(col - 13)?)?
            } else {
                0
            };
            Some(Tolerance::from_int(dev + tol, dev))
        }
        "m" | "n" | "p" | "r" | "s" | "t" | "u" | "v" | "x" | "y" | "z" | "za" | "zb" | "zc" => {
            let row = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
            let dev = um_to_nm(*DEVIATIONS_K_ZC[row].get(col - 13)?)?;
            Some(Tolerance::from_int(dev + tol, dev))
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
