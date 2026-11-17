# Repurpose RayX.Dtcg (formerly dXn.DtcgNsh) as RayX's token exporter

Status: **planned, not started** (written 2026-11-16). Nothing below is implemented except the
format draft (`docs/rayx-theme-format.md`), the prototype (`todo/prototype/rayx_theme_proto.py`)
and the "do not export" guards in RayX.Root (Phase 0, done).

Read this whole file before starting. It is the context of a conversation with the owner, written
so that an agent with no other history can do the work and knows *why* each piece is the way it is.

---

## 1. The story

### What exists

There are two projects that turn the RayX design system's DTCG tokens into a theme file:

1. **`theme_codegen.py`** in RayX.Root (`.claude/skills/rayx-components/scripts/theme_codegen.py`,
   ~450 lines of stdlib Python, tests in `test_theme_document.py`). It reads the DTCG export in
   `RayX.Design/tokens/dtcg/` (`rayx.resolver.json` and its token documents) and writes
   `RayX/crates/rayx_components/theme.json`. **This is what RayX uses today.** It is written for
   RayX: it knows RayX's collections, shortens names by RayX's rule and carries RayX's table of roles.
2. **This library, RayX.Dtcg**, formerly dXn.DtcgNsh (`crates/rayx_dtcg`, library `export(...)`, CLI
   `rayx-dtcg`). It
   was built in the NSH fork's "theme collections" project
   (`dXn/nsh/nsh-root/_projects/theme-collections/`) as a **generic** DTCG → NSH theme document
   exporter for *any* project: it guesses kinds from `dev.nsh.use` / `com.figma.scopes` / path
   segments, names tokens by NSH's namespaces over the full path, takes the appearance from a flag,
   and knows nothing of RayX. **RayX does not use it**: no RayX crate depends on it, and nothing in
   RayX.Root or RayX.Design calls it.

So RayX ended up with two implementations of one job, and the library, the better-engineered of
the two (Rust, `dxn_dtcg` resolution, an exact check of every mode combination, tests, releases),
was aimed at the wrong target.

### What the owner decided

> "This lib should actually be RayX.Dtcg and have the same functionality the Python script has
> now; instead we made it for NSH, which we do not need."

1. **Repurpose this library for RayX only.** It becomes RayX's token exporter, with **100% parity**
   with `theme_codegen.py` (same tokens, values, modes, roles, descriptions), and then replaces the
   script.
2. **No NSH in the library at all.** It writes a **RayX theme format** (designed in
   `docs/rayx-theme-format.md`), not NSH's theme document. Remove every NSH concept: NSH kinds and
   namespaces, `dev.nsh.*`, `--appearance`, `names.json`, NSH's `cssVars` / `modifiers` layout.
   RayX's own runtime (`rayx_components`) is what turns the RayX format into what NSH needs, since
   RayX is an NSH app framework and its components render through NSH (option "B" of the
   discussion; option "A", keeping NSH's document shape and calling it RayX's, was rejected).
3. **RayX's needs go into DTCG `$extensions`**, not into code. The DTCG files say, per token, its
   short name (`dev.rayx.name`) and the roles it plays (`dev.rayx.roles`). This follows DTCG and
   uses its extension mechanism, so the library has no RayX tables or naming rules in code: the
   data drives it.
4. **The DTCG files are maintained by hand for now.** The Figma export (RayX.FigmaTools via the
   `rayx-figma` skill) does not write these extensions and would wipe them, so it is **paused**.
   Later, **Six.TokenEditor** will keep the extensions per variable in its own format, and a **Figma
   connector** will pull variables from Figma and let the token editor sync them with its own
   variable tree. Until then the hand-maintained files in RayX.Design are the test and development
   source.
5. **Refactor this repository in place, now.** The owner has already transferred it to the RayX
   GitHub organization and renamed it **RayX.Dtcg** (`github.com/rayx-ui/RayX.Dtcg`); on 2026-11-17
   the crate became `rayx_dtcg` and the binary `rayx-dtcg` (the rename planned as Phase 6). The old
   names (`dXn.DtcgNsh`, `dxn_dtcg_nsh`, `dtcg-nsh`) survive only in `v0.1.0` and its changelog.

