---
agentx:
  kind: knowledge
  scope: knowledge/release
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T01:26:10+01:00
  topics:
    - release-plz
    - dependencies
  supersedes: []
  superseded_by: []
---
# release: knowledge

- release-plz counts only commits that change the crate's packaged files (`crates/rayx_dtcg/` and the
  `README.md` it ships). That is why `dxn_dtcg` is declared in the crate's `Cargo.toml`: moving its tag
  (`fix(deps):`, or `feat(deps):` when the export uses something new) releases this crate.
- CI checks `cargo package --workspace --list`, not a registry build, because `dxn_dtcg` comes by git tag
  only.
- The release jobs use the `RELEASE_PLZ_TOKEN` secret (a fine-grained token shared by the dXnSix library
  repositories); events made with the default `GITHUB_TOKEN` would start no CI on the release pull
  request.
- The planned switch to the RayX format is one breaking commit (`feat!: …` with a `Migration:` footer),
  so release-plz proposes `v0.2.0`.
