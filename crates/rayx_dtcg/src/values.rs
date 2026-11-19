//! A DTCG value in NSH's forms: colors as sRGB hex, lengths as numbers of
//! points, durations as `<n>ms`, easings as `[x1, y1, x2, y2]`, and the
//! compounds as NSH's objects. An alias stays a reference: a whole value
//! `{a.b}` becomes `{<name of a.b>}`, and so does a compound's member.
//!
//! The conversion builds the value's JSON form and reads it with
//! [`nsh_theme::Value::from_json`], so what it writes is what NSH reads.

use std::collections::HashMap;

use dxn_dtcg::{
    ValueExtensions,
    values::{ColorSpace, Dimension, TypedValue},
};
use nsh_theme::ValueType;
use serde_json::{Map, Value, json};

/// A token an alias may name.
#[derive(Clone, Debug)]
pub struct Target {
    /// Its name in the document.
    pub name: String,
    /// Its value type.
    pub value_type: ValueType,
    /// Its DTCG `$type`.
    pub token_type: String,
}

/// What a conversion reads besides the value: pixels per `rem`, and the
/// tokens aliases may name, by path.
pub struct Converter<'a> {
    pub rem: f64,
    pub targets: &'a HashMap<String, Target>,
}

impl Converter<'_> {
    /// `value`, of DTCG type `token_type`, as a value of `value_type`;
    /// `extensions` are its definition's own (a gradient's geometry). `Err`
    /// says why NSH cannot take it.
    pub fn convert(
        &self,
        token_type: &str,
        value_type: ValueType,
        value: &Value,
        extensions: &Value,
    ) -> Result<nsh_theme::Value, String> {
        if let Some(path) = alias(value) {
            let target = self.target(path)?;
            if target.value_type != value_type {
                return Err(format!(
                    "it aliases `{path}`, a {} token, where a {value_type} is needed",
                    target.value_type
                ));
            }
            return Ok(nsh_theme::Value::reference(target.name.clone()));
        }
        refuse_pointer(value)?;
        let written = self.literal(token_type, value, extensions)?;
        nsh_theme::Value::from_json(value_type, &written)
            .map_err(|e| format!("it is no {value_type} NSH reads ({written}): {e}"))
    }

    fn target(&self, path: &str) -> Result<&Target, String> {
        self.targets
            .get(path)
            .ok_or_else(|| format!("it aliases `{path}`, which is no token (a group, or nothing)"))
    }

    /// A compound's member: a reference to a token of `needed`, or the
    /// literal `literal` writes.
    fn member(
        &self,
        what: &str,
        value: &Value,
        needed: ValueType,
        literal: impl Fn(&Value) -> Result<Value, String>,
    ) -> Result<Value, String> {
        if let Some(path) = alias(value) {
            let target = self.target(path)?;
            if target.value_type != needed {
                return Err(format!(
                    "its {what} aliases `{path}`, a {} token, where a {needed} is needed",
                    target.value_type
                ));
            }
            return Ok(format!("{{{}}}", target.name).into());
        }
        refuse_pointer(value).map_err(|e| format!("its {what}: {e}"))?;
        literal(value).map_err(|e| format!("its {what}: {e}"))
    }

    /// The JSON form of a value of DTCG type `token_type` whose members may
    /// be aliases.
    fn literal(
        &self,
        token_type: &str,
        value: &Value,
        extensions: &Value,
    ) -> Result<Value, String> {
        let rem = self.rem;
        Ok(match token_type {
            "color" => color(value)?,
            "dimension" => dimension(value, rem)?,
            "number" => number(value)?,
            "fontWeight" => font_weight(value)?,
            "fontFamily" => font_family(value)?,
            "duration" => duration(value)?,
            "cubicBezier" => cubic_bezier(value)?,
            "dxn.dtcg.string" => match value {
                Value::String(_) => value.clone(),
                other => return Err(format!("{other} is no string")),
            },
            "dxn.dtcg.boolean" => match value {
                Value::Bool(_) => value.clone(),
                other => return Err(format!("{other} is no boolean")),
            },
            "typography" => {
                let object = object(value, "typography")?;
                let mut out = Map::new();
                let get = |name: &str| {
                    object
                        .get(name)
                        .ok_or_else(|| format!("its `{name}` is missing"))
                };
                out.insert(
                    "fontFamily".into(),
                    self.member(
                        "`fontFamily`",
                        get("fontFamily")?,
                        ValueType::Font,
                        font_family,
                    )?,
                );
                out.insert(
                    "fontSize".into(),
                    self.member("`fontSize`", get("fontSize")?, ValueType::Number, |v| {
                        dimension(v, rem)
                    })?,
                );
                out.insert(
                    "fontWeight".into(),
                    self.member(
                        "`fontWeight`",
                        get("fontWeight")?,
                        ValueType::Number,
                        font_weight,
                    )?,
                );
                out.insert(
                    "letterSpacing".into(),
                    self.member(
                        "`letterSpacing`",
                        get("letterSpacing")?,
                        ValueType::Number,
                        |v| dimension(v, rem),
                    )?,
                );
                let line_height = get("lineHeight")?;
                if let Some(path) = alias(line_height)
                    && self.target(path)?.token_type != "dimension"
                {
                    // NSH reads a `lineHeight` reference as points; a
                    // multiple is written only as a literal.
                    return Err(format!(
                        "its `lineHeight` aliases `{path}`, a {} token: NSH reads a referenced \
                         line height as points, so it must alias a dimension",
                        self.target(path)?.token_type
                    ));
                }
                out.insert(
                    "lineHeight".into(),
                    self.member(
                        "`lineHeight`",
                        line_height,
                        ValueType::Number,
                        |v| match v {
                            Value::Number(_) => Ok(v.clone()),
                            _ => Ok(format!("{}px", decimal(px(v, rem)?, "a line height")?).into()),
                        },
                    )?,
                );
                Value::Object(out)
            }
            "shadow" => {
                let layers = match value {
                    Value::Array(layers) => layers.iter().collect(),
                    layer => vec![layer],
                };
                let mut out = Vec::with_capacity(layers.len());
                for (i, layer) in layers.into_iter().enumerate() {
                    let what = |m: &str| format!("layer {i}'s `{m}`");
                    let object = object(layer, "shadow layer")?;
                    let get = |name: &str| {
                        object
                            .get(name)
                            .ok_or_else(|| format!("layer {i}'s `{name}` is missing"))
                    };
                    let mut entry = Map::new();
                    entry.insert(
                        "color".into(),
                        self.member(&what("color"), get("color")?, ValueType::Color, color)?,
                    );
                    for (from, to) in [
                        ("offsetX", "x"),
                        ("offsetY", "y"),
                        ("blur", "blur"),
                        ("spread", "spread"),
                    ] {
                        entry.insert(
                            to.into(),
                            self.member(&what(from), get(from)?, ValueType::Number, |v| {
                                dimension(v, rem)
                            })?,
                        );
                    }
                    match object.get("inset") {
                        None => {}
                        Some(Value::Bool(inset)) => {
                            entry.insert("inset".into(), (*inset).into());
                        }
                        Some(other) => {
                            return Err(format!(
                                "{} is {other}, not `true` or `false`",
                                what("inset")
                            ));
                        }
                    }
                    out.push(Value::Object(entry));
                }
                Value::Array(out)
            }
            "gradient" => {
                let Value::Array(stops) = value else {
                    return Err(format!("{value} is no list of gradient stops"));
                };
                if stops.len() < 2 {
                    return Err(format!(
                        "it has {} stop: NSH draws a gradient of two stops or more",
                        stops.len()
                    ));
                }
                let mut written = Vec::with_capacity(stops.len());
                for (i, stop) in stops.iter().enumerate() {
                    let object = object(stop, "gradient stop")?;
                    let get = |name: &str| {
                        object
                            .get(name)
                            .ok_or_else(|| format!("stop {i}'s `{name}` is missing"))
                    };
                    written.push(json!({
                        "color": self.member(&format!("stop {i}'s `color`"), get("color")?, ValueType::Color, color)?,
                        "position": self.member(
                            &format!("stop {i}'s `position`"),
                            get("position")?,
                            ValueType::Number,
                            // DTCG's consumers clamp a position to [0, 1].
                            |v| Ok(json_number(number_of(v)?.clamp(0.0, 1.0))),
                        )?,
                    }));
                }
                let geometry = extensions.get("dev.nsh").and_then(|n| n.get("gradient"));
                let kind = match geometry.map(|g| g.get("type")) {
                    None | Some(None) => "linear",
                    Some(Some(Value::String(kind))) => kind.as_str(),
                    Some(Some(other)) => {
                        return Err(format!(
                            "its $extensions[\"dev.nsh\"].gradient.type is {other}, not a string"
                        ));
                    }
                };
                let angle = geometry.and_then(|g| g.get("angle"));
                let mut out = Map::new();
                out.insert("type".into(), kind.into());
                match (kind, angle) {
                    ("linear" | "conic", Some(Value::Number(angle))) => {
                        out.insert("angle".into(), Value::Number(angle.clone()));
                    }
                    ("linear" | "conic", Some(other)) => {
                        return Err(format!(
                            "its $extensions[\"dev.nsh\"].gradient.angle is {other}, not a number"
                        ));
                    }
                    ("linear", None) => {
                        out.insert("angle".into(), 180.into());
                    }
                    ("conic", None) => {}
                    ("radial", None) => {}
                    ("radial", Some(_)) => {
                        return Err(
                            "a radial gradient takes no $extensions[\"dev.nsh\"].gradient.angle"
                                .into(),
                        );
                    }
                    (other, _) => {
                        return Err(format!(
                            "its $extensions[\"dev.nsh\"].gradient.type is `{other}`, not \
                             `linear`, `radial` or `conic`"
                        ));
                    }
                }
                out.insert("stops".into(), Value::Array(written));
                Value::Object(out)
            }
            "dxn.dtcg.blur" => {
                let object = object(value, "blur")?;
                let kind = match object.get("kind") {
                    Some(Value::String(kind)) if kind == "layer" || kind == "background" => {
                        kind.clone()
                    }
                    other => {
                        return Err(format!(
                            "its `kind` is {}, not `layer` or `background`",
                            other.map_or("missing".to_owned(), Value::to_string)
                        ));
                    }
                };
                let radius = object
                    .get("radius")
                    .ok_or_else(|| "its `radius` is missing".to_owned())?;
                json!({
                    "type": kind,
                    "radius": self.member("`radius`", radius, ValueType::Number, |v| dimension(v, rem))?,
                })
            }
            "border" => {
                let object = object(value, "border")?;
                let get = |name: &str| {
                    object
                        .get(name)
                        .ok_or_else(|| format!("its `{name}` is missing"))
                };
                let style = match get("style")? {
                    Value::String(style)
                        if ["solid", "dashed", "dotted"].contains(&style.as_str()) =>
                    {
                        style.clone()
                    }
                    other => {
                        return Err(format!(
                            "its `style` is {other}: NSH draws a `solid`, `dashed` or `dotted` border"
                        ));
                    }
                };
                json!({
                    "color": self.member("`color`", get("color")?, ValueType::Color, color)?,
                    "width": self.member("`width`", get("width")?, ValueType::Number, |v| dimension(v, rem))?,
                    "style": style,
                })
            }
            "transition" => {
                let object = object(value, "transition")?;
                let get = |name: &str| {
                    object
                        .get(name)
                        .ok_or_else(|| format!("its `{name}` is missing"))
                };
                json!({
                    "duration": self.member("`duration`", get("duration")?, ValueType::Duration, duration)?,
                    "delay": self.member("`delay`", get("delay")?, ValueType::Duration, duration)?,
                    "easing": self.member("`timingFunction`", get("timingFunction")?, ValueType::Easing, cubic_bezier)?,
                })
            }
            other => return Err(format!("a `{other}` has no value type in NSH")),
        })
    }
}

