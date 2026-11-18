---
agentx:
  kind: rules
  scope: knowledge/governance/memory
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - memory-taxonomy
    - upkeep
  supersedes: []
  superseded_by: []
---
# governance/memory: rules

- When the output format changes, update `exporter/state.md` and `README.md` in the same change, replacing
  the old facts rather than keeping both.
- Add a subdomain only for a stable module boundary with durable facts of its own; record a decision
  when top-level domains change.
- Memory commits are `chore(agentx): …`.
