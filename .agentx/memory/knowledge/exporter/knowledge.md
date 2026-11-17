---
agentx:
  kind: knowledge
  scope: knowledge/exporter
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
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
- **Error phases.** `export` collects reasons rather than stopping at the first, in three phases, and
  returns the first phase that has any: (1) the appearance (unknown modifier or context), (2) the
  structure check, (3) values, names, each context's changes and the exact layers together, sorted and
  deduplicated. Load failures propagate immediately.
- **`left_out` is not a refusal.** A token whose `$type` has no NSH kind (`border`, `transition`,
  `strokeStyle`) is skipped and reported in `Export.left_out`; the command prints it as `left out:`.
- **Runtime options.** The `dxn_dtcg` runtime runs with `ValueExtensions::all()` (so `dxn.dtcg.blur`,
  `dxn.dtcg.string`, `dxn.dtcg.boolean` resolve) and `RuntimeLimits::with_max_selections(max_combinations)`.
  An unknown `$type` (RayX's `dev.rayx.blur`) is refused by `dxn_dtcg` before anything is written; the
  plan fixes that in the DTCG data (`dxn.dtcg.blur`), not here.