### Why `$extensions`

- The script hides RayX's rules in Python (a list of collections, a name-shortening rule, a
  hard-coded `ROLE_TOKENS` table). Anyone reading the DTCG files cannot see a token's code name or
  roles, and a tool other than the script cannot reproduce them.
- With `dev.rayx.name` and `dev.rayx.roles` on each token, the DTCG project fully describes the
  theme. The exporter becomes small and rule-free; the token editor can later show and edit names
  and roles per variable.
- The export already uses the `dev.rayx.*` namespace (`dev.rayx.scopes` on ~2,850 tokens,
  `dev.rayx.hidden`, `dev.rayx.paragraphSpacing`), so the new keys follow an existing convention:
  flat keys `"dev.rayx.name"`, `"dev.rayx.roles"` inside `$extensions`.

---

## 2. Where things are

| What | Path (from `~/work-private/projects`) | Repository / rules |
| --- | --- | --- |
| This library | `RayX/RayX.Dtcg/` | git, `github.com/rayx-ui/RayX.Dtcg`, branch `main`; `AGENTS.md` here (Conventional Commits, release-plz, test groups) |
| DTCG resolution library it uses | `dXn/dXn.Dtcg/` (`dxn_dtcg`, tag `v1.4.0`) | its own repo; a change needed there is released there first |
| The DTCG token files | `RayX/RayX.Design/tokens/dtcg/` (1.7 MB) | git, local; `RayX/RayX.Design/AGENTS.md` |
| Today's generator (to replace) | `RayX/RayX.Root/.claude/skills/rayx-components/scripts/theme_codegen.py` (+ `test_theme_document.py`) | git, local; `RayX/RayX.Root/AGENTS.md` (sessions start in RayX.Root) |
| Today's theme file (NSH format) | `RayX/RayX/crates/rayx_components/theme.json` (710 tokens, ~300 KB) | git, `github.com/rayx-ui/RayX`; `RayX/RayX/AGENTS.md` |
| Its consumers in RayX | `crates/rayx_components/src/theme/mod.rs` (`theme_keys!("theme.json")`, `DOCUMENT`), `src/lib.rs:61` (`.theme_document(theme::DOCUMENT)`), `tests/support/mod.rs` (`Document`, the tests' oracle), `tests/theme_*.rs` | RayX |
| NSH's `theme_keys!` macro (read for Phase 3) | `dXn/nsh/nsh/crates/nsh-macros/src/theme_keys.rs` (+ docs at `lib.rs` ~1886) | NSH fork, read only |
| The format draft | `RayX/RayX.Dtcg/docs/rayx-theme-format.md` | this repo |
| The prototype | `RayX/RayX.Dtcg/todo/prototype/rayx_theme_proto.py` | this repo |
| NSH project notes that cite this library | `dXn/nsh/nsh-root/_projects/theme-collections/` (`README.md`, `theme-document.md`) | NSH root; update in Phase 5 |

Workspace rules that apply (from `RayX/RayX.Root/AGENTS.md` and `RayX/RayX/AGENTS.md`):
dependencies from git tags only (never paths, branches or commits); Conventional Commits; **no AI
attribution anywhere**; commit only when the owner asks; **pushing and tagging need the owner's
go-ahead**; one commit per repository a task touches; test only what changed (feature test groups,
never the whole suite for scoped work); raw diagnostic output in an ignored `artifacts-temp/`.

---

## 3. The RayX theme format

Full spec: **`docs/rayx-theme-format.md`** (draft, to be finalized in Phase 2). In short:

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
    "roles-primary-bg": {
      "type": "color",
      "path": "theme.colors.roles.primary.bg",
      "description": "Resting background of the element …",
      "roles": ["primary", "sidebar-primary"],
      "value": "#007796ff",
      "modes": { "accent": {
        "sky":  { "light": "#075e8eff", "dark": "#055d8dff" },
        "blue": { "light": "#0052c1ff", "dark": "#0053bbff" } } }
    },
    "spacing-size-default-lvl5-height": {
      "type": "dimension", "path": "platform.density.size.default.lvl5.height",
      "description": "…", "value": 32, "modes": { "density": { "touch": 48 } }
    },
    "shadow-md": {
      "type": "shadow", "path": "theme.effects.shadow.md",
      "description": "The MD shadow style: 2 layers, drawn bottom first.",
      "value": [ { "x": 0, "y": 4, "blur": 5, "spread": 2, "color": "#0a111833" },
                 { "x": 0, "y": 1, "blur": 1, "spread": 1, "color": "#0a111826" } ]
    }
  }
}
```

- **One entry per token**, everything about it together. `value` is the value in every mode's
  default context; `modes` lists, for at most one mode, only the contexts that change it.
- **Light/Dark** is the `appearance` mode: a value is written once when Light and Dark agree, else
  `{ "light": …, "dark": … }`.
- **Types** are RayX's: `color`, `dimension`, `fontSize`, `radius`, `strokeWidth`, `blur`, `number`,
  `fontFamily`, `fontWeight`, `shadow` (no NSH kinds).
- **Shadows** are structured layers, not CSS strings.
- **Rules the library enforces**: names unique; a role played by one token; a token depends on
  `appearance` and at most one other mode (the existing "exact layers" check across every
  combination proves it); colors sRGB; dimensions px (`rem` × 16).

### The extensions it reads

| Extension | On | Meaning |
| --- | --- | --- |
| `dev.rayx.name` | a token, or a shadow group | The token's name in the theme. **Only named tokens are in the theme** (replaces the script's list of collections; primitives and styles stay out because they have no name). |
| `dev.rayx.roles` | a named token | Roles it plays: the parts the platform draws itself (default background, borders, focus ring, selection, radius, fonts). |
| `dev.rayx.type` | a named token | Optional override of the derived type. |
| `dev.rayx.appearance` | a resolver modifier | `true` marks the Light/Dark modifier (`colorMode`); it becomes `appearance`. |

Type derivation (from `$type`, then `$extensions["dev.rayx.scopes"]`, in this order): `color` ←
`$type: color`; `fontFamily`, `fontWeight` ← their `$type`; `number` ← `$type: number`;
`fontSize` ← scope `FONT_SIZE`; `radius` ← `CORNER_RADIUS`; `strokeWidth` ← `STROKE_FLOAT`;
`blur` ← `EFFECT_FLOAT`; any other dimension → `dimension`; a group of `layerN` groups (each with
`color`, `offsetX`, `offsetY`, `blur`, `spread`) → `shadow`. **The order matters**:
`platform.density.borders.focusRingWidth` has both `STROKE_FLOAT` and `EFFECT_FLOAT` and must be
`strokeWidth` (today's `number`). The prototype found this the hard way.

### Proven parity

`todo/prototype/rayx_theme_proto.py` (imports `theme_codegen.py`, simulates the extensions with the
script's own rules) writes the RayX format from today's export and converts it back the way RayX's
loader would. The result **equals today's `rayx_components/theme.json`** (compared as JSON data):
710 tokens (400 `color`, 209 `dimension`, 33 `fontSize`, 33 `fontWeight`, 16 `radius`, 7 `blur`,
6 `shadow`, 4 `strokeWidth`, 2 `fontFamily`), 33 roles on 21 tokens, 15 tokens changing with
`accent`, 152 with `density`, none with `os`, 322 differing between Light and Dark. Run it with:

```bash
python3 todo/prototype/rayx_theme_proto.py /path/to/out/rayx.theme.json
```

The prototype's file is the **expected output** of the refactored library on the annotated export
(Phase 1), and its inverse is the **reference for RayX's loader** (Phase 3).

---

## 4. Exactly what the script does (the parity contract)

Port these behaviors; `theme_codegen.py` is the reference when in doubt.

- **Resolution**: resolver `resolutionOrder`, sets then the picked context of each modifier,
  deep-merged; aliases `{a.b.c}` followed to a literal (cycle → error). The library already does this
  through `dxn_dtcg`.
- **Tokens included**: today the collections `theme.colors`, `theme.effects`, `platform.density`,
  `platform.os`, in that order, each walked in document order. After Phase 1: every token (or shadow
  group) carrying `dev.rayx.name`, in the order of the merged structure. Check that order equals
  today's; if it does not, order by the resolution order of the defining documents and say so.
- **Names**: today computed (path after the collection, kebab case, `bgHover` → `bg-hover`, a group
  key starting with a digit takes `s`, a token key keeps its digits, a group named as its tokens'
  namespace adds no prefix, the kind's namespace in front: `spacing-`, `radius-`, `text-`,
  `font-weight-`, `blur-`, `font-`, `shadow-`, none for colors and numbers). After Phase 1:
  `dev.rayx.name`, verbatim. The library has **no** naming rule.
- **Descriptions**: `$description`, HTML-unescaped three times, whitespace collapsed to single
  spaces. A shadow's description is synthesized: `The {NAME} shadow style: {n} layer(s), drawn
  bottom first.`, `{NAME}` from the first layer token's description (`of the (\w[\w ]*?) shadow
  style`), `this` if none matches; `layer` singular for one.
- **Values**:
  - color: sRGB only (else error); `hex` if present, else `components` × 255 rounded; alpha
    `round(alpha × 255)`; written lowercase `#rrggbbaa`.
  - dimension-like: px; `rem` × 16; a bare number is px; written as an integer when whole
    (`32`, not `32.0`).
  - fontFamily: the first family when a list. fontWeight: number.
  - shadow: each layer's `offsetX`, `offsetY`, `blur`, `spread` (px) and `color`, in key order.
