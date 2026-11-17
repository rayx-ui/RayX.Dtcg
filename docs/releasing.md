# Releasing `rayx_dtcg`

A release is a version tag `vX.Y.Z` on `main` with a GitHub Release of the same name. Nothing is
published to crates.io: consumers depend on the tag,

```toml
[dependencies]
rayx_dtcg = { git = "https://github.com/rayx-ui/RayX.Dtcg", tag = "vX.Y.Z" }
```

and `Cargo.lock` records the commit it points to. The command installs from the same tag:

```bash
cargo install --git https://github.com/rayx-ui/RayX.Dtcg --tag vX.Y.Z rayx_dtcg
```

`v0.1.0` was released as `dxn_dtcg_nsh` (command `dtcg-nsh`) from `dXnSix/dXn.DtcgNsh`, before the
repository became `rayx-ui/RayX.Dtcg`.

No registry release is made, and none can be while `dxn_dtcg` comes by git tag only: CI checks the
package's file list (`cargo package --workspace --list`), not a registry build.

**release-plz** does the work (`release-plz.toml`, `.github/workflows/release-plz.yml`) in
git-only mode (`git_only = true`, `publish = false`): versions come from the `v*` tags, and on
every push to `main` it

- keeps a release pull request with the next version and its `CHANGELOG.md` section, and
- when `Cargo.toml` holds a version that has no tag yet, tags `vX.Y.Z` and creates the GitHub
  Release with that version's `CHANGELOG.md` section as its notes.

The first release, `v0.1.0`, needs no pull request: `Cargo.toml` says `0.1.0` and no `v*` tag
exists, so the `release` job tags it on the first push that holds the workflow. Its changelog
section was written by hand; later sections are generated.

## The 0.x policy

| Commit | Release |
| --- | --- |
| `fix:`, `perf:`, `feat:` | patch (`0.1.0` → `0.1.1`) |
| breaking: `!` after the type or scope, or a `BREAKING CHANGE:` footer | minor (`0.1.x` → `0.2.0`) |
| `docs:`, `test:`, `ci:`, `build:`, `refactor:`, `chore:` | none on their own; listed in the next release's changelog |

A breaking commit's body states the break and a `Migration:` footer tells consumers what to
change (a theme document's shape, the command's flags, the Rust API); the changelog prints it under
the entry. `chore(agentx)`, `chore(release)` and `chore(deps)` commits are left out of the
changelog. `CHANGELOG.md` is written by release-plz and never edited by hand between releases.

## Taking a new `dxn_dtcg` release

release-plz counts only the commits that change the crate's packaged files
(`crates/rayx_dtcg/` with the `README.md` it ships). That is why `dxn_dtcg` is declared in
`crates/rayx_dtcg/Cargo.toml`, not in the workspace table. Move its tag in a `fix(deps):`
commit (or `feat(deps):` when the new release brings something the export uses), so that it
releases `rayx_dtcg` on it and consumers can read which `dxn_dtcg` tag to pair with. A change
this crate needs from dXn.Dtcg is released there first.

## Before merging a release pull request

- The proposed version is the expected one: a minor bump only when a commit since the last tag is
  breaking.
- It changes only `Cargo.toml`, `Cargo.lock` and `CHANGELOG.md`.
- Its CI is green.

A line of the changelog that reads badly is fixed on the release pull request right before merging
it; when `main` moves on, release-plz replaces a release pull request that carries human commits.

## The `RELEASE_PLZ_TOKEN` secret

The release jobs use the fine-grained personal access token the `dXnSix` library repositories
share (Contents and Pull requests read and write), stored as the repository secret
`RELEASE_PLZ_TOKEN`; this repository must be on the token's access list. Events made with the
default `GITHUB_TOKEN` start no workflows, so with it the release pull request would get no CI.
When the token expires the Release-plz runs fail at checkout and releases stop until it is
replaced (see dXn.Dtcg's `docs/releasing.md` for rotation).

## Tags never move

The tag ruleset "Release tags" covers `refs/tags/v*`: tags can be created, but not updated or
deleted. A bad release is fixed forward with a new patch release.

## Pushing to `main`

Work goes straight to `main`, rebased on the remote first:

```bash
git fetch origin && git rebase origin/main
git push --force-with-lease origin main
```

There are no feature branches and no pull requests except release-plz's release pull requests.
