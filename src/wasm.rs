use wasm_bindgen::prelude::*;

use super::lookup;

#[wasm_bindgen]
pub fn limits(size: f64, tolerance_class: &str) -> Result<lookup::Tolerance, JsError> {
    lookup::limits(size, tolerance_class).map_err(|e| JsError::new(&e.to_string()))
}

#[wasm_bindgen]
pub fn grades() -> Vec<String> {
    lookup::grades().iter().map(|s| s.to_string()).collect()
}

#[wasm_bindgen(js_name = "holeDeviations")]
pub fn hole_deviations() -> Vec<String> {
    lookup::hole_deviations()
}

#[wasm_bindgen(js_name = "shaftDeviations")]
pub fn shaft_deviations() -> Vec<String> {
    lookup::shaft_deviations().iter().map(|s| s.to_string()).collect()
}
