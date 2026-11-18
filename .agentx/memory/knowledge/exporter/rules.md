---
agentx:
  kind: rules
  scope: knowledge/exporter
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:00:00+01:00
  topics:
    - exporter
    - tests
    - dependencies
  supersedes: []
  superseded_by: []
---
# exporter: rules

- Keep the library I/O-free: everything it reads comes through `load`; file-system access belongs in
  `src/main.rs`.
- A new feature or behavior change gets its own group `crates/rayx_dtcg/tests/<feature>.rs` built on
  `tests/common`; scoped work runs `cargo test --test <feature>` (`AGENTS.md`).
- Never depend on a UI or app framework (`gpui*`, `rayx*`, any `nsh-*` but `nsh-theme`); CI's guard and the
  `dependencies` test group reject it. `nsh-theme` (NSH's theme format alone, serde and serde_json only) is
  allowed, by an `nshx-*` tag. RayX consumes the generated file and does not depend on this crate.
- The output is NSH's theme format (`tokens`, `modifiers`, `resolutionOrder`, `roles`), built as an
  `nsh_theme::ThemeDocument` and written by `nsh_theme::write`; the old `cssVars` layout, NSH's old kinds and
  namespaces, `--appearance` and `--names` are gone and are not brought back.
- Dependencies come from git release tags only (never paths, branches or commits). A change this crate
  needs from `dxn_dtcg` is released in dXn.Dtcg first.
- Report every refusal reason in one `ExportError`, naming the token (and the selection when it matters);
  the feature groups (`format`, `values`, `references`, `layers`, `exactness`, `roles`, `cli`) assert on that
  text.
