//! `rayx-dtcg`: exports a DTCG project as an NSH theme document.
//!
//! ```text
//! rayx-dtcg <ROOT> -o <FILE> [--role-map <FILE>] [--roles <FILE>] [--rem <PX>]
//!           [--max-combinations <N>] [--check]
//! ```
//!
//! `--check` writes nothing: it fails (exit 1) when `-o`'s file is missing
//! or is not what the export gives, for CI. A refused project fails (exit 1)
//! with every reason; a usage error exits with 2.

use std::{path::PathBuf, process::ExitCode};

use rayx_dtcg::{Options, export};

const USAGE: &str = "usage: rayx-dtcg <ROOT> -o <FILE> [--role-map <FILE>] [--roles <FILE>] [--rem <PX>] [--max-combinations <N>] [--check]";

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("rayx-dtcg: {message}");
            ExitCode::from(2)
        }
    }
}

fn read(path: &PathBuf) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn run(args: Vec<String>) -> Result<ExitCode, String> {
    let mut root = None;
    let mut out = None;
    let mut role_map = None;
    let mut roles = None;
    let mut check = false;
    let mut options = Options::default();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        let mut value = |flag: &str| {
            args.next()
                .ok_or_else(|| format!("{flag} takes a value\n{USAGE}"))
        };
        match arg.as_str() {
            "-o" | "--out" => out = Some(PathBuf::from(value("-o")?)),
            "--role-map" => role_map = Some(PathBuf::from(value("--role-map")?)),
            "--roles" => roles = Some(PathBuf::from(value("--roles")?)),
            "--rem" => {
                options.rem = value("--rem")?
                    .parse()
                    .map_err(|_| format!("--rem takes a number of pixels\n{USAGE}"))?;
            }
            "--max-combinations" => {
                options.max_combinations = value("--max-combinations")?
                    .parse()
                    .map_err(|_| format!("--max-combinations takes a whole number\n{USAGE}"))?;
            }
            "--check" => check = true,
            "--appearance" => {
                return Err(format!(
                    "--appearance was removed: light and dark are an ordinary modifier of the theme document now\n{USAGE}"
                ));
            }
            "--names" => {
                return Err(format!(
                    "--names was removed: each token's DTCG path is its `path` in the theme document\n{USAGE}"
                ));
            }
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(ExitCode::SUCCESS);
            }
            _ if arg.starts_with('-') => return Err(format!("unknown option {arg}\n{USAGE}")),
            _ if root.is_none() => root = Some(PathBuf::from(arg)),
            _ => return Err(format!("one root document only\n{USAGE}")),
        }
    }
    let root = root.ok_or(USAGE)?;
    let out = out.ok_or_else(|| format!("-o <FILE> is required\n{USAGE}"))?;
    if let Some(path) = &role_map {
        options.role_map = Some(read(path)?);
    }
    if let Some(path) = &roles {
        options.roles = Some(read(path)?);
    }
    // Resource ids are relative to the root's folder, as its `$ref`s are.
    let dir = root.parent().map(PathBuf::from).unwrap_or_default();
    let id = root
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("{} is no file", root.display()))?
        .to_owned();
    let load = |id: &str| std::fs::read_to_string(dir.join(id)).ok();
    let export = match export(&id, &load, &options) {
        Ok(export) => export,
        Err(e) => {
            for line in &e.0 {
                eprintln!("rayx-dtcg: {line}");
            }
            return Ok(ExitCode::FAILURE);
        }
    };
    for line in &export.warnings {
        eprintln!("rayx-dtcg: {line}");
    }
    let text = export.text();
    if check {
        return Ok(match std::fs::read_to_string(&out) {
            Ok(current) if current == text => ExitCode::SUCCESS,
            Ok(_) => {
                eprintln!(
                    "rayx-dtcg: {} is stale: it is not the export of {}; run rayx-dtcg without --check",
                    out.display(),
                    root.display()
                );
                ExitCode::FAILURE
            }
            Err(_) => {
                eprintln!(
                    "rayx-dtcg: {} is missing: run rayx-dtcg without --check to write it",
                    out.display()
                );
                ExitCode::FAILURE
            }
        });
    }
    std::fs::write(&out, text).map_err(|e| format!("{}: {e}", out.display()))?;
    Ok(ExitCode::SUCCESS)
}
