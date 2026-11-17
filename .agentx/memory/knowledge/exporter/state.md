---
agentx:
  kind: state
  scope: knowledge/exporter
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
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
what the document holds in [theme-format](../theme-format/state.md).

## Current Technical Shape

- One workspace member, `crates/rayx_dtcg` (edition 2024, toolchain 1.97.0 from `rust-toolchain.toml`):
  a library and the binary `rayx-dtcg` (`src/main.rs`). Until 2026-11-17 they were `dxn_dtcg_nsh` and
  `dtcg-nsh` (see [governance](../governance/state.md)).
- Public API (`src/lib.rs`): `export(root: &str, load: &dyn Fn(&str) -> Option<String>, options: &Options)
  -> Result<Export, ExportError>`. The library reads nothing itself; `load` maps a resource id (relative,
  as the project's `$ref`s are) to its text.
  - `Options { appearance: Option<Appearance>, rem: 16.0, max_combinations: 4096 }`;
    `Appearance { modifier, light, dark }`.
  - `Export { document: Value, names: Vec<(path, name)>, left_out: Vec<String> }`; `Export::text()` is
    pretty JSON plus a final `\n`.
  - `ExportError(Vec<String>)`: every reason found, joined by newlines in `Display`.
- Modules: `lib.rs` (the pipeline, `Project`, `Values`, `css_vars`, `product`), `kinds.rs`, `names.rs`,
  `values.rs`. The last three define the NSH output and are described under theme-format.
- Dependencies: `dxn_dtcg` by git tag `v1.4.0`, declared in the crate's own `Cargo.toml` (not the
  workspace table, see [release](../release/state.md)); `serde_json` with `preserve_order`. No UI or app
  framework: CI fails when `cargo tree` lists an `nsh-`, `gpui` or `rayx` crate.
- Tests: one integration group per feature, `crates/rayx_dtcg/tests/<feature>.rs`, selected with
  `cargo test --test <feature>`: `document` (output shape), `refusals` (each refused project, asserting on
  message text), `command` (the built binary). `tests/common/mod.rs` holds the in-memory loader
  (`run(files, options)`, rooted at the first file) and a small design system (`RESOLVER`, `project()`:
  foundation set, appearance, brand, density).
- Planned, not started: Phase 2 of `todo/repurpose-for-rayx.md` rewrites the output for RayX (new
  `types.rs`, `theme.rs`; NSH modules and options removed).

## Backlinks

- [crate manifest](../../../../crates/rayx_dtcg/Cargo.toml)
- [lib.rs](../../../../crates/rayx_dtcg/src/lib.rs)
- [tests](../../../../crates/rayx_dtcg/tests)
- [test fixtures](../../../../crates/rayx_dtcg/tests/common/mod.rs)
- [workspace manifest](../../../../Cargo.toml)
- [rust-toolchain.toml](../../../../rust-toolchain.toml)
- 2026-11-17 current [exporter rules](rules.md)
- 2026-11-17 current [resolution state](resolution/state.md)
- 2026-11-17 current [command state](command/state.md)
- 2026-11-17 current [theme-format state](../theme-format/state.md)
