# wordle-solver

High-performance Wordle solver compiled from Rust to WebAssembly. Suggest an
optimal next guess, then feed back the real Wordle result (gray/yellow/green)
to shrink the candidate set — repeat until solved.

- 🦀 Core is pure Rust; no runtime allocations in steady state.
- 🚀 First guess is a precomputed optimal opener (`roate`); later turns run a
  full maximum-information search over all 12,972 legal guesses.
- ⚛️ Works in any JS context (plain ESM, Vite, Astro, Node 18+).
- 📦 Distributed as an npm tarball on every release — no registry account.

## Install

```json
{
  "dependencies": {
    "wordle-solver": "https://github.com/ysoseriouz/wordle-solver/releases/download/v0.1.0/wordle-solver-0.1.0.tgz"
  }
}
```

See [docs/INSTALL.md](docs/INSTALL.md) for npm, bun, pnpm, yarn, direct
download, and building from source.

## Quickstart

```js
import init, { createSolver } from "wordle-solver";

await init(); // load the wasm (one-time)
const s = createSolver();
console.log(s.suggestGuess()); // "roate"
s.applyFeedback("roate", "BYBBY"); // record the real result
console.log(s.remainingCount()); // how many words are still possible
s.reset(); // start a new game
```

Feedback is 5 chars of `B`/`Y`/`G` (black · yellow · green), left → right.
Full API reference: [docs/API.md](docs/API.md).

## CLI

Play a live Wordle game on a real board while the solver proposes the next
guess:

```bash
cargo run --release --bin play
```

Each turn it prints a guess; type the board's real 5-char feedback
(`B`/`Y`/`G`) and it narrows the candidates. Empty input means the guess was
the answer. Maxes at 5 guesses, within Wordle's 6-guess limit.

## Docs

| Doc | For |
| --- | --- |
| [docs/INSTALL.md](docs/INSTALL.md) | Installing (package managers, direct, source) |
| [docs/INTEGRATE.md](docs/INTEGRATE.md) | Wiring the wasm into your site (Vite, Astro, Node, MIME) |
| [docs/API.md](docs/API.md) | JS API reference |
| [docs/ALGORITHM.md](docs/ALGORITHM.md) | Scoring, results, design notes |
| [docs/BUILD.md](docs/BUILD.md) | Building, testing, benchmarking |
| [docs/RELEASE.md](docs/RELEASE.md) | Cutting a release (maintainers) |

## License

MIT — see [LICENSE](LICENSE).