- **Modes**: `colorMode` is the appearance; `accent`, `density`, `os` the others, with the
  resolver's defaults. For each other mode's non-default context, with every other mode at its
  default, a token whose (light, dark) pair differs from the default is listed for that context.
  The script refuses a context that changes a token of another mode's collection; the library's
  existing exact-layers check across all combinations is stricter and replaces that.
- **Roles**: one role, one token; a role on a token not in the theme is an error.
- **Output**: pretty JSON, 2-space indent, non-ASCII kept, `\n` at the end (`json.dumps(…,
  indent=2, ensure_ascii=False) + "\n"`); keys in the order of the spec.
- **Commands**: `write` and `check` (exit 1 when the file is not what the export gives, for CI).
  The CLI's `-o` and `--check` already do this.

---

## 5. The plan

### Phase 0: guard the hand-maintained DTCG files (DONE, uncommitted)

- `RayX/RayX.Root/AGENTS.md`: the `tokens/dtcg/` row now says the files are maintained by hand,
  carry `dev.rayx.*` extensions and must never be re-exported from Figma; the Common tasks row
  "Export the tokens" is **Paused**; the Tokens paragraph says "after every change to the DTCG
  files" instead of "after every token export".
- `RayX/RayX.Root/.claude/skills/rayx-figma/references/export-tokens.md`: a "Paused. Do not run
  this export." note at the top.
