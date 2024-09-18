mod parser;
mod utils;

extern crate cfg_if;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    utils::set_panic_hook();
    parser::parse(input.to_string())
}
