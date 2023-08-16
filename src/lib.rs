use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) {
  print!("Hi there {name}");
}

// wasm-pack build --target web