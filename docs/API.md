# API

The JS surface is intentionally tiny. TypeScript definitions in
[`index.d.ts`](../index.d.ts) mirror it exactly.

## Loading

```js
import init, { createSolver } from "wordle-solver";

await init();              // resolves + fetches the wasm (browser)
await init(wasmBytes);     // explicit bytes (Node.js, see INTEGRATE.md)
```

- `init()` — async; resolves the wasm and instantiates it. Idempotent.
- `initSync(module)` — synchronous variant taking a `WebAssembly.Module` or
  bytes; blocked-thread use only.

## `createSolver()`

Returns a fresh `Solver` over the full answer set (2,315 words). Must be
called after `init()`.

## `solver` methods

| Method | Returns | Notes |
| --- | --- | --- |
| `suggestGuess()` | `string \| null` | Best next guess. `null` when no candidates remain. |
| `applyFeedback(guess, feedback)` | `void` | Records the real result; narrows candidates. Throws on invalid input or a feedback string that contradicts all remaining candidates. |
| `remainingCount()` | `number` | Words still consistent with all recorded feedback. |
| `remainingCandidates()` | `string[]` | The candidates (order not guaranteed). |
| `reset()` | `void` | Start over from the full answer set. |

## Feedback encoding

`feedback` is a 5-character string, one tile per guess letter, left → right:

| Char | Meaning |
| --- | --- |
| `B` | black (gray) — letter not in the word |
| `Y` | yellow — letter in the word, wrong position |
| `G` | green — letter in the word, right position |

```js
s.applyFeedback("roate", "BYBBG");
```

## Behavior notes

- The first guess is a precomputed optimal opener; later turns run a full
  information search over all legal guesses.
- Guesses are always drawn from the full 12,972-word legal list, even when
  few candidates remain — a discriminating non-answer word is sometimes the
  right play.
- If the official answer list is exhausted but legal guesses still fit, the
  solver falls back to those automatically.
- `applyFeedback` throws on: non-5-letter or unknown words, malformed
  feedback, and feedback that contradicts every remaining candidate.