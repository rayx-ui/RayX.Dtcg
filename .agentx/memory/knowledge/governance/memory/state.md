---
agentx:
  kind: state
  scope: knowledge/governance/memory
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - memory-taxonomy
    - agentx
  supersedes: []
  superseded_by: []
---
# governance/memory: state

## Purpose And Usage

Read before adding, moving or pruning memory in this repository.

## Current Technical Shape

- Knowledge domains: `exporter` (with `resolution`, `command`), `theme-format`, `release`, `governance`
  (with `memory`). Each folder has `INDEX.md`, `state.md`, `knowledge.md`, `rules.md`, `hypotheses.md`.
- Decisions live under `.agentx/memory/decisions/<domain>/<subdomain>/` as
  `YYYY-MM-DD-<domain>-<subdomain>-<topic>.md`.
- `state.md` backlinks to source paths route an epic's touched files to domains, so keep them pointing at
  the files each domain covers.
- The AgentX workflow block is in `AGENTS.md` only, since `CLAUDE.md` imports it.

## Backlinks

- [AGENTS.md](../../../../../AGENTS.md)
- 2026-11-17 current [knowledge index](../../INDEX.md)
- 2026-11-17 current [taxonomy decision](../../../decisions/governance/memory/2026-11-17-governance-memory-taxonomy.md)
