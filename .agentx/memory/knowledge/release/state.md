---
agentx:
  kind: state
  scope: knowledge/release
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T02:45:00+01:00
  topics:
    - release
    - release-plz
    - ci
    - repository-move
  supersedes: []
  superseded_by: []
---
# release: state

## Purpose And Usage

Read before releasing, moving the `dxn_dtcg` tag or editing CI. `docs/releasing.md` is canonical; this
records the shape and what changed since it was written.

## Current Technical Shape

- A release is a `vX.Y.Z` tag on `main` with a GitHub Release; nothing goes to crates.io
  (`git_only = true`, `publish = false`). `v0.1.0` was released on 2026-11-16, `v0.2.0` (the rename to
  `rayx_dtcg`, made by hand) on 2026-11-17.
- release-plz (`release-plz.toml`, `.github/workflows/release-plz.yml`) runs on pushes to `main`:
  `release` tags a `Cargo.toml` version that has no tag; `release-pr` keeps the release pull request.
  Only `feat`, `fix` and `perf` commits start a release; `chore(agentx)`, `chore(release)` and
  `chore(deps)` are left out of the changelog.
- 0.x policy: `feat`/`fix`/`perf` bump the patch; a breaking change bumps the minor and carries a
  `Migration:` footer.
- CI (`.github/workflows/ci.yml`) on Ubuntu, macOS and Windows with toolchain 1.97.0: `cargo fmt --check`,
  a guard against UI frameworks in the dependency tree (`nsh-`, `gpui`, `rayx` crates, except
  `rayx_dtcg` itself), `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`,
  `cargo package --workspace --list`.
- Work is pushed straight to `main` (rebase on `origin/main`, `--force-with-lease`); pull requests exist
  only for releases.
- Both release-plz jobs run only when `github.repository == 'rayx-ui/RayX.Dtcg'` (it named
  `dXnSix/dXn.DtcgNsh` until 2026-11-17, which skipped them after the transfer).
- `v0.1.0` was released as `dxn_dtcg_nsh` (command `dtcg-nsh`) from `dXnSix/dXn.DtcgNsh`; `v0.2.0` is
  the first release named `rayx_dtcg`, and `CHANGELOG.md`'s header names it too.
- `release-pr` works again: it failed from the rename until `v0.2.0` existed, because release-plz looked
  for `rayx_dtcg` in `v0.1.0`'s tree (`cannot find package`). The next releases go through release pull
  requests as usual.

## Backlinks

- [releasing guide](../../../../docs/releasing.md)
- [release-plz.toml](../../../../release-plz.toml)
- [release workflow](../../../../.github/workflows/release-plz.yml)
- [CI workflow](../../../../.github/workflows/ci.yml)
- [CHANGELOG](../../../../CHANGELOG.md)
- [workspace manifest](../../../../Cargo.toml)
- [crate manifest](../../../../crates/rayx_dtcg/Cargo.toml)
- 2026-11-17 current [release rules](rules.md)
- 2026-11-17 current [release hypotheses](hypotheses.md)
