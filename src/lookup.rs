use super::tables::{
    DELTA, DEVIATION_MAP, DEVIATIONS_A_G, DEVIATIONS_K_ZC, GRADE_MAP, LOWER_J,
    STANDARD_TOLERANCE_GRADES, UPPER_J,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tolerance {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

impl Tolerance {
    fn from_limits(upper: f64, lower: f64) -> Self {
        Self {
            upper,
            middle: (upper + lower) / 2.0,
            lower,
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
    let idx_dev = DEVIATION_MAP
        .iter()
        .position(|&d| d.eq_ignore_ascii_case(deviation))
        .ok_or_else(|| Error::InvalidToleranceClass(tolerance_class.to_owned()))?
        + 1;
    let idx_tol = STANDARD_TOLERANCE_GRADES
        .iter()
        .position(|&s| s[0] >= int_size)
        .ok_or(Error::SizeOutOfRange(size))?;

    let tolerance = *STANDARD_TOLERANCE_GRADES[idx_tol]
        .get(idx_grade)
        .ok_or(Error::SizeOutOfRange(size))?
        * 100;
    if tolerance == -100 {
        return Err(Error::UnsupportedCombination {
            tolerance_class: tolerance_class.to_owned(),
            size,
        });
    }

    let result = if hole {
        lookup_hole(int_size, tolerance, idx_dev, idx_grade)
    } else {
        lookup_shaft(int_size, tolerance, idx_dev, idx_grade)
    };

    result.ok_or_else(|| Error::UnsupportedCombination {
        tolerance_class: tolerance_class.to_owned(),
        size,
    })
}

// Convert nanometre integer to millimetre float.
fn flt(d: i32) -> f64 {
    d as f64 / 1_000_000.0
}

// Retrieve lookup value, filtering sentinel -1, converting micrometre to nanometre.
fn rtv(d: i32) -> Option<i32> {
    if d != -1 { Some(d * 1000) } else { None }
}

fn lookup_hole(size: i32, tol: i32, idx_dev: usize, idx_grade: usize) -> Option<Tolerance> {
    if (0..11).contains(&idx_dev) {
        // A to G
        let idx_size = DEVIATIONS_A_G.iter().position(|&s| s[0] >= size)?;
        let dev = rtv(*DEVIATIONS_A_G[idx_size].get(idx_dev)?)?;
        if (idx_dev == 1 || idx_dev == 2) && size == 1 {
            None
        } else {
            Some(Tolerance::from_limits(flt(dev + tol), flt(dev)))
        }
    } else if idx_dev == 11 {
        // H
        Some(Tolerance::from_limits(flt(tol), 0.0))
    } else if idx_dev == 12 {
        // JS
        Some(Tolerance::from_limits(flt(tol / 2), -flt(tol / 2)))
    } else if idx_dev == 13 && (8..11).contains(&idx_grade) {
        // J
        let idx_size = UPPER_J.iter().position(|&s| s[0] >= size)?;
        let dev = rtv(*UPPER_J[idx_size].get(idx_grade - 7)?)?;
        Some(Tolerance::from_limits(flt(dev), flt(dev - tol)))
    } else if idx_dev == 14 {
        // K
        let idx_size = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
        let dev = -rtv(*DEVIATIONS_K_ZC[idx_size].get(idx_dev - 13)?)? + delta(size, idx_grade);
        if idx_grade > 10 && size > 3 {
            None
        } else {
            Some(Tolerance::from_limits(flt(dev), flt(dev - tol)))
        }
    } else if idx_dev == 15 {
        // M
        let idx_size = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
        let mut dev = -rtv(*DEVIATIONS_K_ZC[idx_size].get(idx_dev - 13)?)? + delta(size, idx_grade);
        if idx_grade == 8 && size > 250 && size <= 315 {
            dev += 2_000;
        }
        Some(Tolerance::from_limits(flt(dev), flt(dev - tol)))
    } else if idx_dev == 16 {
        // N
        let idx_size = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
        let dev = -rtv(*DEVIATIONS_K_ZC[idx_size].get(idx_dev - 13)?)? + delta(size, idx_grade);
        if (idx_grade > 10 && size > 500) || (idx_grade > 10 && size <= 1) {
            None
        } else {
            Some(Tolerance::from_limits(flt(dev), flt(dev - tol)))
        }
    } else if (17..30).contains(&idx_dev) {
        // P to ZC
        let idx_size = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
        let dev = -rtv(*DEVIATIONS_K_ZC[idx_size].get(idx_dev - 13)?)?
            + if idx_grade < 10 {
                delta(size, idx_grade)
            } else {
                0
            };
        Some(Tolerance::from_limits(flt(dev), flt(dev - tol)))
    } else {
        None
    }
}

fn lookup_shaft(size: i32, tol: i32, idx_dev: usize, idx_grade: usize) -> Option<Tolerance> {
    if (0..11).contains(&idx_dev) {
        // a to g
        let idx_size = DEVIATIONS_A_G.iter().position(|&s| s[0] >= size)?;
        let dev = -rtv(*DEVIATIONS_A_G[idx_size].get(idx_dev)?)?;
        if dev == -1 || ((idx_dev == 1 || idx_dev == 2) && size == 1) {
            None
        } else {
            Some(Tolerance::from_limits(flt(dev), flt(dev - tol)))
        }
    } else if idx_dev == 11 {
        // h
        Some(Tolerance::from_limits(0.0, -flt(tol)))
    } else if idx_dev == 12 {
        // js
        Some(Tolerance::from_limits(flt(tol / 2), -flt(tol / 2)))
    } else if idx_dev == 13 && idx_grade > 6 && idx_grade < 11 {
        // j
        let idx_size = LOWER_J.iter().position(|&s| s[0] >= size)?;
        let dev = -rtv(*LOWER_J[idx_size].get(idx_grade.max(8) - 7)?)?;
        Some(Tolerance::from_limits(flt(dev + tol), flt(dev)))
    } else if idx_dev == 14 {
        // k
        let idx_size = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
        let dev = if idx_grade > 5 && idx_grade < 10 {
            rtv(*DEVIATIONS_K_ZC[idx_size].get(idx_dev - 13)?)?
        } else {
            0
        };
        Some(Tolerance::from_limits(flt(dev + tol), flt(dev)))
    } else if (15..28).contains(&idx_dev) {
        // m to zc
        let idx_size = DEVIATIONS_K_ZC.iter().position(|&s| s[0] >= size)?;
        let dev = rtv(*DEVIATIONS_K_ZC[idx_size].get(idx_dev - 13)?)?;
        Some(Tolerance::from_limits(flt(dev + tol), flt(dev)))
    } else {
        None
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
