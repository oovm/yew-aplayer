use wasm_bindgen::prelude::*;
use web_sys::Element;

mod options;

pub use options::{build_aplayer, APlayerAudio, APlayerOptions};

#[wasm_bindgen(module = "/src/aplayer.min.js")]
extern "C" {
    #[wasm_bindgen(js_name = build_aplayer)]
    pub fn create_aplayer(e: &Element, o: &JsValue);
}
