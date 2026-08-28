# Building from source

## Prerequisites

- Rust stable toolchain (see `rust-toolchain.toml`)
- [wasm-pack] (for the wasm build)
- Node 18+ (dev only; the smoke test)

## Commands

```bash
cargo test                # unit + 200-answer sample (fast)
cargo test --release -- --ignored   # exhaustive 2,315-answer suite
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo bench               # solver throughput
cargo run --release --bin play          # play a live game against the CLI
cargo run --release --bin best_opener   # recompute the precomputed opener
```

### Building the wasm

```bash
npm run build             # wasm-pack build --target web --release --out-dir pkg --features wasm
node scripts/smoke.mjs    # boots the built wasm in Node and checks the bindings
```

Output goes to `pkg/` (gitignored): the loader `wordle_solver.js`, the
engine `wordle_solver_bg.wasm`, generated types, and a `package.json` — the
same manifest `npm pack ./pkg` turns into a release tarball.

## Version bumps

The version is read from `Cargo.toml` and propagates: wasm-pack writes it
into `pkg/package.json`, which names the tarball. A release tag must match it
exactly — the release workflow enforces this. See [Releases](RELEASE.md).

## Word lists

The canonical NYT Wordle answer list (2,315) and legal guess list (12,972)
are vendored under `data/` (provenance: the widely-mirrored `wordle-words`
npm package).

[wasm-pack]: https://rustwasm.github.io/wasm-pack/