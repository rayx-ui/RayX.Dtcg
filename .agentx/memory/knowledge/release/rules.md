---
agentx:
  kind: rules
  scope: knowledge/release
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - release
    - changelog
    - commits
  supersedes: []
  superseded_by: []
---
# release: rules

- Never edit `CHANGELOG.md` by hand; release-plz writes it.
- Before merging a release pull request, check its version and that it changes only `Cargo.toml`,
  `Cargo.lock` and `CHANGELOG.md`.
- Tags never move; a bad release is fixed forward with a new patch.
- A breaking commit states the break in its body and carries a `Migration:` footer.
- Memory-only commits use `chore(agentx): …`, which stays out of the changelog and starts no release.
- Pushing, tagging and merging a release pull request need the owner's go-ahead.
