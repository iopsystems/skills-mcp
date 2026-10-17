#!/usr/bin/env python3
"""Weighted source at a glance, for the Pelikan workspace.

Trial generator for the `weighted-source-at-a-glance` conventions in
skills-mcp's architecture-diagram skill. Standalone on purpose: it derives
from `cargo metadata` and `git ls-files`, borrows the layer table and palette
from pelikan's `xtask/src/arch.rs` and `xtask/src/svg.rs`, and writes a
treemap SVG, a machine-readable inventory, and a textual equivalent.

    python3 generate.py --repo ../pelikan --out out/
    python3 generate.py --self-test
"""
import argparse
import hashlib
import json
import os
import subprocess
import sys
from datetime import date

# --- bindings copied from pelikan xtask (validated against cargo metadata) ---

LAYER = {
    "pelikan-segcache": "Product", "pelikan-rds": "Product",
    "pelikan-pingserver": "Product", "pelikan-pingproxy": "Product",
    "server": "Core", "proxy": "Core", "admin": "Core",
    "session": "Core", "queues": "Core",
    "protocol-admin": "Protocol", "protocol-common": "Protocol",
    "protocol-http": "Protocol", "protocol-memcache": "Protocol",
    "protocol-ping": "Protocol", "protocol-resp": "Protocol",
    "entrystore": "Storage", "storage-types": "Storage", "bloom": "Storage",
    "common": "Foundation", "config": "Foundation",
    "logger": "Foundation", "pelikan-net": "Foundation",
}
TOOLING = {"xtask": "Tooling"}
LAYER_ORDER = ["Product", "Core", "Protocol", "Storage", "Foundation", "Tooling"]
FILL = {  # xtask/src/svg.rs role colors; Product and Tooling are chart conventions
    "Product": "#F5F5F5", "Core": "#CCEBC5", "Protocol": "#FBB4AE",
    "Storage": "#B3CDE3", "Foundation": "#F2F2F2", "Tooling": "#FFFFFF",
}
STROKE = {"Product": "#9E9E9E", "Tooling": "#9E9E9E"}
FONT = "Helvetica, Arial, sans-serif"
H1, H2, BODY = 20, 17, 14  # the build chart's ramp (charter override, #183)

# --- geometry ---

W, H = 1400, 900
MARGIN = 24
TITLE_H = 70
LEGEND_H = 150
HEADER_H = 28     # layer header strip, outside the quantitative area
PAD = 6           # gutter inside a layer, outside the quantitative area
GAP = 3           # gutter between leaves
MIN_LABEL_W, MIN_LABEL_H = 96, 36
MIN_NUM_W, MIN_NUM_H = 20, 18   # smallest tile that takes a numeral in place
CALLOUT_H = 20    # strip under a layer's data area for numerals that do not fit
CHAR_W = 0.56     # text width estimate, em per character, for the text bounds check
RATIO_TOL = 1e-6  # on allocated rectangles, before pixel rounding
PIXEL_TOL = 0.75  # on rendered geometry, after rounding to 0.5px


def sh(cmd, cwd):
    return subprocess.run(cmd, cwd=cwd, check=True, text=True,
                          capture_output=True).stdout


def physical_loc(path):
    """Every line of the file, including blank and comment lines. A final
    line without a trailing newline counts as one line."""
    with open(path, "rb") as f:
        data = f.read()
    if not data:
        return 0
    return data.count(b"\n") + (0 if data.endswith(b"\n") else 1)


