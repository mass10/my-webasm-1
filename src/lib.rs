extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let message = format!("{}", name);
   	alert(&message);
}

#[wasm_bindgen]
#[no_mangle]
pub fn wasm_to_uppercase(name: &str) -> String {
    return name.to_uppercase();
}
