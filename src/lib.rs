mod parser;
mod utils;

extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

// use crate::parser::*;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// A macro target_scale provide `log!(..)`-style syntax for `console.log` logging.
// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    utils::set_panic_hook();
    parser::parse(input.to_string())
}
