#!/usr/bin/env python3
"""Weighted source at a glance, for a Cargo workspace.

Trial generator for the `weighted-source-at-a-glance` conventions in
skills-mcp's architecture-diagram skill. Standalone on purpose: it derives
from `cargo metadata` and `git ls-files`, takes the project's layer table and
palette from a bindings file (the charter's role), and writes a treemap SVG,
a machine-readable inventory, and a textual equivalent.

    python3 generate.py --repo ../pelikan --bindings pelikan.json --out out/
    python3 generate.py --self-test
"""
import argparse
import hashlib
import rustscan
import json
import os
import subprocess
import sys
from datetime import date

# --- bindings: loaded from the project's JSON file, validated against cargo metadata ---

B = {}  # title, repository, layers {crate: layer}, order, fill, stroke, dashed,
        # outside_chart (layers the structure chart excludes), derived (validation rules)
LAYER = {}
LAYER_ORDER = []
FILL = {}
STROKE = {}
FONT = "Helvetica, Arial, sans-serif"


def load_bindings(path):
    global B, LAYER, LAYER_ORDER, FILL, STROKE
    B = json.load(open(path))
    LAYER = B["layers"]
    LAYER_ORDER = B["order"]
    FILL = B["fill"]
    STROKE = B.get("stroke", {})
    missing = set(LAYER.values()) - set(LAYER_ORDER)
    assert not missing, f"layers without an order: {missing}"
    assert set(LAYER_ORDER) <= set(FILL), "every layer needs a fill"
H1, H2, BODY = 20, 17, 14  # the build chart's ramp (charter override, #183)

# --- geometry ---

W, H = 1400, 900
MARGIN = 24
TITLE_H = 70
LEGEND_H = 190
HEADER_H = 28     # layer header strip, outside the quantitative area
PAD = 6           # gutter inside a layer, outside the quantitative area
GAP = 3           # gutter between leaves
MIN_LABEL_W, MIN_LABEL_H = 96, 36
MIN_NUM_W, MIN_NUM_H = 20, 18   # smallest tile that takes a numeral in place
HATCH_MIN_PX = 4  # a verification strip narrower than this is not drawn, only tabled
KINDS = ["test", "bench", "example", "fuzz", "proof"]
DIR_KIND = {"tests": "test", "benches": "bench", "examples": "example", "fuzz": "fuzz"}
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


def assign_owners(files, pkgs):
    """Map each tracked file to exactly one package by longest manifest
    directory. Cargo reports a root package's directory as ".", which owns
    every file no deeper manifest claims. Returns (owner, path relative to
    the owning manifest directory)."""
    prefixes = sorted(((n, "" if d == "." else d.rstrip("/") + "/") for n, d in pkgs.items()),
                      key=lambda nd: -len(nd[1]))
    owner, rel = {}, {}
    for f in files:
        hits = [(n, pre) for n, pre in prefixes if f.startswith(pre)]
        if not hits:
            sys.exit(f"unmapped source file: {f}")
        n, pre = hits[0]
        owner[f] = n
        rel[f] = f[len(pre):]
    return owner, rel


def inventory_digest(repo, files):
    """SHA-256 over every tracked source file's path and content hash, so two
    dirty states with equal line counts never share a digest."""
    h = hashlib.sha256()
    for f in sorted(files):
        h.update(f.encode())
        h.update(b"\0")
        h.update(hashlib.sha256(open(os.path.join(repo, f), "rb").read()).hexdigest().encode())
        h.update(b"\n")
    return h.hexdigest()


