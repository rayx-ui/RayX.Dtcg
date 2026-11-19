//! The project's layers as DTCG authors them: each `resolutionOrder` step,
//! a set or a modifier, with the token definitions of its sources. These
//! become NSH's layers one to one: the sets' entries the tokens' own values,
//! each modifier's contexts its entries.

use nsh_theme::Map;
use serde_json::Value;

/// A token's definition in one source: its authored `$value` and its own
/// `$extensions`.
#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub value: Value,
    pub extensions: Value,
}

/// Definitions by path, in authored order; a later definition of a path
/// replaces an earlier one in its place.
pub type Entries = Map<Definition>;

/// One `resolutionOrder` step's sources, before they are expanded.
#[derive(Clone, Debug)]
pub enum Step {
    /// A set: its name and its sources.
    Set { name: String, sources: Vec<Value> },
    /// A modifier: its name and each context's sources, in authored order.
    Modifier {
        name: String,
        contexts: Vec<(String, Vec<Value>)>,
    },
}

/// The steps of the resolver `root` (its JSON), in `resolutionOrder`.
pub fn steps(root: &Value) -> Result<Vec<Step>, String> {
    let order = root
        .get("resolutionOrder")
        .and_then(Value::as_array)
        .ok_or("the resolver has no `resolutionOrder`")?;
    let mut steps = Vec::with_capacity(order.len());
    for (index, step) in order.iter().enumerate() {
        let what = format!("`resolutionOrder` step {index}");
        let object = step
            .as_object()
            .ok_or_else(|| format!("{what} is no object"))?;
        // A reference to a set or a modifier of the root, its other members
        // overriding the definition's; or an inline step.
        let (kind, name, definition) = match object.get("$ref").and_then(Value::as_str) {
            Some(reference) => {
                let parts: Vec<String> = reference
                    .strip_prefix("#/")
                    .ok_or_else(|| {
                        format!(
                            "{what} refers to `{reference}`: the export reads a resolver's own sets and \
                             modifiers (`#/sets/<name>`, `#/modifiers/<name>`) and inline steps"
                        )
                    })?
                    .split('/')
                    .map(|p| p.replace("~1", "/").replace("~0", "~"))
                    .collect();
                let [kind, name] = parts.as_slice() else {
                    return Err(format!(
                        "{what} refers to `{reference}`, no set or modifier"
                    ));
                };
                let kind = match kind.as_str() {
                    "sets" => "set",
                    "modifiers" => "modifier",
                    _ => {
                        return Err(format!(
                            "{what} refers to `{reference}`, no set or modifier"
                        ));
                    }
                };
                let mut definition = root
                    .get(format!("{kind}s"))
                    .and_then(|d| d.get(name))
                    .and_then(Value::as_object)
                    .cloned()
                    .ok_or_else(|| format!("{what} refers to `{reference}`, which is not there"))?;
                for (member, value) in object.iter().filter(|(m, _)| *m != "$ref") {
                    definition.insert(member.clone(), value.clone());
                }
                (kind, name.clone(), definition)
            }
            None => {
                let kind = match object.get("type").and_then(Value::as_str) {
                    Some(kind @ ("set" | "modifier")) => kind,
                    _ => return Err(format!("{what} is no set or modifier")),
                };
                let name = object
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or_else(|| format!("{what} has no name"))?;
                (kind, name.to_owned(), object.clone())
            }
        };
        let list = |value: Option<&Value>, what: &str| -> Result<Vec<Value>, String> {
            value
                .and_then(Value::as_array)
                .cloned()
                .ok_or_else(|| format!("{what} is no list of sources"))
        };
        steps.push(if kind == "set" {
            Step::Set {
                sources: list(
                    definition.get("sources"),
                    &format!("set `{name}`'s `sources`"),
                )?,
                name,
            }
        } else {
            let contexts = definition
                .get("contexts")
                .and_then(Value::as_object)
                .ok_or_else(|| format!("modifier `{name}` has no `contexts`"))?
                .iter()
                .map(|(context, sources)| {
                    list(
                        Some(sources),
                        &format!("modifier `{name}`'s context `{context}`"),
                    )
                    .map(|sources| (context.clone(), sources))
                })
                .collect::<Result<_, _>>()?;
            Step::Modifier { name, contexts }
        });
    }
    Ok(steps)
}

/// A group's `$extends`: the group at `group` copies the tokens of the group
/// at `target`, its own members winning.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Extends {
    pub group: String,
    pub target: String,
}

/// Adds the token definitions of `source`, a token document's JSON (or the
/// part of one a source names), to `entries`, a later definition replacing
/// an earlier one, and its groups' `$extends` to `extends`.
pub fn flatten(
    source: &Value,
    entries: &mut Entries,
    extends: &mut Vec<Extends>,
) -> Result<(), String> {
    match source.as_object() {
        Some(_) => group("", source, entries, extends),
        None => Err("a source is no JSON object".into()),
    }
}

/// `prefix.name`, or `name` at the root.
pub fn join(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_owned()
    } else {
        format!("{prefix}.{name}")
    }
}

/// The group path an `$extends` names: an alias `{a.b}` or a local JSON
/// pointer `#/a/b`, as `dxn_dtcg` reads it.
fn extended(prefix: &str, reference: &Value) -> Result<String, String> {
    let fail = || {
        format!(
            "the group `{prefix}` extends {reference}, which is no group alias `{{path}}` or local \
             JSON pointer `#/path`"
        )
    };
    let text = reference.as_str().ok_or_else(fail)?;
    let path = match text.strip_prefix("#/") {
        Some(pointer) => pointer
            .split('/')
            .map(|p| p.replace("~1", "/").replace("~0", "~"))
            .collect::<Vec<_>>()
            .join("."),
        None => text
            .strip_prefix('{')
            .and_then(|t| t.strip_suffix('}'))
            .ok_or_else(fail)?
            .to_owned(),
    };
    if path.is_empty() {
        return Err(fail());
    }
    Ok(path)
}

fn group(
    prefix: &str,
    value: &Value,
    entries: &mut Entries,
    extends: &mut Vec<Extends>,
) -> Result<(), String> {
    let Some(object) = value.as_object() else {
        return Ok(());
    };
    if let Some(reference) = object.get("$extends") {
        extends.push(Extends {
            group: prefix.to_owned(),
            target: extended(prefix, reference)?,
        });
    }
    for (name, member) in object {
        let path = match name.as_str() {
            "$root" => format!("{prefix}.$root"),
            n if n.starts_with('$') => continue,
            n => join(prefix, n),
        };
        let Some(token) = member.as_object() else {
            continue;
        };
        if let Some(value) = token.get("$value") {
            entries.insert(
                path,
                Definition {
                    value: value.clone(),
                    extensions: token.get("$extensions").cloned().unwrap_or(Value::Null),
                },
            );
        } else if let Some(target) = token.get("$ref") {
            return Err(format!(
                "`{path}` is a token-level `$ref` ({target}), which NSH's references cannot express: \
                 write an alias `{{path}}`"
            ));
        } else if name != "$root" {
            group(&path, member, entries, extends)?;
        }
    }
    Ok(())
}
