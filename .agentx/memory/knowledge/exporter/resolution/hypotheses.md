---
agentx:
  kind: hypotheses
  scope: knowledge/exporter/resolution
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - dxn_dtcg
    - refactor
  supersedes: []
  superseded_by: []
---
# exporter/resolution: hypotheses

Unverified against `dxn_dtcg` v1.4.0; check its API before designing Phase 2.

- `dxn_dtcg::ModifierAxis` may not expose a modifier's `$extensions`. The plan's fallback for reading
  `dev.rayx.appearance` is to parse the resolver document through the loader.
- The plan relies on `ResolvedGroup.extensions` (merged group `$extensions`) to read `dev.rayx.name` on
  shadow groups.
