//! Aliases stay references: whole values, compound members and chains; an
//! alias into a modifier's token follows the picked context; and the alias
//! forms NSH cannot express are refused.

mod common;

use common::{assert_refused, entry, project, read_back, run, tokens};
use nsh_theme::{Selection, Value};
use rayx_dtcg::Options;
use serde_json::json;

#[test]
fn a_whole_value_alias_is_a_reference() {
    let export = run(&project(), &Options::default()).unwrap();
    assert_eq!(entry(&export, "colors-link")["value"], "{colors-accent}");
    assert_eq!(
        export.document.token("colors-link").unwrap().value,
        Value::reference("colors-accent")
    );
}

#[test]
fn a_member_alias_is_a_member_reference() {
    let export = tokens(
        r##"{
      "ink": { "$type": "color", "$value": { "colorSpace": "srgb", "components": [0, 0, 0], "alpha": 0.25 } },
      "lift": { "$type": "dimension", "$value": { "value": 2, "unit": "px" } },
      "family": { "$type": "fontFamily", "$value": "Inter" },
      "shadow": { "$type": "shadow", "$value": {
        "color": "{ink}", "offsetX": { "value": 0, "unit": "px" }, "offsetY": "{lift}",
        "blur": { "value": 4, "unit": "px" }, "spread": { "value": 0, "unit": "px" } } },
      "body": { "$type": "typography", "$value": {
        "fontFamily": "{family}", "fontSize": { "value": 14, "unit": "px" }, "fontWeight": 400,
        "letterSpacing": { "value": 0, "unit": "px" }, "lineHeight": 1.4 } }
    }"##,
    )
    .unwrap();
    read_back(&export);
    assert_eq!(
        entry(&export, "shadow")["value"],
        json!([{ "color": "{ink}", "y": "{lift}", "blur": 4 }])
    );
    assert_eq!(entry(&export, "body")["value"]["fontFamily"], "{family}");
}

#[test]
fn a_chain_of_aliases_stays_a_chain() {
    let export = tokens(
        r##"{
      "c": { "$type": "number", "$value": 3 },
      "b": { "$type": "number", "$value": "{c}" },
      "a": { "$type": "number", "$value": "{b}" }
    }"##,
    )
    .unwrap();
    assert_eq!(entry(&export, "a")["value"], "{b}");
    assert_eq!(entry(&export, "b")["value"], "{c}");
    let resolved = nsh_theme::resolve(&export.document, &Selection::new(), &[]);
    assert_eq!(resolved.get("a"), Some(&Value::Number(3.0)));
}

#[test]
fn an_alias_into_a_modifiers_token_follows_the_pick() {
    let export = run(&project(), &Options::default()).unwrap();
    let link = |selection: Selection| {
        nsh_theme::resolve(&export.document, &selection, &[])
            .get("colors-link")
            .cloned()
    };
    assert_eq!(
        link(Selection::new()),
        Some(Value::Color(nsh_theme::Color::rgb(0, 0, 255)))
    );
    assert_eq!(
        link(Selection::new().with("theme", "dark")),
        Some(Value::Color(nsh_theme::Color::rgb(128, 128, 255)))
    );
    assert_eq!(
        link(
            Selection::new()
                .with("theme", "dark")
                .with("brand", "forest")
        ),
        Some(Value::Color(nsh_theme::Color::rgb(0, 128, 0)))
    );
}

#[test]
fn a_json_pointer_alias_is_refused() {
    assert_refused(
        tokens(
            r##"{ "a": { "$type": "number", "$value": 1 }, "b": { "$type": "number", "$value": { "$ref": "#/a/$value" } } }"##,
        ),
        &["`b`", "JSON-pointer", "#/a/$value"],
    );
}

#[test]
fn an_alias_to_a_group_is_refused() {
    assert_refused(
        tokens(
            r##"{ "grp": { "x": { "$type": "number", "$value": 1 } }, "b": { "$type": "number", "$value": "{grp}" } }"##,
        ),
        &["`b`", "`grp`"],
    );
}

#[test]
fn an_alias_to_a_token_of_another_value_type_is_refused() {
    // `family` is a font by its pin, `label` a string: NSH's reference
    // needs a string.
    assert_refused(
        tokens(
            r##"{
      "family": { "$type": "dxn.dtcg.string", "$value": "Inter", "$extensions": { "dev.nsh": { "type": "font" } } },
      "label": { "$type": "dxn.dtcg.string", "$value": "{family}" }
    }"##,
        ),
        &[
            "`label`",
            "`family`",
            "a font token, where a string is needed",
        ],
    );
    // `family` is a DTCG font family, pinned to NSH's string type: a
    // typography's `fontFamily` reference needs a font.
    assert_refused(
        tokens(
            r##"{
      "family": { "$type": "fontFamily", "$value": "Inter", "$extensions": { "dev.nsh": { "type": "string" } } },
      "body": { "$type": "typography", "$value": {
        "fontFamily": "{family}", "fontSize": { "value": 14, "unit": "px" }, "fontWeight": 400,
        "letterSpacing": { "value": 0, "unit": "px" }, "lineHeight": 1.4 } }
    }"##,
        ),
        &[
            "`body`",
            "`fontFamily`",
            "`family`",
            "a string token, where a font is needed",
        ],
    );
}
