# The RayX theme format (draft)

The library reads a DTCG 2025.10 project (a resolver, its sets and modifiers) and writes one
**RayX theme file**: every token RayX ships, with its resolved values in every mode. Nothing in the
format is NSH's. RayX's runtime (`rayx_components`) reads it at startup and hands NSH what NSH needs.

The DTCG files stay the source. What RayX needs beyond DTCG (a token's short name, the roles it
plays, which modifier is Light/Dark) lives in their `$extensions`, under the `dev.rayx.*` keys the
export already uses (`dev.rayx.scopes`, `dev.rayx.hidden`).

## The file

```json
{
  "format": "rayx.theme",
  "name": "rayx",
  "modes": {
    "appearance": { "default": "light",   "contexts": ["light", "dark"] },
    "accent":     { "default": "cyan",    "contexts": ["cyan", "sky", "blue"] },
    "density":    { "default": "desktop", "contexts": ["desktop", "touch"] },
    "os":         { "default": "windows", "contexts": ["windows", "macos", "gnome", "kde"] }
  },
  "tokens": {
    "surfaces-app": {
      "type": "color",
      "path": "theme.colors.surfaces.app",
      "description": "Background of the whole application window; …",
      "roles": ["background"],
      "value": { "light": "#f5f5f5ff", "dark": "#0a0a0aff" }
    },
    "roles-primary-bg": {
      "type": "color",
      "path": "theme.colors.roles.primary.bg",
      "description": "Resting background of the element (button, tab, chip, alert, and similar). …",
      "roles": ["primary", "sidebar-primary"],
      "value": "#007796ff",
      "modes": {
        "accent": {
          "sky":  { "light": "#075e8eff", "dark": "#055d8dff" },
          "blue": { "light": "#0052c1ff", "dark": "#0053bbff" }
        }
      }
    },
    "spacing-size-default-lvl5-height": {
      "type": "dimension",
      "path": "platform.density.size.default.lvl5.height",
      "description": "Level 5: height of interactive controls (…), default size.",
      "value": 32,
      "modes": { "density": { "touch": 48 } }
    },
    "shadow-md": {
      "type": "shadow",
      "path": "theme.effects.shadow.md",
      "description": "The MD shadow style: 2 layers, drawn bottom first.",
      "value": [
        { "x": 0, "y": 4, "blur": 5, "spread": 2, "color": "#0a111833" },
        { "x": 0, "y": 1, "blur": 1, "spread": 1, "color": "#0a111826" }
      ]
    }
  }
}
```

### Top level

| Key | Holds |
| --- | --- |
| `format` | Always `"rayx.theme"`: tells a reader what the file is. |
| `name` | The resolver's `name`. |
| `modes` | Every mode a value can depend on: its `default` and its `contexts`, in the resolver's order. `appearance` is Light/Dark; the others keep their resolver modifier names. |
| `tokens` | Every token, by its name, in the export's order. |

### A token

| Key | Holds |
| --- | --- |
| `type` | What the value is (table below). |
| `path` | The token's DTCG path (the Figma variable's WEB code syntax). Code keys are made from it. |
| `description` | Its `$description`, HTML entities decoded and whitespace collapsed. |
| `roles` | Optional. The roles it plays: the parts the platform draws itself (default background, borders, focus ring, selection, fonts). A role is played by one token. |
| `value` | Its value in every mode's default context. |
| `modes` | Optional. For one mode, the contexts whose value differs from `value`; contexts not listed keep `value`. |

A `value` (and each context's value in `modes`) is either the value itself, when Light and Dark
agree, or `{ "light": …, "dark": … }`, keyed by the `appearance` contexts. A token depends on
`appearance` and at most one other mode; the library refuses one that changes with two.

### Types and values

| `type` | Value | From DTCG |
| --- | --- | --- |
| `color` | `"#rrggbbaa"` | `$type: color` (sRGB) |
| `dimension` | number, px | `$type: dimension` with no scope below (sizes, gaps, insets, line heights, letter spacing) |
| `fontSize` | number, px | scope `FONT_SIZE` |
| `radius` | number, px | scope `CORNER_RADIUS` |
| `strokeWidth` | number, px | scope `STROKE_FLOAT` |
| `blur` | number, px | scope `EFFECT_FLOAT` |
| `number` | number | `$type: number` |
| `fontFamily` | string (the first family) | `$type: fontFamily` |
| `fontWeight` | number | `$type: fontWeight` |
| `shadow` | array of layers `{ x, y, blur, spread, color }`, bottom first | a group of `layerN` groups, each with `color`, `offsetX`, `offsetY`, `blur`, `spread` |

Scopes are `$extensions["dev.rayx.scopes"]`, checked in the table's order (a token with both
`STROKE_FLOAT` and `EFFECT_FLOAT`, like `borders.focusRingWidth`, is a `strokeWidth`).
`$extensions["dev.rayx.type"]` overrides the derived type. Aliases are followed to the value; `rem`
is 16 px.

## What the DTCG files carry

```json
"bg": {
  "$type": "color",
  "$value": "{primitives.colors.sets.roles.cyan.bg}",
  "$description": "Resting background of the element …",
  "$extensions": {
    "dev.rayx.scopes": ["FRAME_FILL", "SHAPE_FILL"],
    "dev.rayx.name": "roles-primary-bg",
    "dev.rayx.roles": ["primary", "sidebar-primary"]
  }
}
```

| Extension | On | Means |
| --- | --- | --- |
| `dev.rayx.name` | a token, or a shadow group | The token's name in the theme. **Only tokens with a name are in the theme**, so primitives and anything else stay out without a list of collections. Names are unique. |
| `dev.rayx.roles` | a named token | The roles it plays. |
| `dev.rayx.type` | a named token | Overrides the type derived from `$type` and the scopes. |
| `dev.rayx.appearance` | a resolver modifier | `true` marks the Light/Dark modifier (`colorMode`); it becomes the `appearance` mode. |

The DTCG files are maintained by hand until Six.TokenEditor writes these extensions; a Figma export
would drop them.

## Parity with `theme_codegen.py`

A prototype wrote this format from today's export (names and roles filled in from the script's
rules), converted it back the way the RayX loader would, and got exactly today's
`rayx_components/theme.json`: 710 tokens (400 `color`, 209 `dimension`, 33 `fontSize`, 33
`fontWeight`, 16 `radius`, 7 `blur`, 6 `shadow`, 4 `strokeWidth`, 2 `fontFamily`), 21 tokens
playing 33 roles, 15 tokens changing with `accent` and 152 with `density`, 322 with Light/Dark.
About 300 KB, the same as today.

The loader's mapping to NSH (in `rayx_components`, not the library): `value`/`modes` become NSH's
`cssVars` and modifier contexts, `type` becomes NSH's kind (`dimension` → `spacing`, `fontSize` →
`text`, `fontFamily` → `font`, `fontWeight` → `weight`, `strokeWidth` → `number`, a shadow's layers
→ a CSS shadow string), `roles` become NSH's role map.
