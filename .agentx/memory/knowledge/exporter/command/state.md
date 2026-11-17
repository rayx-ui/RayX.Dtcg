---
agentx:
  kind: state
  scope: knowledge/exporter/command
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - rayx-dtcg
    - cli
    - check
  supersedes: []
  superseded_by: []
---
# exporter/command: state

## Purpose And Usage

Read when changing the command's flags, its outputs or its exit codes.

## Current Technical Shape

- `rayx-dtcg <root.json> [-o <theme.json>] [--appearance <modifier>[=<light>,<dark>]] [--rem <px>]
  [--names <names.json>] [--check]`; `-o` is also `--out`, `-h`/`--help` prints the usage. Arguments are
  parsed by hand (no argument-parser dependency).
- The root file's folder is the loader's base; the root's resource id is its file name.
- Without `-o` the document goes to stdout; with `-o` it is written there. `--check` (needs `-o`) writes
  nothing: it compares the file's text with `Export::text()` byte for byte.
- `--names` writes `{ path: name }` as pretty JSON; it is written even under `--check`.
- Exit codes: 0 done or current; 1 the project is refused (each reason on stderr, prefixed `rayx-dtcg:`)
  or `--check` found the file stale; 2 a usage or I/O error.
- `left_out` entries go to stderr as `rayx-dtcg: left out: …`.

## Backlinks

- [main.rs](../../../../../crates/rayx_dtcg/src/main.rs)
- [command tests](../../../../../crates/rayx_dtcg/tests/command.rs)
- [README](../../../../../README.md)
- 2026-11-17 current [exporter state](../state.md)
