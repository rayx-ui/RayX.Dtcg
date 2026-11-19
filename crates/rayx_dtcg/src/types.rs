//! Which of NSH's value types a DTCG token becomes: a fixed table from
//! `$type`, which `$extensions["dev.nsh"].type` overrides per token.

use nsh_theme::ValueType;
use serde_json::Value;

/// The value type of a DTCG `$type`, by the table:
///
/// | `$type` | value type |
/// |---|---|
/// | `color` | `color` |
/// | `dimension`, `number`, `fontWeight` | `number` |
/// | `fontFamily` | `font` |
/// | `duration` | `duration` |
/// | `cubicBezier` | `easing` |
/// | `typography` | `typography` |
/// | `shadow` | `shadow` |
/// | `gradient` | `gradient` |
/// | `border` | `border` |
/// | `transition` | `transition` |
/// | `dxn.dtcg.blur` | `blur` |
/// | `dxn.dtcg.string` | `string` |
/// | `dxn.dtcg.boolean` | `boolean` |
///
/// `None` for any other type (`strokeStyle` among them).
pub fn value_type_of(token_type: &str) -> Option<ValueType> {
    Some(match token_type {
        "color" => ValueType::Color,
        "dimension" | "number" | "fontWeight" => ValueType::Number,
        "fontFamily" => ValueType::Font,
        "duration" => ValueType::Duration,
        "cubicBezier" => ValueType::Easing,
        "typography" => ValueType::Typography,
        "shadow" => ValueType::Shadow,
        "gradient" => ValueType::Gradient,
        "border" => ValueType::Border,
        "transition" => ValueType::Transition,
        "dxn.dtcg.blur" => ValueType::Blur,
        "dxn.dtcg.string" => ValueType::String,
        "dxn.dtcg.boolean" => ValueType::Boolean,
        _ => return None,
    })
}

/// The value type `$extensions["dev.nsh"].type` names: `None` when the
/// token names none, `Some(Err(name))` when the name is no value type.
pub fn pinned(extensions: &Value) -> Option<Result<ValueType, String>> {
    let name = extensions.get("dev.nsh")?.get("type")?;
    Some(match name.as_str() {
        Some(name) => ValueType::from_name(name).ok_or_else(|| name.to_owned()),
        None => Err(name.to_string()),
    })
}
