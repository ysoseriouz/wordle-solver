# Installing

The solver ships as an npm tarball attached to each GitHub Release. A tarball
from a URL works with **npm, bun, pnpm, and yarn** alike — no registry
account needed.

## As a dependency (recommended)

In `package.json`:

```json
{
  "dependencies": {
    "wordle-solver": "https://github.com/ysoseriouz/wordle-solver/releases/download/v0.1.0/wordle-solver-0.1.0.tgz"
  }
}
```

Then install with your package manager (`npm install`, `bun install`, ...).
Your lockfile pins the tarball URL **and** its integrity hash, so an
unexpectedly replaced asset fails the install instead of silently changing
code.

The package name inside the tarball is `wordle-solver`; it does not clash
with the npm registry package of the same name because it never touches the
registry.

### Upgrading

The version lives in the URL. Edit it and reinstall:

```bash
npm install wordle-solver@https://github.com/ysoseriouz/wordle-solver/releases/download/v0.1.1/wordle-solver-0.1.1.tgz
# or: edit package.json, then `npm install`
```

## Without a package manager

Download `wordle-solver-<version>.tgz` from the [releases page] and unpack it
with any tar tool:

```bash
tar -xzf wordle-solver-0.1.0.tgz   # extracts to ./package/
```

It contains `wordle_solver.js` (the loader), `wordle_solver_bg.wasm` (the
engine), and `wordle_solver.d.ts` (types). Drop them into your project and
import relatively — see [Integrating](INTEGRATE.md).

[releases page]: https://github.com/ysoseriouz/wordle-solver/releases

## From source

Clone the repo and build yourself — see [Building](BUILD.md).