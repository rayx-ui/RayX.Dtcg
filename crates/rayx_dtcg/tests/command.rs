//! `rayx-dtcg`: writes the export, `--check` fails on a file that is not
//! the export any more, `--names` writes each path's NSH name, and a
//! refused project fails with every reason.

mod common;

use std::{path::PathBuf, process::Command};

fn project_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rayx-dtcg-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (name, text) in common::project() {
        std::fs::write(dir.join(name), text).unwrap();
    }
    dir
}

fn rayx_dtcg(dir: &PathBuf, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rayx-dtcg"))
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn it_writes_the_export_and_checks_it() {
    let dir = project_dir("check");
    let out = rayx_dtcg(
        &dir,
        &[
            "acme.resolver.json",
            "-o",
            "theme.json",
            "--names",
            "names.json",
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stderr).contains("left out: `outline`"));
    let theme: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("theme.json")).unwrap()).unwrap();
    assert_eq!(theme["cssVars"]["theme"]["radius-card"], "12px");
    let names: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("names.json")).unwrap()).unwrap();
    assert_eq!(names["brand.name"], "primary");
    // Current: the check passes.
    let check = rayx_dtcg(&dir, &["acme.resolver.json", "-o", "theme.json", "--check"]);
    assert!(
        check.status.success(),
        "{}",
        String::from_utf8_lossy(&check.stderr)
    );
    // The source changes: the file is not its export any more.
    std::fs::write(
        dir.join("compact.json"),
        r#"{ "radius": { "card": { "$value": { "value": 4, "unit": "px" } } } }"#,
    )
    .unwrap();
    let check = rayx_dtcg(&dir, &["acme.resolver.json", "-o", "theme.json", "--check"]);
    assert_eq!(check.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&check.stderr)
            .contains("theme.json is not the export of acme.resolver.json")
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_refused_project_fails_with_every_reason() {
    let dir = project_dir("refused");
    std::fs::write(
        dir.join("clash.json"),
        r#"{ "a-b": { "c": { "$type": "number", "$value": 1 } }, "a": { "b-c": { "$type": "number", "$value": 2 } } }"#,
    )
    .unwrap();
    let out = rayx_dtcg(&dir, &["clash.json"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("are both named `a-b-c`"));
    let _ = std::fs::remove_dir_all(&dir);
}
