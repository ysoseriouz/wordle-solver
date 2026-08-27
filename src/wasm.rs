//! `wasm-bindgen` bindings exposing the solver to JavaScript.
//!
//! Compiled only with the `wasm` feature; native tests/benches skip it. The
//! JS surface is intentionally tiny (see index.d.ts). Validation lives at this
//! trust boundary: never trust strings from JS.

use crate::solver::Solver;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WordleSolver {
    inner: Solver,
}

#[wasm_bindgen]
impl WordleSolver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WordleSolver {
        WordleSolver {
            inner: Solver::new(),
        }
    }

    /// Best next guess, or `null` when no candidates remain.
    #[wasm_bindgen(js_name = suggestGuess)]
    pub fn suggest_guess(&mut self) -> Option<String> {
        self.inner.suggest_guess()
    }

    #[wasm_bindgen(js_name = applyFeedback)]
    pub fn apply_feedback(&mut self, guess: &str, feedback: &str) -> Result<(), JsError> {
        self.inner
            .apply_feedback(guess, feedback)
            .map_err(|e| JsError::new(&e.to_string()))
    }

    #[wasm_bindgen(js_name = remainingCount)]
    pub fn remaining_count(&self) -> u32 {
        self.inner.remaining_count()
    }

    #[wasm_bindgen(js_name = remainingCandidates)]
    pub fn remaining_candidates(&self) -> Vec<String> {
        self.inner.remaining_words()
    }

    #[wasm_bindgen(js_name = reset)]
    pub fn reset(&mut self) {
        self.inner.reset()
    }
}

impl Default for WordleSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen(js_name = createSolver)]
pub fn create_solver() -> WordleSolver {
    // Panics surface as a JS exception; data is validated at init.
    let inner = Solver::new();
    WordleSolver { inner }
}
