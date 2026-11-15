//! Exports a DTCG 2025.10 design-token project (a token document, or a
//! resolver with its sets and modifiers) as an NSH theme document: the
//! `cssVars` NSH reads, a `tokens` section that declares each token (its
//! kind, its path, its doc), and the project's other modifiers as
//! precomputed `modifiers`, each with its `default` and its `contexts`.
//!
//! - **Appearance.** One modifier is the appearance: its two contexts
//!   become `cssVars`' `light` and `dark` ([`Appearance`]). A value the same
//!   in both goes to `theme`.
//! - **Other modifiers** keep their names and their contexts' names. The
//!   default context's values are in `cssVars`; every other context holds
//!   only what it changes.
//! - **Names.** A token's NSH name is its pinned one
//!   (`$extensions["dev.nsh"].name`), else its kind's namespace and its path
//!   in kebab case ([`names::name_of`]); two paths of one name are refused.
//! - **Kinds.** `$type`, and for a dimension or a number its use
//!   ([`kinds::kind_of`]).
//! - **Exact layers.** NSH lays each modifier's picked context over the
//!   values, in order. The export resolves every combination of contexts and
//!   refuses a token whose value there is not what the layers give, naming
//!   the token and the combination.
//!
//! The library reads nothing itself: [`export`] takes the project's files
//! from a loader. The `dtcg-nsh` command reads them from disk.

pub mod kinds;
pub mod names;
pub mod values;

use std::collections::{BTreeMap, HashMap};

use dxn_dtcg::{DtcgRuntime, Resolution, ResolvedTokens, RuntimeOptions, ValueExtensions};
use serde_json::{Map, Value};

use crate::kinds::Kind;

/// Which modifier is NSH's appearance, and which of its contexts are light
/// and dark.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Appearance {
    /// The modifier's name (`theme`).
    pub modifier: String,
    /// Its light context (`light`).
    pub light: String,
    /// Its dark context (`dark`).
    pub dark: String,
}

/// How a project is exported.
#[derive(Clone, Debug)]
pub struct Options {
    /// The appearance modifier; `None`: the one whose contexts are `light`
    /// and `dark`, if there is one, else every value is both appearances'.
    pub appearance: Option<Appearance>,
    /// Pixels per `rem`.
    pub rem: f64,
    /// The most combinations of contexts the exactness check resolves.
    pub max_combinations: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            appearance: None,
            rem: 16.0,
            max_combinations: 4096,
        }
    }
}

/// An exported project.
#[derive(Clone, Debug)]
pub struct Export {
    /// The NSH theme document.
    pub document: Value,
    /// Each token's path and the NSH name it took, in path order.
    pub names: Vec<(String, String)>,
    /// What was left out and why (a type NSH has no kind of).
    pub left_out: Vec<String>,
}

impl Export {
    /// The document as it is written: pretty JSON with a final newline.
    pub fn text(&self) -> String {
        let mut text = serde_json::to_string_pretty(&self.document).unwrap_or_default();
        text.push('\n');
        text
    }
}

/// Why a project cannot be exported: every reason found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportError(pub Vec<String>);

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.join("\n"))
    }
}

impl std::error::Error for ExportError {}

