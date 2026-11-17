//! A resolved DTCG value as NSH's theme file writes it: colors as hex in
//! sRGB, lengths in pixels, durations in milliseconds, shadows, gradients
//! and easings as CSS, strings and booleans as JSON.

use dxn_dtcg::values::{Dimension, LineHeight, TypedValue};
use serde_json::Value;

use crate::kinds::Kind;

/// What a value becomes: the token's own value, and, for a text style, its
/// parts (`--line-height`, `--letter-spacing`, `--font-weight`), each a
/// suffix of the token's name with its value.
pub type Written = Vec<(&'static str, Value)>;

/// `value` of kind `kind` as NSH reads it; `rem` pixels per `rem`;
/// `extensions` the token's `$extensions` (a gradient's geometry). `Err` says
/// why it cannot be written.
pub fn write(
    kind: Kind,
    value: &TypedValue,
    raw: &Value,
    rem: f64,
    extensions: &Value,
) -> Result<Written, String> {
    let px = |d: &Dimension| format!("{}px", number(d.to_px(rem)));
    Ok(match (kind, value) {
        (Kind::String, _) => match raw {
            Value::String(_) => vec![("", raw.clone())],
            _ => return Err(format!("{raw} is no string")),
        },
        (Kind::Boolean, _) => match raw {
            Value::Bool(_) => vec![("", raw.clone())],
            _ => return Err(format!("{raw} is no boolean")),
        },
        (_, TypedValue::Color(c)) => vec![("", hex(c).into())],
        (Kind::Leading, TypedValue::Number(n))
        | (Kind::Weight, TypedValue::Number(n))
        | (Kind::Number, TypedValue::Number(n)) => {
            vec![("", number(*n).into())]
        }
        (_, TypedValue::Number(n)) => vec![("", format!("{}px", number(*n)).into())],
        (_, TypedValue::Dimension(d)) => vec![("", px(d).into())],
        (_, TypedValue::FontWeight(w)) => vec![("", number(w.0).into())],
        (_, TypedValue::FontFamily(f)) => vec![("", f.0.join(", ").into())],
        (_, TypedValue::Duration(d)) => vec![("", format!("{}ms", number(d.to_millis())).into())],
        (_, TypedValue::CubicBezier(b)) => {
            let [a, b2, c, d] = b.0;
            vec![(
                "",
                format!(
                    "cubic-bezier({}, {}, {}, {})",
                    number(a),
                    number(b2),
                    number(c),
                    number(d)
                )
                .into(),
            )]
        }
        (_, TypedValue::Shadow(s)) => {
            let layers: Vec<String> =
                s.0.iter()
                    .map(|l| {
                        format!(
                            "{}{} {} {} {} {}",
                            if l.inset { "inset " } else { "" },
                            px(&l.offset_x),
                            px(&l.offset_y),
                            px(&l.blur),
                            px(&l.spread),
                            hex(&l.color)
                        )
                    })
                    .collect();
            vec![("", layers.join(", ").into())]
        }
        (_, TypedValue::Gradient(g)) => {
            let geometry = extensions.get("dev.nsh").and_then(|n| n.get("gradient"));
            let kind = geometry
                .and_then(|g| g.get("kind"))
                .and_then(Value::as_str)
                .unwrap_or("linear");
            let angle = geometry
                .and_then(|g| g.get("angle"))
                .and_then(Value::as_f64)
                .unwrap_or(180.0);
            let head = match kind {
                "linear" => format!("linear-gradient({}deg", number(angle)),
                "radial" => "radial-gradient(circle".to_owned(),
                "conic" => format!("conic-gradient(from {}deg", number(angle)),
                other => {
                    return Err(format!(
                        "a gradient of kind `{other}` (linear, radial or conic)"
                    ));
                }
            };
            let mut stops: Vec<String> =
                g.0.iter()
                    .map(|s| format!("{} {}%", hex(&s.color), number(s.position * 100.0)))
                    .collect();
            // NSH draws a gradient of two stops or more: one is a flat one.
            if stops.len() == 1 {
                stops.push(stops[0].clone());
            }
            vec![("", format!("{head}, {})", stops.join(", ")).into())]
        }
        (_, TypedValue::Typography(t)) => {
            let size = t.font_size.to_px(rem);
            let line_height = match t.line_height {
                LineHeight::Multiplier(m) => m,
                // A length, as a multiple of the size, which `leading-[n]` reads.
                LineHeight::Dimension(d) if size > 0.0 => d.to_px(rem) / size,
                LineHeight::Dimension(_) => 1.0,
            };
            vec![
                ("", format!("{}px", number(size)).into()),
                ("--line-height", number(line_height).into()),
                ("--letter-spacing", px(&t.letter_spacing).into()),
                ("--font-weight", number(t.font_weight.0).into()),
            ]
        }
        (_, TypedValue::Blur(b)) => vec![("", px(&b.radius).into())],
        (kind, _) => return Err(format!("no {} value", kind.name())),
    })
}

/// A color as `#rrggbb` (`#rrggbbaa` when it is not opaque), in sRGB.
fn hex(color: &dxn_dtcg::values::Color) -> String {
    // Converted into sRGB and clipped there.
    let hex = color.to_hex();
    if color.alpha < 1.0 {
        format!(
            "{hex}{:02x}",
            (color.alpha.clamp(0.0, 1.0) * 255.0).round() as u8
        )
    } else {
        hex
    }
}

/// A number as JSON and CSS write it: `12`, `1.25`, no `12.0`.
fn number(n: f64) -> String {
    let rounded = (n * 10_000.0).round() / 10_000.0;
    if rounded == rounded.trunc() && rounded.abs() < 1e15 {
        format!("{}", rounded as i64)
    } else {
        format!("{rounded}")
    }
}
