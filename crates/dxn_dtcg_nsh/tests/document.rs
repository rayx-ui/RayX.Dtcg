//! A design system exported whole: its values in `cssVars` (one in both
//! appearances in `theme`), each token declared with its kind, path and doc,
//! its other modifiers with only what each context changes.

mod common;

use dxn_dtcg_nsh::Options;
use serde_json::json;

#[test]
fn the_values_are_nsh_s_by_kind_and_appearance() {
    let export = common::run(&common::project(), &Options::default()).unwrap();
    let vars = &export.document["cssVars"];
    assert_eq!(vars["light"]["colors-backgrounds-primary"], "#ffffff");
    assert_eq!(vars["dark"]["colors-backgrounds-primary"], "#000000");
    assert_eq!(vars["light"]["colors-accent"], "#0000ff");
    assert_eq!(vars["dark"]["colors-accent"], "#8080ff");
    let theme = &vars["theme"];
    assert_eq!(theme["colors-overlay"], "#00000080");
    assert_eq!(theme["radius-card"], "12px");
    assert_eq!(theme["spacing-gutter"], "16px");
    assert_eq!(theme["spacing-gap-md"], "8px");
    assert_eq!(theme["text-title"], "24px");
    assert_eq!(theme["text-title--line-height"], "1.25");
    assert_eq!(theme["text-title--letter-spacing"], "-0.5px");
    assert_eq!(theme["text-title--font-weight"], "650");
    assert_eq!(theme["font-type-body"], "Inter, sans-serif");
    assert_eq!(theme["shadow-raised"], "0px 1px 2px 0px #0000001a");
    assert_eq!(theme["duration-motion-fast"], "120ms");
    assert_eq!(theme["ease-motion-snappy"], "cubic-bezier(0.2, 0, 0, 1)");
    assert_eq!(
        theme["background-image-hero"],
        "linear-gradient(90deg, #ff0000 0%, #0000ff 100%)"
    );
    assert_eq!(theme["primary"], "#1a1a1a");
    // A stroke style has no NSH kind: left out, and said.
    assert!(theme.get("outline").is_none());
    assert_eq!(
        export.left_out,
        ["`outline`: NSH has no kind of a `strokeStyle` token"]
    );
}

#[test]
fn each_token_is_declared_with_its_kind_path_and_doc() {
    let export = common::run(&common::project(), &Options::default()).unwrap();
    let tokens = &export.document["tokens"];
    assert_eq!(
        tokens["colors-backgrounds-primary"],
        json!({ "kind": "color", "path": "colors.backgrounds.primary", "doc": "The page." })
    );
    for (name, kind, path) in [
        ("radius-card", "radius", "radius.card"),
        ("spacing-gutter", "spacing", "spacing.gutter"),
        ("spacing-gap-md", "spacing", "gap.md"),
        ("text-title", "typography", "type.title"),
        ("font-type-body", "font", "type.body"),
        ("shadow-raised", "shadow", "elevation.raised"),
        ("duration-motion-fast", "duration", "motion.fast"),
        ("ease-motion-snappy", "easing", "motion.snappy"),
        ("background-image-hero", "gradient", "gradients.hero"),
        ("primary", "color", "brand.name"),
    ] {
        assert_eq!(tokens[name]["kind"], kind, "{name}");
        assert_eq!(tokens[name]["path"], path, "{name}");
    }
    // A text style's parts are declared by it, not listed.
    assert!(tokens.get("text-title--line-height").is_none());
    assert!(
        export
            .names
            .contains(&("brand.name".to_owned(), "primary".to_owned()))
    );
}

#[test]
fn other_modifiers_hold_only_what_each_context_changes() {
    let export = common::run(&common::project(), &Options::default()).unwrap();
    assert_eq!(
        export.document["modifiers"],
        json!({
            "brand": { "default": "ocean", "contexts": {
                "ocean": {},
                "forest": { "theme": { "colors-accent": "#008000" } } } },
            "density": { "default": "comfortable", "contexts": {
                "comfortable": {},
                "compact": { "theme": { "radius-card": "6px" } } } }
        })
    );
    // The appearance is NSH's own, no modifier.
    assert!(export.document["modifiers"].get("theme").is_none());
}

/// Strings and booleans (`dxn.dtcg.string`, `dxn.dtcg.boolean`) are NSH's
/// string and boolean tokens, as JSON.
#[test]
fn strings_and_booleans_are_written_as_json() {
    let tokens = r##"{
      "brand": { "name": { "$type": "dxn.dtcg.string", "$value": "Acme" } },
      "features": { "beta": { "$type": "dxn.dtcg.boolean", "$value": true } }
    }"##;
    let export = common::run(&[("tokens.json", tokens)], &Options::default()).unwrap();
    assert_eq!(export.document["cssVars"]["theme"]["brand-name"], "Acme");
    assert_eq!(export.document["cssVars"]["theme"]["features-beta"], true);
    assert_eq!(export.document["tokens"]["brand-name"]["kind"], "string");
    assert_eq!(
        export.document["tokens"]["features-beta"]["kind"],
        "boolean"
    );
}
