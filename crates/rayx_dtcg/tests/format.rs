//! The document's members, token names (pins, kebab paths, collisions),
//! value types (the table, the override), paths, docs, deterministic bytes,
//! and the refusals.

mod common;

use common::{assert_refused, entry, json, project, read_back, refused_with, run, tokens};
use nsh_theme::ValueType;
use rayx_dtcg::Options;

#[test]
fn a_resolver_writes_its_name_tokens_modifiers_and_order() {
    let export = run(&project(), &Options::default()).unwrap();
    let doc = json(&export);
    let members: Vec<&str> = doc
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(members, ["name", "tokens", "modifiers", "resolutionOrder"]);
    assert_eq!(doc["name"], "Acme");
    assert!(doc.get("cssVars").is_none());
    assert!(doc.get("roles").is_none(), "no role map, no roles");
    let read = read_back(&export);
    assert_eq!(read, export.document);
}

#[test]
fn a_token_document_writes_tokens_alone() {
    let export = tokens(r##"{ "size": { "$type": "number", "$value": 4 } }"##).unwrap();
    let doc = json(&export);
    let members: Vec<&str> = doc
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(members, ["tokens"]);
}

#[test]
fn names_are_kebab_paths_unless_pinned() {
    let export = tokens(
        r##"{
      "theme": { "colors": { "surfaces": { "textMuted": { "$type": "color", "$value": { "colorSpace": "srgb", "components": [0, 0, 0] } } } } },
      "platform": { "density": { "size": { "default": { "lvl5": { "height": { "$type": "dimension", "$value": { "value": 32, "unit": "px" } } } } } } },
      "text": { "title1Bold": { "$type": "number", "$value": 1 } },
      "intents": { "primary": { "bgHover": { "$type": "number", "$value": 2 } } },
      "brand": { "$type": "number", "$value": 3, "$extensions": { "dev.nsh": { "name": "brand pinned" } } }
    }"##,
    )
    .unwrap();
    let names: Vec<&str> = export.document.tokens.keys().collect();
    assert_eq!(
        names,
        [
            "theme-colors-surfaces-text-muted",
            "platform-density-size-default-lvl5-height",
            "text-title1-bold",
            "intents-primary-bg-hover",
            "brand pinned",
        ]
    );
    assert_eq!(entry(&export, "brand pinned")["path"], "brand");
}

#[test]
fn two_tokens_of_one_name_are_refused_naming_both_paths() {
    assert_refused(
        tokens(
            r##"{
      "a": { "bB": { "$type": "number", "$value": 1 } },
      "aB": { "b": { "$type": "number", "$value": 2 } }
    }"##,
        ),
        &["`a.bB`", "`aB.b`", "`a-b-b`"],
    );
    assert_refused(
        tokens(
            r##"{
      "one": { "$type": "number", "$value": 1 },
      "two": { "$type": "number", "$value": 2, "$extensions": { "dev.nsh": { "name": "one" } } }
    }"##,
        ),
        &["`one`", "`two`", "both named"],
    );
}

#[test]
fn a_pinned_name_nsh_refuses_is_refused() {
    assert_refused(
        tokens(
            r##"{ "a": { "$type": "number", "$value": 1, "$extensions": { "dev.nsh": { "name": "a{b}" } } } }"##,
        ),
        &["`a`", "bad-name", "a{b}"],
    );
}

#[test]
fn kinds_follow_the_table() {
    let export = tokens(
        r##"{
      "c": { "$type": "color", "$value": { "colorSpace": "srgb", "components": [1, 0, 0] } },
      "d": { "$type": "dimension", "$value": { "value": 4, "unit": "px" } },
      "n": { "$type": "number", "$value": 0.5 },
      "w": { "$type": "fontWeight", "$value": 700 },
      "f": { "$type": "fontFamily", "$value": "Inter" },
      "t": { "$type": "duration", "$value": { "value": 1, "unit": "s" } },
      "e": { "$type": "cubicBezier", "$value": [0, 0, 1, 1] },
      "s": { "$type": "dxn.dtcg.string", "$value": "OK" },
      "b": { "$type": "dxn.dtcg.boolean", "$value": true }
    }"##,
    )
    .unwrap();
    let kinds: Vec<(&str, ValueType)> = export
        .document
        .tokens
        .iter()
        .map(|(n, t)| (n, t.value_type))
        .collect();
    assert_eq!(
        kinds,
        [
            ("c", ValueType::Color),
            ("d", ValueType::Number),
            ("n", ValueType::Number),
            ("w", ValueType::Number),
            ("f", ValueType::Font),
            ("t", ValueType::Duration),
            ("e", ValueType::Easing),
            ("s", ValueType::String),
            ("b", ValueType::Boolean),
        ]
    );
}

#[test]
fn dev_nsh_type_overrides_the_table() {
    let export = tokens(
        r##"{ "family": { "$type": "dxn.dtcg.string", "$value": "Inter, sans-serif", "$extensions": { "dev.nsh": { "type": "font" } } } }"##,
    )
    .unwrap();
    assert_eq!(entry(&export, "family")["kind"], "font");
    assert_eq!(entry(&export, "family")["value"], "Inter, sans-serif");
    read_back(&export);
}

#[test]
fn a_dev_nsh_type_that_names_no_value_type_is_refused() {
    assert_refused(
        tokens(
            r##"{ "a": { "$type": "number", "$value": 1, "$extensions": { "dev.nsh": { "type": "radius" } } } }"##,
        ),
        &["`a`", "`radius`", "no NSH value type"],
    );
}

#[test]
fn a_type_the_table_has_no_row_for_is_refused() {
    assert_refused(
        tokens(r##"{ "outline": { "$type": "strokeStyle", "$value": "dashed" } }"##),
        &["`outline`", "`strokeStyle`", "no value type"],
    );
}

#[test]
fn paths_and_docs_are_written() {
    let export = run(&project(), &Options::default()).unwrap();
    let page = entry(&export, "colors-backgrounds-primary");
    assert_eq!(page["path"], "colors.backgrounds.primary");
    assert_eq!(page["doc"], "The page.");
    assert!(entry(&export, "colors-accent").get("doc").is_none());
}

#[test]
fn the_same_project_gives_the_same_bytes() {
    let first = run(&project(), &Options::default()).unwrap().text();
    let second = run(&project(), &Options::default()).unwrap().text();
    assert_eq!(first, second);
    assert!(first.ends_with("}\n"));
    assert_eq!(nsh_theme::write(&nsh_theme::read(&first).document), first);
}

#[test]
fn every_refusal_is_reported_at_once() {
    let error = tokens(
        r##"{
      "outline": { "$type": "strokeStyle", "$value": "dashed" },
      "a": { "$type": "number", "$value": 1, "$extensions": { "dev.nsh": { "name": "a{b}" } } }
    }"##,
    )
    .unwrap_err();
    assert!(refused_with(&error, &["`outline`"]), "{error:?}");
    assert!(refused_with(&error, &["`a`", "bad-name"]), "{error:?}");
}
