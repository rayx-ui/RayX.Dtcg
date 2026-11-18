---
agentx:
  kind: knowledge
  scope: knowledge/exporter/command
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:10:00+01:00
  topics:
    - check
    - ci
  supersedes: []
  superseded_by: []
---
# exporter/command: knowledge

- `--check` is how a consumer's CI proves its committed theme file is current; RayX.Root runs it against
  `../RayX/crates/rayx_components/theme.json`.
- The appearance is no option: light and dark are an ordinary modifier of the theme document, and each
  token's DTCG path is its `path` in the document, so `--appearance` and `--names` have no use.
