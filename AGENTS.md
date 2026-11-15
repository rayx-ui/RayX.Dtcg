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

`dxn_dtcg` is pinned to a dXn.Dtcg release tag (`v1.4.0`); move to a new release by changing the tag. A change this
crate needs from dXn.Dtcg is released there first.

## Commits

Conventional Commits with meaningful scopes. Never add Claude co-author lines.