- Commit in RayX.Root when the owner asks (`docs: pause the Figma token export while the DTCG files
  carry RayX extensions`).

### Phase 1: annotate the DTCG files (RayX.Design)

Goal: the DTCG project carries everything RayX needs, so the library can be rule-free.

1. Write a **one-off annotation script** (scratch, not committed; or in `RayX.Root/artifacts-temp/`)
   that reuses `theme_codegen.py`'s `declared()` and `ROLE_TOKENS`:
   - `dev.rayx.name` = today's name on every included token, **in every document that defines the
     token** (a theme color is defined in `theme.colors.{cyan,sky,blue}.tokens.json`, a density
     token in both `platform.density.*` files): the extension must be the same in every context.
   - `dev.rayx.name` on each shadow **group** (`theme.effects.shadow.{sm,default,md,lg,xl,xxl}`)
     in the group's own `$extensions`. `dxn_dtcg`'s `ResolvedGroup.extensions` exposes merged group
     extensions, so the library can read it.
   - `dev.rayx.roles` from `ROLE_TOKENS` (21 tokens, 33 roles).
   - `"$extensions": { "dev.rayx.appearance": true }` on the resolver's `colorMode` modifier.
   - **Change `$type: "dev.rayx.blur"` to `"dxn.dtcg.blur"`** on the 16 blur style tokens in
     `theme.styles.tokens.json` (`theme.styles.bgBlur.*`, `theme.styles.blur.*`; same value shape
     `{ kind, radius }`). Reason: `dxn_dtcg` refuses unknown types ("unsupported DTCG type
     `dev.rayx.blur` for final 2025.10"), which today makes the library fail on RayX's export before
     it writes anything (verified 2026-11-16). `dxn_dtcg` supports `dxn.dtcg.blur` behind
     `ValueExtensions::blur`, which the library already enables. These tokens are not in the theme;
     the fix only lets resolution pass. (`rayx-figma`'s `figma_export.py` writes `dev.rayx.blur`; the
     export is paused, and the token editor will write the new type.)
   - Keep the files' formatting (2-space JSON, key order, `\n` at the end) so the diff shows only
     the added extensions and the 16 type changes.
2. Verify: `python3 .claude/skills/rayx-components/scripts/theme_codegen.py check` (from RayX.Root)
   still passes; the script ignores the new extensions and the styles, so the theme is unchanged.
3. Commit in RayX.Design when asked (`feat(tokens): name the theme's tokens and their roles in
   dev.rayx extensions`).

### Phase 2: refactor the library (this repo)

Goal: `export(...)` and `rayx-dtcg` write the RayX theme format from an annotated DTCG project, with
parity. The repository, crate and binary already carry their RayX names (see §1, decision 5).

Read first: `AGENTS.md`, `src/lib.rs` (export, `Project`, `Values`, the exact-layers check),
`src/kinds.rs`, `src/names.rs`, `src/values.rs`, `src/main.rs`, `tests/*`.

**Remove** (NSH and generic guessing):
- `kinds.rs`'s NSH `Kind` (`Spacing`, `Text`, `Weight`, `Tracking`, `Leading`, `Gradient`,
  `Duration`, `Easing`, `Typography`, `String`, `Boolean` …) and `kind_of`'s `dev.nsh.use`,
  `com.figma.scopes` and path-segment fallbacks.
