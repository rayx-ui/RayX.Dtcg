---
agentx:
  kind: decision
  scope: decisions/exporter/resolution
  status: current
  created: 2026-11-19T13:30:00+01:00
  updated: 2026-11-19T13:30:00+01:00
  topics:
    - exporter
    - nsh-theme
    - theme-format
    - layers
    - names
  supersedes: []
  superseded_by: []
  promoted_from: theme-roles
---
# Decision: the exporter writes NSH's theme format through `nsh-theme`, DTCG's layers one to one

**Date:** 2026-11-18, the owner's settled points. **Status:** current; promoted from the epic
`theme-roles` (RayX.Root `.agentx/tasks/theme-roles/design.json`: `D-EXPORT`, `D-LAYERS`,
`D-NAMES`, `D-TYPES`), closed 2026-11-19.

## Context

RayX's default theme is an NSH theme document that maps RayX's roles onto the design system's
tokens. The exporter wrote NSH's older `cssVars` layout, and a plan for a RayX-only format
(`todo/repurpose-for-rayx.md`, `docs/rayx-theme-format.md`) was what the owner called "a mistake".

## Decision

- **NSH's format, NSH's model.** `export` builds an `nsh_theme::ThemeDocument` and writes it with
  `nsh_theme::write`; the exactness check `verify_exact` compares against `nsh_theme::resolve`
  under every combination, and `export` runs it before it returns. No second implementation of
  the format's rules.
- **DTCG's resolution one to one, never flattened.** Sets merge into `tokens`; each DTCG modifier
  becomes an NSH modifier with the same contexts and entries; `resolutionOrder` keeps DTCG's
  order; aliases become `{name}` references. A set after a modifier that sets a modifier's token
  is refused.
- **Names and types are data-driven:** a token's name is its DTCG path in kebab case per segment
  joined by `-` (or its `dev.nsh` pin); `$type` maps to NSH's value types by a fixed table,
  overridable per token. Duplicates and unknown types are refused.
- **No project specifics in the code.** RayX's specifics are in its data (`dev.nsh.*` pins, the
  role map) and in RayX.Root's `theme_export.py`, which passes every path; the output is
  deterministic.
- Kept from before: the I/O-free loader-driven library, `dxn_dtcg`'s resolution and structure
  check, `max_combinations`, `--check`'s byte comparison. Removed: `cssVars`, `kinds.rs`,
  `dev.nsh.use`, `--appearance`, `--names`.

## Alternatives considered

- A RayX-only format: retired with its documents and knowledge domain.
- Flattening the modifiers into combinations: loses NSH's runtime modifier picks.

## Trade-offs accepted

The exporter follows the NSH fork's format and tag (RayX's
`decisions/dependencies/nsh-pin/2026-11-19-dependencies-nsh-pin-one-tag.md`); `nsh-theme` is the
one NSH crate CI allows.

## Supersedes

None. The memory taxonomy change it caused is
[no format domain](../../governance/memory/2026-11-18-governance-memory-no-format-domain.md).
