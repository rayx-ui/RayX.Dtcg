//! Values in NSH's forms: one case per row of the value-type table, colors'
//! rounding and color spaces, `rem`, typography's `lineHeight` forms, single
//! and layered shadows, gradient geometry, blur, border styles, and each
//! refusal; NSH reads every written value as its declared type.

mod common;

use common::{assert_refused, read_back, run, tokens};
use nsh_theme::ValueType;
use rayx_dtcg::{Export, Options};
use serde_json::{Value, json};

/// A token document of one token `t`.
fn one(token_type: &str, value: Value, extensions: Option<Value>) -> String {
    let mut token = json!({ "$type": token_type, "$value": value });
    if let Some(extensions) = extensions {
        token["$extensions"] = extensions;
    }
    json!({ "t": token }).to_string()
}

/// The written value of `t`, checked to read back as `kind`.
fn written(export: &Export, kind: ValueType) -> Value {
    let read = read_back(export);
    assert_eq!(read.tokens.get("t").unwrap().value_type, kind);
    common::entry(export, "t")["value"].clone()
}

fn value_of(token_type: &str, value: Value, kind: ValueType) -> Value {
    let export = tokens(&one(token_type, value, None)).unwrap();
    written(&export, kind)
}

fn srgb(r: f64, g: f64, b: f64) -> Value {
    json!({ "colorSpace": "srgb", "components": [r, g, b] })
}

#[test]
fn colors_are_srgb_hex_rounded_half_away_from_zero() {
    assert_eq!(
        value_of("color", srgb(1.0, 0.0, 0.0), ValueType::Color),
        "#ff0000"
    );
    // 0.5 × 255 = 127.5 rounds to 128.
    assert_eq!(
        value_of("color", srgb(0.5, 0.5, 0.5), ValueType::Color),
        "#808080"
    );
    assert_eq!(
        value_of(
            "color",
            json!({ "colorSpace": "srgb", "components": [0, 0, 0], "alpha": 0.5 }),
            ValueType::Color
        ),
        "#00000080"
    );
    assert_eq!(
        value_of(
            "color",
            json!({ "colorSpace": "srgb", "components": [0, 0, 0], "alpha": 1 }),
            ValueType::Color
        ),
        "#000000"
    );
}

#[test]
fn a_color_in_another_space_is_converted_to_srgb() {
    assert_eq!(
        value_of(
            "color",
            json!({ "colorSpace": "hsl", "components": [120, 100, 50] }),
            ValueType::Color
        ),
        "#00ff00"
    );
    assert_eq!(
        value_of(
            "color",
            json!({ "colorSpace": "oklch", "components": [1, 0, 0] }),
            ValueType::Color
        ),
        "#ffffff"
    );
}

#[test]
fn dimensions_are_points_and_rem_takes_the_option() {
    let px = json!({ "value": 12, "unit": "px" });
    let rem = json!({ "value": 1.5, "unit": "rem" });
    assert_eq!(value_of("dimension", px, ValueType::Number), 12);
    assert_eq!(value_of("dimension", rem.clone(), ValueType::Number), 24);
    let options = Options {
        rem: 10.0,
        ..Options::default()
    };
    let text = one("dimension", rem, None);
    let export = run(&[("tokens.json", &text)], &options).unwrap();
    assert_eq!(written(&export, ValueType::Number), 15);
}