/// The path a DTCG alias `{a.b.c}` names.
pub fn alias(value: &Value) -> Option<&str> {
    let text = value.as_str()?;
    let inner = text.strip_prefix('{')?.strip_suffix('}')?;
    (!inner.is_empty() && !inner.contains(['{', '}'])).then_some(inner)
}

/// Refuses a JSON-pointer `$ref`, which NSH's references cannot express.
fn refuse_pointer(value: &Value) -> Result<(), String> {
    match value.get("$ref") {
        Some(target) => Err(format!(
            "it is a JSON-pointer alias ({target}), which NSH cannot express: write an alias `{{path}}`"
        )),
        None => Ok(()),
    }
}

fn object<'a>(value: &'a Value, what: &str) -> Result<&'a Map<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| format!("{value} is no {what} object"))
}

/// `value` read as DTCG type `token_type`, validated by `dxn_dtcg`.
fn typed(token_type: &str, value: &Value) -> Result<TypedValue, String> {
    TypedValue::from_resolved(token_type, value, &ValueExtensions::all()).map_err(|e| e.to_string())
}

/// A color as `#rrggbb`, or `#rrggbbaa` below full opacity, converted into
/// sRGB and clipped there, each 8-bit channel rounded half away from zero.
fn color(value: &Value) -> Result<Value, String> {
    let TypedValue::Color(color) = typed("color", value)? else {
        unreachable!("a color")
    };
    let srgb = color.convert(ColorSpace::Srgb);
    let [r, g, b] = srgb.coordinates().map(channel);
    let a = channel(srgb.alpha);
    Ok(if a == 255 {
        format!("#{r:02x}{g:02x}{b:02x}")
    } else {
        format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
    }
    .into())
}

