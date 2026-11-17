---
agentx:
  kind: state
  scope: knowledge/theme-format
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - theme-format
    - nsh-theme-document
    - rayx-theme-format
    - kinds
    - names
    - values
  supersedes: []
  superseded_by: []
---
# theme-format: state

## Purpose And Usage

Read when changing what the export writes or which `$extensions` it reads. The code writes NSH's theme
document; on 2026-11-16 the owner decided to replace it with the RayX theme format. That work is
planned, not started.

## Current Technical Shape

Written today (NSH theme document; `README.md` is the spec):

- `cssVars`: `theme` (same in both appearances), `light`, `dark`; empty sections are omitted.
- `tokens`: `{ name: { kind, path, doc? } }`, sorted by name.
- `modifiers` (omitted when there is none besides the appearance): `{ name: { default, contexts } }`,
  each non-default context holding only what it changes, the default `{}`.
- Kinds (`kinds.rs`, `kind_of`): `$type`; for a dimension or a number its use, from `dev.nsh.use`, then
  `com.figma.scopes`, then the path's first segment, else a number. `border`, `transition`,
  `strokeStyle` have no kind.
- Names (`names.rs`): a pinned `dev.nsh.name`, else the kind's namespace and the kebab-case path, a
  first segment that repeats the namespace dropped; `refused` applies NSH's name rules.
- Values (`values.rs`, `write`): colors `#rrggbb`/`#rrggbbaa` in sRGB, lengths in px (`rem` × `Options.rem`),
  durations in ms, shadows, gradients and easings as CSS, typography as `text-<n>` plus suffixed parts,
  strings and booleans as JSON.

Planned (`docs/rayx-theme-format.md`, draft; full plan with the parity contract in
`todo/repurpose-for-rayx.md`):

- `{ format: "rayx.theme", name, modes, tokens }`; one entry per token: `type`, `path`, `description`,
  `roles?`, `value`, `modes?` (one mode at most besides the appearance).
- Only tokens or shadow groups carrying `dev.rayx.name` are in the theme; `dev.rayx.roles`,
  `dev.rayx.type` and `dev.rayx.appearance` (on a resolver modifier) supply the rest. RayX types
  (`color`, `dimension`, `fontSize`, `radius`, `strokeWidth`, `blur`, `number`, `fontFamily`,
  `fontWeight`, `shadow`); shadows as structured layers.
- Parity target: RayX.Root's `theme_codegen.py`. The prototype `todo/prototype/rayx_theme_proto.py`
  writes the expected file from RayX.Design's DTCG export.

Consumers: none. RayX still generates its `theme.json` with `theme_codegen.py`.

## Backlinks

- [README](../../../../README.md)
- [kinds.rs](../../../../crates/rayx_dtcg/src/kinds.rs)
- [names.rs](../../../../crates/rayx_dtcg/src/names.rs)
- [values.rs](../../../../crates/rayx_dtcg/src/values.rs)
- [document tests](../../../../crates/rayx_dtcg/tests/document.rs)
- [RayX theme format draft](../../../../docs/rayx-theme-format.md)
- [repurpose plan](../../../../todo/repurpose-for-rayx.md)
- [parity prototype](../../../../todo/prototype/rayx_theme_proto.py)
- 2026-11-17 current [theme-format rules](rules.md)
- 2026-11-17 current [exporter state](../exporter/state.md)
