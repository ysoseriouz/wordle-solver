# wordle-solver

High-performance Wordle solver compiled from Rust to WebAssembly. Suggest an
optimal next guess, then feed back the real Wordle result (gray/yellow/green)
to shrink the candidate set — repeat until solved.

- 🦀 Core is pure Rust; no runtime allocations in steady state.
- 🚀 First guess is a precomputed optimal opener (`roate`); later turns run a
  full maximum-information search over all 12,972 legal guesses.
- ⚛️ Works in any JS context (plain ESM, Vite, Astro, Node 18+).

## Quickstart

```bash
npm i wordle-solver
```

```js
import { createSolver } from "wordle-solver";

const s = createSolver();
console.log(s.suggestGuess()); // "roate"
s.applyFeedback("roate", "BYBBY"); // record the real result
console.log(s.remainingCount()); // how many words are still possible
console.log(s.remainingCandidates());
s.reset(); // start a new game
```

API:
`createSolver() → Solver`, `solver.suggestGuess() → string | null`,
`solver.applyFeedback(guess, feedback)`, `solver.remainingCount()`,
`solver.remainingCandidates()`, `solver.reset()`.

Feedback is 5 chars of `B`/`Y`/`G` (black · yellow · green), left → right.
`applyFeedback` throws on invalid input or on feedback that contradicts every
remaining candidate.

## Algorithm

Each guess is scored by the sum of squares of pattern-bucket sizes over the
current candidates: `Σ nᵢ²`. Minimizing this is equivalent to maximizing
Shannon entropy `H = -Σ p·log p` over the same distribution, but needs zero
floating point or log calls, so it is exact and fast. The first turn is
short-circuited to a precomputed optimal opener rather than re-searching a
full answer set.

## Results

Exhaustive play over **all 2,315 answer words** (cargo test -- --ignored):

| metric          | value                     |
| --------------- | ------------------------- |
| Success rate    | **100%** (2315/2315)      |
| Max guesses     | **5** (Wordle limit is 6) |
| Average guesses | **3.656**                 |

## Design notes

- **Pattern evaluation** is on-the-fly and bit-parallel (O(1) per word pair),
  instead of a precomputed ~30 MB guess×answer table. Grease match the whole
  word in registers, and the small persistent table is avoided — keeping the
  WASM download tiny. A `pattern-table` feature can add the full table for
  native benchmarking if it ever pays off.
- **Scoring** uses integer `Σ nᵢ²` rather than the literal Shannon formula;
  see "Algorithm" — they choose the same guess ordering. A `shannon` feature
  keeps the literal formula for cross-checking.
- **Guess space** is always the full legal list, even when few candidates
  remain: with a small set a discriminating non-answer word (e.g. splitting
  the `-ound` family) can be required, and scoring is cheap there anyway.

## Dev

```bash
cargo test                # unit + 200-answer sample (fast)
cargo test --release -- --ignored   # exhaustive 2315-answer suite
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo run --release --bin best_opener   # recompute the opening constant
npm run build             # wasm-pack → pkg/
```

Word list provenance: the canonical NYT Wordle answer list (2,315) and legal
guess list (12,972) as published in the widely-mirrored `wordle-words` npm
package; vendored under `data/`.
