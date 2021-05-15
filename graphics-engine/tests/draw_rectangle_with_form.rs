//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate graphics_engine;
use graphics_engine::form::Form;
use web_sys::{window};


wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let canvas = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("root");
}
