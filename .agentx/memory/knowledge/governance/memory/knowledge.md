---
agentx:
  kind: knowledge
  scope: knowledge/governance/memory
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:10:00+01:00
  topics:
    - memory-taxonomy
  supersedes: []
  superseded_by: []
---
# governance/memory: knowledge

- No domain covers the output format: `README.md` describes it and the modules that write it (`names.rs`,
  `types.rs`, `values.rs`, `layers.rs`, `roles.rs`) belong to `exporter` (`resolution` for `layers.rs`).
  `theme-format` was a domain until 2026-11-18; why it went, and the alternatives:
  [format decision](../../../decisions/governance/memory/2026-11-18-governance-memory-no-format-domain.md).
  The first taxonomy: [taxonomy decision](../../../decisions/governance/memory/2026-11-17-governance-memory-taxonomy.md).
