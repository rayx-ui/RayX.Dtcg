//! The role map and the role declarations: which token plays each role, and
//! what a library reads.
//!
//! - **The role map** is a JSON object from a role's name to a DTCG path
//!   (`{ "primary": "theme.colors.intents.primary.bg" }`). The document's
//!   `roles` maps each role onto the name of that path's token, in the map's
//!   order.
//! - **The declarations** are NSH's role-declaration JSON,
//!   `[{ "name", "type", "doc"?, "required"? }]`. With them the export checks
//!   the document with [`nsh_theme::check_roles`].

use nsh_theme::{RoleDeclaration, ValueType};
use serde_json::Value;

/// The role map's mappings, in order: `(role, path)`.
pub fn parse_map(text: &str) -> Result<Vec<(String, String)>, Vec<String>> {
    let json: Value =
        serde_json::from_str(text).map_err(|e| vec![format!("the role map is no JSON: {e}")])?;
    let Value::Object(members) = json else {
        return Err(vec![
            "the role map is no JSON object of a role's name to a DTCG path".into(),
        ]);
    };
    let mut errors = Vec::new();
    let mut out = Vec::with_capacity(members.len());
    for (role, path) in members {
        match path {
            Value::String(path) => out.push((role, path)),
            other => errors.push(format!(
                "role `{role}` maps onto {other}: the role map gives each role a DTCG path"
            )),
        }
    }
    if errors.is_empty() {
        Ok(out)
    } else {
        Err(errors)
    }
}

/// The declarations: `[{ "name", "type", "doc"?, "required"? }]`.
pub fn parse_declarations(text: &str) -> Result<Vec<RoleDeclaration>, Vec<String>> {
    const SHAPE: &str = "role declarations are `[{ \"name\", \"type\", \"doc\"?, \"required\"? }]`";
    let json: Value = serde_json::from_str(text)
        .map_err(|e| vec![format!("the role declarations are no JSON: {e}")])?;
    let Value::Array(items) = json else {
        return Err(vec![format!("the role declarations are no list: {SHAPE}")]);
    };
    let mut errors = Vec::new();
    let mut out = Vec::with_capacity(items.len());
    for (i, item) in items.iter().enumerate() {
        let Some(object) = item.as_object() else {
            errors.push(format!("role declaration {i} is no object: {SHAPE}"));
            continue;
        };
        let mut wrong = Vec::new();
        for member in object.keys() {
            if !["name", "type", "doc", "required"].contains(&member.as_str()) {
                wrong.push(format!("has an unknown member `{member}`"));
            }
        }
        let name = object.get("name").and_then(Value::as_str);
        if name.is_none() {
            wrong.push("has no string `name`".into());
        }
        let value_type = match object.get("type").and_then(Value::as_str) {
            Some(t) => match ValueType::from_name(t) {
                Some(t) => Some(t),
                None => {
                    wrong.push(format!("names `{t}`, which is no value type"));
                    None
                }
            },
            None => {
                wrong.push("has no string `type`".into());
                None
            }
        };
        let doc = match object.get("doc") {
            None => None,
            Some(Value::String(doc)) => Some(doc.clone()),
            Some(_) => {
                wrong.push("has a `doc` that is no string".into());
                None
            }
        };
        let required = match object.get("required") {
            None => false,
            Some(Value::Bool(required)) => *required,
            Some(_) => {
                wrong.push("has a `required` that is no boolean".into());
                false
            }
        };
        if !wrong.is_empty() {
            let what = name.map_or(format!("role declaration {i}"), |n| {
                format!("role declaration {i} (`{n}`)")
            });
            errors.extend(wrong.into_iter().map(|w| format!("{what} {w}: {SHAPE}")));
            continue;
        }
        let (Some(name), Some(value_type)) = (name, value_type) else {
            continue;
        };
        let mut declaration = RoleDeclaration::new(name, value_type).with_required(required);
        if let Some(doc) = doc {
            declaration = declaration.with_doc(doc);
        }
        out.push(declaration);
    }
    if errors.is_empty() {
        Ok(out)
    } else {
        Err(errors)
    }
}
