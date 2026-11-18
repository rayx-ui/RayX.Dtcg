//! The dependency tree: `nsh-theme` and `dxn_dtcg` from git tags, and no
//! UI or app framework (no `nsh-*` but `nsh-theme`, no `gpui*`, no `rayx*`
//! but this crate). CI's guard rejects the same.

use std::{path::PathBuf, process::Command};

use serde_json::Value;

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn metadata() -> Value {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let out = Command::new(cargo)
        .current_dir(workspace())
        .args(["metadata", "--format-version", "1", "--locked", "--offline"])
        .output()
        .expect("cargo metadata runs");
    assert!(
        out.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).unwrap()
}

/// Each package of the tree as `(name, source)`; this crate's source is none.
fn packages() -> Vec<(String, Option<String>)> {
    metadata()["packages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            (
                p["name"].as_str().unwrap().to_owned(),
                p["source"].as_str().map(str::to_owned),
            )
        })
        .collect()
}

/// Why `name` may not be in the tree, if it may not.
fn disallowed(name: &str) -> Option<&'static str> {
    if name.starts_with("nsh-") && name != "nsh-theme" {
        Some("an NSH crate other than nsh-theme")
    } else if name.starts_with("gpui") {
        Some("a GPUI crate")
    } else if name.starts_with("rayx") && name != "rayx_dtcg" {
        Some("a RayX crate")
    } else {
        None
    }
}

/// The tag a git source names: `git+<url>?tag=<tag>#<commit>`.
fn git_tag<'a>(source: &'a str, url: &str) -> Result<&'a str, String> {
    let rest = source
        .strip_prefix(&format!("git+{url}?tag="))
        .ok_or_else(|| format!("{source} is not {url} at a git tag"))?;
    let (tag, commit) = rest
        .split_once('#')
        .ok_or_else(|| format!("{source} records no commit"))?;
    if commit.len() != 40 {
        return Err(format!("{source} records no full commit"));
    }
    Ok(tag)
}

#[test]
fn nsh_theme_and_dxn_dtcg_come_from_git_tags() {
    let packages = packages();
    let source = |name: &str| {
        packages
            .iter()
            .find(|(n, _)| n == name)
            .and_then(|(_, s)| s.clone())
            .unwrap_or_else(|| panic!("{name} is not in the tree"))
    };
    let nsh_source = source("nsh-theme");
    let nsh = git_tag(&nsh_source, "https://github.com/dXnSix/nsh").unwrap();
    assert!(
        nsh.starts_with("nshx-"),
        "nsh-theme comes from `{nsh}`, not an `nshx-*` tag"
    );
    let dtcg_source = source("dxn_dtcg");
    let dtcg = git_tag(&dtcg_source, "https://github.com/dXnSix/dXn.Dtcg").unwrap();
    assert!(
        dtcg.starts_with('v'),
        "dxn_dtcg comes from `{dtcg}`, not a release tag"
    );
}

#[test]
fn every_nsh_and_dxn_dtcg_source_is_a_git_tag() {
    for (name, source) in packages() {
        let Some(source) = source else { continue };
        let url = if name.starts_with("nsh-") {
            "https://github.com/dXnSix/nsh"
        } else if name.starts_with("dxn_dtcg") {
            "https://github.com/dXnSix/dXn.Dtcg"
        } else {
            continue;
        };
        if let Err(e) = git_tag(&source, url) {
            panic!("`{name}`: {e}");
        }
    }
}

#[test]
fn no_ui_or_app_framework_is_in_the_tree() {
    let found: Vec<String> = packages()
        .into_iter()
        .filter_map(|(name, _)| disallowed(&name).map(|why| format!("`{name}` ({why})")))
        .collect();
    assert!(found.is_empty(), "disallowed crates: {}", found.join(", "));
}

#[test]
fn the_check_names_a_disallowed_crate() {
    assert_eq!(disallowed("nsh-theme"), None);
    assert_eq!(disallowed("rayx_dtcg"), None);
    assert!(disallowed("nsh-core").is_some());
    assert!(disallowed("gpui").is_some());
    assert!(disallowed("rayx_app").is_some());
    assert!(
        git_tag(
            "git+https://github.com/dXnSix/nsh?branch=main#0123",
            "https://github.com/dXnSix/nsh"
        )
        .is_err()
    );
    assert!(
        git_tag(
            "git+https://github.com/dXnSix/nsh#df7b2dfa6d62e76f30477bb424a3f41cfea6600c",
            "https://github.com/dXnSix/nsh"
        )
        .is_err()
    );
}

/// The `run` script of CI's dependency guard.
fn guard() -> String {
    let ci = std::fs::read_to_string(workspace().join(".github/workflows/ci.yml")).unwrap();
    let mut lines = ci
        .lines()
        .skip_while(|l| !l.contains("name: No UI framework in the dependency tree"));
    lines.next().expect("CI has the dependency guard");
    let mut script = String::new();
    let mut indent = None;
    for line in lines
        .skip_while(|l| !l.trim_start().starts_with("run: |"))
        .skip(1)
    {
        let spaces = line.len() - line.trim_start().len();
        if !line.trim().is_empty() && indent.is_some_and(|i| spaces < i) {
            break;
        }
        indent.get_or_insert(spaces);
        script.push_str(line.trim_start());
        script.push('\n');
    }
    script
}

#[test]
fn ci_guards_the_tree() {
    let script = guard();
    assert!(
        script.contains("cargo tree --workspace --edges all"),
        "{script}"
    );
    assert!(script.contains("nsh-theme"), "{script}");
}

/// Runs CI's guard with `cargo tree` printing `tree`.
#[cfg(unix)]
fn guard_passes(tree: &str) -> bool {
    use std::os::unix::fs::PermissionsExt;
    let dir = workspace().join(format!(
        "target/tmp/dependency-guard-{}-{}",
        std::process::id(),
        tree.len()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("tree.txt"), tree).unwrap();
    let cargo = dir.join("cargo");
    std::fs::write(
        &cargo,
        format!("#!/bin/sh\ncat '{}'\n", dir.join("tree.txt").display()),
    )
    .unwrap();
    std::fs::set_permissions(&cargo, std::fs::Permissions::from_mode(0o755)).unwrap();
    let path = format!(
        "{}:{}",
        dir.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let status = Command::new("bash")
        .args(["-c", &guard()])
        .env("PATH", path)
        .output()
        .expect("bash runs");
    status.status.success()
}

#[cfg(unix)]
#[test]
fn ci_guard_allows_nsh_theme_alone() {
    let allowed = "rayx_dtcg v0.2.0 (/w)\ndxn_dtcg v1.4.0\nnsh-theme v0.6.0 (https://github.com/dXnSix/nsh?tag=nshx-0.6.0-1#df7b2dfa)\nnsh-theme feature \"default\"\nserde v1.0.0\n";
    assert!(guard_passes(allowed));
    for crate_line in [
        "nsh-core v0.6.0",
        "nsh-theme-extra v0.1.0",
        "gpui v0.2.0",
        "rayx_app v0.12.0",
    ] {
        assert!(
            !guard_passes(&format!("{allowed}{crate_line}\n")),
            "the guard passes a tree with {crate_line}"
        );
    }
}
