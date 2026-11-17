---
agentx:
  kind: rules
  scope: knowledge/exporter
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
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
- Never depend on a UI or app framework (`nsh-*`, `gpui`, `rayx*`); CI rejects it. RayX consumes the
  generated file and does not depend on this crate.
- Dependencies come from git release tags only (never paths, branches or commits). A change this crate
  needs from `dxn_dtcg` is released in dXn.Dtcg first.
- Report every refusal reason in one `ExportError`, naming the token (and the selection when it matters);
  the `refusals` tests assert on that text.
