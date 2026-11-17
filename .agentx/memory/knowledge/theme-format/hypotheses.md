---
agentx:
  kind: hypotheses
  scope: knowledge/theme-format
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - rayx-theme-format
    - parity
  supersedes: []
  superseded_by: []
---
# theme-format: hypotheses

- Byte parity with the prototype's file (the planned `rayx_parity` group) may trip on serializer
  differences between `serde_json::to_string_pretty` and Python's `json.dumps(indent=2,
  ensure_ascii=False)`, for example float formatting or escaping. Unchecked.
- Ordering tokens by the merged structure (inclusion by `dev.rayx.name`) may differ from
  `theme_codegen.py`'s collection order; the plan says to compare and, if needed, order by the defining
  documents' resolution order.
- Open owner questions (plan §6), with their defaults: names keep their type prefixes; Phase 3 uses a
  RayX proc macro rather than `build.rs`; `format` carries no version.
