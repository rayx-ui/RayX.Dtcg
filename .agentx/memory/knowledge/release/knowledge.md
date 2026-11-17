---
agentx:
  kind: knowledge
  scope: knowledge/release
  status: current
  created: 2026-11-17T01:30:00+01:00
  updated: 2026-11-17T02:45:00+01:00
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
  request. In `rayx-ui/RayX.Dtcg` the token checks out, pushes tags and creates GitHub Releases (it
  released `v0.2.0` on 2026-11-17).
- A package rename breaks `release-pr` until a tag holds the new name: release-plz compares the package
  with the latest tag's tree. The first release after a rename is made by hand (version and changelog
  section in one `chore(release):` commit; `docs/releasing.md`).
- The planned switch to the RayX format is one breaking commit (`feat!: …` with a `Migration:` footer),
  so release-plz proposes `v0.3.0`.
