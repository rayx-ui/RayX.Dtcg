//! The role map written in order, missing paths and bad names refused, the
//! role check failing on `role-type` and `role-unmapped`, `role-unknown`
//! kept with a warning, and a malformed declaration file.

mod common;

use common::{assert_refused, json, project, read_back, run};
use rayx_dtcg::{Export, ExportError, Options};
use serde_json::json;

fn with(role_map: Option<serde_json::Value>, roles: Option<&str>) -> Result<Export, ExportError> {
    let options = Options {
        role_map: role_map.map(|m| m.to_string()),
        roles: roles.map(str::to_owned),
        ..Options::default()
    };
    run(&project(), &options)
}

const DECLARED: &str = r#"[
  { "name": "primary-bg", "type": "color", "doc": "The main action's background.", "required": true },
  { "name": "page", "type": "color" },
  { "name": "radius-default", "type": "number" }
]"#;

#[test]
fn the_role_map_is_written_in_its_order() {
    let export = with(
        Some(json!({
            "radius-default": "radius.card",
            "primary-bg": "colors.accent",
            "page": "colors.backgrounds.primary"
        })),
        None,
    )
    .unwrap();
    read_back(&export);
    let roles = json(&export)["roles"].clone();
    let order: Vec<(&str, &str)> = roles
        .as_object()
        .unwrap()
        .iter()
        .map(|(r, t)| (r.as_str(), t.as_str().unwrap()))
        .collect();
    assert_eq!(
        order,
        [
            ("radius-default", "radius-card"),
            ("primary-bg", "colors-accent"),
            ("page", "colors-backgrounds-primary")
        ]
    );
}

#[test]
fn without_a_role_map_there_are_no_roles() {
    let export = run(&project(), &Options::default()).unwrap();
    assert!(json(&export).get("roles").is_none());
}

#[test]
fn a_path_that_names_no_token_is_refused() {
    assert_refused(
        with(Some(json!({ "primary-bg": "colors.nothing" })), None),
        &["role `primary-bg`", "`colors.nothing`", "names no token"],
    );
    assert_refused(
        with(Some(json!({ "primary-bg": "colors" })), None),
        &["role `primary-bg`", "`colors`"],
    );
}

#[test]
fn a_role_name_nsh_refuses_is_refused() {
    assert_refused(
        with(Some(json!({ "a{b}": "colors.accent" })), None),
        &["role `a{b}`", "refuses the name"],
    );
}

#[test]
fn a_malformed_role_map_is_refused() {
    assert_refused(with(Some(json!(["colors.accent"])), None), &["role map"]);
    assert_refused(
        with(Some(json!({ "page": 3 })), None),
        &["role `page`", "DTCG path"],
    );
}

#[test]
fn the_roles_are_checked_against_the_declarations() {
    let export = with(
        Some(json!({
            "primary-bg": "colors.accent",
            "page": "colors.backgrounds.primary",
            "radius-default": "radius.card"
        })),
        Some(DECLARED),
    )
    .unwrap();
    assert!(export.warnings.is_empty(), "{:?}", export.warnings);
}

#[test]
fn a_role_onto_a_token_of_another_type_is_refused() {
    assert_refused(
        with(
            Some(json!({ "primary-bg": "colors.accent", "page": "radius.card" })),
            Some(DECLARED),
        ),
        &["error role-type /roles/page:", "radius-card"],
    );
}

#[test]
fn an_unmapped_required_role_is_refused() {
    assert_refused(
        with(
            Some(json!({ "page": "colors.backgrounds.primary" })),
            Some(DECLARED),
        ),
        &["warning role-unmapped /roles/primary-bg:", "required"],
    );
}

#[test]
fn an_undeclared_role_is_kept_with_a_warning() {
    let export = with(
        Some(json!({ "primary-bg": "colors.accent", "gutter": "spacing.gutter" })),
        Some(DECLARED),
    )
    .unwrap();
    assert_eq!(json(&export)["roles"]["gutter"], "spacing-gutter");
    assert_eq!(export.warnings.len(), 1, "{:?}", export.warnings);
    assert!(
        export.warnings[0].starts_with("warning role-unknown /roles/gutter:"),
        "{:?}",
        export.warnings
    );
}

#[test]
fn a_role_map_without_declarations_is_written_unchecked() {
    let export = with(Some(json!({ "gutter": "spacing.gutter" })), None).unwrap();
    assert!(export.warnings.is_empty());
    assert_eq!(json(&export)["roles"]["gutter"], "spacing-gutter");
}

#[test]
fn a_malformed_declaration_file_is_refused() {
    let map = Some(json!({ "primary-bg": "colors.accent" }));
    assert_refused(
        with(map.clone(), Some("{}")),
        &["role declarations", "no list"],
    );
    assert_refused(
        with(map.clone(), Some("not json")),
        &["role declarations are no JSON"],
    );
    assert_refused(
        with(map.clone(), Some(r#"[{ "name": "primary-bg" }]"#)),
        &["role declaration 0 (`primary-bg`)", "no string `type`"],
    );
    assert_refused(
        with(
            map.clone(),
            Some(r#"[{ "name": "primary-bg", "type": "colour" }]"#),
        ),
        &["`colour`", "no value type"],
    );
    assert_refused(
        with(
            map,
            Some(r#"[{ "name": "primary-bg", "type": "color", "required": "yes" }]"#),
        ),
        &["`required`", "no boolean"],
    );
}