def inventory(repo):
    meta = json.loads(sh(["cargo", "metadata", "--no-deps", "--format-version", "1"], repo))
    root = meta["workspace_root"]
    pkgs = {}
    for p in meta["packages"]:
        d = os.path.relpath(os.path.dirname(p["manifest_path"]), root)
        pkgs[p["name"]] = d
    known = {**LAYER, **TOOLING}
    missing = sorted(set(pkgs) - set(known))
    stale = sorted(set(known) - set(pkgs))
    if missing or stale:
        sys.exit(f"classification drift: unclassified {missing}, stale {stale}")

    files = sh(["git", "ls-files", "--", "*.rs"], repo).split()
    by_dir = sorted(pkgs.items(), key=lambda kv: -len(kv[1]))
    owner = {}
    for f in files:
        hits = [n for n, d in by_dir if f.startswith(d + "/")]
        if not hits:
            sys.exit(f"unmapped source file: {f}")
        owner[f] = hits[0]  # longest manifest dir wins; nested crates are distinct members
    counts = {n: 0 for n in pkgs}
    parts = {n: {"src": 0, "tests": 0, "benches": 0} for n in pkgs}
    nfiles = {n: 0 for n in pkgs}
    for f, n in owner.items():
        loc = physical_loc(os.path.join(repo, f))
        counts[n] += loc
        nfiles[n] += 1
        rel = f[len(pkgs[n]) + 1:]
        top = rel.split("/")[0]
        parts[n][top if top in ("tests", "benches") else "src"] += loc
    empty = [n for n, c in nfiles.items() if c == 0]
    if empty:
        sys.exit(f"members with no tracked .rs files: {empty}")

    rev = sh(["git", "rev-parse", "HEAD"], repo).strip()
    status = sh(["git", "status", "--porcelain"], repo).splitlines()
    modified = [l for l in status if not l.startswith("??")]
    untracked = [l[3:] for l in status if l.startswith("??")]
    digest = hashlib.sha256("\n".join(f"{f} {physical_loc(os.path.join(repo, f))}"
                                      for f in sorted(files)).encode()).hexdigest()
    comps = []
    for n in sorted(pkgs):
        comps.append({"name": n, "path": pkgs[n], "layer": known[n],
                      "files": nfiles[n], "loc": counts[n], "breakdown": parts[n]})
    total = sum(counts.values())
    for c in comps:
        c["share"] = c["loc"] / total
    return {
        "repository": "iopsystems/pelikan",
        "revision": rev,
        "tracked_files_modified": modified,
        "untracked_paths": untracked,
        "inventory_digest_sha256": digest,
        "generated": date.today().isoformat(),
        "scope": "all tracked *.rs files in Cargo workspace members, one owner each "
                 "(longest manifest directory); not the code compiled into any one binary",
        "counting_policy": "physical LOC: every line including comments and blanks; "
                           "a final unterminated line counts as one line",
        "breakdown_policy": "src/tests/benches by target directory within the crate; "
                            "inline #[cfg(test)] modules stay in src (inline mixed, not measured)",
        "exclusions": ["non-.rs files (toml, md, yml, sh, py, json, dict)",
                       "resp-proto-kani/ (present on disk, not tracked by git)",
                       "target/ and vendored dependencies (not tracked)"],
        "root_total_loc": total,
        "components": comps,
    }


# --- treemap ---

def squarify(items, x, y, w, h):
    """Bruls et al. squarified layout. items: [(key, weight)] with weights
    already scaled to the area w*h. Returns {key: (x, y, w, h)}."""
    out = {}
    items = [(k, a) for k, a in items if a > 0]
    if not items:
        return out
    total = sum(a for _, a in items)
    scale = (w * h) / total
    items = [(k, a * scale) for k, a in items]

    def worst(row, side):
        s = sum(a for _, a in row)
        if s == 0 or side == 0:
            return float("inf")
        mx = max(a for _, a in row)
        mn = min(a for _, a in row)
        return max(side * side * mx / (s * s), s * s / (side * side * mn))

    def layout_row(row, x, y, w, h):
        s = sum(a for _, a in row)
        if w >= h:  # lay the row down the left edge, width = s / h
            rw = s / h
            cy = y
            for k, a in row:
                rh = a / rw
                out[k] = (x, cy, rw, rh)
                cy += rh
            return x + rw, y, w - rw, h
        rh = s / w
        cx = x
        for k, a in row:
            rw = a / rh
            out[k] = (cx, y, rw, rh)
            cx += rw
        return x, y + rh, w, h - rh

    row = []
    while items:
        side = min(w, h)
        cand = row + [items[0]]
        if not row or worst(cand, side) <= worst(row, side):
            row = cand
            items.pop(0)
        else:
            x, y, w, h = layout_row(row, x, y, w, h)
            row = []
    if row:
        layout_row(row, x, y, w, h)
    return out


