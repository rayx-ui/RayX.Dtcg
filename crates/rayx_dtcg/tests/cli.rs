//! `rayx-dtcg`: writing, `--check` on a current, a stale and a missing
//! file, `--max-combinations`, the role files, a refusal's exit code and
//! message, and the removed `--appearance` and `--names`.

mod common;

use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

fn project_dir(tag: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("cli-{tag}"));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (name, text) in common::project() {
        std::fs::write(dir.join(name), text).unwrap();
    }
    dir
}

fn rayx_dtcg(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rayx-dtcg"))
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn it_writes_the_export() {
    let dir = project_dir("write");
    let out = rayx_dtcg(&dir, &["acme.resolver.json", "-o", "theme.json"]);
    assert!(out.status.success(), "{}", stderr(&out));
    let written = std::fs::read_to_string(dir.join("theme.json")).unwrap();
    let export = common::run(&common::project(), &rayx_dtcg::Options::default()).unwrap();
    assert_eq!(written, export.text());
}

#[test]
fn check_passes_a_current_file_and_fails_a_stale_or_missing_one() {
    let dir = project_dir("check");
    assert!(
        rayx_dtcg(&dir, &["acme.resolver.json", "-o", "theme.json"])
            .status
            .success()
    );
    let current = rayx_dtcg(&dir, &["acme.resolver.json", "-o", "theme.json", "--check"]);
    assert!(current.status.success(), "{}", stderr(&current));

    let mut text = std::fs::read_to_string(dir.join("theme.json")).unwrap();
    text.push(' ');
    std::fs::write(dir.join("theme.json"), &text).unwrap();
    let stale = rayx_dtcg(&dir, &["acme.resolver.json", "-o", "theme.json", "--check"]);
    assert_eq!(stale.status.code(), Some(1));
    assert!(
        stderr(&stale).contains("theme.json is stale"),
        "{}",
        stderr(&stale)
    );
    assert_eq!(
        std::fs::read_to_string(dir.join("theme.json")).unwrap(),
        text,
        "--check writes nothing"
    );

    let missing = rayx_dtcg(
        &dir,
        &["acme.resolver.json", "-o", "absent.json", "--check"],
    );
    assert_eq!(missing.status.code(), Some(1));
    assert!(
        stderr(&missing).contains("absent.json is missing"),
        "{}",
        stderr(&missing)
    );
    assert!(!dir.join("absent.json").exists());
}

#[test]
fn max_combinations_bounds_the_checks() {
    let dir = project_dir("max");
    let low = rayx_dtcg(
        &dir,
        &[
            "acme.resolver.json",
            "-o",
            "theme.json",
            "--max-combinations",
            "4",
        ],
    );
    assert_eq!(low.status.code(), Some(1));
    assert!(stderr(&low).contains("8 combinations"), "{}", stderr(&low));
    let enough = rayx_dtcg(
        &dir,
        &[
            "acme.resolver.json",
            "-o",
            "theme.json",
            "--max-combinations",
            "8",
        ],
    );
    assert!(enough.status.success(), "{}", stderr(&enough));
}

#[test]
fn the_role_files_are_read() {
    let dir = project_dir("roles");
    std::fs::write(
        dir.join("roles.map.json"),
        r#"{ "primary-bg": "colors.accent", "gutter": "spacing.gutter" }"#,
    )
    .unwrap();
    std::fs::write(
        dir.join("roles.json"),
        r#"[{ "name": "primary-bg", "type": "color", "required": true }]"#,
    )
    .unwrap();
    let out = rayx_dtcg(
        &dir,
        &[
            "acme.resolver.json",
            "-o",
            "theme.json",
            "--role-map",
            "roles.map.json",
            "--roles",
            "roles.json",
            "--rem",
            "10",
        ],
    );
    assert!(out.status.success(), "{}", stderr(&out));
    assert!(
        stderr(&out).contains("warning role-unknown /roles/gutter:"),
        "{}",
        stderr(&out)
    );
    let written: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(dir.join("theme.json")).unwrap()).unwrap();
    assert_eq!(written["roles"]["primary-bg"], "colors-accent");
    assert_eq!(written["tokens"]["spacing-gutter"]["value"], 10, "--rem");
}

#[test]
fn a_refusal_exits_with_1_and_prints_every_reason() {
    let dir = project_dir("refusal");
    std::fs::write(
        dir.join("foundation.json"),
        r#"{ "a": { "$type": "strokeStyle", "$value": "solid" }, "b": { "$type": "strokeStyle", "$value": "dashed" } }"#,
    )
    .unwrap();
    std::fs::write(dir.join("dark.json"), "{}").unwrap();
    std::fs::write(dir.join("forest.json"), "{}").unwrap();
    std::fs::write(dir.join("compact.json"), "{}").unwrap();
    let out = rayx_dtcg(&dir, &["acme.resolver.json", "-o", "theme.json"]);
    assert_eq!(out.status.code(), Some(1));
    let err = stderr(&out);
    assert!(
        err.contains("rayx-dtcg: `a`: a `strokeStyle` token"),
        "{err}"
    );
    assert!(
        err.contains("rayx-dtcg: `b`: a `strokeStyle` token"),
        "{err}"
    );
    assert!(!dir.join("theme.json").exists());
}

#[test]
fn the_removed_flags_are_usage_errors_that_name_them() {
    let dir = project_dir("removed");
    for flag in ["--appearance", "--names"] {
        let out = rayx_dtcg(&dir, &["acme.resolver.json", "-o", "theme.json", flag, "x"]);
        assert_eq!(out.status.code(), Some(2), "{flag}");
        assert!(
            stderr(&out).contains(&format!("{flag} was removed")),
            "{}",
            stderr(&out)
        );
    }
    let out = rayx_dtcg(&dir, &["acme.resolver.json"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(
        stderr(&out).contains("-o <FILE> is required"),
        "{}",
        stderr(&out)
    );
}
