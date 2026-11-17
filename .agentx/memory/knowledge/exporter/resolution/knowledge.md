---
agentx:
  kind: knowledge
  scope: knowledge/exporter/resolution
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - exact-layers
    - structure-check
  supersedes: []
  superseded_by: []
---
# exporter/resolution: knowledge

- The exact-layers check is what makes per-modifier overlays safe to precompute: any combination equals
  the base with each picked context laid over it. Two modifiers that change a token together (one
  aliases a token the other changes) cannot be expressed as layers and are refused.
- In the planned RayX format the same check proves that a token depends on the appearance and at most
  one other mode; Phase 2 keeps it, and it replaces `theme_codegen.py`'s weaker refusal of a context
  that changes another mode's collection.
- Combinations of 0 or 1 non-default contexts are skipped: they are the base or one layer by
  construction.
- `max_combinations` bounds two things: the structure check's selection space (all modifiers, the
  appearance included) and the exact-layers product (the other axes only). RayX's modes (accent 3,
  density 2, os 4) make 24 combinations of other axes.
- Comparisons are on written values (`Values::flat()`), after formatting, not on raw resolved values.
