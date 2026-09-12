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
pub fn find_preferred(size: f64, tolerance_class: &str) -> Result<String, JsError> {
    lookup::find_preferred(size, tolerance_class).map_err(|e| JsError::new(&e.to_string()))
}