- `names.rs`'s NSH namespaces, `name_of`, `refused` and `dev.nsh.name`.
- `Appearance` / `Options.appearance` and the CLI's `--appearance`; `Export.names` and `--names`;
  `left_out` (an unnamed token is simply not in the theme; a named token of a type with no RayX type
  is an error).
- NSH's output layout (`cssVars` with `theme`/`light`/`dark`, `modifiers.*.contexts` overlays,
  `tokens.{kind,path,doc}`), and every mention of NSH in docs, crate description and keywords
  (`Cargo.toml`: description, `keywords = [..., "nsh", ...]`).

**Keep**: the loader-based API (`export(root, load, options)`, no I/O in the library), `dxn_dtcg`
resolution and its structure check across selections, `max_combinations`, `rem`, the exact-layers
check (now proving "appearance + at most one other mode per token"), `Export::text()`, the CLI's
`-o` and `--check`, the error type collecting every reason.

**Add**:
- `rayx` output model (a `types.rs` with the RayX `Type` enum and derivation of §3; a `theme.rs`
  writing the document in the spec's key order).
- Inclusion by `dev.rayx.name`; shadow groups as tokens (name on the group, layers from its
  `layerN` children); roles from `dev.rayx.roles`; appearance from the resolver modifier marked
  `dev.rayx.appearance` (if `dxn_dtcg` does not expose modifier `$extensions`, read them from the
  resolver document the loader returns; if no modifier is marked, there is no `appearance` mode and
  every value is plain).
- Descriptions and value formatting exactly as §4.
- Errors: duplicate name; role played twice; role on an unnamed token; a token varying with two
  non-appearance modes; a named token whose type has no RayX type; a non-sRGB color; a unit other
  than px/rem.

**Tests** (each feature its own group in `crates/rayx_dtcg/tests/<feature>.rs`, run with
`cargo test --test <feature>`; replace the NSH groups `document.rs`, adapt `refusals.rs` and
`command.rs`): `theme_format` (top level and modes), `inclusion` (named only, order), `types`
(derivation incl. the scope order), `shadows`, `roles`, `appearance`, `values` (colors, px/rem,
integers), `descriptions`, `refusals`, `command` (`-o`, `--check`). Small synthetic DTCG fixtures
under `tests/common`. Plus a **`rayx_parity`** group that runs only when `RAYX_DTCG` points at
`RayX.Design/tokens/dtcg` and `RAYX_THEME_EXPECTED` at the prototype's output, and compares the
library's file with it byte for byte (skips, with a message, when unset; never part of CI).

**Docs**: `README.md` (what it is now: DTCG → RayX theme), `AGENTS.md` (first paragraph: RayX's
exporter, no NSH), crate docs in `lib.rs` and `main.rs`, `docs/rayx-theme-format.md` (drop
"(draft)", make it the reference). `CHANGELOG.md` is release-plz's, never edited by hand.

**Validation** (this repo's `AGENTS.md`): the touched test groups, then `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` before committing.

**Commit** when asked: one breaking commit, e.g. `feat!: export the RayX theme format in place of
NSH theme documents`, body stating the break, `Migration:` footer (the output format and the CLI
flags changed; DTCG projects need `dev.rayx.name`). release-plz then proposes `v0.2.0`; pushing and
merging its release PR need the owner's go-ahead.

### Phase 3: read the RayX format in RayX (`RayX/`)

Goal: `rayx_components` ships the RayX theme file and hands NSH what NSH needs, with the same keys,
modifiers and values as today.

Today `theme/mod.rs` uses NSH's `theme_keys!("theme.json")` (reads an NSH document at compile time,
relative to `CARGO_MANIFEST_DIR`, emits one `TokenKey::new(name)` per token in modules by path, and
modifier enums `Accent`/`Density`/`Os` with `NAME`, `ALL`, `DEFAULT`, `name()`, `set()`,
`current()`), and `DOCUMENT = include_str!("../../theme.json")` is registered as an NSH theme
document. Neither can read the RayX format.

Recommended design (confirm with the owner before building): **a RayX proc macro**, e.g.
`rayx::theme!("theme.json")` in `rayx_app_macros`, that reads the RayX file at compile time and
emits (1) the same typed keys (`nsh_core::element::TokenKey::new` is a public `const fn`; key value
types per type as NSH's macro does: color `AdaptiveColor`, dimension-like `Points`, weight/number
`f32`, font `SharedString`, shadow `AdaptiveShadow`), (2) the same modifier enums (read
`nsh-macros/src/theme_keys.rs` and generate the same API), and (3) `DOCUMENT: &str`, the NSH theme
document converted from the RayX file at compile time (the prototype's loader, ported: `value` /
`modes` → `cssVars` and modifier contexts, `type` → NSH kind: `dimension` → `spacing`,
`fontSize` → `text`, `fontFamily` → `font`, `fontWeight` → `weight`, `strokeWidth`/`number` →
`number`, `radius`, `blur`, `color`, `shadow` layers → `"{x}px {y}px {blur}px {spread}px {color}"`
joined by `", "`; `roles` → NSH's role map). No NSH file is committed and nothing converts at
startup. Alternative if a macro is unwanted: a `build.rs` writing the NSH document to `OUT_DIR`
(then keys need a macro that accepts that path).

Steps: replace `crates/rayx_components/theme.json` with the library's output (file name stays
`theme.json`; its `format` field says what it is); switch `theme/mod.rs` to the new macro; port
`tests/support/mod.rs`'s `Document` oracle to the RayX format; adapt `tests/theme_document.rs`.
**Parity check during the switch**: a test that compares the macro's `DOCUMENT` (parsed) with the
old NSH `theme.json` kept as a temporary fixture; the existing `theme_keys_view`,
`theme_modifiers_*` and `button_*` tests must pass unchanged. Update the doc comments in
`theme/mod.rs`, `lib.rs`, `docs/architecture.md`, `docs/components.md`, `AGENTS.md`. Focused tests
per `RayX/AGENTS.md`; run the gallery once. Commit in RayX when asked (`feat(components)!: read
RayX's theme format`). RayX does **not** depend on this library: the file is generated outside the
build and committed.

### Phase 4: switch the workflow (RayX.Root)

- The `rayx-components` skill runs the library's CLI instead of `theme_codegen.py`:
  `cargo install --git https://github.com/rayx-ui/RayX.Dtcg --tag v0.2.0 --locked rayx_dtcg`
  (during development `cargo run --manifest-path ../RayX.Dtcg/Cargo.toml --bin rayx-dtcg
  --`), then from RayX.Root:
  `rayx-dtcg ../RayX.Design/tokens/dtcg/rayx.resolver.json -o ../RayX/crates/rayx_components/theme.json`
  (`--check` to verify).
- Delete `theme_codegen.py` and `test_theme_document.py` once Phases 2–3 match; keep the prototype
  only in this repo's `todo/`.
- Update `rayx-components`' `SKILL.md`, `references/theming.md`, `references/building-components.md`;
  `RayX.Root/AGENTS.md` (the `RayX/` section's theme sentence, the "Tokens" paragraph under "How
  design and code talk", Common tasks "Use RayX's tokens" / export rows); record the decision in
  `rayx-figma/references/decisions.md` (2026-11-16: RayX theme format, names and roles in
  `dev.rayx.*` extensions, DTCG maintained by hand, export paused).
- Commit in RayX.Root when asked.

### Phase 5: tidy references

- `dXn/nsh/nsh-root/_projects/theme-collections/` (`README.md` §§ around "Steps 5 and 6",
  `theme-document.md`): note that dXn.DtcgNsh became RayX.Dtcg, RayX's exporter, and no longer
  writes NSH theme documents. Ask the owner before editing the NSH root.

### Phase 6: later (not part of this work)

- Done early: the repository is `rayx-ui/RayX.Dtcg`, the crate `rayx_dtcg`, the binary `rayx-dtcg`
  (2026-11-17). A `rayx theme` subcommand of `rayx_cli` may still wrap it later.
- Six.TokenEditor keeps `dev.rayx.*` per variable; its Figma connector pulls variables from Figma;
  then lift the export pause in RayX.Root.

---

## 6. Open questions for the owner

1. **Names keep their type prefixes?** Today's names start with a prefix by kind (`spacing-gap1`,
   `radius-…`, `font-weight-…`, `shadow-md`, `blur-md`). The plan keeps them verbatim in
   `dev.rayx.name` (exact parity; RayX's loader passes names to NSH unchanged, and NSH requires
   those prefixes, so the loader should report a name that lacks its type's prefix). Dropping them
   gives shorter names (`gap1`, `md`), but `shadow-md` and `blur-md` would both be `md`, so names
   would be unique only per type and the loader would add the prefixes. Asked on 2026-11-16, not yet
   answered: **default to keeping them.**
2. **Phase 3 mechanism**: RayX proc macro (recommended) or `build.rs`.
3. **A version in `format`?** The draft has `"format": "rayx.theme"` with no version (the design
   system is never versioned; a format version may still help readers). Default: none.