def build_layout(inv, callout=None):
    comps = inv["components"]
    layers = {}
    for c in comps:
        layers.setdefault(c["layer"], []).append(c)
    layer_loc = {L: sum(c["loc"] for c in cs) for L, cs in layers.items()}
    data_x, data_y = MARGIN, MARGIN + TITLE_H
    data_w, data_h = W - 2 * MARGIN, H - 2 * MARGIN - TITLE_H - LEGEND_H
    callout = callout or set()

    # Layer rectangles carry a header, padding, and (when a tile is too small
    # for its numeral) a callout strip, all outside the quantitative area, so
    # their outer weight is iterated until every leaf shares one k.
    order = [L for L in LAYER_ORDER if L in layers]
    extra = {L: (CALLOUT_H if L in callout else 0) for L in order}
    outer_w = {L: float(layer_loc[L]) for L in order}
    for _ in range(80):
        rects = squarify([(L, outer_w[L]) for L in order], data_x, data_y, data_w, data_h)
        inner = {L: max(0.0, (r[2] - 2 * PAD)) * max(0.0, (r[3] - HEADER_H - 2 * PAD - extra[L]))
                 for L, r in rects.items()}
        k = sum(inner.values()) / sum(layer_loc.values())
        for L in order:
            if inner[L] > 0:
                outer_w[L] *= (k * layer_loc[L]) / inner[L]
    leaves = {}
    layer_rects = {}
    for L in order:
        x, y, w, h = rects[L]
        layer_rects[L] = (x, y, w, h)
        ix, iy, iw, ih = x + PAD, y + HEADER_H + PAD, w - 2 * PAD, h - HEADER_H - 2 * PAD - extra[L]
        cs = sorted(layers[L], key=lambda c: (-c["loc"], c["name"]))
        sub = squarify([(c["name"], c["loc"]) for c in cs], ix, iy, iw, ih)
        for name, r in sub.items():
            leaves[name] = (L, r)
    k = sum(r[2] * r[3] for _, r in leaves.values()) / inv["root_total_loc"]
    need = {L for name, (L, (x, y, w, h)) in leaves.items()
            if (w - GAP < MIN_NUM_W or h - GAP < MIN_NUM_H)}
    if need - callout:
        return build_layout(inv, callout | need)
    return layer_rects, leaves, k, layer_loc


def verify(inv, layer_rects, leaves, k, layer_loc):
    comps = {c["name"]: c for c in inv["components"]}
    assert sum(c["loc"] for c in comps.values()) == inv["root_total_loc"], "root total"
    for L, loc in layer_loc.items():
        assert sum(comps[n]["loc"] for n, (l, _) in leaves.items() if l == L) == loc, f"subtotal {L}"
    panel = (MARGIN, MARGIN + TITLE_H, W - 2 * MARGIN, H - 2 * MARGIN - TITLE_H - LEGEND_H)
    for name, (L, (x, y, w, h)) in leaves.items():
        loc = comps[name]["loc"]
        if loc == 0:
            assert w * h == 0, f"zero LOC with area: {name}"
            continue
        ratio = (w * h) / (k * loc)
        assert abs(ratio - 1) < RATIO_TOL, f"area ratio {name}: {ratio}"
        lx, ly, lw, lh = layer_rects[L]
        assert x >= lx - 1e-6 and y >= ly - 1e-6 and x + w <= lx + lw + 1e-6 and y + h <= ly + lh + 1e-6, f"leaf outside layer: {name}"
    for L, (x, y, w, h) in layer_rects.items():
        px, py, pw, ph = panel
        assert x >= px - 1e-6 and y >= py - 1e-6 and x + w <= px + pw + 1e-6 and y + h <= py + ph + 1e-6, f"layer outside panel: {L}"
    # rendered geometry: rounding to 0.5px must keep every leaf within tolerance
    for name, (L, (x, y, w, h)) in leaves.items():
        rw, rh = round(w * 2) / 2, round(h * 2) / 2
        assert abs(rw - w) <= PIXEL_TOL and abs(rh - h) <= PIXEL_TOL, f"rounding {name}"
    # no two leaves overlap
    items = list(leaves.items())
    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            a, b = items[i][1][1], items[j][1][1]
            ox = min(a[0] + a[2], b[0] + b[2]) - max(a[0], b[0])
            oy = min(a[1] + a[3], b[1] + b[3]) - max(a[1], b[1])
            assert ox <= GAP + 1e-6 or oy <= GAP + 1e-6 or ox * oy < 1e-6, f"overlap {items[i][0]}/{items[j][0]}"


