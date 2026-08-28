# Integrating

The wasm is built against wasm-bindgen's **web target**. The loader module
(`wordle_solver.js`) resolves its `.wasm` via
`new URL("wordle_solver_bg.wasm", import.meta.url)`, so any bundler (Vite,
Astro, Rollup, webpack) emits the wasm as an asset, rewrites the path, and
serves it — nothing to configure.

## Browser (Vite / Astro / any bundler)

```js
import init, { createSolver } from "wordle-solver";

await init(); // fetch + instantiate the wasm; safe to call multiple times (no-op)
const s = createSolver();
```

The module is a singleton: `init()` again after the first call is a no-op, so
call it once per page, e.g. at module top level or in a `useEffect`/`onMount`.

## Node.js (ESM, 18+)

Node's `fetch` does not support `file:` URLs, so pass the wasm bytes
explicitly instead of letting `init()` fetch:

```js
import { readFileSync } from "node:fs";
import init, { createSolver } from "wordle-solver";

await init(readFileSync(new URL("./wordle_solver_bg.wasm", import.meta.url)));
const s = createSolver();
```

## MIME type

The server must serve `.wasm` as `application/wasm` (the
`WebAssembly.instantiateStreaming` fast path needs it). Cloudflare Pages,
GitHub Pages, and dev servers (Vite, `astro dev`) all do this automatically.
If the MIME is wrong, the loader logs a warning and falls back to a slower
`instantiate` — it still works.

## Common issues

- **"wasm has not been initialized"** — you called `createSolver()` before
  `await init()`.
- **Failed to fetch the wasm** — the page and the `.wasm` must be same-origin;
  do not copy the loader to a different origin than the wasm asset.
- **Astro/Vite pre-bundling complains** about the dependency — exclude it:

  ```js
  // astro.config.mjs
  export default defineConfig({
    vite: { optimizeDeps: { exclude: ["wordle-solver"] } },
  });
  ```

The wasm is ~58 KB gzipped and mounts instantly; lazy-loading it is optional.