---
agentx:
  kind: knowledge
  scope: knowledge/governance
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - repository-move
    - workspace
  supersedes: []
  superseded_by: []
---
# governance: knowledge

- Name history: built as `dXn.DtcgNsh` under `dXnSix`, a generic exporter for the NSH fork's
  theme-collections project. The owner made it RayX's exporter, moved it to `rayx-ui` as RayX.Dtcg, and
  on 2026-11-17 the crate and binary took the RayX names.
- Stays dXn: `dxn_dtcg` (dXn.Dtcg, `github.com/dXnSix/dXn.Dtcg`), the DTCG resolution library this
  crate depends on, is a separate dXn library and keeps its name, URL and tag. So do its `dxn.dtcg.*`
  value types (`dxn.dtcg.blur`, `dxn.dtcg.string`, `dxn.dtcg.boolean`).
- A stale clone exists at `../../dXn/dXn.DtcgNsh`: old remote, behind `main` (at `e3e423d`). Work in
  this checkout.
