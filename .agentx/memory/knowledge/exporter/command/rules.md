---
agentx:
  kind: rules
  scope: knowledge/exporter/command
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - check
  supersedes: []
  superseded_by: []
---
# exporter/command: rules

- Keep `--check` a byte-exact comparison with `Export::text()`, failing with exit code 1; consumers commit
  the file and rely on it in CI.
