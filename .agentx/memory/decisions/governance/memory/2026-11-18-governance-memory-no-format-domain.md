---
agentx:
  kind: decision
  scope: decisions/governance/memory
  status: current
  created: 2026-11-18T18:10:00+01:00
  updated: 2026-11-18T18:10:00+01:00
  topics:
    - memory-taxonomy
    - agentx
  supersedes:
    - 2026-11-17-governance-memory-taxonomy.md
  superseded_by: []
---
# Memory taxonomy for RayX.Dtcg: no format domain

## Context

[The first taxonomy](2026-11-17-governance-memory-taxonomy.md) had four top-level knowledge domains,
among them `theme-format`, which covered the NSH document the exporter wrote then (`kinds.rs`,
`names.rs`, `values.rs`) and pointed to a planned RayX format. The exporter was rewritten on 2026-11-18:
it writes NSH's own theme format through the `nsh-theme` crate, copying DTCG's modifiers, aliases and
resolution order one to one. The plan (`todo/`) and the format draft (`docs/rayx-theme-format.md`) were
removed, and `README.md` now describes what the export writes.

## Decision

Three top-level knowledge domains, with decisions mirroring them:

- `exporter`: the crate's API, modules, dependency boundary and feature test groups.
  - `resolution`: loader-driven resolution, the structure check, the layers as authored (`$extends`) and
    the exactness check (`src/lib.rs`, `src/layers.rs`).
  - `command`: the `rayx-dtcg` binary (`src/main.rs`): flags, `--check`, exit codes.
- `release`: release-plz, tags, the 0.x policy, moving the `dxn_dtcg` and `nsh-theme` tags and the CI
  workflow.
- `governance`: the project's names and their history, instruction files, owner go-aheads and the
  sibling-repository layout.
  - `memory`: this tree's taxonomy and upkeep, where this decision lives.

`theme-format` is removed. What the output means is NSH's format (specified in the NSH fork) and
`README.md`, both primary sources; the modules that write it are listed in `exporter/state.md`.

## Alternatives rejected

- Keeping `theme-format` for the new document: the format is no longer this project's own, so a domain
  would copy the NSH specification and `README.md`.
- One subdomain per output module (`names`, `types`, `values`, `roles`): each is small, and their
  durable facts are in `README.md` and the tests.
- A separate CI domain or a tests domain: as in the first taxonomy, they fit `release` and `exporter`.

## Excluded

- UI, components, data storage, services or deployment: the crate has none, and CI forbids every NSH
  crate but `nsh-theme`, and any UI framework.
- RayX's runtime, the DTCG token files and `theme_codegen.py`: they live in sibling repositories and are
  referenced only as pointers.

## Consequences

`exporter/state.md` lists the output modules and the test groups; a subdomain for them is decided from
the module layout when it holds durable facts of its own.