/// A channel in [0, 1] as 0–255, rounded half away from zero.
fn channel(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn px(value: &Value, rem: f64) -> Result<f64, String> {
    let TypedValue::Dimension(d) = typed("dimension", value)? else {
        unreachable!("a dimension")
    };
    Ok(Dimension::to_px(&d, rem))
}

/// A dimension as a number of points: `px` as it is, `rem` times `rem`.
fn dimension(value: &Value, rem: f64) -> Result<Value, String> {
    Ok(json_number(px(value, rem)?))
}

fn number_of(value: &Value) -> Result<f64, String> {
    let TypedValue::Number(n) = typed("number", value)? else {
        unreachable!("a number")
    };
    Ok(n)
}

fn number(value: &Value) -> Result<Value, String> {
    Ok(json_number(number_of(value)?))
}

/// A number, or DTCG's keyword (`bold`) as its number.
fn font_weight(value: &Value) -> Result<Value, String> {
    let TypedValue::FontWeight(w) = typed("fontWeight", value)? else {
        unreachable!("a font weight")
    };
    Ok(json_number(w.0))
}

/// A family, or a list of them joined with `, `.
fn font_family(value: &Value) -> Result<Value, String> {
    let TypedValue::FontFamily(f) = typed("fontFamily", value)? else {
        unreachable!("a font family")
    };
    Ok(f.0.join(", ").into())
}

/// `<n>ms`.
fn duration(value: &Value) -> Result<Value, String> {
    let TypedValue::Duration(d) = typed("duration", value)? else {
        unreachable!("a duration")
    };
    Ok(format!("{}ms", decimal(d.to_millis(), "a duration")?).into())
}

fn cubic_bezier(value: &Value) -> Result<Value, String> {
    let TypedValue::CubicBezier(b) = typed("cubicBezier", value)? else {
        unreachable!("a cubic Bézier")
    };
    Ok(Value::Array(b.0.iter().map(|n| json_number(*n)).collect()))
}

/// `n` as the digits (and fraction) NSH reads in `<n>ms` and `<n>px`.
fn decimal(n: f64, what: &str) -> Result<String, String> {
    if !n.is_finite() || n < 0.0 {
        return Err(format!("{what} of {n}: NSH reads none below 0"));
    }
    // `-0` is written `0`.
    Ok(format!("{}", n + 0.0))
}

/// A JSON number: an integral value as an integer.
fn json_number(n: f64) -> Value {
    if n.fract() == 0.0 && n.abs() <= 9_007_199_254_740_992.0 {
        return Value::from(n as i64);
    }
    serde_json::Number::from_f64(n).map_or(Value::Null, Value::Number)
}
