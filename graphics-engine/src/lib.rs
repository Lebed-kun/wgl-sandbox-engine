use wasm_bindgen::prelude::*;

mod types;
#[macro_use]
mod utils;
pub mod program;
pub mod form;

pub mod examples;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn example() {
    examples::draw_rectangles_with_form::example();
}
