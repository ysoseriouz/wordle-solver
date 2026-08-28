# Releasing

One command, everything else is CI. Releases are tagged from the tip of
`main`.

## Cut a release

1. Bump the version in `Cargo.toml` (semver) and commit:

   ```bash
   cargo set-version x.y.z     # or edit Cargo.toml manually
   git commit -am "chore: release vx.y.z"
   ```

2. Tag and push — the tag name must match the Cargo.toml version:

   ```bash
   git tag vx.y.z
   git push origin vx.y.z
   ```

3. That's it. The [release workflow](../.github/workflows/release.yml)
   builds the wasm, packs the npm tarball, and creates the GitHub Release.

## What CI does

1. **Verifies the tag** matches `Cargo.toml` (fails fast on a mismatch).
2. Builds the wasm (`wasm-pack build --target web --release`).
3. Packs it: `npm pack ./pkg` → `wordle-solver-x.y.z.tgz`.
4. Creates the release and attaches the tarball, with **auto-generated
   release notes** (GitHub's built-in changelog: every merged PR or squashed
   commit since the last tag, grouped by conventional-commit prefix).

## Changelog quality is free — use conventional commits

GitHub's auto-generated notes group by `feat:` / `fix:` / `docs:` / ... and
flag breaking changes. To get useful notes:

- Squash-merge PRs with descriptive conventional-commit titles:
  `feat: add applyFeedback to CLI`, `fix: reject malformed feedback`, ...
- Keep unrelated changes in separate PRs where feasible.
- Reference a breaking change with `feat!: ...` or a `BREAKING CHANGE:`
  footer in the PR body.

The generated release body *is* the changelog. There is no separate
CHANGELOG.md to maintain.

## Consumers

Once the release is up, the [install docs](INSTALL.md) URL pattern applies:

```
https://github.com/ysoseriouz/wordle-solver/releases/download/vx.y.z/wordle-solver-x.y.z.tgz
```

## Re-running / fixing a bad release

The workflow creates the release from the tag, so:

1. Delete the release and the tag:

   ```bash
   gh release delete vx.y.z --cleanup-tag --yes
   git push origin :refs/tags/vx.y.z
   ```

2. Fix, recommit, retag, push. (Version numbers are cheap — prefer cutting
   `vx.y.z+1` over reusing one when the fix itself changed code.)