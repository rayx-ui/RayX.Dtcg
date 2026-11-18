//! Exports a DTCG 2025.10 design-token project (a token document, or a
//! resolver with its sets and modifiers) as an NSH theme document, the format
//! of the NSH fork's `docs/dev/theme-format.md`, built with `nsh-theme`.
//!
//! - **Tokens.** Every token of every set and context, each with its name,
//!   its value type (`kind`), its value, its DTCG `path` and its `doc`.
//! - **Names.** `$extensions["dev.nsh"].name` when the token pins one, else
//!   its path's segments in kebab case joined by `-` ([`names`]).
//! - **Value types.** A fixed table from `$type` ([`types`]), which
//!   `$extensions["dev.nsh"].type` overrides.
//! - **Values** in NSH's forms ([`values`]); an alias stays a reference.
//! - **Layers.** The sets are merged into `tokens`; each modifier keeps its
//!   name, its contexts and its default, and each context holds exactly its
//!   DTCG entries; `resolutionOrder` keeps DTCG's order of the modifiers.
//! - **Roles.** With a role map, `roles` maps each role onto its path's
//!   token; with role declarations, the document is checked against them.
//! - **Exactness.** [`verify_exact`] resolves every combination of contexts
//!   with `dxn_dtcg` and the document with `nsh_theme::resolve`, and refuses
//!   a token whose values differ. [`export`] runs it before it returns.
//!
//! The library reads nothing itself: [`export`] takes the project's files
//! from a loader. The `rayx-dtcg` command reads them from disk.

pub mod layers;
pub mod names;
pub mod roles;
pub mod types;
pub mod values;

use std::collections::{HashMap, HashSet};

use dxn_dtcg::{
    DtcgRuntime, ModifierAxis, Resolution, ResolvedTokens, RuntimeLimits, RuntimeOptions,
    ValueExtensions,
};
use nsh_theme::{Context, Modifier, Selection, ThemeDocument, Token, ValueType};
use serde_json::Value;

use crate::{
    layers::{Definition, Entries, Extends, Step},
    values::{Converter, Target},
};

/// How a project is exported.
#[derive(Clone, Debug)]
pub struct Options {
    /// Points per `rem`.
    pub rem: f64,
    /// The most combinations of contexts the structure and exactness checks
    /// resolve.
    pub max_combinations: usize,
    /// The role map's text: a JSON object from a role's name to a DTCG path.
    pub role_map: Option<String>,
    /// The role declarations' text: `[{ "name", "type", "doc"?, "required"? }]`.
    pub roles: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            rem: 16.0,
            max_combinations: 4096,
            role_map: None,
            roles: None,
        }
    }
}

/// An exported project.
#[derive(Clone, Debug)]
pub struct Export {
    /// The NSH theme document.
    pub document: ThemeDocument,
    /// What the role check warned about and kept (`role-unknown`), each as
    /// `<severity> <code> <pointer>: <message>`.
    pub warnings: Vec<String>,
}

