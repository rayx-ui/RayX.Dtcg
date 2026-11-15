//! What NSH kind a DTCG token becomes: its `$type`, and for a dimension or
//! a number its *use* (which classes read it), as NSH's kinds name both.

use serde_json::Value;

/// An NSH token kind, as a theme document's `tokens` section spells it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    Color,
    Number,
    Radius,
    Spacing,
    Text,
    Weight,
    Tracking,
    Leading,
    Blur,
    Font,
    Shadow,
    Gradient,
    String,
    Boolean,
    Duration,
    Easing,
    Typography,
}

impl Kind {
    /// Its name in a theme document (`"radius"`).
    pub fn name(self) -> &'static str {
        match self {
            Kind::Color => "color",
            Kind::Number => "number",
            Kind::Radius => "radius",
            Kind::Spacing => "spacing",
            Kind::Text => "text",
            Kind::Weight => "weight",
            Kind::Tracking => "tracking",
            Kind::Leading => "leading",
            Kind::Blur => "blur",
            Kind::Font => "font",
            Kind::Shadow => "shadow",
            Kind::Gradient => "gradient",
            Kind::String => "string",
            Kind::Boolean => "boolean",
            Kind::Duration => "duration",
            Kind::Easing => "easing",
            Kind::Typography => "typography",
        }
    }

    /// The kind a use names (`"radius"`, `"spacing"`, …), as
    /// `$extensions["dev.nsh"].use` writes it.
    pub fn from_use(name: &str) -> Option<Kind> {
        Some(match name {
            "radius" => Kind::Radius,
            "spacing" => Kind::Spacing,
            "text" => Kind::Text,
            "weight" => Kind::Weight,
            "tracking" => Kind::Tracking,
            "leading" => Kind::Leading,
            "blur" => Kind::Blur,
            "number" => Kind::Number,
            _ => return None,
        })
    }

    /// The namespace its NSH name starts with (`radius-`), which its classes
    /// read it by; none for a color, a number, a string or a boolean.
    pub fn namespace(self) -> &'static str {
        match self {
            Kind::Color | Kind::Number | Kind::String | Kind::Boolean => "",
            Kind::Radius => "radius-",
            Kind::Spacing => "spacing-",
            Kind::Text | Kind::Typography => "text-",
            Kind::Weight => "font-weight-",
            Kind::Tracking => "tracking-",
            Kind::Leading => "leading-",
            Kind::Blur => "blur-",
            Kind::Font => "font-",
            Kind::Shadow => "shadow-",
            Kind::Gradient => "background-image-",
            Kind::Duration => "duration-",
            Kind::Easing => "ease-",
        }
    }

    /// The path segments that mean its namespace, which a name does not
    /// repeat (`radius.card` is `radius-card`, not `radius-radius-card`).
    pub(crate) fn namespace_words(self) -> &'static [&'static str] {
        match self {
            Kind::Radius => &["radius", "radii", "corner", "corners"],
            Kind::Spacing => &["spacing", "space", "spaces"],
            Kind::Text => &["text", "font-size", "font-sizes", "size", "sizes"],
            Kind::Typography => &["text", "typography", "type", "text-style", "text-styles"],
            Kind::Weight => &["font-weight", "font-weights", "weight", "weights"],
            Kind::Tracking => &["tracking", "letter-spacing"],
            Kind::Leading => &["leading", "line-height", "line-heights"],
            Kind::Blur => &["blur", "blurs"],
            Kind::Font => &["font", "fonts", "font-family", "font-families"],
            Kind::Shadow => &["shadow", "shadows", "elevation"],
            Kind::Gradient => &["gradient", "gradients", "background-image"],
            Kind::Duration => &["duration", "durations"],
            Kind::Easing => &["ease", "easing", "easings"],
            Kind::Color | Kind::Number | Kind::String | Kind::Boolean => &[],
        }
    }
}

/// The kind of a token of DTCG type `token_type` at `path` with
/// `$extensions` `extensions`; `None` for a type NSH has no kind of
/// (`border`, `transition`, `strokeStyle`).
///
/// A dimension's and a number's use is, in order: `$extensions["dev.nsh"].use`;
/// a Figma scope (`$extensions["com.figma.scopes"]`: `CORNER_RADIUS`, `GAP`,
/// `WIDTH_HEIGHT`, `FONT_SIZE`, `LETTER_SPACING`, `LINE_HEIGHT`, `FONT_WEIGHT`,
/// `EFFECT_FLOAT`); the path's first segment (`radius.card`, `spacing.4`,
/// `font-size.lg`); else a plain number.
pub fn kind_of(token_type: &str, path: &str, extensions: &Value) -> Option<Kind> {
    Some(match token_type {
        "color" => Kind::Color,
        "fontFamily" => Kind::Font,
        "fontWeight" => Kind::Weight,
        "duration" => Kind::Duration,
        "cubicBezier" => Kind::Easing,
        "shadow" => Kind::Shadow,
        "gradient" => Kind::Gradient,
        "typography" => Kind::Typography,
        "dxn.dtcg.blur" => Kind::Blur,
        "dxn.dtcg.string" => Kind::String,
        "dxn.dtcg.boolean" => Kind::Boolean,
        "dimension" | "number" => use_of(path, extensions).unwrap_or(Kind::Number),
        _ => return None,
    })
}

/// A dimension's or a number's use ([`kind_of`]).
fn use_of(path: &str, extensions: &Value) -> Option<Kind> {
    if let Some(kind) = extensions
        .get("dev.nsh")
        .and_then(|nsh| nsh.get("use"))
        .and_then(Value::as_str)
        .and_then(Kind::from_use)
    {
        return Some(kind);
    }
    let scopes = extensions.get("com.figma.scopes").and_then(Value::as_array);
    for scope in scopes.into_iter().flatten().filter_map(Value::as_str) {
        let kind = match scope {
            "CORNER_RADIUS" => Kind::Radius,
            "GAP" | "WIDTH_HEIGHT" => Kind::Spacing,
            "FONT_SIZE" => Kind::Text,
            "LETTER_SPACING" => Kind::Tracking,
            "LINE_HEIGHT" => Kind::Leading,
            "FONT_WEIGHT" => Kind::Weight,
            "EFFECT_FLOAT" => Kind::Blur,
            _ => continue,
        };
        return Some(kind);
    }
    let first = crate::names::kebab(path.split('.').next()?);
    [
        Kind::Radius,
        Kind::Spacing,
        Kind::Text,
        Kind::Weight,
        Kind::Tracking,
        Kind::Leading,
        Kind::Blur,
    ]
    .into_iter()
    .find(|kind| kind.namespace_words().contains(&first.as_str()))
}
