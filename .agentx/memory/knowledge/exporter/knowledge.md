---
agentx:
  kind: knowledge
  scope: knowledge/exporter
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-18T18:10:00+01:00
  topics:
    - exporter
    - export-api
    - errors
  supersedes: []
  superseded_by: []
---
# exporter: knowledge

- **Loader contract.** `Project::provide` answers each `dxn_dtcg` `Resolution::Missing` request by calling
  `load(id)` and `provide_requested_json`, then retries. An id the loader cannot read fails at once with
  ``cannot read `<id>` ``; a document `dxn_dtcg` rejects fails with `` `<id>`: <error> ``. The command's
  loader reads ids relative to the root file's folder.
- **Error phases.** `export` collects reasons rather than stopping at the first within a phase, and
  returns the first phase that has any: (1) loading, the combination count and the structure check (these
  fail at once), (2) names, value types and pins, (3) layer placement, values and modifiers, (4) the role
  map, (5) what `nsh_theme::check` and the role declarations refuse, (6) exactness. Reasons are sorted and
  deduplicated.
- **Nothing is left out.** A token whose `$type` has no NSH value type (`strokeStyle`, any other) is
  refused, not skipped; `$extensions["dev.nsh"].type` can name an NSH value type instead.
- **Runtime options.** The `dxn_dtcg` runtime runs with `ValueExtensions::all()` (so `dxn.dtcg.blur`,
  `dxn.dtcg.string`, `dxn.dtcg.boolean` resolve) and `RuntimeLimits::with_max_selections(max_combinations)`.
  An unknown `$type` is refused by `dxn_dtcg` before anything is written.
- **Warnings.** Only the role check warns (`role-unknown`: a role mapping onto a non-color token that no
  declaration names); the mapping is kept.