/// Exports the project whose root document is `root`, its documents read
/// by `load` (a resource id, relative as the project's `$ref`s are, to its
/// text).
pub fn export(
    root: &str,
    load: &dyn Fn(&str) -> Option<String>,
    options: &Options,
) -> Result<Export, ExportError> {
    let fail = |e: dxn_dtcg::DtcgError| ExportError(vec![e.to_string()]);
    let mut project = Project {
        runtime: DtcgRuntime::with_options(
            RuntimeOptions::default().with_extensions(ValueExtensions::all()),
        ),
        load,
    };
    project.runtime.set_root(root).map_err(fail)?;
    let axes = project.axes()?;
    let mut errors = Vec::new();

    // The appearance, and the other modifiers with their defaults.
    let appearance = match &options.appearance {
        Some(a) => {
            let Some(axis) = axes.iter().find(|x| x.name == a.modifier) else {
                return Err(ExportError(vec![format!(
                    "the project has no modifier `{}`",
                    a.modifier
                )]));
            };
            for context in [&a.light, &a.dark] {
                if !axis.contexts.contains(context) {
                    errors.push(format!(
                        "the modifier `{}` has no context `{context}`",
                        a.modifier
                    ));
                }
            }
            Some(a.clone())
        }
        None => axes
            .iter()
            .find(|x| {
                x.contexts.iter().any(|c| c == "light") && x.contexts.iter().any(|c| c == "dark")
            })
            .map(|x| Appearance {
                modifier: x.name.clone(),
                light: "light".into(),
                dark: "dark".into(),
            }),
    };
    let others: Vec<Axis> = axes
        .iter()
        .filter(|x| appearance.as_ref().is_none_or(|a| a.modifier != x.name))
        .map(|x| Axis {
            name: x.name.clone(),
            default: x.default.clone().unwrap_or_else(|| x.contexts[0].clone()),
            contexts: x.contexts.clone(),
        })
        .collect();
    // DTCG gives a modifier two contexts or more, as NSH does.
    if !errors.is_empty() {
        return Err(ExportError(errors));
    }

    // The values under the default contexts, in each appearance.
    let picks = |picked: &[(&str, &str)]| -> Vec<(String, String)> {
        others
            .iter()
            .map(|axis| {
                let context = picked
                    .iter()
                    .find(|(name, _)| *name == axis.name)
                    .map_or(axis.default.as_str(), |(_, c)| c);
                (axis.name.clone(), context.to_owned())
            })
            .collect()
    };
    let base = project.values(&picks(&[]), appearance.as_ref(), options, &mut errors)?;

    // Names: one token per name, and NSH's name forms.
    let mut by_name: HashMap<&str, &str> = HashMap::new();
    for (path, token) in &base.tokens {
        if let Some(why) = names::refused(&token.name, token.kind) {
            errors.push(format!(
                "`{path}`: {why}; pin a name with $extensions[\"dev.nsh\"].name"
            ));
        }
        if let Some(other) = by_name.insert(&token.name, path) {
            errors.push(format!(
                "`{other}` and `{path}` are both named `{}`; pin one's name with $extensions[\"dev.nsh\"].name",
                token.name
            ));
        }
    }

    // Each other modifier's contexts: what each changes.
    let mut modifiers = Map::new();
    let mut layers: Vec<(String, BTreeMap<String, Written>)> = Vec::new();
    for axis in &others {
        let mut contexts = Map::new();
        let mut changes = BTreeMap::new();
        for context in &axis.contexts {
            if *context == axis.default {
                contexts.insert(context.clone(), Value::Object(Map::new()));
                continue;
            }
            let values = project.values(
                &picks(&[(&axis.name, context)]),
                appearance.as_ref(),
                options,
                &mut errors,
            )?;
            let changed = base.changed_by(&values);
            contexts.insert(context.clone(), css_vars(&changed));
            changes.insert(context.clone(), changed);
        }
        modifiers.insert(
            axis.name.clone(),
            serde_json::json!({ "default": axis.default, "contexts": contexts }),
        );
        layers.push((axis.name.clone(), changes));
    }

    // Exact layers: every combination resolves to the base with each
    // picked context over it, in order.
    let combinations: usize = others.iter().map(|a| a.contexts.len()).product();
    if combinations > options.max_combinations {
        errors.push(format!(
            "{combinations} combinations of contexts to check, more than {}",
            options.max_combinations
        ));
    } else if others.len() > 1 {
        for combination in product(&others) {
            if combination
                .iter()
                .filter(|(a, c)| others.iter().any(|x| x.name == *a && x.default != *c))
                .count()
                < 2
            {
                // The default, or one context alone: what its layer is.
                continue;
            }
            let picked: Vec<(&str, &str)> = combination
                .iter()
                .map(|(a, c)| (a.as_str(), c.as_str()))
                .collect();
            let actual =
                project.values(&picks(&picked), appearance.as_ref(), options, &mut errors)?;
            let mut expected = base.flat();
            for (axis, changes) in &layers {
                let context = &combination.iter().find(|(a, _)| a == axis).unwrap().1;
                if let Some(changed) = changes.get(context) {
                    expected.extend(changed.iter().map(|(k, v)| (k.clone(), v.clone())));
                }
            }
            let named: Vec<String> = combination
                .iter()
                .map(|(a, c)| format!("{a}={c}"))
                .collect();
            for (name, value) in actual.flat() {
                if expected.get(&name) != Some(&value) {
                    errors.push(format!(
                        "`{name}` under {} is not what its modifiers' layers give: their contexts change it together",
                        named.join(", ")
                    ));
                }
            }
        }
    }
    if !errors.is_empty() {
        errors.sort();
        errors.dedup();
        return Err(ExportError(errors));
    }

    let mut tokens = Map::new();
    let mut declared: Vec<(&String, &Token)> = base.tokens.iter().collect();
    declared.sort_by(|a, b| a.1.name.cmp(&b.1.name));
    for (path, token) in &declared {
        let mut entry = Map::new();
        entry.insert("kind".into(), token.kind.name().into());
        entry.insert("path".into(), (*path).clone().into());
        if let Some(doc) = &token.doc {
            entry.insert("doc".into(), doc.clone().into());
        }
        tokens.insert(token.name.clone(), Value::Object(entry));
    }
    let mut document = Map::new();
    document.insert("cssVars".into(), css_vars(&base.flat()));
    document.insert("tokens".into(), Value::Object(tokens));
    if !modifiers.is_empty() {
        document.insert("modifiers".into(), Value::Object(modifiers));
    }
    let mut names: Vec<(String, String)> = base
        .tokens
        .iter()
        .map(|(p, t)| (p.clone(), t.name.clone()))
        .collect();
    names.sort();
    Ok(Export {
        document: Value::Object(document),
        names,
        left_out: base.left_out,
    })
}

