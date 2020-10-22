use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen(module = "/src/aplayer.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = build_aplayer)]
    pub fn build_aplayer(e: &Element, expr: &str);
}
