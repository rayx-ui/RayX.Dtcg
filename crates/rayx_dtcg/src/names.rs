//! A token's name in the theme document: its pinned name, or its DTCG path
//! in kebab case. The path itself stays beside it, in the token's `path`.

use serde_json::Value;

/// `segment` in kebab case: a lowercase letter or a digit followed by an
/// uppercase letter gets a `-` there, and every letter becomes lowercase
/// (`bgHover` → `bg-hover`, `title1Bold` → `title1-bold`, `lvl5` → `lvl5`).
pub fn kebab(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len() + 4);
    let mut previous: Option<char> = None;
    for c in segment.chars() {
        if c.is_uppercase() && previous.is_some_and(|p| p.is_lowercase() || p.is_ascii_digit()) {
            out.push('-');
        }
        out.extend(c.to_lowercase());
        previous = Some(c);
    }
    out
}

/// The name a token's path gives it: each segment in kebab case, joined by
/// `-` (`theme.colors.surfaces.textMuted` → `theme-colors-surfaces-text-muted`).
pub fn path_name(path: &str) -> String {
    path.split('.').map(kebab).collect::<Vec<_>>().join("-")
}

/// The name `$extensions["dev.nsh"].name` pins, if the token pins one.
pub fn pinned(extensions: &Value) -> Option<&str> {
    extensions
        .get("dev.nsh")
        .and_then(|nsh| nsh.get("name"))
        .and_then(Value::as_str)
}

/// Whether NSH takes `name` as a token's or a role's name: a non-empty
/// string without `{` or `}`.
pub fn valid(name: &str) -> bool {
    !name.is_empty() && !name.contains(['{', '}'])
}