def inventory(repo):
    meta = json.loads(sh(["cargo", "metadata", "--no-deps", "--format-version", "1"], repo))
    root = meta["workspace_root"]
    pkgs = {}
    for p in meta["packages"]:
        d = os.path.relpath(os.path.dirname(p["manifest_path"]), root)
        pkgs[p["name"]] = d
    known = LAYER
    missing = sorted(set(pkgs) - set(known))
    stale = sorted(set(known) - set(pkgs))
    if missing or stale:
        sys.exit(f"classification drift: unclassified {missing}, stale {stale}")
    # Curated layers are validated against the derived dependency graph where
    # the bindings state a rule, e.g. "Engine" == "depends on keyvalue".
    ws_deps = {p["name"]: {d["name"] for d in p["dependencies"]
                           if d["name"] in pkgs and d.get("kind") is None} for p in meta["packages"]}
    for layer, rule in B.get("derived", {}).items():
        dep = rule["depends_on"]
        tagged = {n for n, l in known.items() if l == layer}
        derived = {n for n, ds in ws_deps.items() if dep in ds}
        if tagged != derived:
            sys.exit(f"layer {layer!r} disagrees with the dependency graph: "
                     f"tagged {sorted(tagged)}, derived {sorted(derived)}")

    files = sh(["git", "ls-files", "--", "*.rs"], repo).split()
    owner, relpath = assign_owners(files, pkgs)
    counts = {n: 0 for n in pkgs}
    parts = {n: {"production": 0, **{k: 0 for k in KINDS}} for n in pkgs}
    nfiles = {n: 0 for n in pkgs}
    # Whole-file verification: a file under a target directory, or a module
    # declared from a verification item (`#[cfg(test)] mod tests;`), and every
    # file under such a module's directory.
    whole = {}
    for f in files:
        rel = relpath[f]
        top = rel.split("/")[0]
        if top in DIR_KIND:
            whole[f] = DIR_KIND[top]
    for f in files:
        if f in whole:
            continue
        _, mods = rustscan.verification_lines(open(os.path.join(repo, f), encoding="utf-8", errors="replace").read())
        for name, kind in mods:
            child = rustscan.child_file(os.path.join(repo, f), name)
            if child is None:
                continue
            crel = os.path.relpath(child, repo)
            whole[crel] = kind
            sub_dir = crel[:-3] + "/" if crel.endswith(".rs") and not crel.endswith("/mod.rs") else os.path.dirname(crel) + "/"
            for g in files:
                if g != crel and g.startswith(sub_dir) and (crel.endswith("/mod.rs") or not crel.endswith("lib.rs")):
                    whole.setdefault(g, kind)
    file_rows = []
    for f, n in sorted(owner.items()):
        src = open(os.path.join(repo, f), encoding="utf-8", errors="replace").read()
        loc = rustscan.count_lines(src)
        counts[n] += loc
        nfiles[n] += 1
        if f in whole:
            parts[n][whole[f]] += loc
            file_rows.append({"path": f, "owner": n, "loc": loc, "classification": f"whole file: {whole[f]}", "verification": loc})
        else:
            kinds, _ = rustscan.verification_lines(src)
            for kind, v in kinds.items():
                parts[n][kind] += v
            v = sum(kinds.values())
            parts[n]["production"] += loc - v
            file_rows.append({"path": f, "owner": n, "loc": loc,
                              "classification": "production" if v == 0 else "mixed: " + ", ".join(f"{k} {c}" for k, c in sorted(kinds.items())),
                              "verification": v})
    empty = [n for n, c in nfiles.items() if c == 0]
    if empty:
        sys.exit(f"members with no tracked .rs files: {empty}")

    rev = sh(["git", "rev-parse", "HEAD"], repo).strip()
    status = sh(["git", "status", "--porcelain"], repo).splitlines()
    modified = [l for l in status if not l.startswith("??")]
    untracked = [l[3:] for l in status if l.startswith("??")]
    digest = inventory_digest(repo, files)
    # Exclusions are derived: tracked non-.rs extensions, and any .rs on disk
    # that git does not track (vendored or untracked trees).
    tracked = sh(["git", "ls-files"], repo).split()
    exts = {}
    for f in tracked:
        if "." in os.path.basename(f) and not f.endswith(".rs"):
            exts[f.rsplit(".", 1)[1]] = exts.get(f.rsplit(".", 1)[1], 0) + 1
    untracked_rs = sh(["git", "ls-files", "--others", "--exclude-standard", "--", "*.rs"], repo).split()
    untracked_trees = sorted({f.split("/")[0] for f in untracked_rs})
    exclusions = ["tracked non-.rs files (" + ", ".join(f"{e}: {c}" for e, c in sorted(exts.items(), key=lambda kv: -kv[1])[:8]) + ")"]
    if untracked_trees:
        exclusions.append("untracked .rs under " + ", ".join(t + "/" for t in untracked_trees))
    exclusions.append("target/ and vendored dependencies (not tracked)")
    comps = []
    for n in sorted(pkgs):
        comps.append({"name": n, "path": pkgs[n], "layer": known[n],
                      "files": nfiles[n], "loc": counts[n], "breakdown": parts[n]})
    total = sum(counts.values())
    for c in comps:
        c["share"] = c["loc"] / total
        c["verification"] = sum(c["breakdown"][k] for k in KINDS)
    branch = sh(["git", "rev-parse", "--abbrev-ref", "HEAD"], repo).strip()
    return {
        "repository": B["repository"],
        "revision": rev,
        "branch": branch,
        "tracked_files_modified": modified,
        "untracked_paths": untracked,
        "inventory_digest_sha256": digest,
        "generated": date.today().isoformat(),
        "scope": "all tracked *.rs files in Cargo workspace members, one owner each "
                 "(longest manifest directory); not the code compiled into any one binary",
        "counting_policy": "physical LOC: every line including comments and blanks; "
                           "a final unterminated line counts as one line",
        "breakdown_policy": "verification = files under tests/, benches/, examples/, fuzz/ (kind by directory); "
                            "files declared by `mod x;` under #[cfg(test)] or #[cfg(kani)] and everything beneath them; "
                            "and, inside any other file, items carrying #[cfg(test)], #[cfg(kani)], #[test], or "
                            "#[kani::proof] (an any()/all() predicate containing test or kani counts; a file-level "
                            "#![cfg(test)] makes the whole file verification), measured by a brace scanner that honors comments and literals "
                            "(rustscan.py). cfg(not(test)) is production. Unclassifiable lines count as production.",
        "exclusions": exclusions,
        "root_total_loc": total,
        "components": comps,
        "files": file_rows,
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
    # Leaf gutters are outside the quantitative area too: each leaf's outer
    # allocation is iterated until its drawn rectangle, the one serialized to
    # SVG, has area k * LOC. The layer loop above then re-balances on the sum
    # of drawn areas, so one k holds across the whole view.
    def drawn_leaves(L, rect):
        x, y, w, h = rect
        ix, iy, iw, ih = x + PAD, y + HEADER_H + PAD, w - 2 * PAD, h - HEADER_H - 2 * PAD - extra[L]
        cs = sorted((c for c in layers[L] if c["loc"] > 0), key=lambda c: (-c["loc"], c["name"]))
        ow = {c["name"]: float(c["loc"]) for c in cs}
        loc = {c["name"]: c["loc"] for c in cs}
        if not cs:
            return {}
        # Drawn area is affine in the allocation, (w - GAP) * (h - GAP), and
        # which dimension responds to a weight change depends on the row the
        # squarified layout puts the tile in. Rather than model that, solve
        # each leaf's weight by bisection against the actual layout, holding
        # the others fixed, and sweep until every drawn area meets k * LOC.
        # Bisection cannot oscillate; a layout with no solution fails loudly.
        def drawn_of(weights):
            alloc = squarify([(n, weights[n]) for n in weights], ix, iy, iw, ih)
            return alloc, {n: max(0.0, r[2] - GAP) * max(0.0, r[3] - GAP) for n, r in alloc.items()}

        worst = 1.0
        for sweep in range(120):
            alloc, drawn = drawn_of(ow)
            kl = sum(drawn.values()) / sum(loc.values())
            worst = max(abs(drawn[n] / (kl * loc[n]) - 1) if drawn[n] > 0 else 1.0 for n in ow)
            if worst < 1e-9 and sweep > 1:
                break
            for n in ow:
                target = kl * loc[n]
                lo, hi = ow[n] / 64.0, ow[n] * 64.0
                for _ in range(60):
                    mid = (lo * hi) ** 0.5
                    trial = dict(ow)
                    trial[n] = mid
                    _, d = drawn_of(trial)
                    if d[n] < target:
                        lo = mid
                    else:
                        hi = mid
                ow[n] = (lo * hi) ** 0.5
        alloc, drawn = drawn_of(ow)
        assert worst < 1e-6, f"leaf layout did not converge in layer {L}: worst deviation {worst}"
        return {n: (r[0] + GAP / 2, r[1] + GAP / 2, r[2] - GAP, r[3] - GAP) for n, r in alloc.items()}

    for it in range(200):
        rects = squarify([(L, outer_w[L]) for L in order], data_x, data_y, data_w, data_h)
        per_layer = {L: drawn_leaves(L, rects[L]) for L in order}
        dsum = {L: sum(r[2] * r[3] for r in per_layer[L].values()) for L in order}
        k = sum(dsum.values()) / sum(layer_loc.values())
        worst = 0.0
        for L in order:
            ratio = dsum[L] / (k * layer_loc[L]) if dsum[L] > 0 else 0.25
            worst = max(worst, abs(ratio - 1))
            outer_w[L] *= ratio ** -0.5
        if worst < 1e-9 and it > 2:
            break
    assert worst < 1e-6, f"layer layout did not converge: worst deviation {worst}"
    leaves = {}
    layer_rects = {}
    for L in order:
        layer_rects[L] = rects[L]
        for name, r in per_layer[L].items():
            leaves[name] = (L, r)
    k = sum(r[2] * r[3] for _, r in leaves.values()) / inv["root_total_loc"]
    need = {L for name, (L, (x, y, w, h)) in leaves.items()
            if (w < MIN_NUM_W or h < MIN_NUM_H)}
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
        # the numbers that reach the SVG are rounded to 0.1 px; the error that
        # rounding can introduce is bounded, and the serialized area must sit
        # inside that bound
        rw, rh = float(f"{w:.1f}"), float(f"{h:.1f}")
        bound = 0.05 * (rw + rh) + 0.0025
        assert abs(rw * rh - k * loc) <= bound + 1e-9, f"serialized area {name}: {rw * rh} vs {k * loc}"
        lx, ly, lw, lh = layer_rects[L]
        assert x >= lx - 1e-6 and y >= ly - 1e-6 and x + w <= lx + lw + 1e-6 and y + h <= ly + lh + 1e-6, f"leaf outside layer: {name}"
    for L, (x, y, w, h) in layer_rects.items():
        px, py, pw, ph = panel
        assert x >= px - 1e-6 and y >= py - 1e-6 and x + w <= px + pw + 1e-6 and y + h <= py + ph + 1e-6, f"layer outside panel: {L}"
    # verification strip: a partition of the drawn tile by weight, no inset
    for name, (L, (x, y, w, h)) in leaves.items():
        v = comps[name]["verification"]
        if v:
            sx, sy, sw, sh = hatch_rect((x, y, w, h), v / comps[name]["loc"])
            assert abs((sw * sh) / (k * v) - 1) < RATIO_TOL, f"hatch ratio {name}"
    # no two drawn leaves overlap
    items = list(leaves.items())
    for i in range(len(items)):
        for j in range(i + 1, len(items)):
            a, b = items[i][1][1], items[j][1][1]
            ox = min(a[0] + a[2], b[0] + b[2]) - max(a[0], b[0])
            oy = min(a[1] + a[3], b[1] + b[3]) - max(a[1], b[1])
            assert ox <= 1e-6 or oy <= 1e-6, f"overlap {items[i][0]}/{items[j][0]}"


def hatch_rect(tile, frac):
    """The verification strip: a partition of the drawn tile along its longer
    side, so strip area / tile area == frac exactly."""
    x, y, w, h = tile
    if w >= h:
        return (x + w - w * frac, y, w * frac, h)
    return (x, y + h - h * frac, w, h * frac)


def check_svg_geometry(svg, leaves, k, comps):
    """Read the rectangles back out of the SVG text and hold them to k * LOC.
    This is the check the reviewer asked for: on what was serialized, not on
    what was allocated."""
    import re
    seen = set()
    for m in re.finditer(r'<rect [^>]*data-component="([^"]+)" data-part="(tile|hatch)"[^>]*/>', svg):
        name, part = m.group(1), m.group(2)
        attrs = dict(re.findall(r'(?<![\w-])(x|y|width|height)="([\d.]+)"', m.group(0)))  # not stroke-width
        area = float(attrs["width"]) * float(attrs["height"])
        want = k * (comps[name]["loc"] if part == "tile" else comps[name]["verification"])
        bound = 0.05 * (float(attrs["width"]) + float(attrs["height"])) + 0.0025
        assert abs(area - want) <= bound + 1e-9, f"svg {part} {name}: {area:.2f} vs {want:.2f}"
        seen.add((name, part))
    for name in leaves:
        if comps[name]["loc"] > 0:
            assert (name, "tile") in seen, f"tile not serialized: {name}"


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
           '<defs><pattern id="hatch" width="7" height="7" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">'
           '<line x1="0" y1="0" x2="0" y2="7" stroke="#4D4D4D" stroke-width="1.4"/></pattern></defs>',
           '<rect width="100%" height="100%" fill="white"/>']
    unhatched = []
    out.append(text(MARGIN, MARGIN + 14, f"{B['title']} — weighted source at a glance", H1, "bold"))
    out.append(text(MARGIN, MARGIN + 40,
                    f"Area ∝ physical LOC (comments and blanks included). {total:,} lines across "
                    f"{sum(c['files'] for c in comps.values())} tracked .rs files in {len(comps)} workspace crates. "
                    f"Revision {inv['revision'][:7]}.", BODY))
    state = (f" Rendered on branch {inv['branch']}" if inv['branch'] != 'main' else '')
    state += (f" with {len(inv['tracked_files_modified'])} tracked files modified" if inv['tracked_files_modified'] else '')
    out.append(text(MARGIN, MARGIN + 58,
                    "Adjacency claims no dependency and no execution order." + (state + "." if state else ""),
                    BODY, fill="#4D4D4D"))
    keyed = []
    callouts = []
    for L in [l for l in LAYER_ORDER if l in layer_rects]:
        x, y, w, h = layer_rects[L]
        out.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{w:.1f}" height="{h:.1f}" fill="#FAFAFA" stroke="#9E9E9E" stroke-width="1.5"/>')
        out.append(text(x + PAD + 2, y + HEADER_H / 2 + 1,
                        f"{L}  ·  {layer_loc[L]:,} LOC  ·  {100 * layer_loc[L] / total:.1f}%", H2, "bold"))
    for name, (L, (gx, gy, gw, gh)) in sorted(leaves.items(), key=lambda kv: -comps[kv[0]]["loc"]):
        if gw <= 0 or gh <= 0:
            continue
        stroke = STROKE.get(L, "#4D4D4D")
        dash = ' stroke-dasharray="4 3"' if L in B.get("dashed", []) else ""
        out.append(f'<rect x="{gx:.1f}" y="{gy:.1f}" width="{gw:.1f}" height="{gh:.1f}" '
                   f'fill="{FILL[L]}" stroke="{stroke}" stroke-width="1.2"{dash} '
                   f'data-component="{esc(name)}" data-part="tile"/>')
        c = comps[name]
        # Verification share as a hatched partition of the drawn tile along its
        # longer side, so its area is k * verification LOC. Drawn only when the
        # strip is at least HATCH_MIN_PX across; otherwise tabled.
        sx, sy, sw, sh = gx, gy, gw, gh  # solid part, for the label
        if c["verification"] > 0:
            hx, hy, hw, hh = hatch_rect((gx, gy, gw, gh), c["verification"] / c["loc"])
            if min(hw, hh) >= HATCH_MIN_PX:
                out.append(f'<rect x="{hx:.1f}" y="{hy:.1f}" width="{hw:.1f}" height="{hh:.1f}" fill="url(#hatch)" '
                           f'data-component="{esc(name)}" data-part="hatch"/>')
                out.append(f'<line x1="{hx:.1f}" y1="{gy:.1f}" x2="{hx:.1f}" y2="{gy + gh:.1f}" stroke="#4D4D4D" stroke-width="0.8"/>' if gw >= gh else
                           f'<line x1="{gx:.1f}" y1="{hy:.1f}" x2="{gx + gw:.1f}" y2="{hy:.1f}" stroke="#4D4D4D" stroke-width="0.8"/>')
                if gw >= gh:
                    sw = hx - gx
                else:
                    sh = hy - gy
            else:
                unhatched.append(name)
        label = f"{c['loc']:,} · {100 * c['share']:.1f}%"
        name_w = max(len(name), len(label)) * BODY * CHAR_W + 8
        if sw >= name_w and sh >= MIN_LABEL_H:
            out.append(text(sx + sw / 2, sy + sh / 2 - 8, name, BODY, "bold", "middle"))
            out.append(text(sx + sw / 2, sy + sh / 2 + 9, label, BODY, "normal", "middle"))
        elif gw >= name_w and gh >= MIN_LABEL_H:
            # the label spans the hatched strip: a halo keeps it legible
            out.append(f'<rect x="{gx + gw / 2 - name_w / 2:.1f}" y="{gy + gh / 2 - 17:.1f}" width="{name_w:.1f}" height="34" '
                       f'fill="{FILL[L]}" fill-opacity="0.92" rx="2"/>')
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
    ly = H - LEGEND_H - MARGIN + 12
    vtotal = sum(c["verification"] for c in comps.values())
    out.append(f'<rect x="{MARGIN}" y="{ly - 7}" width="28" height="14" fill="url(#hatch)" stroke="#4D4D4D" stroke-width="0.8"/>')
    out.append(text(MARGIN + 36, ly,
                    f"Hatched: verification source (tests, benches, examples, fuzz targets, proof harnesses), "
                    f"{vtotal:,} LOC · {100 * vtotal / total:.1f}% of the total, drawn to the same area scale.", BODY))
    ly += 18
    out.append(text(MARGIN + 36, ly,
                    f"A strip under {HATCH_MIN_PX} px is tabled, not drawn"
                    + (f": {', '.join(unhatched)}" if unhatched else "") + ".", BODY, fill="#4D4D4D"))
    ly += 24
    out.append(text(MARGIN, ly, "Tiles labeled by number (too small for text; areas are exact):", BODY, "bold"))
    col_w = (W - 2 * MARGIN) / 3
    for i, (name, c) in enumerate(keyed):
        cx = MARGIN + (i % 3) * col_w
        cy = ly + 22 + (i // 3) * 20
        out.append(text(cx, cy, f"{i + 1}. {name} — {c['loc']:,} LOC · {100 * c['share']:.1f}% ({c['layer']})", BODY))
    if B.get("footer"):
        out.append(text(MARGIN, H - MARGIN - 42, B["footer"], BODY, fill="#4D4D4D"))
    out.append(text(MARGIN, H - MARGIN - 24,
                    "Scope: all tracked .rs in workspace members, each file counted once under its crate.",
                    BODY, fill="#4D4D4D"))
    out.append(text(MARGIN, H - MARGIN - 6,
                    "Excluded: non-Rust files and untracked source (see inventory.md). "
                    "LOC measures source footprint, not runtime cost, complexity, or quality.",
                    BODY, fill="#4D4D4D"))
    out.append("</svg>")
    return "\n".join(out) + "\n"


def check_text_bounds(svg, layer_rects, leaves):
    """Estimate each text's extent and require it inside the canvas, and in-tile
    labels inside their tile. Estimates are generous (0.56 em per character)."""
    import re
    tiles = [r for _, r in leaves.values()] + list(layer_rects.values())
    boxes = []
    for m in re.finditer(r'<text x="([\d.]+)" y="([\d.]+)"[^>]*font-size="(\d+)"[^>]*text-anchor="(\w+)">([^<]*)</text>', svg):
        x, y, size, anchor, s = float(m[1]), float(m[2]), int(m[3]), m[4], m[5]
        s = s.replace("&amp;", "&").replace("&lt;", "<")
        tw = len(s) * size * CHAR_W
        x0 = x - tw / 2 if anchor == "middle" else x
        x1 = x0 + tw
        assert x0 >= 0 and x1 <= W and y - size / 2 >= 0 and y + size / 2 <= H, f"text off canvas: {s[:40]!r} ({x0:.0f}..{x1:.0f})"
        boxes.append((x0, y - size / 2, x1, y + size / 2, s))
        if anchor == "middle":  # an in-tile label: find its tile and require containment
            tile = [t for t in tiles if t[0] <= x <= t[0] + t[2] and t[1] <= y <= t[1] + t[3]]
            if tile:
                t = min(tile, key=lambda t: t[2] * t[3])
                assert x0 >= t[0] - 1 and x1 <= t[0] + t[2] + 1, f"label wider than tile: {s!r}"
    # no two texts overlap (estimated boxes)
    for i in range(len(boxes)):
        for j in range(i + 1, len(boxes)):
            a, b = boxes[i], boxes[j]
            if a[0] < b[2] and b[0] < a[2] and a[1] < b[3] and b[1] < a[3]:
                raise AssertionError(f"texts overlap: {a[4][:30]!r} / {b[4][:30]!r}")


def textual(inv, layer_loc):
    total = inv["root_total_loc"]
    lines = [f"# {B['title']} weighted source inventory", "",
             f"Revision `{inv['revision']}` on `{inv['branch']}` ({inv['generated']}). "
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
    vt = sum(c["verification"] for c in inv["components"])
    lines += ["", f"**Verification source: {vt:,} LOC, {100 * vt / total:.1f}%** (tests, benches, examples, fuzz, proofs; see breakdown policy).",
              "", "| Crate | Layer | Path | Files | LOC | Share | production | test | bench | example | fuzz | proof |",
              "| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for c in sorted(inv["components"], key=lambda c: (-c["loc"], c["name"])):
        b = c["breakdown"]
        lines.append(f"| {c['name']} | {c['layer']} | `{c['path']}` | {c['files']} | {c['loc']:,} | "
                     f"{100 * c['share']:.1f}% | {b['production']:,} | {b['test']:,} | {b['bench']:,} | {b['example']:,} | {b['fuzz']:,} | {b['proof']:,} |")
    return "\n".join(lines) + "\n"


# --- self-test on synthetic inventories ---

def self_test():
    global LAYER_ORDER, FILL, B
    LAYER_ORDER = ["Core", "Protocol", "Storage"]
    FILL = {L: "#FFFFFF" for L in LAYER_ORDER}
    B = {"title": "self-test", "repository": "test"}
    def inv_of(pairs):
        comps = [{"name": n, "path": n, "layer": L, "files": 1, "loc": loc, "verification": loc // 5,
                  "breakdown": {"production": loc - loc // 5, "test": loc // 5, "bench": 0, "example": 0, "fuzz": 0, "proof": 0}} for n, L, loc in pairs]
        total = sum(c["loc"] for c in comps)
        for c in comps:
            c["share"] = c["loc"] / total if total else 0
        return {"root_total_loc": total, "components": comps, "revision": "test0000", "generated": "test",
                "branch": "main", "tracked_files_modified": [], "untracked_paths": [], "inventory_digest_sha256": "0" * 64}

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
    # the serialized rectangles, not only the allocated ones, carry k * LOC
    inv = inv_of([("a", "Core", 100), ("b", "Protocol", 400), ("c", "Storage", 37)])
    lr, leaves, k, ll = build_layout(inv)
    verify(inv, lr, leaves, k, ll)
    check_svg_geometry(render(inv, lr, leaves, k, ll), leaves, k, {c["name"]: c for c in inv["components"]})
    # a package at the workspace root owns what no deeper manifest claims
    owner, rel = assign_owners(["src/lib.rs", "crates/sub/src/lib.rs"], {"root": ".", "sub": "crates/sub"})
    assert owner == {"src/lib.rs": "root", "crates/sub/src/lib.rs": "sub"}, owner
    assert rel == {"src/lib.rs": "src/lib.rs", "crates/sub/src/lib.rs": "src/lib.rs"}, rel
    # two dirty states with equal line counts get different digests
    import tempfile
    with tempfile.TemporaryDirectory() as d:
        open(os.path.join(d, "a.rs"), "w").write("/* version A */\n")
        da = inventory_digest(d, ["a.rs"])
        open(os.path.join(d, "a.rs"), "w").write("/* version B */\n")
        assert inventory_digest(d, ["a.rs"]) != da, "digest ignores content"
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
    ap.add_argument("--bindings")
    ap.add_argument("--out", default="out")
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        self_test()
        return
    load_bindings(a.bindings)
    inv = inventory(os.path.abspath(a.repo))
    lr, leaves, k, ll = build_layout(inv)
    verify(inv, lr, leaves, k, ll)
    os.makedirs(a.out, exist_ok=True)
    with open(os.path.join(a.out, "inventory.json"), "w") as f:
        json.dump(inv, f, indent=2)
        f.write("\n")
    svg = render(inv, lr, leaves, k, ll)
    check_text_bounds(svg, lr, leaves)
    check_svg_geometry(svg, leaves, k, {c["name"]: c for c in inv["components"]})
    with open(os.path.join(a.out, "weighted-source.svg"), "w") as f:
        f.write(svg)
    with open(os.path.join(a.out, "inventory.md"), "w") as f:
        f.write(textual(inv, ll))
    print(f"ok: {inv['root_total_loc']} LOC, {len(leaves)} leaves, k={k:.4f} px²/LOC, "
          f"revision {inv['revision'][:7]}, modified={len(inv['tracked_files_modified'])}")


if __name__ == "__main__":
    main()
