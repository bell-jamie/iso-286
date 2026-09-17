use wasm_bindgen::prelude::*;

use super::lookup;

#[wasm_bindgen]
pub fn limits(size: f64, tolerance_class: &str) -> Result<lookup::Tolerance, JsError> {
    lookup::limits(size, tolerance_class).map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen]
pub fn grades() -> Vec<String> {
    lookup::grades()
}

#[wasm_bindgen(js_name = "holeDeviations")]
pub fn hole_deviations() -> Vec<String> {
    lookup::hole_deviations()
}

#[wasm_bindgen(js_name = "shaftDeviations")]
pub fn shaft_deviations() -> Vec<String> {
    lookup::shaft_deviations()
}

#[wasm_bindgen(js_name = "holePreferredTolerances")]
pub fn hole_preferred_tolerances() -> Vec<String> {
    lookup::hole_preferred_tolerances()
}

#[wasm_bindgen(js_name = "shaftPreferredTolerances")]
pub fn shaft_preferred_tolerances() -> Vec<String> {
    lookup::shaft_preferred_tolerances()
}

#[wasm_bindgen(js_name = "findPreferred")]
pub fn find_preferred(size: f64, tolerance_class: &str) -> Result<lookup::Match, JsError> {
    lookup::find_preferred(size, tolerance_class).map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen(js_name = "listPreferred")]
pub fn list_preferred(size: f64, tolerance_class: &str) -> Result<Vec<lookup::Match>, JsError> {
    lookup::list_preferred(size, tolerance_class).map_err(|e| JsError::new(&e.to_string()))
}

/// `results` and `decimals` are signed because JavaScript has no unsigned integer to
/// pass: a negative number reaches an unsigned parameter as a very large positive one,
/// silently, so it has to be caught on this side.
#[wasm_bindgen(js_name = "listClosest")]
pub fn list_closest(
    size: f64,
    upper: f64,
    lower: f64,
    feature: lookup::Feature,
    strict: bool,
    results: i32,
    decimals: i32,
) -> Result<Vec<lookup::Match>, JsError> {
    let results = usize::try_from(results)
        .map_err(|_| JsError::new(&format!("results must not be negative, got {results}")))?;
    let decimals = usize::try_from(decimals)
        .map_err(|_| JsError::new(&format!("decimals must not be negative, got {decimals}")))?;

    lookup::list_closest(size, upper, lower, feature, strict, results, decimals)
        .map_err(|e| JsError::new(&e.to_string()))
}
