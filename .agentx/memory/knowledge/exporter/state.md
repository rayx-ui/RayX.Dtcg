---
agentx:
  kind: state
  scope: knowledge/exporter
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-19T03:30:00+01:00
  topics:
    - exporter
    - rayx_dtcg
    - export-api
    - tests
  supersedes: []
  superseded_by: []
---
# exporter: state

## Purpose And Usage

Read when changing the crate's public API, its module layout, its dependencies or its test groups.
Resolution and checks are in [resolution](resolution/state.md); the CLI in [command](command/state.md);
what the document holds in `README.md`.

## Current Technical Shape

- One workspace member, `crates/rayx_dtcg` (edition 2024, toolchain 1.97.0 from `rust-toolchain.toml`):
  a library and the binary `rayx-dtcg` (`src/main.rs`). Until 2026-11-17 they were `dxn_dtcg_nsh` and
  `dtcg-nsh` (see [governance](../governance/state.md)).
- The output is NSH's theme format (`docs/dev/theme-format.md` of the NSH fork): one document with
  `tokens`, `modifiers`, `resolutionOrder`, `{name}` references and `roles`, built as an
  `nsh_theme::ThemeDocument` and written by `nsh_theme::write`. DTCG modifiers, aliases and resolution
  order are copied one to one; the layers are not flattened.
- Public API (`src/lib.rs`):
  - `export(root: &str, load: &dyn Fn(&str) -> Option<String>, options: &Options) -> Result<Export,
    ExportError>`. The library reads nothing itself; `load` maps a resource id (relative, as the project's
    `$ref`s are) to its text.
  - `verify_exact(root, load, options, document) -> Result<(), ExportError>`: the exactness check on any
    document; `export` runs it before it returns.
  - `Options { rem: 16.0, max_combinations: 4096, role_map: Option<String>, roles: Option<String> }`
    (the role map's and the role declarations' texts).
  - `Export { document: ThemeDocument, warnings: Vec<String> }`; `Export::text()` is `nsh_theme::write`'s
    text, which ends in a newline. `warnings` holds the role check's kept diagnostics (`role-unknown`).
  - `ExportError(Vec<String>)`: every reason found, sorted and once each, joined by newlines in `Display`.
- Modules: `lib.rs` (the pipeline, `Project`, the layer expansion, `verify_exact`), `names.rs` (token
  names), `types.rs` (`$type` to NSH value type, and the `dev.nsh` type pin), `values.rs` (values in NSH's
  forms, aliases as references), `layers.rs` (the sets and modifiers as authored, `$extends` copies),
  `roles.rs` (the role map and the role declarations). What each writes is in `README.md`.
- Dependencies, each declared in the crate's own `Cargo.toml` (not the workspace table, see
  [release](../release/state.md)): `dxn_dtcg` by git tag `v1.4.0`; `nsh-theme` by the NSH fork's tag
  `nshx-0.6.0-5` (the document model, `write`, `resolve`, `check`, `check_roles`); `serde_json` from the
  workspace. `nsh-theme` is the one NSH crate allowed: CI fails when `cargo tree` lists any other `nsh-`,
  `gpui` or `rayx` crate.
- Tests: one integration group per feature, `crates/rayx_dtcg/tests/<feature>.rs`, selected with
  `cargo test --test <feature>`: `dependencies` (the dependency boundary), `format` (names, kinds, document
  shape), `values` (value forms), `references` (aliases), `layers` (sets, modifiers, `$extends`),
  `exactness` (`verify_exact` and the export's refusals), `roles` (role map and declarations), `cli` (the
  built binary). `tests/common/mod.rs` holds the in-memory loader and helpers (`run`, `verify`, `tokens`,
  `read_back`, `json`, `entry`).

## Backlinks

- [crate manifest](../../../../crates/rayx_dtcg/Cargo.toml)
- [lib.rs](../../../../crates/rayx_dtcg/src/lib.rs)
- [tests](../../../../crates/rayx_dtcg/tests)
- [test fixtures](../../../../crates/rayx_dtcg/tests/common/mod.rs)
- [workspace manifest](../../../../Cargo.toml)
- [rust-toolchain.toml](../../../../rust-toolchain.toml)
- 2026-11-19 current [decision: NSH's theme format through `nsh-theme`](../../decisions/exporter/resolution/2026-11-19-exporter-resolution-nsh-theme-format.md) (epic `theme-roles`)
- 2026-11-17 current [exporter rules](rules.md)
- 2026-11-17 current [resolution state](resolution/state.md)
- 2026-11-17 current [command state](command/state.md)
