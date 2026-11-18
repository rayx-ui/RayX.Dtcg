---
agentx:
  kind: state
  scope: knowledge/exporter/command
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:10:00+01:00
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

- `rayx-dtcg <ROOT> -o <FILE> [--role-map <FILE>] [--roles <FILE>] [--rem <PX>] [--max-combinations <N>]
  [--check]`; `-o` is also `--out` and is required, `-h`/`--help` prints the usage. Arguments are parsed by
  hand (no argument-parser dependency).
- The root file's folder is the loader's base; the root's resource id is its file name.
- `--role-map` and `--roles` are read as text into `Options.role_map` and `Options.roles`.
- The document is written to `-o`. `--check` writes nothing: it compares the file's text with
  `Export::text()` byte for byte and says whether the file is missing or stale.
- `--appearance` and `--names` are refused with a message saying they were removed; any other unknown
  option is a usage error.
- Exit codes: 0 done or current; 1 the project is refused (each reason on stderr, prefixed `rayx-dtcg:`)
  or `--check` found the file missing or stale; 2 a usage or I/O error.
- The role check's warnings (`Export.warnings`) go to stderr as `rayx-dtcg: …`.

## Backlinks

- [main.rs](../../../../../crates/rayx_dtcg/src/main.rs)
- [cli tests](../../../../../crates/rayx_dtcg/tests/cli.rs)
- [README](../../../../../README.md)
- 2026-11-17 current [exporter state](../state.md)
