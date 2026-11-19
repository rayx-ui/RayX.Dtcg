//! The exactness check: every combination of contexts resolves to the same
//! values under `dxn_dtcg` and under `nsh_theme::resolve`; a document that
//! differs is refused naming the token and the selection; and the
//! combination limit.

mod common;

use common::{project, refused_with, run, verify};
use nsh_theme::Value;
use rayx_dtcg::Options;

#[test]
fn every_combination_of_the_project_matches() {
    let export = run(&project(), &Options::default()).unwrap();
    verify(&project(), &Options::default(), &export.document).unwrap();
}

#[test]
fn an_altered_document_is_refused_naming_the_token_and_selection() {
    let mut document = run(&project(), &Options::default()).unwrap().document;
    let compact = document
        .modifiers
        .get_mut("density")
        .unwrap()
        .contexts
        .get_mut("compact")
        .unwrap();
    compact.set("radius-card", Value::Number(7.0));
    let error = verify(&project(), &Options::default(), &document).unwrap_err();
    assert!(
        refused_with(
            &error,
            &[
                "`radius.card`",
                "under theme=light, brand=ocean, density=compact",
                "6",
                "7"
            ]
        ),
        "{error:#?}"
    );
    assert!(
        refused_with(
            &error,
            &[
                "`radius.card`",
                "under theme=dark, brand=forest, density=compact"
            ]
        ),
        "every selection that differs is named: {error:#?}"
    );
    assert!(
        !refused_with(&error, &["density=comfortable"]),
        "the other selections match: {error:#?}"
    );
}

#[test]
fn a_document_missing_a_token_is_refused() {
    let mut document = run(&project(), &Options::default()).unwrap().document;
    document.remove_token("motion-fast");
    let error = verify(&project(), &Options::default(), &document).unwrap_err();
    assert!(
        refused_with(&error, &["`motion.fast`", "no token of the document"]),
        "{error:#?}"
    );
}

#[test]
fn a_project_above_the_combination_limit_is_refused_with_its_count() {
    // 2 × 2 × 2 = 8 combinations.
    let low = Options {
        max_combinations: 4,
        ..Options::default()
    };
    let error = run(&project(), &low).unwrap_err();
    assert!(refused_with(&error, &["8 combinations", "4"]), "{error:#?}");
    let document = run(&project(), &Options::default()).unwrap().document;
    assert!(verify(&project(), &low, &document).is_err());
    let raised = Options {
        max_combinations: 8,
        ..Options::default()
    };
    run(&project(), &raised).unwrap();
}