impl Export {
    /// The document as it is written: [`nsh_theme::write`]'s text, which ends
    /// in a newline.
    pub fn text(&self) -> String {
        nsh_theme::write(&self.document)
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
    let mut project = Project::new(root, load, options)?;
    let axes = project.axes()?;
    project.combinations(&axes, options)?;
    project.structure()?;
    let base = project.resolve(&[])?;
    let root_json = project.root_json()?;
    let mut errors = Vec::new();

    // The layers, as DTCG authors them.
    let mut layers = project.layers(&root_json, &axes)?;
    expand(&mut layers);

    // Tokens: names and value types.
    let mut targets: HashMap<String, Target> = HashMap::new();
    let mut by_name: HashMap<String, String> = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    for token in base.iter() {
        let path = token.path.to_string();
        let token_type = token.token_type.as_deref().unwrap_or("").to_owned();
        let pins: Vec<&Value> = std::iter::once(&token.extensions)
            .chain(
                layers
                    .iter()
                    .flat_map(Layer::entries)
                    .filter_map(|entries| entries.get(&path))
                    .map(|d| &d.extensions),
            )
            .collect();
        let pinned_name = agreed(
            &path,
            "name",
            pins.iter().map(|e| names::pinned(e)),
            &mut errors,
        );
        let pinned_type = agreed(
            &path,
            "type",
            pins.iter().map(|e| {
                e.get("dev.nsh")
                    .and_then(|n| n.get("type"))
                    .map(Value::to_string)
            }),
            &mut errors,
        );
        let Some(table) = types::value_type_of(&token_type) else {
            errors.push(format!(
                "`{path}`: a `{token_type}` token has no value type in NSH"
            ));
            continue;
        };
        let value_type = match pinned_type.and_then(|_| pins.iter().find_map(|e| types::pinned(e)))
        {
            None => table,
            Some(Ok(t)) => t,
            Some(Err(name)) => {
                errors.push(format!(
                    "`{path}`: $extensions[\"dev.nsh\"].type names `{name}`, which is no NSH value type"
                ));
                continue;
            }
        };
        let name = pinned_name.map_or_else(|| names::path_name(&path), str::to_owned);
        if !names::valid(&name) {
            errors.push(format!(
                "`{path}`: NSH refuses the name `{name}` (bad-name): a name is a non-empty string \
                 without `{{` or `}}`"
            ));
            continue;
        }
        if let Some(other) = by_name.insert(name.clone(), path.clone()) {
            errors.push(format!(
                "`{other}` and `{path}` are both named `{name}`; pin one's name with \
                 $extensions[\"dev.nsh\"].name"
            ));
            continue;
        }
        targets.insert(
            path.clone(),
            Target {
                name,
                value_type,
                token_type,
            },
        );
        order.push(path);
    }
    if !errors.is_empty() {
        return Err(failed(errors));
    }

    let converter = Converter {
        rem: options.rem,
        targets: &targets,
    };
    let convert =
        |path: &str, definition: &layers::Definition, errors: &mut Vec<String>, at: &str| {
            let target = &targets[path];
            match converter.convert(
                &target.token_type,
                target.value_type,
                &definition.value,
                &definition.extensions,
            ) {
                Ok(value) => Some(value),
                Err(e) => {
                    errors.push(format!("`{path}`{at}: {e}"));
                    None
                }
            }
        };

    // A set after a modifier may set no token that modifier sets: NSH's
    // tokens are its lowest layer.
    for (i, layer) in layers.iter().enumerate() {
        let Layer::Set {
            name: set, entries, ..
        } = layer
        else {
            continue;
        };
        for earlier in &layers[..i] {
            let Layer::Modifier {
                name: modifier,
                contexts,
                ..
            } = earlier
            else {
                continue;
            };
            for path in entries.keys() {
                if contexts.iter().any(|(_, c)| c.contains_key(path)) {
                    errors.push(format!(
                        "set `{set}` comes after modifier `{modifier}` in `resolutionOrder` and sets \
                         `{path}`, which `{modifier}` also sets: NSH's tokens are its lowest layer, \
                         below every modifier"
                    ));
                }
            }
        }
    }

    // Every token's own value: the last set's entry, else the entry of the
    // last modifier whose default context sets it.
    let mut document = ThemeDocument::new();
    if root_json.get("resolutionOrder").is_some() {
        document.name = root_json
            .get("name")
            .and_then(Value::as_str)
            .map(str::to_owned);
    }
    for path in &order {
        let Some((definition, at)) = own_definition(&layers, path) else {
            errors.push(format!(
                "`{path}` is set by no set and by no modifier's default context the export can read"
            ));
            continue;
        };
        let Some(value) = convert(path, definition, &mut errors, &at) else {
            continue;
        };
        let target = &targets[path];
        let mut token = Token::new(target.value_type, value).with_path(path.clone());
        if let Some(doc) = base.get(path).and_then(|t| t.description.as_deref()) {
            token = token.with_doc(doc);
        }
        document.insert_token(target.name.clone(), token);
    }

    // The modifiers, one to one.
    for layer in &layers {
        let Layer::Modifier {
            name,
            default,
            contexts,
            ..
        } = layer
        else {
            continue;
        };
        let mut modifier = Modifier::new(default.clone());
        for (context_name, entries) in contexts {
            let mut context = Context::new();
            for (path, definition) in entries.iter() {
                let Some(target) = targets.get(path) else {
                    errors.push(format!(
                        "`{path}` in `{name}={context_name}` is no token of the default selection"
                    ));
                    continue;
                };
                if let Some(value) = convert(
                    path,
                    definition,
                    &mut errors,
                    &format!(" in `{name}={context_name}`"),
                ) {
                    context.set(target.name.clone(), value);
                }
            }
            modifier.insert_context(context_name.clone(), context);
        }
        document.insert_modifier(name.clone(), modifier);
        document.resolution_order.push(name.clone());
    }
    if !errors.is_empty() {
        return Err(failed(errors));
    }

    // Roles.
    let mut warnings = Vec::new();
    if let Some(text) = &options.role_map {
        let map = roles::parse_map(text).map_err(failed)?;
        for (role, path) in map {
            if !names::valid(&role) {
                errors.push(format!(
                    "role `{role}`: NSH refuses the name: a name is a non-empty string without `{{` or `}}`"
                ));
                continue;
            }
            match targets.get(&path) {
                Some(target) => {
                    document.set_role(role, target.name.clone());
                }
                None => errors.push(format!(
                    "role `{role}` maps onto `{path}`, which names no token"
                )),
            }
        }
    }
    if !errors.is_empty() {
        return Err(failed(errors));
    }

    // What NSH would refuse in the document.
    for diagnostic in nsh_theme::check(&document) {
        if diagnostic.is_error() {
            errors.push(described(&diagnostic));
        }
    }
    if let Some(text) = &options.roles {
        let declarations = roles::parse_declarations(text).map_err(failed)?;
        let resolved = nsh_theme::resolve(&document, &Selection::new(), &[]);
        for diagnostic in nsh_theme::check_roles(&document, &[], &resolved, &declarations, &[]) {
            if diagnostic.is_error() || diagnostic.code == nsh_theme::code::ROLE_UNMAPPED {
                errors.push(described(&diagnostic));
            } else {
                warnings.push(described(&diagnostic));
            }
        }
    }
    if !errors.is_empty() {
        return Err(failed(errors));
    }

    project.exact(&axes, &document, options)?;
    Ok(Export { document, warnings })
}

/// Checks that `document` resolves, under every combination of one context
/// per modifier, to the values `dxn_dtcg` resolves the project to: every
/// token's value in NSH's forms. Refuses each token that differs, naming it
/// and the selection, and a project of more combinations than
/// `options.max_combinations`, with its count.
pub fn verify_exact(
    root: &str,
    load: &dyn Fn(&str) -> Option<String>,
    options: &Options,
    document: &ThemeDocument,
) -> Result<(), ExportError> {
    let mut project = Project::new(root, load, options)?;
    let axes = project.axes()?;
    project.exact(&axes, document, options)
}

/// `<severity> <code> <pointer>: <message>`.
fn described(diagnostic: &nsh_theme::Diagnostic) -> String {
    format!(
        "{} {} {}: {}",
        diagnostic.severity.name(),
        diagnostic.code,
        diagnostic.pointer,
        diagnostic.message
    )
}

/// The reasons, sorted and once each.
fn failed(mut errors: Vec<String>) -> ExportError {
    errors.sort();
    errors.dedup();
    ExportError(errors)
}

/// The one value a token's definitions pin as `$extensions["dev.nsh"].<what>`;
/// two different ones are refused.
fn agreed<T: PartialEq + std::fmt::Debug>(
    path: &str,
    what: &str,
    pins: impl Iterator<Item = Option<T>>,
    errors: &mut Vec<String>,
) -> Option<T> {
    let mut found: Option<T> = None;
    for pin in pins.flatten() {
        match &found {
            None => found = Some(pin),
            Some(first) if *first != pin => {
                errors.push(format!(
                    "`{path}`'s definitions pin different $extensions[\"dev.nsh\"].{what}s \
                     ({first:?} and {pin:?}); a token has one"
                ));
                return None;
            }
            Some(_) => {}
        }
    }
    found
}

/// A `resolutionOrder` step with its sources' definitions and their
/// groups' `$extends`.
enum Layer {
    Set {
        name: String,
        entries: Entries,
        extends: Vec<Extends>,
    },
    Modifier {
        name: String,
        default: String,
        contexts: Vec<(String, Entries)>,
        /// Each context's, in `contexts`' order.
        extends: Vec<Vec<Extends>>,
    },
}

impl Layer {
    fn entries(&self) -> Vec<&Entries> {
        match self {
            Layer::Set { entries, .. } => vec![entries],
            Layer::Modifier { contexts, .. } => contexts.iter().map(|(_, e)| e).collect(),
        }
    }
}

/// A token's own value: the last set's entry, else the entry of the last
/// modifier whose default context sets it, with where it is for messages.
fn own_definition<'a>(layers: &'a [Layer], path: &str) -> Option<(&'a Definition, String)> {
    let own = layers.iter().rev().find_map(|layer| match layer {
        Layer::Set { entries, .. } => entries.get(path).map(|d| (d, String::new())),
        Layer::Modifier { .. } => None,
    });
    own.or_else(|| {
        layers.iter().rev().find_map(|layer| match layer {
            Layer::Modifier {
                name,
                default,
                contexts,
                ..
            } => contexts
                .iter()
                .find(|(c, _)| c == default)
                .and_then(|(_, entries)| entries.get(path))
                .map(|d| (d, format!(" in `{name}={default}`"))),
            Layer::Set { .. } => None,
        })
    })
}

