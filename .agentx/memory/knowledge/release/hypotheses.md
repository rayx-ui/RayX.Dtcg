---
agentx:
  kind: hypotheses
  scope: knowledge/release
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T02:20:00+01:00
  topics:
    - release
    - repository-move
  supersedes: []
  superseded_by: []
---
# release: hypotheses

Unverified since the repository became `rayx-ui/RayX.Dtcg`; check before the next release.

- `RELEASE_PLZ_TOKEN` reads `rayx-ui/RayX.Dtcg` (both jobs checked out with it). Whether it may also
  push a tag and open a release pull request there is not yet shown; the first real release will tell.
- The "Release tags" ruleset (`refs/tags/v*` cannot be updated or deleted) may not have moved with the
  repository.
- Consumers pinned to `github.com/dXnSix/dXn.DtcgNsh` at `v0.1.0` rely on GitHub's redirect after the
  transfer.
