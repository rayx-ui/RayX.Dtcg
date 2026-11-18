//! Projects of files in memory, as `export`'s loader reads them.

#![allow(dead_code)]

use std::collections::HashMap;

use rayx_dtcg::{Export, ExportError, Options, export, verify_exact};
use serde_json::Value;

/// Exports the project `files` (resource id, text), rooted at the first.
pub fn run(files: &[(&str, &str)], options: &Options) -> Result<Export, ExportError> {
    let load = loader(files);
    export(files[0].0, &load, options)
}

/// [`verify_exact`] on the project `files` and `document`.
pub fn verify(
    files: &[(&str, &str)],
    options: &Options,
    document: &nsh_theme::ThemeDocument,
) -> Result<(), ExportError> {
    let load = loader(files);
    verify_exact(files[0].0, &load, options, document)
}

fn loader(files: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> + use<> {
    let map: HashMap<String, String> = files
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect();
    move |id: &str| map.get(id).cloned()
}

/// Exports a token document `text` (`tokens.json`) with the default options.
pub fn tokens(text: &str) -> Result<Export, ExportError> {
    run(&[("tokens.json", text)], &Options::default())
}

/// The export's text read back by NSH: it must give no error.
pub fn read_back(export: &Export) -> nsh_theme::ThemeDocument {
    let read = nsh_theme::read(&export.text());
    let errors: Vec<_> = read
        .diagnostics
        .iter()
        .filter(|d| d.is_error())
        .map(ToString::to_string)
        .collect();
    assert!(errors.is_empty(), "NSH refuses the export: {errors:?}");
    read.document
}

/// The written JSON of the export.
pub fn json(export: &Export) -> Value {
    serde_json::from_str(&export.text()).unwrap()
}

/// The written JSON entry of the token `name`.
pub fn entry(export: &Export, name: &str) -> Value {
    json(export)["tokens"][name].clone()
}

/// Whether some reason of `error` contains every one of `parts`.
pub fn refused_with(error: &ExportError, parts: &[&str]) -> bool {
    error
        .0
        .iter()
        .any(|reason| parts.iter().all(|p| reason.contains(p)))
}

/// Asserts that `result` is refused with a reason containing every one of
/// `parts`.
pub fn assert_refused(result: Result<Export, ExportError>, parts: &[&str]) {
    match result {
        Ok(export) => panic!(
            "exported, expected a refusal with {parts:?}:\n{}",
            export.text()
        ),
        Err(error) => assert!(
            refused_with(&error, parts),
            "no reason contains {parts:?}: {error:#?}"
        ),
    }
}

/// A design system: a foundation set, an appearance, a brand and a density.
pub const RESOLVER: &str = r##"{
  "name": "Acme",
  "version": "2025.10",
  "sets": { "foundation": { "sources": [{ "$ref": "foundation.json" }] } },
  "modifiers": {
    "theme": { "default": "light", "contexts": { "light": [], "dark": [{ "$ref": "dark.json" }] } },
    "brand": { "default": "ocean", "contexts": { "ocean": [], "forest": [{ "$ref": "forest.json" }] } },
    "density": { "default": "comfortable", "contexts": { "comfortable": [], "compact": [{ "$ref": "compact.json" }] } }
  },
  "resolutionOrder": [
    { "$ref": "#/sets/foundation" },
    { "$ref": "#/modifiers/theme" },
    { "$ref": "#/modifiers/brand" },
    { "$ref": "#/modifiers/density" }
  ]
}"##;

pub const FOUNDATION: &str = r##"{
  "colors": {
    "$type": "color",
    "backgrounds": {
      "primary": { "$value": { "colorSpace": "srgb", "components": [1, 1, 1] }, "$description": "The page." }
    },
    "accent": { "$value": { "colorSpace": "srgb", "components": [0, 0, 1] } },
    "overlay": { "$value": { "colorSpace": "srgb", "components": [0, 0, 0], "alpha": 0.5 } },
    "link": { "$value": "{colors.accent}" }
  },
  "radius": { "$type": "dimension", "card": { "$value": { "value": 12, "unit": "px" } } },
  "spacing": { "$type": "dimension", "gutter": { "$value": { "value": 1, "unit": "rem" } } },
  "type": {
    "title": { "$type": "typography", "$value": {
      "fontFamily": ["Inter"], "fontSize": { "value": 24, "unit": "px" }, "fontWeight": 650,
      "letterSpacing": { "value": -0.5, "unit": "px" }, "lineHeight": 1.25 } },
    "body": { "$type": "fontFamily", "$value": ["Inter", "sans-serif"] }
  },
  "elevation": { "raised": { "$type": "shadow", "$value": {
    "color": { "colorSpace": "srgb", "components": [0, 0, 0], "alpha": 0.1 },
    "offsetX": { "value": 0, "unit": "px" }, "offsetY": { "value": 1, "unit": "px" },
    "blur": { "value": 2, "unit": "px" }, "spread": { "value": 0, "unit": "px" } } } },
  "motion": {
    "fast": { "$type": "duration", "$value": { "value": 120, "unit": "ms" } },
    "snappy": { "$type": "cubicBezier", "$value": [0.2, 0, 0, 1] }
  },
  "brand": { "name": { "$type": "color", "$extensions": { "dev.nsh": { "name": "primary" } },
    "$value": { "colorSpace": "srgb", "components": [0.1, 0.1, 0.1] } } }
}"##;

pub const DARK: &str = r##"{
  "colors": {
    "backgrounds": { "primary": { "$value": { "colorSpace": "srgb", "components": [0, 0, 0] } } },
    "accent": { "$value": { "colorSpace": "srgb", "components": [0.5, 0.5, 1] } }
  }
}"##;

pub const FOREST: &str = r##"{
  "colors": { "accent": { "$value": { "colorSpace": "srgb", "components": [0, 0.5, 0] } } }
}"##;

pub const COMPACT: &str = r##"{
  "radius": { "card": { "$value": { "value": 6, "unit": "px" } } }
}"##;

/// The design system's files.
pub fn project() -> Vec<(&'static str, &'static str)> {
    vec![
        ("acme.resolver.json", RESOLVER),
        ("foundation.json", FOUNDATION),
        ("dark.json", DARK),
        ("forest.json", FOREST),
        ("compact.json", COMPACT),
    ]
}
