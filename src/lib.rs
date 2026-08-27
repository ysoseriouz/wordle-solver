//! wordle-solver: high-performance Wordle solver, native and WASM targets.

pub mod bitset;
pub mod entropy;
pub mod evaluator;
pub mod pattern;
pub mod solver;
pub mod word;
pub mod wordlist;

pub use solver::{Error, Solver};
pub use word::Word;

#[cfg(feature = "wasm")]
pub mod wasm;
