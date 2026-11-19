//! Modifiers and contexts copied one to one with their order and defaults,
//! sets merged into the tokens, a token's own value from the last modifier's
//! default context, the set-after-modifier refusal, and groups that
//! `$extends` another, in a set or a context, with overrides.

mod common;

use common::{assert_refused, json, read_back, run};
use rayx_dtcg::Options;
use serde_json::json;

fn number(n: f64) -> serde_json::Value {
    json!({ "$type": "number", "$value": n })
}

/// A resolver of the sets `base` (first) and `late` (last) and the
/// modifiers `size` (contexts `large`, `small`; default `small`) and `mode`
/// (`y`, `x`, authored out of alphabetical order; default `x`), with the
/// documents given.
fn project(documents: &[(&'static str, serde_json::Value)]) -> Vec<(&'static str, String)> {
    let resolver = json!({
        "version": "2025.10",
        "sets": {
            "base": { "sources": [{ "$ref": "base.json" }] },
            "late": { "sources": [{ "$ref": "late.json" }] }
        },
        "modifiers": {
            "size": { "default": "small", "contexts": {
                "large": [{ "$ref": "large.json" }], "small": [{ "$ref": "small.json" }] } },
            "mode": { "default": "x", "contexts": { "y": [{ "$ref": "y.json" }], "x": [{ "$ref": "x.json" }] } }
        },
        "resolutionOrder": [
            { "$ref": "#/sets/base" }, { "$ref": "#/modifiers/size" },
            { "$ref": "#/modifiers/mode" }, { "$ref": "#/sets/late" }
        ]
    });
    let mut files = vec![("p.resolver.json", resolver.to_string())];
    for name in [
        "base.json",
        "late.json",
        "large.json",
        "small.json",
        "x.json",
        "y.json",
    ] {
        let text = documents
            .iter()
            .find(|(n, _)| *n == name)
            .map_or_else(|| "{}".to_owned(), |(_, d)| d.to_string());
        files.push((name, text));
    }
    files
}

fn export(files: &[(&'static str, String)]) -> Result<rayx_dtcg::Export, rayx_dtcg::ExportError> {
    let files: Vec<(&str, &str)> = files.iter().map(|(n, t)| (*n, t.as_str())).collect();
    run(&files, &Options::default())
}

#[test]
fn modifiers_and_contexts_are_copied_one_to_one() {
    let files = project(&[
        (
            "base.json",
            json!({ "gap": number(4.0), "radius": number(2.0), "label": number(0.0) }),
        ),
        ("large.json", json!({ "gap": number(8.0) })),
        ("small.json", json!({ "gap": number(4.0) })),
        ("x.json", json!({ "radius": number(2.0) })),
        (
            "y.json",
            json!({ "radius": number(6.0), "gap": number(5.0) }),
        ),
        // A set after the modifiers that sets no modifier's token.
        (
            "late.json",
            json!({ "label": number(1.0), "extra": number(9.0) }),
        ),
    ]);
    let export = export(&files).unwrap();
    read_back(&export);
    let doc = json(&export);
    assert_eq!(doc["resolutionOrder"], json!(["size", "mode"]));
    assert_eq!(
        doc["modifiers"],
        json!({
            "size": { "default": "small", "contexts": { "large": { "gap": 8 }, "small": { "gap": 4 } } },
            "mode": { "default": "x", "contexts": { "y": { "radius": 6, "gap": 5 }, "x": { "radius": 2 } } }
        }),
        "each context holds its DTCG entries and nothing else, default contexts too"
    );
    // JSON maps compare without order: check the authored order as lists.
    let keys = |value: &serde_json::Value| -> Vec<String> {
        value.as_object().unwrap().keys().cloned().collect()
    };
    let modifiers = &doc["modifiers"];
    assert_eq!(
        keys(modifiers),
        ["size", "mode"],
        "modifiers in authored order"
    );
    assert_eq!(
        keys(&modifiers["size"]["contexts"]),
        ["large", "small"],
        "`size`'s contexts in authored order"
    );
    assert_eq!(
        keys(&modifiers["mode"]["contexts"]),
        ["y", "x"],
        "`mode`'s contexts in authored order, not sorted"
    );
    assert_eq!(
        keys(&modifiers["mode"]["contexts"]["y"]),
        ["radius", "gap"],
        "a context's entries in authored order"
    );
    let values: Vec<(&str, serde_json::Value)> = doc["tokens"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(n, t)| (n.as_str(), t["value"].clone()))
        .collect();
    assert_eq!(
        values,
        [
            ("gap", json!(4)),
            ("radius", json!(2)),
            ("label", json!(1)),
            ("extra", json!(9)),
        ],
        "the sets merged, the later set winning"
    );
}

#[test]
fn a_token_only_modifiers_set_takes_the_last_default_contexts_entry() {
    let files = project(&[
        // `tone` is set by `size`'s default context, and by `mode` only in
        // its non-default context `y`.
        (
            "small.json",
            json!({ "tone": number(1.0), "both": number(10.0) }),
        ),
        (
            "large.json",
            json!({ "tone": number(3.0), "both": number(30.0) }),
        ),
        ("y.json", json!({ "tone": number(2.0) })),
        // `both` is set by both modifiers' default contexts: `mode`'s wins.
        ("x.json", json!({ "both": number(20.0) })),
    ]);
    let export = export(&files).unwrap();
    let doc = json(&export);
    assert_eq!(doc["tokens"]["tone"]["value"], 1);
    assert_eq!(doc["tokens"]["both"]["value"], 20);
    assert_eq!(
        doc["modifiers"]["mode"]["contexts"]["y"],
        json!({ "tone": 2 })
    );
}

#[test]
fn a_set_after_a_modifier_that_sets_its_token_is_refused() {
    let files = project(&[
        ("base.json", json!({ "gap": number(4.0) })),
        ("large.json", json!({ "gap": number(8.0) })),
        ("late.json", json!({ "gap": number(6.0) })),
    ]);
    assert_refused(export(&files), &["set `late`", "modifier `size`", "`gap`"]);
}

#[test]
fn a_modifier_after_a_set_may_set_its_tokens() {
    let files = project(&[
        ("base.json", json!({ "gap": number(4.0) })),
        ("y.json", json!({ "gap": number(5.0) })),
    ]);
    let export = export(&files).unwrap();
    assert_eq!(json(&export)["tokens"]["gap"]["value"], 4);
}

#[test]
fn a_set_extending_a_group_copies_its_tokens_and_keeps_its_overrides() {
    let files = project(&[
        (
            "base.json",
            json!({
                "button": { "$type": "number", "height": { "$value": 32 }, "pad": { "$value": 8 },
                            "gap": { "$value": "{button.pad}" } },
                "compact": { "$extends": "{button}", "height": { "$value": 24 } }
            }),
        ),
        // A context that sets the extended group's token: the copy follows.
        ("large.json", json!({ "button": { "pad": number(12.0) } })),
    ]);
    let export = export(&files).unwrap();
    read_back(&export);
    let doc = json(&export);
    let value = |name: &str| doc["tokens"][name]["value"].clone();
    assert_eq!(
        value("compact-height"),
        24,
        "the extending group's own member wins"
    );
    assert_eq!(
        value("compact-pad"),
        8,
        "a token copied from the extended group"
    );
    assert_eq!(
        value("compact-gap"),
        "{button-pad}",
        "a copied alias stays an alias"
    );
    assert_eq!(
        doc["modifiers"]["size"]["contexts"]["large"],
        json!({ "button-pad": 12, "compact-pad": 12 }),
        "the copy follows the context that sets the extended token"
    );
}

#[test]
fn a_context_extending_a_group_in_another_document_copies_into_the_context() {
    let files = project(&[
        (
            "base.json",
            json!({ "palette": { "$type": "number", "a": { "$value": 1 }, "b": { "$value": 2 } } }),
        ),
        (
            "x.json",
            json!({ "alias": { "$type": "number", "a": { "$value": 0 }, "b": { "$value": 0 } } }),
        ),
        // `mode=y` extends `palette`, from `base.json`, and overrides `b`.
        (
            "y.json",
            json!({ "alias": { "$extends": "{palette}", "b": { "$value": 5 } } }),
        ),
    ]);
    let export = export(&files).unwrap();
    read_back(&export);
    let doc = json(&export);
    assert_eq!(doc["tokens"]["alias-a"]["value"], 0);
    assert_eq!(
        doc["modifiers"]["mode"]["contexts"]["y"],
        json!({ "alias-b": 5, "alias-a": 1 })
    );
    assert_eq!(
        doc["modifiers"]["mode"]["contexts"]["x"],
        json!({ "alias-a": 0, "alias-b": 0 })
    );
}

#[test]
fn an_extends_by_json_pointer_copies_too() {
    let files = project(&[(
        "base.json",
        json!({
            "button": { "$type": "number", "height": { "$value": 32 } },
            "compact": { "$extends": "#/button" }
        }),
    )]);
    let export = export(&files).unwrap();
    assert_eq!(json(&export)["tokens"]["compact-height"]["value"], 32);
}
