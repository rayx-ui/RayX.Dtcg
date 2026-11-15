# dXn.DtcgNsh

Exports a [DTCG 2025.10](https://www.designtokens.org/TR/2025.10/) design-token project (a token document, or a
resolver with its sets and modifiers) as an **NSH theme document**: the theme file an NSH app ships. Design tools
(a token editor, RayX Sketch) keep DTCG as their source; apps get only the finished values and what each token is,
and never parse DTCG.

```text
dtcg-nsh design/acme.resolver.json -o theme.json --names names.json
dtcg-nsh design/acme.resolver.json -o theme.json --check     # in CI: fails when theme.json is stale
```

The library, `dxn_dtcg_nsh`, does the same without I/O: `export(root, &load, &options)` takes the project's files
from a loader and returns the document, each path's NSH name and what it left out. It resolves the project with
[`dxn_dtcg`](https://github.com/dXnSix/dXn.Dtcg).

## What the export writes

```json
{
  "cssVars": { "theme": { "radius-card": "12px" }, "light": { "colors-accent": "#0000ff" }, "dark": { "colors-accent": "#8080ff" } },
  "tokens": {
    "colors-accent": { "kind": "color", "path": "colors.accent" },
    "radius-card": { "kind": "radius", "path": "radius.card" }
  },
  "modifiers": {
    "brand": { "default": "ocean", "contexts": { "ocean": {}, "forest": { "theme": { "colors-accent": "#008000" } } } }
  }
}
```

- **Appearance.** One modifier is NSH's appearance: its two contexts become `cssVars`' `light` and `dark`
  (`--appearance <modifier>=<light>,<dark>`; by default the modifier with contexts `light` and `dark`). A value the
  same in both goes to `theme`.
- **Other modifiers** keep their names, their contexts' names and their defaults. The default context's values
  are in `cssVars`; every other context holds only what it changes.
- **Kinds.** From `$type`, and for a dimension or a number its *use*: `$extensions["dev.nsh"].use` (`radius`,
  `spacing`, `text`, `weight`, `tracking`, `leading`, `blur`, `number`), else a Figma scope
  (`$extensions["com.figma.scopes"]`: `CORNER_RADIUS`, `GAP`, `WIDTH_HEIGHT`, `FONT_SIZE`, `LETTER_SPACING`,
  `LINE_HEIGHT`, `FONT_WEIGHT`, `EFFECT_FLOAT`), else the path's first segment (`radius.*`, `spacing.*`), else a
  number. `fontFamily` is a font, `fontWeight` a weight, `duration` a duration, `cubicBezier` an easing, `shadow`,
  `gradient` and `typography` their own, `dxn.dtcg.blur` a blur, `dxn.dtcg.string` and `dxn.dtcg.boolean` a string
  and a boolean. `border`, `transition` and `strokeStyle` have no NSH kind: left out, and said.
- **Names.** A token's NSH name is `$extensions["dev.nsh"].name` if it pins one (`primary`), else its kind's
  namespace and its path in kebab case (`radius.card` → `radius-card`, `colors.backgrounds.primary` →
  `colors-backgrounds-primary`, `motion.snappy` → `ease-motion-snappy`). Two paths of one name are refused, as is a
  name NSH refuses (a color under `font-`); `--names` writes every path with its name.
- **Values.** Colors as `#rrggbb` (`#rrggbbaa` when not opaque) in sRGB, lengths in pixels (`--rem`, 16 by
  default), durations in milliseconds, shadows, gradients and easings as CSS. A gradient's geometry comes from
  `$extensions["dev.nsh"].gradient` (`{ "kind": "linear" | "radial" | "conic", "angle": <deg> }`; linear, 180° by
  default). A typography token is `text-<n>` with the parts `--line-height`, `--letter-spacing`, `--font-weight`.
- **One structure.** Every selection has the same tokens with the same types (`dxn_dtcg`'s structure check): a
  context that adds a token, removes one or changes a type is refused, as NSH's contexts add no names and change no
  kind.
- **Exact layers.** NSH lays each modifier's picked context over the values, in order. The export resolves every
  combination of contexts (up to `max_combinations`, 4 096) and refuses a token whose value there is not what the
  layers give, naming the token and the combination: two modifiers that change a token *together* (one aliases a
  token the other changes) cannot be precomputed as layers.

## Install

```toml
[dependencies]
dxn_dtcg_nsh = { git = "https://github.com/dXnSix/dXn.DtcgNsh", tag = "vX.Y.Z" }
```

## License

MIT OR Apache-2.0.
