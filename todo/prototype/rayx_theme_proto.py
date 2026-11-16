"""Prototype: the RayX theme format from today's export, and back to today's theme.json.

Simulates the `dev.rayx.name` / `dev.rayx.roles` extensions with the script's own rules, writes
the RayX format, converts it back to the NSH document the way rayx_components' loader would, and
compares that with the committed theme.json.
"""
import json
import sys
from pathlib import Path

ROOT = Path("/Users/dinko/work-private/projects/RayX")
sys.path.insert(0, str(ROOT / "RayX.Root/.claude/skills/rayx-components/scripts"))
import theme_codegen as tc  # noqa: E402

OUT = Path(sys.argv[1])
export = tc.Export(tc.locations.TOKENS)
tokens = tc.declared(export.tree())
roles_of = {}
for name, played in tc.ROLE_TOKENS.items():
    roles_of[name] = list(played)


def rayx_type(t):
    node = tc.node_at(export.tree(), t.path)
    if t.kind == "shadow":
        return "shadow"
    ty = node.get("$type")
    scopes = node.get("$extensions", {}).get("dev.rayx.scopes", [])
    if ty == "color":
        return "color"
    if ty == "fontFamily":
        return "fontFamily"
    if ty == "fontWeight":
        return "fontWeight"
    if ty == "number":
        return "number"
    if "FONT_SIZE" in scopes:
        return "fontSize"
    if "CORNER_RADIUS" in scopes:
        return "radius"
    if "STROKE_FLOAT" in scopes:
        return "strokeWidth"
    if "EFFECT_FLOAT" in scopes:
        return "blur"
    return "dimension"


def shadow_layers(tree, path):
    node = tc.node_at(tree, path)
    layers = []
    for key in tc.children(node):
        layer = node[key]
        get = lambda k: tc.resolve(tree, layer[k]["$value"])  # noqa: E731
        layers.append({
            "x": tc.number(tc.dimension(get("offsetX"), path)),
            "y": tc.number(tc.dimension(get("offsetY"), path)),
            "blur": tc.number(tc.dimension(get("blur"), path)),
            "spread": tc.number(tc.dimension(get("spread"), path)),
            "color": tc.rgba(tc.color_hex(get("color"), path)),
        })
    return layers


def value(tree, t):
    return shadow_layers(tree, t.path) if t.kind == "shadow" else tc.value(tree, t)


def pairs(**picked):
    light = export.tree(colorMode="light", **picked)
    dark = export.tree(colorMode="dark", **picked)
    return {t.name: (value(light, t), value(dark, t)) for t in tokens}


def by_appearance(pair):
    light, dark = pair
    return light if light == dark else {"light": light, "dark": dark}


defaults = {m: export.default(m) for m in tc.MODIFIERS}
base = pairs(**defaults)
changes = {t.name: {} for t in tokens}
for m in tc.MODIFIERS:
    for c in export.contexts(m):
        if c == defaults[m]:
            continue
        for n, p in pairs(**{**defaults, m: c}).items():
            if p != base[n]:
                changes[n].setdefault(m, {})[c] = by_appearance(p)

doc = {
    "format": "rayx.theme",
    "name": export.resolver["name"],
    "modes": {
        "appearance": {"default": export.default("colorMode"), "contexts": export.contexts("colorMode")},
        **{m: {"default": defaults[m], "contexts": export.contexts(m)} for m in tc.MODIFIERS},
    },
    "tokens": {},
}
for t in tokens:
    entry = {"type": rayx_type(t), "path": t.path, "description": t.doc}
    if t.name in roles_of:
        entry["roles"] = roles_of[t.name]
    entry["value"] = by_appearance(base[t.name])
    if changes[t.name]:
        if len(changes[t.name]) > 1:
            raise SystemExit(f"{t.name} varies with {list(changes[t.name])}")
        entry["modes"] = changes[t.name]
    doc["tokens"][t.name] = entry

OUT.write_text(json.dumps(doc, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")

# ---- The loader: RayX format -> NSH document (what rayx_components would do at startup) ----
KIND = {"color": "color", "fontFamily": "font", "fontWeight": "weight", "fontSize": "text",
        "radius": "radius", "blur": "blur", "strokeWidth": "number", "number": "number",
        "dimension": "spacing", "shadow": "shadow"}


def css(v, ty):
    if ty == "shadow":
        return ", ".join(f"{l['x']}px {l['y']}px {l['blur']}px {l['spread']}px {l['color']}" for l in v)
    return v


def split(v):
    return (v["light"], v["dark"]) if isinstance(v, dict) else (v, v)


def place(out, name, v, ty):
    light, dark = split(v)
    light, dark = css(light, ty), css(dark, ty)
    if light == dark:
        out.setdefault("theme", {})[name] = light
    else:
        out.setdefault("light", {})[name] = light
        out.setdefault("dark", {})[name] = dark


rayx = json.loads(OUT.read_text())
nsh = {"cssVars": {}, "tokens": {}, "modifiers": {}, "roles": {}}
for m, spec in rayx["modes"].items():
    if m != "appearance":
        nsh["modifiers"][m] = {"default": spec["default"], "contexts": {c: {} for c in spec["contexts"]}}
for name, tok in rayx["tokens"].items():
    nsh["tokens"][name] = {"kind": KIND[tok["type"]], "path": tok["path"], "doc": tok["description"]}
    place(nsh["cssVars"], name, tok["value"], tok["type"])
    for m, ctxs in tok.get("modes", {}).items():
        for c, v in ctxs.items():
            place(nsh["modifiers"][m]["contexts"][c], name, v, tok["type"])
    for r in tok.get("roles", []):
        nsh["roles"][r] = name
# The script lists roles in ROLE_TOKENS order; compare as data.
current = json.loads((ROOT / "RayX/crates/rayx_components/theme.json").read_text())
print("round trip equal:", nsh == current)
if nsh != current:
    for k in current:
        print(k, nsh[k] == current[k])
from collections import Counter
print(Counter(t["type"] for t in rayx["tokens"].values()))
print("varying:", Counter(next(iter(t["modes"])) for t in rayx["tokens"].values() if "modes" in t))
print("by appearance:", sum(isinstance(t["value"], dict) and "light" in t["value"] for t in rayx["tokens"].values()))
print("bytes:", OUT.stat().st_size)
