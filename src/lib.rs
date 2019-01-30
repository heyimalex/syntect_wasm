extern crate cfg_if;
extern crate wasm_bindgen;
extern crate syntect;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet};
use syntect::html::highlighted_html_for_string;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn highlight_rust(input: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ss.find_syntax_by_extension("rs").unwrap();
    let theme = &ts.themes["base16-ocean.dark"];
    return highlighted_html_for_string(input, &ss, syntax, theme);
}
