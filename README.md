# RayX.Dtcg

Exports a [DTCG 2025.10](https://www.designtokens.org/TR/2025.10/) design-token project (a token document, or a
resolver with its sets and modifiers) as an **NSH theme document**: the format of NSH's `docs/dev/theme-format.md`
(in the [NSH fork](https://github.com/dXnSix/nsh)), written with its `nsh-theme` crate. Design tools keep DTCG as
their source; an app ships one theme document, whose tokens, modifiers, references and roles NSH resolves itself.

```text
rayx-dtcg design/acme.resolver.json -o theme.json --role-map design/acme.role-map.json --roles roles.json
rayx-dtcg design/acme.resolver.json -o theme.json --check      # in CI: fails when theme.json is missing or stale
```

The library, `rayx_dtcg`, does the same without I/O: `export(root, &load, &options)` takes the project's files
from a loader and returns the document (an `nsh_theme::ThemeDocument`) and the role check's warnings. It resolves
the project with [`dxn_dtcg`](https://github.com/dXnSix/dXn.Dtcg).

## What the export writes

```json
{
  "name": "Acme",
  "tokens": {
    "colors-accent": { "kind": "color", "value": "#0000ff", "path": "colors.accent" },
    "colors-link": { "kind": "color", "value": "{colors-accent}", "path": "colors.link" },
    "radius-card": { "kind": "number", "value": 12, "path": "radius.card", "doc": "A card's corners." }
  },
  "modifiers": {
    "theme": { "default": "light", "contexts": { "light": {}, "dark": { "colors-accent": "#8080ff" } } },
    "density": { "default": "comfortable", "contexts": { "comfortable": {}, "compact": { "radius-card": 6 } } }
  },
  "resolutionOrder": ["theme", "density"],
  "roles": { "primary": "colors-accent" }
}
```

- **Tokens.** Every token of every set and context, each with its name, its value type (`kind`), its value, its
  DTCG `path` (joined by `.`) and its `$description` as `doc`.
- **Names.** `$extensions["dev.nsh"].name` when the token pins one, else its path's segments in kebab case joined by
  `-` (`theme.colors.surfaces.textMuted` → `theme-colors-surfaces-text-muted`, `title1Bold` → `title1-bold`). Two
  tokens of one name are refused, naming both paths, and so is a pinned name NSH refuses.
- **Value types**, from `$type`, unless `$extensions["dev.nsh"].type` names one of NSH's value types instead:

  | `$type` | `kind` |
  |---|---|
  | `color` | `color` |
  | `dimension`, `number`, `fontWeight` | `number` |
  | `fontFamily` | `font` |
  | `duration` | `duration` |
  | `cubicBezier` | `easing` |
  | `typography`, `shadow`, `gradient`, `border`, `transition` | the same |
  | `dxn.dtcg.blur` | `blur` |
  | `dxn.dtcg.string`, `dxn.dtcg.boolean` | `string`, `boolean` |

  `strokeStyle` and every other type are refused. Figma scopes are not read.
- **Values** in NSH's forms. Colors as sRGB `#rrggbb` (`#rrggbbaa` below full opacity), other color spaces converted
  by `dxn_dtcg`, each channel rounded half away from zero. Dimensions as numbers of points (`rem` times `--rem`, 16
  by default). Font weights as numbers, font families as one string (a list joined with `, `), durations as `<n>ms`,
  cubic Béziers as easings. A typography's `lineHeight` stays a multiple when it is a number, becomes `"<n>px"` when
  it is a dimension, and stays a reference when it aliases a dimension token. A shadow's `offsetX` and `offsetY`
  become `x` and `y`. A gradient's geometry comes from `$extensions["dev.nsh"].gradient` (`{ "type": "linear" |
  "radial" | "conic", "angle": <deg> }`; linear at 180° by default). A border's style is `solid`, `dashed` or
  `dotted`; a transition's `timingFunction` becomes its `easing`.
- **References.** An alias stays a reference, never flattened: `{a.b}` becomes `{<name of a.b>}`, as a whole value
  or a compound's member, so NSH resolves it after its layers, as DTCG resolves aliases after its sources. A
  JSON-pointer `$ref`, an alias to a group and an alias to a token of the wrong value type are refused.
- **Layers.** NSH's layers are DTCG's resolution, copied one to one: the sets are merged into `tokens` (the later
  set winning), each modifier keeps its name, its contexts and its default, each context holds exactly its DTCG
  entries, and `resolutionOrder` keeps DTCG's order of the modifiers. A token only modifiers set takes the entry of
  the last modifier whose default context sets it. A set placed after a modifier may set no token that modifier
  sets: NSH's tokens are its lowest layer, so that is refused. Every selection must have the same tokens with the
  same types (`dxn_dtcg`'s structure check), since NSH's contexts add no names.
- **`$extends`.** A group that extends another (`"$extends": "{group}"` or `"#/group"`, in any document) gets a copy
  of the extended group's token definitions under its own path, its own members winning and copied aliases staying
  aliases. An `$extends` in a set copies the target's entries in every layer, so the copy follows the contexts that
  set the target; one in a context copies into that context.
- **Roles.** `--role-map <file>` is a JSON object from a role's name to a DTCG path (`{ "primary": "colors.accent"
  }`); the document's `roles` maps each role onto that path's token, in the file's order. `--roles <file>` is NSH's
  role-declaration JSON (`[{ "name", "type", "doc"?, "required"? }]`): with it the export runs
  `nsh_theme::check_roles` and prints each diagnostic as `<severity> <code> <pointer>: <message>`. It fails on
  `role-type` and `role-unmapped`. A mapping nothing declares is kept: onto a color token silently (NSH's bare
  color form reads any name), onto any other token with the warning `role-unknown`.
- **Exactness.** Before it writes, the export resolves every combination of one context per modifier (at most
  `--max-combinations`, 4 096 by default) with `dxn_dtcg` and the document with `nsh_theme::resolve`, and refuses
  any token whose values differ, naming it and the selection (`under accent=sky, density=touch`). The library's
  `verify_exact(root, &load, &options, &document)` runs the same check on any document.

Every refusal is reported at once, each reason naming the token. The text is `nsh_theme::write`'s, with a final
newline: the same project always gives the same bytes.

## The command

```text
rayx-dtcg <ROOT> -o <FILE> [--role-map <FILE>] [--roles <FILE>] [--rem <PX>] [--max-combinations <N>] [--check]
```

- `--check` writes nothing: it compares the export with `FILE` byte for byte and exits with 1 when the file is
  missing or stale, saying which.
- A refused project exits with 1 and prints every reason; a usage error exits with 2.

## Install

```toml
[dependencies]
rayx_dtcg = { git = "https://github.com/rayx-ui/RayX.Dtcg", tag = "vX.Y.Z" }
```

The command:

```bash
cargo install --git https://github.com/rayx-ui/RayX.Dtcg --tag vX.Y.Z rayx_dtcg
```

`v0.2.0` and earlier wrote NSH's old `cssVars` theme document. `v0.1.0` predates the rename: its package is
`dxn_dtcg_nsh` and its command `dtcg-nsh`.

Releases are listed on the [Releases page](https://github.com/rayx-ui/RayX.Dtcg/releases); see
[CHANGELOG.md](CHANGELOG.md) and [docs/releasing.md](docs/releasing.md).

## License

MIT OR Apache-2.0.