/// One layer's entries: `(layer, context)`, the context 0 for a set.
type Slot = (usize, usize);

fn slot(layers: &[Layer], (layer, context): Slot) -> &Entries {
    match &layers[layer] {
        Layer::Set { entries, .. } => entries,
        Layer::Modifier { contexts, .. } => &contexts[context].1,
    }
}

fn slot_mut(layers: &mut [Layer], (layer, context): Slot) -> &mut Entries {
    match &mut layers[layer] {
        Layer::Set { entries, .. } => entries,
        Layer::Modifier { contexts, .. } => &mut contexts[context].1,
    }
}

/// Copies each extended group's token definitions under the extending
/// group's path, as DTCG's `$extends` does: the extending group's own
/// members win, and a copied alias stays an alias.
///
/// - An `$extends` in a set holds in every selection, so the target's
///   definitions in every layer are copied into that layer: the copy then
///   follows whatever context sets the target.
/// - An `$extends` in a context holds only when it is picked, so the copies
///   go into that context: the target's definition there, else the
///   target's own value.
/// - A member the extending group defines itself, in the same layer or in
///   any set, is not copied over.
///
/// The target may be in any document, and may itself be extended: the
/// copies repeat until nothing changes. The exactness check proves the
/// result under every combination.
fn expand(layers: &mut [Layer]) {
    let mut slots: Vec<Slot> = Vec::new();
    let mut directives: Vec<(Slot, bool, Extends)> = Vec::new();
    for (i, layer) in layers.iter().enumerate() {
        match layer {
            Layer::Set { extends, .. } => {
                slots.push((i, 0));
                directives.extend(extends.iter().map(|e| ((i, 0), true, e.clone())));
            }
            Layer::Modifier { extends, .. } => {
                for (c, context) in extends.iter().enumerate() {
                    slots.push((i, c));
                    directives.extend(context.iter().map(|e| ((i, c), false, e.clone())));
                }
            }
        }
    }
    // A member a set defines is the extending group's own in every
    // selection, so it is never copied over.
    let in_sets: HashSet<String> = layers
        .iter()
        .filter_map(|layer| match layer {
            Layer::Set { entries, .. } => Some(entries.keys().map(str::to_owned)),
            Layer::Modifier { .. } => None,
        })
        .flatten()
        .collect();
    let mut copied: HashSet<(Slot, String)> = HashSet::new();
    for _ in 0..=directives.len() {
        let mut changes: Vec<(Slot, String, Definition)> = Vec::new();
        for (at, in_set, extends) in &directives {
            let prefix = format!("{}.", extends.target);
            if *in_set {
                for &each in &slots {
                    for (path, definition) in slot(layers, each).iter() {
                        if let Some(rest) = path.strip_prefix(&prefix) {
                            changes.push((
                                each,
                                layers::join(&extends.group, rest),
                                definition.clone(),
                            ));
                        }
                    }
                }
            } else {
                let mut paths: Vec<&str> = Vec::new();
                for &each in &slots {
                    for path in slot(layers, each).keys() {
                        if path.starts_with(&prefix) && !paths.contains(&path) {
                            paths.push(path);
                        }
                    }
                }
                for path in paths {
                    let definition = slot(layers, *at)
                        .get(path)
                        .or_else(|| own_definition(layers, path).map(|(d, _)| d));
                    if let Some(definition) = definition {
                        changes.push((
                            *at,
                            layers::join(&extends.group, &path[prefix.len()..]),
                            definition.clone(),
                        ));
                    }
                }
            }
        }
        let mut changed = false;
        for (at, path, definition) in changes {
            let key = (at, path);
            let entries = slot_mut(layers, at);
            if in_sets.contains(&key.1) || (entries.contains_key(&key.1) && !copied.contains(&key))
            {
                // The extending group's own member wins.
                continue;
            }
            if entries.get(&key.1) != Some(&definition) {
                entries.insert(key.1.clone(), definition);
                copied.insert(key);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

/// `a=x, b=y`.
fn named(selection: &[(String, String)]) -> String {
    selection
        .iter()
        .map(|(a, c)| format!("{a}={c}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Every combination of one context per modifier, the first modifier
/// varying slowest.
fn product(axes: &[ModifierAxis]) -> Vec<Vec<(String, String)>> {
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

/// A modifier's default context: its `default`, else its first.
fn default_of(axis: &ModifierAxis) -> String {
    axis.default
        .clone()
        .unwrap_or_else(|| axis.contexts[0].clone())
}

/// The project as the runtime resolves it, its documents loaded on demand.
struct Project<'a> {
    runtime: DtcgRuntime,
    root: String,
    load: &'a dyn Fn(&str) -> Option<String>,
}

impl<'a> Project<'a> {
    fn new(
        root: &str,
        load: &'a dyn Fn(&str) -> Option<String>,
        options: &Options,
    ) -> Result<Self, ExportError> {
        let mut project = Project {
            runtime: DtcgRuntime::with_options(
                RuntimeOptions::default()
                    .with_extensions(ValueExtensions::all())
                    .with_limits(
                        RuntimeLimits::default().with_max_selections(options.max_combinations),
                    ),
            ),
            root: root.to_owned(),
            load,
        };
        project
            .runtime
            .set_root(root)
            .map_err(|e| ExportError(vec![e.to_string()]))?;
        Ok(project)
    }

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

    /// Runs `step` until the documents it needs are loaded.
    fn ready<T>(
        &mut self,
        step: impl Fn(&DtcgRuntime) -> Result<Resolution<T>, dxn_dtcg::DtcgError>,
    ) -> Result<T, ExportError> {
        loop {
            let resolution = step(&self.runtime).map_err(|e| ExportError(vec![e.to_string()]))?;
            if let Some(value) = self.provide(resolution)? {
                return Ok(value);
            }
        }
    }

    fn root_json(&self) -> Result<Value, ExportError> {
        let text = (self.load)(&self.root)
            .ok_or_else(|| ExportError(vec![format!("cannot read `{}`", self.root)]))?;
        serde_json::from_str(&text).map_err(|e| ExportError(vec![format!("`{}`: {e}", self.root)]))
    }

    fn axes(&mut self) -> Result<Vec<ModifierAxis>, ExportError> {
        self.ready(|runtime| runtime.modifiers())
    }

    /// Refuses more combinations of contexts than `max_combinations`.
    fn combinations(&self, axes: &[ModifierAxis], options: &Options) -> Result<usize, ExportError> {
        let count = axes
            .iter()
            .try_fold(1usize, |n, a| n.checked_mul(a.contexts.len()))
            .unwrap_or(usize::MAX);
        if count > options.max_combinations {
            return Err(ExportError(vec![format!(
                "the project has {count} combinations of contexts, more than the {} the checks \
                 resolve (--max-combinations)",
                options.max_combinations
            )]));
        }
        Ok(count)
    }

    fn resolve(&mut self, selection: &[(String, String)]) -> Result<ResolvedTokens, ExportError> {
        let selection: Vec<(&str, &str)> = selection
            .iter()
            .map(|(a, c)| (a.as_str(), c.as_str()))
            .collect();
        self.ready(|runtime| runtime.resolve(&selection))
    }

    /// The definitions of `source`'s token documents, in order.
    fn definitions(
        &mut self,
        source: &Value,
        entries: &mut Entries,
        extends: &mut Vec<Extends>,
    ) -> Result<(), ExportError> {
        let occurrences = self.ready(|runtime| runtime.inspect_source(source, &[]))?;
        for occurrence in occurrences {
            layers::flatten(occurrence.value(), entries, extends).map_err(|e| {
                ExportError(vec![format!("`{}`: {e}", occurrence.resource().as_str())])
            })?;
        }
        Ok(())
    }

    /// Every `resolutionOrder` step with its sources' definitions; a token
    /// document is one set.
    fn layers(&mut self, root: &Value, axes: &[ModifierAxis]) -> Result<Vec<Layer>, ExportError> {
        if root.get("resolutionOrder").is_none() {
            let mut entries = Entries::new();
            let mut extends = Vec::new();
            layers::flatten(root, &mut entries, &mut extends)
                .map_err(|e| ExportError(vec![format!("`{}`: {e}", self.root)]))?;
            return Ok(vec![Layer::Set {
                name: self.root.clone(),
                entries,
                extends,
            }]);
        }
        let steps = layers::steps(root).map_err(|e| ExportError(vec![e]))?;
        let mut out = Vec::with_capacity(steps.len());
        for step in steps {
            match step {
                Step::Set { name, sources } => {
                    let mut entries = Entries::new();
                    let mut extends = Vec::new();
                    for source in &sources {
                        self.definitions(source, &mut entries, &mut extends)?;
                    }
                    out.push(Layer::Set {
                        name,
                        entries,
                        extends,
                    });
                }
                Step::Modifier { name, contexts } => {
                    let Some(axis) = axes.iter().find(|a| a.name == name) else {
                        return Err(ExportError(vec![format!(
                            "the modifier `{name}` is no modifier of the resolver"
                        )]));
                    };
                    let mut read = Vec::with_capacity(contexts.len());
                    let mut extends = Vec::with_capacity(contexts.len());
                    for (context, sources) in contexts {
                        let mut entries = Entries::new();
                        let mut context_extends = Vec::new();
                        for source in &sources {
                            self.definitions(source, &mut entries, &mut context_extends)?;
                        }
                        read.push((context, entries));
                        extends.push(context_extends);
                    }
                    out.push(Layer::Modifier {
                        name,
                        default: default_of(axis),
                        contexts: read,
                        extends,
                    });
                }
            }
        }
        Ok(out)
    }

    /// Every selection's structure compared ([`dxn_dtcg::DtcgRuntime::check_structure`]):
    /// NSH's contexts add no names and change no type.
    fn structure(&mut self) -> Result<(), ExportError> {
        let space = self.ready(|runtime| runtime.selection_space())?;
        let report = self.ready(|runtime| runtime.check_structure(&space))?;
        let mut errors = Vec::new();
        for failure in &report.failures {
            errors.push(if failure.selection.is_empty() {
                failure.error.to_string()
            } else {
                format!("under {}: {}", named(&failure.selection), failure.error)
            });
        }
        for difference in &report.differences {
            let missing = difference.missing_in();
            if missing.is_empty() {
                let types: Vec<&str> = difference
                    .distinct_types()
                    .iter()
                    .map(|t| t.name())
                    .collect();
                errors.push(format!(
                    "`{}` is a {} in some contexts: a token keeps one type in every context",
                    difference.path,
                    types.join(" or a ")
                ));
            } else {
                let first = &report.selections[missing[0]];
                errors.push(format!(
                    "`{}` is missing under {}{}: a context adds no tokens and removes none",
                    difference.path,
                    named(first),
                    match missing.len() {
                        1 => String::new(),
                        n => format!(" (and {} more selections)", n - 1),
                    }
                ));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(failed(errors))
        }
    }

    /// [`verify_exact`] on this project.
    fn exact(
        &mut self,
        axes: &[ModifierAxis],
        document: &ThemeDocument,
        options: &Options,
    ) -> Result<(), ExportError> {
        self.combinations(axes, options)?;
        let mut by_path: HashMap<&str, (&str, ValueType)> = HashMap::new();
        for (name, token) in document.tokens.iter() {
            if let Some(path) = &token.path {
                by_path.insert(path, (name, token.value_type));
            }
        }
        let none = HashMap::new();
        let converter = Converter {
            rem: options.rem,
            targets: &none,
        };
        let mut errors = Vec::new();
        for combination in product(axes) {
            let under = if combination.is_empty() {
                String::new()
            } else {
                format!(" under {}", named(&combination))
            };
            let tokens = self.resolve(&combination)?;
            let mut selection = Selection::new();
            for (modifier, context) in &combination {
                selection.set(modifier.clone(), context.clone());
            }
            let resolved = nsh_theme::resolve(document, &selection, &[]);
            for diagnostic in &resolved.diagnostics {
                errors.push(format!("the document{under}: {}", described(diagnostic)));
            }
            let mut seen = HashSet::new();
            for token in tokens.iter() {
                let path: &str = &token.path;
                let Some(&(name, value_type)) = by_path.get(path) else {
                    errors.push(format!("`{path}`{under} is no token of the document"));
                    continue;
                };
                seen.insert(path);
                let token_type = token.token_type.as_deref().unwrap_or("");
                let expected = match converter.convert(
                    token_type,
                    value_type,
                    &token.value,
                    &token.extensions,
                ) {
                    Ok(value) => value,
                    Err(e) => {
                        errors.push(format!("`{path}`{under}: {e}"));
                        continue;
                    }
                };
                match resolved.get(name) {
                    Some(actual) if *actual == expected => {}
                    actual => errors.push(format!(
                        "`{path}` (`{name}`){under}: DTCG resolves it to {}, the document to {}",
                        expected.to_json(),
                        actual.map_or("nothing".to_owned(), |v| v.to_json().to_string())
                    )),
                }
            }
            for path in by_path.keys() {
                if !seen.contains(path) {
                    errors.push(format!("`{path}`{under} is no token of the project"));
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(failed(errors))
        }
    }
}
