---
agentx:
  kind: hypotheses
  scope: knowledge/release
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T02:45:00+01:00
  topics:
    - release
    - repository-move
  supersedes: []
  superseded_by: []
---
# release: hypotheses

Unverified since the repository became `rayx-ui/RayX.Dtcg`; check before the next release.

- `RELEASE_PLZ_TOKEN` pushed `v0.2.0` and created its GitHub Release; whether it may open a release pull
  request in `rayx-ui` is not yet shown (no pull request has been needed since the move).
- The "Release tags" ruleset (`refs/tags/v*` cannot be updated or deleted) may not have moved with the
  repository.
- Consumers pinned to `github.com/dXnSix/dXn.DtcgNsh` at `v0.1.0` rely on GitHub's redirect after the
  transfer.
