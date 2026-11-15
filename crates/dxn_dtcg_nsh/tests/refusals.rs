//! What the export refuses, naming every case: two paths of one name, a
//! name NSH would refuse, contexts whose layers do not compose, an
//! appearance the project has not.

mod common;

use dxn_dtcg_nsh::{Appearance, Options};

fn errors(files: &[(&str, &str)], options: &Options) -> Vec<String> {
    common::run(files, options).unwrap_err().0
}

#[test]
fn two_paths_of_one_name_and_a_name_nsh_refuses() {
    let tokens = r##"{
      "a-b": { "c": { "$type": "number", "$value": 1 } },
      "a": { "b-c": { "$type": "number", "$value": 2 } },
      "font": { "color": { "$type": "color", "$value": { "colorSpace": "srgb", "components": [0, 0, 0] } } }
    }"##;
    let said = errors(&[("tokens.json", tokens)], &Options::default());
    assert!(
        said.iter()
            .any(|e| e.contains("`a-b.c` and `a.b-c` are both named `a-b-c`")
                || e.contains("`a.b-c` and `a-b.c` are both named `a-b-c`")),
        "{said:?}"
    );
    assert!(
        said.iter()
            .any(|e| e.starts_with("`font.color`: a color may not be named in a font's namespace")),
        "{said:?}"
    );
}

/// `contrast` aliases a token `brand` changes: under both, the value is
/// neither layer's.
#[test]
fn contexts_that_change_a_token_together_are_refused() {
    let resolver = r##"{
      "version": "2025.10",
      "sets": { "base": { "sources": [{ "$ref": "base.json" }] } },
      "modifiers": {
        "brand": { "default": "ocean", "contexts": { "ocean": [], "forest": [{ "$ref": "forest.json" }] } },
        "contrast": { "default": "normal", "contexts": { "normal": [], "high": [{ "$ref": "high.json" }] } }
      },
      "resolutionOrder": [{ "$ref": "#/sets/base" }, { "$ref": "#/modifiers/brand" }, { "$ref": "#/modifiers/contrast" }]
    }"##;
    let base = r##"{ "palette": { "$type": "color",
        "strong": { "$value": { "colorSpace": "srgb", "components": [0, 0, 0.5] } },
        "soft": { "$value": { "colorSpace": "srgb", "components": [0.5, 0.5, 1] } } },
      "accent": { "$value": "{palette.soft}" } }"##;
    let forest = r##"{ "palette": { "strong": { "$value": { "colorSpace": "srgb", "components": [0, 0.5, 0] } } } }"##;
    let high = r##"{ "accent": { "$value": "{palette.strong}" } }"##;
    let files = [
        ("design.resolver.json", resolver),
        ("base.json", base),
        ("forest.json", forest),
        ("high.json", high),
    ];
    let said = errors(&files, &Options::default());
    assert_eq!(
        said,
        [
            "`accent` under brand=forest, contrast=high is not what its modifiers' layers give: their contexts change it together"
        ]
    );
}

#[test]
fn an_appearance_the_project_has_not() {
    let resolver = r##"{
      "version": "2025.10",
      "modifiers": {
        "mode": { "default": "day", "contexts": {
          "day": [{ "ink": { "$type": "color", "$value": { "colorSpace": "srgb", "components": [0, 0, 0] } } }],
          "night": [{ "ink": { "$type": "color", "$value": { "colorSpace": "srgb", "components": [1, 1, 1] } } }] } }
      },
      "resolutionOrder": [{ "$ref": "#/modifiers/mode" }]
    }"##;
    let files = [("ui.resolver.json", resolver)];
    let wrong = Options {
        appearance: Some(Appearance {
            modifier: "mode".into(),
            light: "day".into(),
            dark: "dusk".into(),
        }),
        ..Options::default()
    };
    let said = errors(&files, &wrong);
    assert!(
        said.contains(&"the modifier `mode` has no context `dusk`".to_owned()),
        "{said:?}"
    );
    let missing = Options {
        appearance: Some(Appearance {
            modifier: "theme".into(),
            light: "light".into(),
            dark: "dark".into(),
        }),
        ..Options::default()
    };
    assert_eq!(
        errors(&files, &missing),
        ["the project has no modifier `theme`"]
    );
}

/// A named appearance's contexts become `light` and `dark`.
#[test]
fn a_named_appearance_gives_light_and_dark() {
    let resolver = r##"{
      "version": "2025.10",
      "modifiers": { "mode": { "default": "day", "contexts": {
          "day": [{ "ink": { "$type": "color", "$value": { "colorSpace": "srgb", "components": [0, 0, 0] } } }],
          "night": [{ "ink": { "$type": "color", "$value": { "colorSpace": "srgb", "components": [1, 1, 1] } } }] } } },
      "resolutionOrder": [{ "$ref": "#/modifiers/mode" }]
    }"##;
    let options = Options {
        appearance: Some(Appearance {
            modifier: "mode".into(),
            light: "day".into(),
            dark: "night".into(),
        }),
        ..Options::default()
    };
    let export = common::run(&[("ui.resolver.json", resolver)], &options).unwrap();
    assert_eq!(export.document["cssVars"]["light"]["ink"], "#000000");
    assert_eq!(export.document["cssVars"]["dark"]["ink"], "#ffffff");
    assert!(export.document.get("modifiers").is_none());
}

/// A context that adds a token, or gives one another type, is refused: NSH's
/// contexts add no names and change no kind.
#[test]
fn a_context_that_adds_a_token_or_changes_its_type_is_refused() {
    let resolver = r##"{
      "version": "2025.10",
      "sets": { "base": { "sources": [{ "ink": { "$type": "color", "$value": { "colorSpace": "srgb", "components": [0, 0, 0] } } }] } },
      "modifiers": { "brand": { "default": "ocean", "contexts": {
        "ocean": [],
        "forest": [{ "leaf": { "$type": "number", "$value": 1 },
                     "ink": { "$type": "number", "$value": 2 } }] } } },
      "resolutionOrder": [{ "$ref": "#/sets/base" }, { "$ref": "#/modifiers/brand" }]
    }"##;
    let said = errors(&[("d.resolver.json", resolver)], &Options::default());
    assert_eq!(
        said,
        [
            "`ink` is a color or a number in some contexts: a token keeps one type in every context",
            "`leaf` is missing under brand=ocean: a context adds no tokens and removes none",
        ]
    );
}
