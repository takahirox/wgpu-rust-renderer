// Non-Wasm

#[cfg(not(target_arch = "wasm32"))]
pub fn log(s: &str) {
	println!("{}", s);
}

// Wasm

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	pub fn log(s: &str);
}
