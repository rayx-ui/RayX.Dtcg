---
agentx:
  kind: decision
  scope: decisions/governance/memory
  status: superseded
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:10:00+01:00
  topics:
    - memory-taxonomy
    - agentx
  supersedes: []
  superseded_by:
    - 2026-11-18-governance-memory-no-format-domain.md
---
# Memory taxonomy for RayX.Dtcg

## Context

RayX.Dtcg (formerly dXn.DtcgNsh) is one Rust crate, `rayx_dtcg` (about 1,100 lines of source and 450
of tests): a DTCG 2025.10 exporter over `dxn_dtcg` that writes NSH theme documents, with a library and
the `rayx-dtcg` command, released by release-plz as git tags. On 2026-11-16 the owner decided to make it
RayX's exporter (RayX theme format, no NSH), planned in `todo/repurpose-for-rayx.md` and not started.

## Decision

Four knowledge domains, with decisions mirroring them:

- `exporter`: the crate's API, modules, dependency boundary and feature test groups.
  - `resolution`: loader-driven resolution, the appearance pick, the structure check and the
    exact-layers check (`src/lib.rs`). It survives the repurpose unchanged, so it is a stable boundary.
  - `command`: the `rayx-dtcg` binary (`src/main.rs`): flags, `--check`, exit codes.
- `theme-format`: what the export writes and reads beyond DTCG. It covers today's NSH document
  (`kinds.rs`, `names.rs`, `values.rs`, layout) and points to the planned RayX format, the area the next
  epic changes.
- `release`: release-plz, tags, the 0.x policy, moving the `dxn_dtcg` tag and the CI workflow.
- `governance`: the project's names and their history, instruction files, plans, owner go-aheads and
  the sibling-repository layout.
  - `memory`: this tree's taxonomy and upkeep, where this decision lives.

## Alternatives rejected

- One subdomain per source module (`kinds`, `names`, `values`): those modules are NSH-specific and
  Phase 2 replaces them with `types.rs` and `theme.rs`, so the folders would churn at once.
- `theme-format` split into `nsh-document` and `rayx-format`: the RayX format is not implemented and is
  fully specified in `docs/` and `todo/`; a subdomain would duplicate them.
- A separate CI domain: one workflow that gates pushes and release pull requests fits `release`.
- A tests domain: tests follow the per-feature group layout and belong to `exporter`.

## Excluded

- UI, components, data storage, services or deployment: the crate has none, and CI forbids UI
  frameworks.
- RayX's runtime, the DTCG token files and `theme_codegen.py`: they live in sibling repositories and
  are referenced only as pointers.

## Consequences

When Phase 2 lands, `theme-format` and `exporter` state change in the same commit; subdomains for the
new modules are decided then from the implemented layout.
