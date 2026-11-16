# dXn.DtcgNsh contributor guidance

Exports DTCG 2025.10 projects as NSH theme documents: `crates/dxn_dtcg_nsh` (library `export` and the `dtcg-nsh`
command). It depends on `dxn_dtcg` for resolution and on nothing of NSH: the theme document's format is NSH's
(`nsh-root/_projects/theme-collections/theme-document.md` in the NSH fork's workspace), written as JSON.

## Validation

Run before committing:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Each feature has its own integration-test group in `crates/dxn_dtcg_nsh/tests/<feature>.rs`; select one with
`cargo test --test <feature>`. A new feature or behavior change gets a new group.

## Dependencies

`dxn_dtcg` is pinned to a dXn.Dtcg release tag (`v1.4.0`) in `crates/dxn_dtcg_nsh/Cargo.toml`, not the workspace
table, so that moving it releases this crate. Move to a new release by changing the tag in a `fix(deps):` commit (or
`feat(deps):` when the export uses something new). A change this crate needs from dXn.Dtcg is released there first.

## Commits

Conventional Commits with meaningful scopes. Never add Claude co-author lines.

## Releases

release-plz tags `vX.Y.Z` (Git tags only, no crates.io) and writes `CHANGELOG.md`, which is never edited by hand.
0.x policy: `fix`, `perf` and `feat` → patch; a breaking change (`!` or a `BREAKING CHANGE:` footer) → minor, with
the break stated in the body and a `Migration:` footer. A release is merging release-plz's pull request after checking
its version and that it changes only `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md`. Push straight to `main` with
`git fetch origin && git rebase origin/main`, then `git push --force-with-lease origin main`; pull requests exist only
for releases. See [docs/releasing.md](docs/releasing.md).
