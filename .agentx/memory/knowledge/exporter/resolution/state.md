---
agentx:
  kind: state
  scope: knowledge/exporter/resolution
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - resolution
    - exact-layers
    - structure-check
    - appearance
  supersedes: []
  superseded_by: []
---
# exporter/resolution: state

## Purpose And Usage

Read when touching how `export` resolves a project, picks the appearance, checks structure or checks
exact layers. All of it is in `src/lib.rs`.

## Current Technical Shape

- `Project { runtime: DtcgRuntime, load }`. `provide` loops until `Resolution::Ready`, feeding requested
  documents from `load`. It backs `axes()` (the resolver's modifiers), `structure()` (selection space,
  then `check_structure`) and `resolve(selection)`.
- **Appearance.** `Options.appearance` names the modifier and its light and dark contexts (an unknown
  modifier fails at once, an unknown context is a reason). With `None`, the first modifier that has both
  a `light` and a `dark` context is the appearance; with none, dark equals light everywhere.
- **Other modifiers** (axes) keep their name, contexts and default (the resolver's default, else the
  first context).
- **Structure.** `dxn_dtcg`'s structure check over every selection (refused above `max_combinations`): a
  token missing under some selection, or with different types, is refused.
- **Values of a selection.** The light and dark selections are resolved (appearance context added to the
  picks). Per token: its kind (none: `left_out`), a dark counterpart (else refused), its name, and its
  written values (a typography token writes several names by suffix).
- **Layers.** The base is every axis at its default. For each non-default context of each axis alone,
  `changed_by` gives what it changes over the base: that context's overlay.
- **Exact layers.** With more than one axis, every combination with two or more non-default contexts
  is resolved and compared with the base plus each picked context's overlay, in axis order; a written
  name that differs is refused (``under a=x, b=y … their contexts change it together``). More
  combinations than `max_combinations` is a reason.
- The document is assembled last (layout in [theme-format](../../theme-format/state.md)).

## Backlinks

- [lib.rs](../../../../../crates/rayx_dtcg/src/lib.rs)
- [refusals tests](../../../../../crates/rayx_dtcg/tests/refusals.rs)
- [document tests](../../../../../crates/rayx_dtcg/tests/document.rs)
- 2026-11-17 current [exporter state](../state.md)
- 2026-11-17 current [resolution rules](rules.md)
- 2026-11-17 current [resolution hypotheses](hypotheses.md)