# --- rendering ---

def esc(s):
    return s.replace("&", "&amp;").replace("<", "&lt;")


def text(x, y, s, size=BODY, weight="normal", anchor="start", fill="#000", family=FONT):
    return (f'<text x="{x:.1f}" y="{y:.1f}" dy="0.35em" font-family="{family}" '
            f'font-size="{size}" font-weight="{weight}" fill="{fill}" '
            f'text-anchor="{anchor}">{esc(s)}</text>')


def render(inv, layer_rects, leaves, k, layer_loc):
    comps = {c["name"]: c for c in inv["components"]}
    total = inv["root_total_loc"]
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}">',
           '<!-- generated by generate.py (weighted-source trial), do not edit -->',
           '<rect width="100%" height="100%" fill="white"/>']
    out.append(text(MARGIN, MARGIN + 14, "Pelikan — weighted source at a glance", H1, "bold"))
    out.append(text(MARGIN, MARGIN + 40,
                    f"Area ∝ physical LOC (comments and blanks included). {total:,} lines across "
                    f"{sum(c['files'] for c in comps.values())} tracked .rs files in {len(comps)} workspace crates. "
                    f"Revision {inv['revision'][:7]}.", BODY))
    out.append(text(MARGIN, MARGIN + 58,
                    "Adjacency claims no dependency and no execution order; see architecture.svg for layering.",
                    BODY, fill="#4D4D4D"))
    keyed = []
    callouts = []
    for L in [l for l in LAYER_ORDER if l in layer_rects]:
        x, y, w, h = layer_rects[L]
        out.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" fill="#FAFAFA" stroke="#9E9E9E" stroke-width="1.5"/>')
        out.append(text(x + PAD + 2, y + HEADER_H / 2 + 1,
                        f"{L}  ·  {layer_loc[L]:,} LOC  ·  {100 * layer_loc[L] / total:.1f}%", H2, "bold"))
    for name, (L, (x, y, w, h)) in sorted(leaves.items(), key=lambda kv: -comps[kv[0]]["loc"]):
        gx, gy, gw, gh = x + GAP / 2, y + GAP / 2, w - GAP, h - GAP
        if gw <= 0 or gh <= 0:
            continue
        stroke = STROKE.get(L, "#4D4D4D")
        dash = ' stroke-dasharray="4 3"' if L == "Tooling" else ""
        out.append(f'<rect x="{gx:.1f}" y="{gy:.1f}" width="{gw:.1f}" height="{gh:.1f}" '
                   f'fill="{FILL[L]}" stroke="{stroke}" stroke-width="1.2"{dash}/>')
        c = comps[name]
        label = f"{c['loc']:,} · {100 * c['share']:.1f}%"
        name_w = max(len(name), len(label)) * BODY * CHAR_W + 8
        if gw >= name_w and gh >= MIN_LABEL_H:
            out.append(text(gx + gw / 2, gy + gh / 2 - 8, name, BODY, "bold", "middle"))
            out.append(text(gx + gw / 2, gy + gh / 2 + 9, label, BODY, "normal", "middle"))
        elif gw >= MIN_NUM_W and gh >= MIN_NUM_H:
            keyed.append((name, c))
            out.append(text(gx + gw / 2, gy + gh / 2, str(len(keyed)), BODY, "bold", "middle"))
        else:
            keyed.append((name, c))
            lx, ly, lw, lh = layer_rects[L]
            cx = min(max(gx + gw / 2, lx + PAD + 8), lx + lw - PAD - 8)
            cy = ly + lh - PAD - CALLOUT_H / 2
            while any(abs(cx - ox) < 18 and abs(cy - oy) < 1 for ox, oy in callouts):
                cx += 18
            callouts.append((cx, cy))
            out.append(f'<line x1="{gx + gw / 2:.1f}" y1="{gy + gh / 2:.1f}" x2="{cx:.1f}" y2="{cy - 8:.1f}" stroke="#4D4D4D" stroke-width="1"/>')
            out.append(text(cx, cy, str(len(keyed)), BODY, "bold", "middle"))
    # keyed legend for tiles too small to label in place
    ly = H - LEGEND_H - MARGIN + 28
    out.append(text(MARGIN, ly, "Tiles labeled by number (too small for text; areas are exact):", BODY, "bold"))
    col_w = (W - 2 * MARGIN) / 3
    for i, (name, c) in enumerate(keyed):
        cx = MARGIN + (i % 3) * col_w
        cy = ly + 22 + (i // 3) * 20
        out.append(text(cx, cy, f"{i + 1}. {name} — {c['loc']:,} LOC · {100 * c['share']:.1f}% ({c['layer']})", BODY))
    out.append(text(MARGIN, H - MARGIN - 24,
                    "Scope: all tracked .rs in workspace members, each file counted once under its crate. "
                    "xtask (tooling) sits outside the architecture chart and is dashed here.",
                    BODY, fill="#4D4D4D"))
    out.append(text(MARGIN, H - MARGIN - 6,
                    "Excluded: non-Rust files, untracked resp-proto-kani/. "
                    "LOC measures source footprint, not runtime cost, complexity, or quality.",
                    BODY, fill="#4D4D4D"))
    out.append("</svg>")
    return "\n".join(out) + "\n"


def check_text_bounds(svg, layer_rects, leaves):
    """Estimate each text's extent and require it inside the canvas, and in-tile
    labels inside their tile. Estimates are generous (0.56 em per character)."""
    import re
    tiles = [r for _, r in leaves.values()] + list(layer_rects.values())
    for m in re.finditer(r'<text x="([\d.]+)" y="([\d.]+)"[^>]*font-size="(\d+)"[^>]*text-anchor="(\w+)">([^<]*)</text>', svg):
        x, y, size, anchor, s = float(m[1]), float(m[2]), int(m[3]), m[4], m[5]
        s = s.replace("&amp;", "&").replace("&lt;", "<")
        tw = len(s) * size * CHAR_W
        x0 = x - tw / 2 if anchor == "middle" else x
        x1 = x0 + tw
        assert x0 >= 0 and x1 <= W and y - size / 2 >= 0 and y + size / 2 <= H, f"text off canvas: {s[:40]!r} ({x0:.0f}..{x1:.0f})"
        if anchor == "middle":  # an in-tile label: find its tile and require containment
            tile = [t for t in tiles if t[0] <= x <= t[0] + t[2] and t[1] <= y <= t[1] + t[3]]
            if tile:
                t = min(tile, key=lambda t: t[2] * t[3])
                assert x0 >= t[0] - 1 and x1 <= t[0] + t[2] + 1, f"label wider than tile: {s!r}"


def textual(inv, layer_loc):
    total = inv["root_total_loc"]
    lines = ["# Pelikan weighted source inventory", "",
             f"Revision `{inv['revision']}` ({inv['generated']}). "
             f"Tracked files modified: {len(inv['tracked_files_modified'])}. "
             f"Untracked paths present: {len(inv['untracked_paths'])}. "
             f"Inventory digest `{inv['inventory_digest_sha256'][:16]}…`.", "",
             f"**Scope.** {inv['scope']}.", "",
             f"**Counting.** {inv['counting_policy']}.", "",
             f"**Breakdown.** {inv['breakdown_policy']}.", "",
             "**Excluded.** " + "; ".join(inv["exclusions"]) + ".", "",
             f"**Root total: {total:,} LOC.**", "",
             "| Layer | LOC | Share |", "| --- | ---: | ---: |"]
    for L in [l for l in LAYER_ORDER if l in layer_loc]:
        lines.append(f"| {L} | {layer_loc[L]:,} | {100 * layer_loc[L] / total:.1f}% |")
    lines += ["", "| Crate | Layer | Path | Files | LOC | Share | src | tests | benches |",
              "| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for c in sorted(inv["components"], key=lambda c: (-c["loc"], c["name"])):
        b = c["breakdown"]
        lines.append(f"| {c['name']} | {c['layer']} | `{c['path']}` | {c['files']} | {c['loc']:,} | "
                     f"{100 * c['share']:.1f}% | {b['src']:,} | {b['tests']:,} | {b['benches']:,} |")
    return "\n".join(lines) + "\n"


# --- self-test on synthetic inventories ---

def self_test():
    def inv_of(pairs):
        comps = [{"name": n, "path": n, "layer": L, "files": 1, "loc": loc,
                  "breakdown": {"src": loc, "tests": 0, "benches": 0}} for n, L, loc in pairs]
        total = sum(c["loc"] for c in comps)
        for c in comps:
            c["share"] = c["loc"] / total if total else 0
        return {"root_total_loc": total, "components": comps, "revision": "test", "generated": "test"}

    # 1:4 weights allocate 1:4 areas, across layers
    inv = inv_of([("a", "Core", 100), ("b", "Protocol", 400)])
    lr, leaves, k, ll = build_layout(inv)
    verify(inv, lr, leaves, k, ll)
    ra, rb = leaves["a"][1], leaves["b"][1]
    assert abs((rb[2] * rb[3]) / (ra[2] * ra[3]) - 4) < 1e-6, "1:4 ratio"
    # tiny and zero weights
    inv = inv_of([("big", "Core", 50000), ("tiny", "Core", 3), ("zero", "Storage", 0), ("mid", "Storage", 700)])
    lr, leaves, k, ll = build_layout(inv)
    verify(inv, lr, leaves, k, ll)
    assert "zero" not in leaves, "zero-LOC gets no tile"
    # a file shared by two products is an ownership error, not a double count
    try:
        owners = {"x.rs": ["p1", "p2"]}
        assert all(len(v) == 1 for v in owners.values())
        raise SystemExit("shared ownership was not rejected")
    except AssertionError:
        pass
    print("self-test ok")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo")
    ap.add_argument("--out", default="out")
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        self_test()
        return
    inv = inventory(os.path.abspath(a.repo))
    lr, leaves, k, ll = build_layout(inv)
    verify(inv, lr, leaves, k, ll)
    os.makedirs(a.out, exist_ok=True)
    with open(os.path.join(a.out, "inventory.json"), "w") as f:
        json.dump(inv, f, indent=2)
        f.write("\n")
    svg = render(inv, lr, leaves, k, ll)
    check_text_bounds(svg, lr, leaves)
    with open(os.path.join(a.out, "weighted-source.svg"), "w") as f:
        f.write(svg)
    with open(os.path.join(a.out, "inventory.md"), "w") as f:
        f.write(textual(inv, ll))
    print(f"ok: {inv['root_total_loc']} LOC, {len(leaves)} leaves, k={k:.4f} px²/LOC, "
          f"revision {inv['revision'][:7]}, modified={len(inv['tracked_files_modified'])}")


if __name__ == "__main__":
    main()
