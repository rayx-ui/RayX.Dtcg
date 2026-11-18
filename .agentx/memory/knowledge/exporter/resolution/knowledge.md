---
agentx:
  kind: knowledge
  scope: knowledge/exporter/resolution
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:10:00+01:00
  topics:
    - exact-layers
    - structure-check
  supersedes: []
  superseded_by: []
---
# exporter/resolution: knowledge

- The exactness check proves the copied layers right: NSH resolves the document under every combination of
  contexts to what DTCG resolves the project to. Two modifiers that change a token together (one aliases
  a token the other changes) are caught here by a differing value, since NSH's layers are copied one to
  one and only add values.
- `max_combinations` bounds two things: the structure check's selection space and the number of
  combinations `verify_exact` resolves, both over every modifier. RayX's modes (appearance 2, accent 3,
  density 2, os 4) make 48 combinations.
- The comparison is on values in NSH's forms (`Converter::convert` on the DTCG resolution against
  `nsh_theme::resolve`'s value), so a reference the document keeps is compared after NSH resolved it.
