//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use tortuga_site::run;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn math() {
    assert_eq!(Ok("2".to_string()), run("1 + 1"));
}
