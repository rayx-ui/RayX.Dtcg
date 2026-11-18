---
agentx:
  kind: state
  scope: knowledge/governance
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:10:00+01:00
  topics:
    - governance
    - agents-md
    - workspace
  supersedes: []
  superseded_by: []
---
# governance: state

## Purpose And Usage

Read for the project's names, where its instructions and plans live, and how it sits among its sibling
repositories. Agent memory upkeep is in [memory](memory/state.md).

## Current Technical Shape

- **The project is RayX.Dtcg**: GitHub `rayx-ui/RayX.Dtcg`, checkout folder `RayX.Dtcg`, crate
  `rayx_dtcg` (`crates/rayx_dtcg`), binary `rayx-dtcg`. It was renamed from dXn.DtcgNsh
  (`dxn_dtcg_nsh`, `dtcg-nsh`, `dXnSix/dXn.DtcgNsh`) on 2026-11-17.
- The old names remain only in history: the `v0.1.0` tag and release, `CHANGELOG.md`'s 0.1.0 and 0.2.0
  sections (release-plz owns the file, see [release](../release/state.md)), the notes in `README.md` and
  `docs/releasing.md` that `v0.1.0` predates the rename.
- `AGENTS.md` holds the contributor rules (validation, dependencies, commits, releases) and the AgentX
  block; `CLAUDE.md` only imports it (`@AGENTS.md`).
- `docs/` holds the releasing guide; the output format is specified in the NSH fork's
  `docs/dev/theme-format.md` and described in `README.md`.
- Sibling checkouts (relative to this one): `../RayX` (RayX, consumer of the theme file),
  `../RayX.Design` (the DTCG tokens), `../RayX.Root` (workspace root, `theme_codegen.py`);
  `../../dXn/dXn.Dtcg` (the `dxn_dtcg` library) and `../../dXn/nsh` (the NSH fork, read only).

## Backlinks

- [AGENTS.md](../../../../AGENTS.md)
- [CLAUDE.md](../../../../CLAUDE.md)
- [workspace manifest](../../../../Cargo.toml)
- 2026-11-17 current [governance rules](rules.md)
- 2026-11-17 current [release state](../release/state.md)
- 2026-11-17 current [memory state](memory/state.md)
