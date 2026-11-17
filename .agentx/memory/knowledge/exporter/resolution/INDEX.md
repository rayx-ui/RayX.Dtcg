---
agentx:
  kind: index
  scope: knowledge/exporter/resolution
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - resolution
    - exact-layers
  supersedes: []
  superseded_by: []
---
# exporter/resolution

How `export` resolves a DTCG project and proves its output exact (`src/lib.rs`: `export`, `Project`,
`Values`, `product`).

- [state.md](state.md): the pipeline step by step.
- [knowledge.md](knowledge.md): what the exact-layers check guarantees, its cost.
- [rules.md](rules.md): what a refactor must keep.
- [hypotheses.md](hypotheses.md): `dxn_dtcg` capabilities the RayX refactor assumes.
