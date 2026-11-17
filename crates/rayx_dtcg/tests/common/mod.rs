//! A project of files in memory, as `export`'s loader reads them.

#![allow(dead_code)]

use std::collections::HashMap;

use rayx_dtcg::{Export, ExportError, Options, export};

/// Exports the project `files` (resource id, text), rooted at the first.
pub fn run(files: &[(&str, &str)], options: &Options) -> Result<Export, ExportError> {
    let map: HashMap<String, String> = files
        .iter()
        .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
        .collect();
    let load = move |id: &str| map.get(id).cloned();
    export(files[0].0, &load, options)
}

/// A design system: a foundation set, an appearance, a brand and a density.
pub const RESOLVER: &str = r##"{
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
    "overlay": { "$value": { "colorSpace": "srgb", "components": [0, 0, 0], "alpha": 0.5 } }
  },
  "radius": { "$type": "dimension", "card": { "$value": { "value": 12, "unit": "px" } } },
  "spacing": { "$type": "dimension", "gutter": { "$value": { "value": 1, "unit": "rem" } } },
  "gap": {
    "$type": "dimension",
    "md": { "$value": { "value": 8, "unit": "px" }, "$extensions": { "com.figma.scopes": ["GAP"] } }
  },
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
  "gradients": { "hero": {
    "$type": "gradient",
    "$extensions": { "dev.nsh": { "gradient": { "kind": "linear", "angle": 90 } } },
    "$value": [
      { "color": { "colorSpace": "srgb", "components": [1, 0, 0] }, "position": 0 },
      { "color": { "colorSpace": "srgb", "components": [0, 0, 1] }, "position": 1 } ] } },
  "brand": { "name": { "$type": "color", "$extensions": { "dev.nsh": { "name": "primary" } },
    "$value": { "colorSpace": "srgb", "components": [0.1, 0.1, 0.1] } } },
  "outline": { "$type": "strokeStyle", "$value": "dashed" }
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
