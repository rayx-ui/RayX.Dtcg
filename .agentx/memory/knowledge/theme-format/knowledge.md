---
agentx:
  kind: knowledge
  scope: knowledge/theme-format
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - rayx-theme-format
    - dev.rayx
    - parity
  supersedes: []
  superseded_by: []
---
# theme-format: knowledge

- The RayX DTCG project lives in the sibling repository RayX.Design (`tokens/dtcg/`, root
  `rayx.resolver.json`). It is maintained by hand while it carries `dev.rayx.*` extensions; the Figma
  export (RayX.Root's `rayx-figma` skill) is paused because it would drop them.
- Type derivation order matters: `STROKE_FLOAT` is checked before `EFFECT_FLOAT`, because
  `platform.density.borders.focusRingWidth` has both and must be a `strokeWidth`.
- The parity counts (710 tokens by type, 33 roles on 21 tokens, tokens per mode) are in
  `docs/rayx-theme-format.md`; the behaviors to port, with `theme_codegen.py` as reference, are §4 of
  `todo/repurpose-for-rayx.md`.
- Why: the owner decided the library is RayX's exporter only, with no NSH concept in it, and that RayX's
  rules belong in DTCG `$extensions` rather than in code. RayX's runtime (`rayx_components`) converts the
  RayX format into what NSH needs. Full reasoning: `todo/repurpose-for-rayx.md` §1.
