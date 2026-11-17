---
agentx:
  kind: rules
  scope: knowledge/governance
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - commits
    - go-ahead
  supersedes: []
  superseded_by: []
---
# governance: rules

- Call the project RayX.Dtcg, the crate `rayx_dtcg`, the command `rayx-dtcg`. Write `dXn.DtcgNsh`,
  `dxn_dtcg_nsh`, `dtcg-nsh` or `dXnSix/dXn.DtcgNsh` only when speaking of `v0.1.0` or the history. Leave
  `dxn_dtcg` and its URL alone.
- Commit only when the owner asks; pushing, tagging and merging need the owner's go-ahead.
- No AI attribution or co-author lines in commits, pull requests or files.
- Conventional Commits with meaningful scopes; one commit per repository a task touches. Work in a
  sibling repository follows that repository's own `AGENTS.md`.
- Raw diagnostic output goes to the ignored `artifacts-temp/`.
