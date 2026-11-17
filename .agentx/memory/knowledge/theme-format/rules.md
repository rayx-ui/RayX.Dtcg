---
agentx:
  kind: rules
  scope: knowledge/theme-format
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - rayx-theme-format
    - nsh-theme-document
  supersedes: []
  superseded_by: []
---
# theme-format: rules

- Until Phase 2 lands, `README.md` and the code describe current behavior; treat
  `docs/rayx-theme-format.md` as a draft spec, not as what the export does.
- Add no NSH features. Phase 2 removes every NSH concept (kinds and namespaces, `dev.nsh.*`,
  `--appearance`, `names.json`, the `cssVars`/`modifiers` layout); keeping NSH's document shape under a
  RayX name was rejected.
- Keep RayX's specifics in the DTCG data (`dev.rayx.*`), not in code: no RayX tables, collection lists
  or naming rules in the library.
- The written text is pretty JSON with a final newline and keys in the spec's order (`serde_json` with
  `preserve_order`).
