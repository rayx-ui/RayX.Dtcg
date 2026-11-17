# Changelog

All notable changes to `rayx_dtcg` (the library and the `rayx-dtcg` command) are documented in
this file. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the
project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html). It is generated from
Conventional Commits by release-plz.

## [0.2.0] - 2026-11-17

### Documentation
- plan repurposing the exporter for RayX's theme format
- *(release)* explain releasing by hand after a package rename

### Refactoring
- [**breaking**] rename the crate to rayx_dtcg and the command to rayx-dtcg
  *Migration:* depend on `rayx_dtcg = { git = "https://github.com/rayx-ui/RayX.Dtcg", tag = "..." }` and write `use rayx_dtcg::...` for `use dxn_dtcg_nsh::...`; install with `cargo install --git https://github.com/rayx-ui/RayX.Dtcg --tag ... rayx_dtcg` and run `rayx-dtcg` for `dtcg-nsh`.

## [0.1.0] - 2026-11-16

### Features
- export DTCG projects as NSH theme documents
- check one structure across contexts, and export strings and booleans

### Build and CI
- depend on dxn_dtcg v1.4.0
- release with release-plz and test on every platform
