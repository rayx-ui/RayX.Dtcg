//! A DTCG path as an NSH token name: the CSS name classes, `var()` and
//! `token()` use. The path itself stays beside it, in the theme document's
//! `tokens` section.

use serde_json::Value;

use crate::kinds::Kind;

/// `segment` in kebab case: `backgroundPrimary` and `background_primary`
/// are `background-primary`, `2XL` is `2xl`.
pub fn kebab(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len() + 4);
    let mut previous: Option<char> = None;
    for c in segment.chars() {
        if c == '_' || c == ' ' || c == '-' || c == '/' {
            if !out.ends_with('-') && !out.is_empty() {
                out.push('-');
            }
        } else if c.is_uppercase() {
            if previous.is_some_and(|p| p.is_lowercase() || p.is_ascii_digit())
                && !out.ends_with('-')
            {
                out.push('-');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
        previous = Some(c);
    }
    out.trim_end_matches('-').to_owned()
}

/// The NSH name of the token at `path` of kind `kind`: its pinned name
/// (`$extensions["dev.nsh"].name`), else its kind's namespace and its path
/// in kebab case, joined with `-`, a first segment that says the namespace
/// left out (`radius.card`: `radius-card`; `colors.backgrounds.primary`:
/// `colors-backgrounds-primary`).
pub fn name_of(path: &str, kind: Kind, extensions: &Value) -> String {
    if let Some(name) = extensions
        .get("dev.nsh")
        .and_then(|nsh| nsh.get("name"))
        .and_then(Value::as_str)
    {
        return name.to_owned();
    }
    let mut segments: Vec<String> = path.split('.').map(kebab).collect();
    if segments.len() > 1 && kind.namespace_words().contains(&segments[0].as_str()) {
        segments.remove(0);
    }
    format!("{}{}", kind.namespace(), segments.join("-"))
}

/// Why NSH would refuse `name` for a token of kind `kind`, if it would: a
/// color, a number, a string or a boolean may not take a font's namespace
/// (`font-`), an easing's starts with `ease-`, and a name is lower-case
/// letters, digits and `-`.
pub fn refused(name: &str, kind: Kind) -> Option<String> {
    if name.is_empty()
        || !name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Some(format!(
            "`{name}` is no CSS name of lower-case letters, digits and `-`"
        ));
    }
    let font = name.starts_with("font-") && !name.starts_with("font-weight-");
    match kind {
        Kind::Color | Kind::String | Kind::Boolean if name.starts_with("font-") => Some(format!(
            "a {} may not be named in a font's namespace (`{name}`)",
            kind.name()
        )),
        Kind::Number if font => Some(format!(
            "a number may not be named in a font's namespace (`{name}`)"
        )),
        _ if !name.starts_with(kind.namespace()) => Some(format!(
            "a {} is named in its namespace `{}` (`{name}`)",
            kind.name(),
            kind.namespace()
        )),
        _ => None,
    }
}
