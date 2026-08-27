//! wasm-bindgen tests, run with: `wasm-pack test --node --features wasm`.
//!
//! Compiled only for the wasm32 target so native `cargo test` ignores it.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::wasm_bindgen_test;
use wordle_solver::wasm;

#[wasm_bindgen_test]
fn create_solver_and_suggest() {
    let mut s = wasm::create_solver();
    let guess = s.suggest_guess().unwrap();
    assert_eq!(guess, "roate");
    assert_eq!(s.remaining_count(), 2_315);
}

#[wasm_bindgen_test]
fn guess_reduces_candidates() {
    let mut s = wasm::create_solver();
    let before = s.remaining_count();
    s.apply_feedback("roate", "BBBYB").unwrap();
    assert!(s.remaining_count() < before);
    assert!(s.remaining_count() > 0);
    s.reset();
    assert_eq!(s.remaining_count(), 2_315);
}

#[wasm_bindgen_test]
fn invalid_input_errs() {
    let mut s = wasm::create_solver();
    // not a valid guess
    assert!(s.apply_feedback("zzzzz", "BBBBB").is_err());
    // bad feedback shape
    assert!(s.apply_feedback("roate", "ZZXY").is_err());
    // uppercase guess
    assert!(s.apply_feedback("ROATE", "BBBBB").is_err());
}