#[test]
fn numbers_weights_families_durations_and_easings() {
    assert_eq!(value_of("number", json!(0.5), ValueType::Number), 0.5);
    assert_eq!(value_of("fontWeight", json!(650), ValueType::Number), 650);
    assert_eq!(
        value_of("fontWeight", json!("bold"), ValueType::Number),
        700
    );
    assert_eq!(
        value_of("fontFamily", json!("Inter"), ValueType::Font),
        "Inter"
    );
    assert_eq!(
        value_of("fontFamily", json!(["Inter", "system-ui"]), ValueType::Font),
        "Inter, system-ui"
    );
    assert_eq!(
        value_of(
            "duration",
            json!({ "value": 120, "unit": "ms" }),
            ValueType::Duration
        ),
        "120ms"
    );
    assert_eq!(
        value_of(
            "duration",
            json!({ "value": 0.25, "unit": "s" }),
            ValueType::Duration
        ),
        "250ms"
    );
    assert_eq!(
        value_of("cubicBezier", json!([0.2, 0, 0, 1]), ValueType::Easing),
        json!([0.2, 0, 0, 1])
    );
}

#[test]
fn strings_and_booleans() {
    assert_eq!(
        value_of("dxn.dtcg.string", json!("OK"), ValueType::String),
        "OK"
    );
    assert_eq!(
        value_of("dxn.dtcg.boolean", json!(false), ValueType::Boolean),
        false
    );
}

fn typography(line_height: Value) -> Value {
    json!({
        "fontFamily": ["Inter", "sans-serif"], "fontSize": { "value": 14, "unit": "px" }, "fontWeight": "bold",
        "letterSpacing": { "value": 0.5, "unit": "px" }, "lineHeight": line_height
    })
}

#[test]
fn typography_is_a_compound_with_three_line_height_forms() {
    assert_eq!(
        value_of("typography", typography(json!(1.5)), ValueType::Typography),
        json!({ "fontFamily": "Inter, sans-serif", "fontSize": 14, "fontWeight": 700, "lineHeight": 1.5, "letterSpacing": 0.5 })
    );
    assert_eq!(
        value_of(
            "typography",
            typography(json!({ "value": 20, "unit": "px" })),
            ValueType::Typography
        )["lineHeight"],
        "20px"
    );
    let export = tokens(
        &json!({
            "leading": { "$type": "dimension", "$value": { "value": 18, "unit": "px" } },
            "t": { "$type": "typography", "$value": typography(json!("{leading}")) }
        })
        .to_string(),
    )
    .unwrap();
    assert_eq!(
        written(&export, ValueType::Typography)["lineHeight"],
        "{leading}"
    );
}

#[test]
fn a_line_height_aliasing_a_number_is_refused() {
    assert_refused(
        tokens(
            &json!({
                "multiple": { "$type": "number", "$value": 1.5 },
                "t": { "$type": "typography", "$value": typography(json!("{multiple}")) }
            })
            .to_string(),
        ),
        &["`t`", "`lineHeight`", "`multiple`"],
    );
}

fn layer(y: f64, alpha: f64, inset: Option<bool>) -> Value {
    let mut layer = json!({
        "color": { "colorSpace": "srgb", "components": [0, 0, 0], "alpha": alpha },
        "offsetX": { "value": 0, "unit": "px" }, "offsetY": { "value": y, "unit": "px" },
        "blur": { "value": 3, "unit": "px" }, "spread": { "value": -1, "unit": "px" }
    });
    if let Some(inset) = inset {
        layer["inset"] = inset.into();
    }
    layer
}

#[test]
fn shadows_are_layers() {
    assert_eq!(
        value_of("shadow", layer(1.0, 0.1, None), ValueType::Shadow),
        json!([{ "color": "#0000001a", "y": 1, "blur": 3, "spread": -1 }])
    );
    assert_eq!(
        value_of(
            "shadow",
            json!([layer(1.0, 0.1, None), layer(2.0, 0.2, Some(true))]),
            ValueType::Shadow
        ),
        json!([
            { "color": "#0000001a", "y": 1, "blur": 3, "spread": -1 },
            { "color": "#00000033", "y": 2, "blur": 3, "spread": -1, "inset": true }
        ])
    );
}

fn stops() -> Value {
    json!([
        { "color": { "colorSpace": "srgb", "components": [1, 0, 0] }, "position": 0 },
        { "color": { "colorSpace": "srgb", "components": [0, 0, 1] }, "position": 1 }
    ])
}

