---
agentx:
  kind: rules
  scope: knowledge/exporter/resolution
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:00:00+01:00
  topics:
    - exact-layers
    - refactor
  supersedes: []
  superseded_by: []
---
# exporter/resolution: rules

- Keep the loader-driven resolution, the structure check, `max_combinations` and the exactness check
  (`verify_exact`, which `export` runs before it returns) through any output rewrite.
- Name the token and the selection or combination in each refusal (`under a=x, b=y`), as the existing
  messages do.