/// Written values by name, each with its light and its dark value.
type Written = BTreeMap<String, (Value, Value)>;

/// A modifier other than the appearance.
struct Axis {
    name: String,
    default: String,
    contexts: Vec<String>,
}

/// A token under one selection.
#[derive(Clone)]
struct Token {
    name: String,
    kind: Kind,
    doc: Option<String>,
    /// Its value in the light and the dark appearance: each written name
    /// (the token's, a text style's parts') with its value.
    light: Vec<(String, Value)>,
    dark: Vec<(String, Value)>,
}

/// The values of one selection: by path, and what was left out.
struct Values {
    tokens: BTreeMap<String, Token>,
    left_out: Vec<String>,
}

impl Values {
    /// Every written name with its light and dark value.
    fn flat(&self) -> BTreeMap<String, (Value, Value)> {
        let mut out = BTreeMap::new();
        for token in self.tokens.values() {
            for ((name, light), (_, dark)) in token.light.iter().zip(&token.dark) {
                out.insert(name.clone(), (light.clone(), dark.clone()));
            }
        }
        out
    }

    /// What `other` sets otherwise than these.
    fn changed_by(&self, other: &Values) -> BTreeMap<String, (Value, Value)> {
        let mine = self.flat();
        other
            .flat()
            .into_iter()
            .filter(|(name, value)| mine.get(name) != Some(value))
            .collect()
    }
}

impl Clone for Values {
    fn clone(&self) -> Self {
        Self {
            tokens: self.tokens.clone(),
            left_out: self.left_out.clone(),
        }
    }
}

/// Written values as `cssVars`: one in both appearances in `theme`, the
/// others in `light` and `dark`.
fn css_vars(values: &BTreeMap<String, (Value, Value)>) -> Value {
    let (mut theme, mut light, mut dark) = (Map::new(), Map::new(), Map::new());
    for (name, (l, d)) in values {
        if l == d {
            theme.insert(name.clone(), l.clone());
        } else {
            light.insert(name.clone(), l.clone());
            dark.insert(name.clone(), d.clone());
        }
    }
    let mut out = Map::new();
    for (section, values) in [("theme", theme), ("light", light), ("dark", dark)] {
        if !values.is_empty() {
            out.insert(section.into(), Value::Object(values));
        }
    }
    Value::Object(out)
}

/// Every combination of the modifiers' contexts.
fn product(axes: &[Axis]) -> Vec<Vec<(String, String)>> {
    let mut out: Vec<Vec<(String, String)>> = vec![Vec::new()];
    for axis in axes {
        out = out
            .into_iter()
            .flat_map(|picked| {
                axis.contexts.iter().map(move |c| {
                    let mut next = picked.clone();
                    next.push((axis.name.clone(), c.clone()));
                    next
                })
            })
            .collect();
    }
    out
}

