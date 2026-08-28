# Algorithm & results

## Scoring

Each guess is scored by the sum of squares of pattern-bucket sizes over the
current candidates: `Σ nᵢ²`. Minimizing this is equivalent to maximizing
Shannon entropy `H = -Σ p·log p` over the same distribution, but needs zero
floating point or log calls, so it is exact and fast. The first turn is
short-circuited to a precomputed optimal opener rather than re-searching the
full answer set.

## Results

Exhaustive play over **all 2,315 answer words** (`cargo test -- --ignored`):

| metric          | value                     |
| --------------- | ------------------------- |
| Success rate    | **100%** (2315/2315)      |
| Max guesses     | **5** (Wordle limit is 6) |
| Average guesses | **3.656**                 |

## Design notes

- **Pattern evaluation** is on-the-fly and bit-parallel (O(1) per word pair),
  instead of a precomputed ~30 MB guess×answer table. This keeps the WASM
  download tiny. A `pattern-table` feature can add the full table for native
  benchmarking if it ever pays off.
- **Scoring** uses integer `Σ nᵢ²` rather than the literal Shannon formula;
  see above — they choose the same guess ordering. A `shannon` feature keeps
  the literal formula for cross-checking.
- **Guess space** is always the full legal list, even when few candidates
  remain: with a small set a discriminating non-answer word (e.g. splitting
  the `-ound` family) can be required, and scoring is cheap there anyway.