---
agentx:
  kind: hypotheses
  scope: knowledge/release
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - release
    - repository-move
  supersedes: []
  superseded_by: []
---
# release: hypotheses

Unverified since the repository became `rayx-ui/RayX.Dtcg`; check before the next release.

- `RELEASE_PLZ_TOKEN` was made for the dXnSix library repositories. A fine-grained token has one
  resource owner, so it likely cannot reach a `rayx-ui` repository; the release jobs would then fail at
  checkout until a token owned by `rayx-ui` replaces it.
- The "Release tags" ruleset (`refs/tags/v*` cannot be updated or deleted) may not have moved with the
  repository.
- Consumers pinned to `github.com/dXnSix/dXn.DtcgNsh` at `v0.1.0` rely on GitHub's redirect after the
  transfer.