/// The project as the runtime resolves it, its documents loaded on demand.
struct Project<'a> {
    runtime: DtcgRuntime,
    load: &'a dyn Fn(&str) -> Option<String>,
}

impl Project<'_> {
    /// Loads what `resolution` asks for; `Ok(None)` when it asked.
    fn provide<T>(&mut self, resolution: Resolution<T>) -> Result<Option<T>, ExportError> {
        match resolution {
            Resolution::Ready(value) => Ok(Some(value)),
            Resolution::Missing(requests) => {
                for request in requests {
                    let id = request.resource_id.as_str().to_owned();
                    let Some(text) = (self.load)(&id) else {
                        return Err(ExportError(vec![format!("cannot read `{id}`")]));
                    };
                    self.runtime
                        .provide_requested_json(&request, text)
                        .map_err(|e| ExportError(vec![format!("`{id}`: {e}")]))?;
                }
                Ok(None)
            }
        }
    }

    fn axes(&mut self) -> Result<Vec<dxn_dtcg::ModifierAxis>, ExportError> {
        loop {
            let resolution = self
                .runtime
                .modifiers()
                .map_err(|e| ExportError(vec![e.to_string()]))?;
            if let Some(axes) = self.provide(resolution)? {
                return Ok(axes);
            }
        }
    }

    fn resolve(&mut self, selection: &[(String, String)]) -> Result<ResolvedTokens, ExportError> {
        let selection: Vec<(&str, &str)> = selection
            .iter()
            .map(|(a, c)| (a.as_str(), c.as_str()))
            .collect();
        loop {
            let resolution = self
                .runtime
                .resolve(&selection)
                .map_err(|e| ExportError(vec![e.to_string()]))?;
            if let Some(tokens) = self.provide(resolution)? {
                return Ok(tokens);
            }
        }
    }

    /// The tokens under `picks` (the other modifiers' contexts), in each
    /// appearance; a value that cannot be written joins `errors`.
    fn values(
        &mut self,
        picks: &[(String, String)],
        appearance: Option<&Appearance>,
        options: &Options,
        errors: &mut Vec<String>,
    ) -> Result<Values, ExportError> {
        let with = |context: Option<&String>| {
            let mut selection = picks.to_vec();
            if let (Some(a), Some(c)) = (appearance, context) {
                selection.push((a.modifier.clone(), c.clone()));
            }
            selection
        };
        let light = self.resolve(&with(appearance.map(|a| &a.light)))?;
        let dark = match appearance {
            Some(a) => self.resolve(&with(Some(&a.dark)))?,
            None => light.clone(),
        };
        let extensions = self.runtime.options().extensions.clone();
        let mut out = Values {
            tokens: BTreeMap::new(),
            left_out: Vec::new(),
        };
        for token in light.iter() {
            let path = token.path.to_string();
            let token_type = token.token_type.as_deref().unwrap_or("");
            let Some(kind) = kinds::kind_of(token_type, &path, &token.extensions) else {
                out.left_out.push(format!(
                    "`{path}`: NSH has no kind of a `{token_type}` token"
                ));
                continue;
            };
            let Some(dark_token) = dark.get(&path) else {
                errors.push(format!("`{path}` is in the light appearance only"));
                continue;
            };
            let name = names::name_of(&path, kind, &token.extensions);
            let write = |t: &dxn_dtcg::ResolvedToken| -> Result<Vec<(String, Value)>, String> {
                let typed = t.typed(&extensions).map_err(|e| e.to_string());
                let typed = match (kind, typed) {
                    (Kind::String | Kind::Boolean, _) => dxn_dtcg::values::TypedValue::Number(0.0),
                    (_, typed) => typed?,
                };
                let written = values::write(kind, &typed, &t.value, options.rem, &t.extensions)?;
                Ok(written
                    .into_iter()
                    .map(|(suffix, v)| (format!("{name}{suffix}"), v))
                    .collect())
            };
            match (write(token), write(dark_token)) {
                (Ok(l), Ok(d)) => {
                    out.tokens.insert(
                        path,
                        Token {
                            name,
                            kind,
                            doc: token.description.as_deref().map(str::to_owned),
                            light: l,
                            dark: d,
                        },
                    );
                }
                (Err(e), _) | (_, Err(e)) => errors.push(format!("`{path}`: {e}")),
            }
        }
        Ok(out)
    }
}
