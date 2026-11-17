---
agentx:
  kind: knowledge
  scope: knowledge/exporter/command
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - check
    - ci
  supersedes: []
  superseded_by: []
---
# exporter/command: knowledge

- `--check` is how a consumer's CI proves its committed theme file is current. The repurpose plan
  (Phase 4) runs it from RayX.Root against `../RayX/crates/rayx_components/theme.json`.
- Phase 2 drops `--appearance` (the appearance comes from `dev.rayx.appearance`) and `--names` (names
  come from `dev.rayx.name`); `-o`, `--check` and `--rem` stay.
