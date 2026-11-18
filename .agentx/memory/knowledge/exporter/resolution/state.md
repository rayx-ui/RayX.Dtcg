---
agentx:
  kind: state
  scope: knowledge/exporter/resolution
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:10:00+01:00
  topics:
    - resolution
    - exactness
    - structure-check
    - layers
  supersedes: []
  superseded_by: []
---
# exporter/resolution: state

## Purpose And Usage

Read when touching how `export` resolves a project, checks its structure, copies its layers or checks
exactness. The resolution is in `src/lib.rs` (`Project`, `verify_exact`) and `src/layers.rs`.

## Current Technical Shape

- `Project { runtime: DtcgRuntime, root, load }`. `provide` loops until `Resolution::Ready`, feeding
  requested documents from `load`. It backs `axes()` (the resolver's modifiers), `combinations()` (refuses
  more contexts combinations than `max_combinations`, with the count), `structure()` (selection space,
  then `check_structure`) and `resolve(selection)`.
- **Structure.** `dxn_dtcg`'s structure check over every selection: a token missing under some selection,
  or with different types, is refused, since NSH's contexts add no names.
- **Layers.** `layers.rs` reads the root's `resolutionOrder` as authored: sets and modifiers, each context
  with exactly its DTCG entries. `$extends` groups are copied (`expand()` in `crates/rayx_dtcg/src/lib.rs`): a set's copy follows the
  contexts that set the target, a context's copy lands in that context. The sets are merged into `tokens`
  (the later set winning); a token only modifiers set takes the entry of the last modifier whose default
  context sets it; a set placed after a modifier may set no token that modifier sets (refused).
- **Exactness.** `verify_exact` resolves every combination of one context per modifier (the empty
  selection when there are no modifiers) with `dxn_dtcg` and the document with `nsh_theme::resolve`, and
  compares each token's value in NSH's forms; a token that differs is refused (``` `path` (`name`) under
  a=x, b=y: DTCG resolves it to …, the document to … ```), and so is a token on one side only or a
  diagnostic of the resolution. `export` runs it last, after the document passed `nsh_theme::check` and
  the role check.

## Backlinks

- [lib.rs](../../../../../crates/rayx_dtcg/src/lib.rs)
- [layers.rs](../../../../../crates/rayx_dtcg/src/layers.rs)
- [layers tests](../../../../../crates/rayx_dtcg/tests/layers.rs)
- [exactness tests](../../../../../crates/rayx_dtcg/tests/exactness.rs)
- 2026-11-17 current [exporter state](../state.md)
- 2026-11-17 current [resolution rules](rules.md)
- 2026-11-17 current [resolution hypotheses](hypotheses.md)
