---
agentx:
  kind: index
  scope: knowledge/exporter
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - exporter
    - rayx_dtcg
  supersedes: []
  superseded_by: []
---
# exporter

RayX.Dtcg's crate, `rayx_dtcg` (`crates/rayx_dtcg`): the library `export(root, &load, &options)` and
the `rayx-dtcg` binary.

- [state.md](state.md): module map, public API, dependencies, test groups.
- [knowledge.md](knowledge.md): loader contract, error phases, `left_out`.
- [rules.md](rules.md): I/O-free library, test groups, dependency boundary.
- [hypotheses.md](hypotheses.md)

Subdomains:

- [resolution](resolution/INDEX.md): how `export` resolves and checks a project (`src/lib.rs`).
- [command](command/INDEX.md): the `rayx-dtcg` command (`src/main.rs`).

What the output means (names, value types, values, references, layers, roles) is in `README.md`.
