---
agentx:
  kind: index
  scope: knowledge
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:10:00+01:00
  topics:
    - memory-taxonomy
  supersedes: []
  superseded_by: []
---
# Knowledge index

Secondary context for **RayX.Dtcg** (GitHub `rayx-ui/RayX.Dtcg`): RayX's DTCG 2025.10 theme exporter,
crate `rayx_dtcg`, command `rayx-dtcg`. It was dXn.DtcgNsh until 2026-11-17
([governance](governance/state.md)). Source, tests, `README.md` and `docs/` are primary. Open a domain
only for a concrete question.

| Domain | Holds |
| --- | --- |
| [exporter](exporter/INDEX.md) | The crate: the I/O-free `export` API, its modules, dependency boundary and feature test groups. |
| [exporter/resolution](exporter/resolution/INDEX.md) | Loader-driven resolution over `dxn_dtcg`, the structure check, the layers as authored and the exactness check across every combination of contexts. |
| [exporter/command](exporter/command/INDEX.md) | The `rayx-dtcg` binary: flags, `--check`, exit codes. |
| [release](release/INDEX.md) | release-plz, `vX.Y.Z` tags, the 0.x policy, moving the `dxn_dtcg` tag, the CI workflow. |
| [governance](governance/INDEX.md) | The project's name and its dXn.DtcgNsh past, where instructions and plans live, owner go-aheads, the sibling-repository layout. |
| [governance/memory](governance/memory/INDEX.md) | This memory tree: its taxonomy and upkeep. |

Decisions: [decisions index](../decisions/INDEX.md).
