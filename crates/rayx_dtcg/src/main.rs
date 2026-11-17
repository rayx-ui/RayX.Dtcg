//! `rayx-dtcg`: exports a DTCG project as an NSH theme document.
//!
//! ```text
//! rayx-dtcg <root.json> [-o <theme.json>] [--appearance <modifier>[=<light>,<dark>]]
//!          [--rem <px>] [--names <names.json>] [--check]
//! ```
//!
//! `--check` writes nothing: it fails (exit 1) when `-o`'s file is not what
//! the export gives, for CI. `--names` writes each token's path and the NSH
//! name it took.

use std::{path::PathBuf, process::ExitCode};

use rayx_dtcg::{Appearance, Options, export};

const USAGE: &str = "usage: rayx-dtcg <root.json> [-o <theme.json>] [--appearance <modifier>[=<light>,<dark>]] [--rem <px>] [--names <names.json>] [--check]";

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(code) => code,
        Err(message) => {
            eprintln!("rayx-dtcg: {message}");
            ExitCode::from(2)
        }
    }
}

fn run(args: Vec<String>) -> Result<ExitCode, String> {
    let mut root = None;
    let mut out = None;
    let mut names = None;
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
            "--names" => names = Some(PathBuf::from(value("--names")?)),
            "--rem" => {
                options.rem = value("--rem")?
                    .parse()
                    .map_err(|_| format!("--rem takes a number of pixels\n{USAGE}"))?;
            }
            "--appearance" => {
                let spec = value("--appearance")?;
                let (modifier, contexts) = spec
                    .split_once('=')
                    .unwrap_or((spec.as_str(), "light,dark"));
                let Some((light, dark)) = contexts.split_once(',') else {
                    return Err(format!(
                        "--appearance takes <modifier>=<light>,<dark>\n{USAGE}"
                    ));
                };
                options.appearance = Some(Appearance {
                    modifier: modifier.to_owned(),
                    light: light.to_owned(),
                    dark: dark.to_owned(),
                });
            }
            "--check" => check = true,
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
    for line in &export.left_out {
        eprintln!("rayx-dtcg: left out: {line}");
    }
    let text = export.text();
    if let Some(path) = &names {
        let list: serde_json::Map<String, serde_json::Value> = export
            .names
            .iter()
            .map(|(p, n)| (p.clone(), n.clone().into()))
            .collect();
        let mut json = serde_json::to_string_pretty(&list).unwrap_or_default();
        json.push('\n');
        std::fs::write(path, json).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    match (out, check) {
        (Some(path), true) => {
            let current = std::fs::read_to_string(&path).unwrap_or_default();
            if current == text {
                Ok(ExitCode::SUCCESS)
            } else {
                eprintln!(
                    "rayx-dtcg: {} is not the export of {}: run rayx-dtcg without --check",
                    path.display(),
                    root.display()
                );
                Ok(ExitCode::FAILURE)
            }
        }
        (None, true) => Err(format!("--check needs -o <theme.json>\n{USAGE}")),
        (Some(path), false) => {
            std::fs::write(&path, text).map_err(|e| format!("{}: {e}", path.display()))?;
            Ok(ExitCode::SUCCESS)
        }
        (None, false) => {
            print!("{text}");
            Ok(ExitCode::SUCCESS)
        }
    }
}