fn gradient(geometry: Option<Value>) -> Value {
    let export = tokens(&one(
        "gradient",
        stops(),
        geometry.map(|g| json!({ "dev.nsh": { "gradient": g } })),
    ))
    .unwrap();
    written(&export, ValueType::Gradient)
}

#[test]
fn a_gradient_takes_its_geometry_from_dev_nsh() {
    let stops =
        json!([{ "color": "#ff0000", "position": 0 }, { "color": "#0000ff", "position": 1 }]);
    assert_eq!(
        gradient(None),
        json!({ "type": "linear", "stops": stops }),
        "linear at 180 degrees, the default NSH leaves out"
    );
    assert_eq!(
        gradient(Some(json!({ "type": "linear", "angle": 90 }))),
        json!({ "type": "linear", "angle": 90, "stops": stops })
    );
    assert_eq!(
        gradient(Some(json!({ "type": "radial" }))),
        json!({ "type": "radial", "stops": stops })
    );
    assert_eq!(
        gradient(Some(json!({ "type": "conic", "angle": 45 }))),
        json!({ "type": "conic", "angle": 45, "stops": stops })
    );
}

#[test]
fn a_gradient_nsh_cannot_draw_is_refused() {
    assert_refused(
        tokens(&one(
            "gradient",
            json!([{ "color": { "colorSpace": "srgb", "components": [1, 0, 0] }, "position": 0 }]),
            None,
        )),
        &["`t`", "two stops or more"],
    );
    assert_refused(
        tokens(&one(
            "gradient",
            stops(),
            Some(json!({ "dev.nsh": { "gradient": { "type": "diamond" } } })),
        )),
        &["`t`", "`diamond`"],
    );
}

#[test]
fn a_blur_is_type_and_radius() {
    assert_eq!(
        value_of(
            "dxn.dtcg.blur",
            json!({ "kind": "background", "radius": { "value": 16, "unit": "px" } }),
            ValueType::Blur
        ),
        json!({ "type": "background", "radius": 16 })
    );
    assert_eq!(
        value_of(
            "dxn.dtcg.blur",
            json!({ "kind": "layer", "radius": { "value": 2, "unit": "px" } }),
            ValueType::Blur
        ),
        json!({ "type": "layer", "radius": 2 })
    );
}

fn border(style: Value) -> String {
    one(
        "border",
        json!({ "color": { "colorSpace": "srgb", "components": [0, 0, 0] }, "width": { "value": 1, "unit": "px" }, "style": style }),
        None,
    )
}

#[test]
fn a_border_is_solid_dashed_or_dotted() {
    for style in ["solid", "dashed", "dotted"] {
        let export = tokens(&border(json!(style))).unwrap();
        assert_eq!(
            written(&export, ValueType::Border),
            json!({ "color": "#000000", "width": 1, "style": style })
        );
    }
    assert_refused(
        tokens(&border(json!("double"))),
        &["`t`", "`style`", "double"],
    );
    assert_refused(
        tokens(&border(
            json!({ "dashArray": [{ "value": 2, "unit": "px" }], "lineCap": "round" }),
        )),
        &["`t`", "`style`"],
    );
}

#[test]
fn a_transition_is_duration_delay_and_easing() {
    assert_eq!(
        value_of(
            "transition",
            json!({ "duration": { "value": 100, "unit": "ms" }, "delay": { "value": 0.5, "unit": "s" }, "timingFunction": [0.2, 0, 0, 1] }),
            ValueType::Transition
        ),
        json!({ "duration": "100ms", "delay": "500ms", "easing": [0.2, 0, 0, 1] })
    );
}

#[test]
fn stroke_styles_and_other_types_are_refused() {
    assert_refused(
        tokens(&one("strokeStyle", json!("solid"), None)),
        &["`t`", "`strokeStyle`"],
    );
}
