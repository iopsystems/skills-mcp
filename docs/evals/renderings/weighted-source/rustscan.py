"""Measure verification source inside Rust files without guessing from names.

A line is verification when it lies inside an item carrying a `#[cfg(test)]`,
`#[cfg(kani)]`, `#[test]`, or `#[kani::proof]` attribute, or inside a file
declared from such an item (`#[cfg(test)] mod tests;`). Item extents come from
a brace scanner that honors line and block comments, string, raw-string, byte,
and char literals, and lifetimes. `#[cfg(not(test))]` and friends are
production. Anything the scanner cannot classify is production, which errs
toward undercounting verification rather than inventing it.
"""
import os
import re

ATTR_RE = re.compile(r"#!?\[")
ITEM_HEAD_RE = re.compile(r"\s*(?:pub(?:\([^)]*\))?\s+)?(?:(?:unsafe|async|const|extern(?:\s+\"[^\"]*\")?)\s+)*"
                          r"(mod|fn|impl|struct|enum|trait|union|static|const|type|macro_rules!)\b")


def _skip_block_comment(src, i):
    depth = 1
    i += 2
    while i < len(src) and depth:
        if src.startswith("/*", i):
            depth += 1
            i += 2
        elif src.startswith("*/", i):
            depth -= 1
            i += 2
        else:
            i += 1
    return i


def _skip_string(src, i):
    i += 1
    while i < len(src):
        c = src[i]
        if c == "\\":
            i += 2
            continue
        if c == '"':
            return i + 1
        i += 1
    return i


def _skip_raw_string(src, i):
    # at 'r' or 'br'; count hashes
    j = i + (2 if src[i] == "b" else 1)
    hashes = 0
    while j < len(src) and src[j] == "#":
        hashes += 1
        j += 1
    if j >= len(src) or src[j] != '"':
        return i + 1
    end = src.find('"' + "#" * hashes, j + 1)
    return len(src) if end < 0 else end + 1 + hashes


def tokens(src):
    """Yield (index, kind) for structural characters: '{', '}', '#', ';' and
    newline markers, with comments and literals skipped."""
    i, n = 0, len(src)
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            i = n if j < 0 else j
        elif src.startswith("/*", i):
            i = _skip_block_comment(src, i)
        elif c == '"':
            i = _skip_string(src, i)
        elif (c == "r" and i + 1 < n and src[i + 1] in '#"') or (src.startswith("br", i) and i + 2 < n and src[i + 2] in '#"'):
            if i > 0 and (src[i - 1].isalnum() or src[i - 1] == "_"):
                i += 1
            else:
                i = _skip_raw_string(src, i)
        elif c == "b" and i + 1 < n and src[i + 1] == '"' and not (i > 0 and (src[i - 1].isalnum() or src[i - 1] == "_")):
            i = _skip_string(src, i + 1)
        elif c == "'":
            if i + 1 < n and src[i + 1] == "\\":
                j = src.find("'", i + 2)
                i = n if j < 0 else j + 1
            elif i + 2 < n and src[i + 2] == "'":
                i += 3
            else:
                i += 1  # lifetime or label
        elif c in "{};#":
            yield i, c
            i += 1
        else:
            i += 1


def _attr_is_verification(text):
    """None, 'test', or 'proof'."""
    t = re.sub(r"\s+", "", text)
    if t.startswith("#[test]"):
        return "test"
    if t.startswith("#[kani::proof"):
        return "proof"
    m = re.match(r"#!?\[cfg\((.*)\)\]$", t)
    if not m:
        return None
    expr = m.group(1)
    # strip not(...) groups so `cfg(not(test))` does not count
    while True:
        new = re.sub(r"not\((?:[^()]|\([^()]*\))*\)", "", expr)
        if new == expr:
            break
        expr = new
    if re.search(r"\bkani\b", expr):
        return "proof"
    if re.search(r"\btest\b", expr):
        return "test"
    return None


def scan(src):
    """Return (verification_line_ranges, declared_verification_mods).
    Ranges are (start_line, end_line) inclusive, 1-based; mods are names
    declared as `mod name;` under a verification attribute."""
    ranges = []
    mods = []
    n = len(src)
    line_of = [0] * (n + 1)
    ln = 1
    for i, c in enumerate(src):
        line_of[i] = ln
        if c == "\n":
            ln += 1
    line_of[n] = ln

    toks = list(tokens(src))
    depth = 0
    pending = None  # (attr_start_index, is_verification) for the item that follows
    k = 0
    while k < len(toks):
        i, c = toks[k]
        if c == "#" and src.startswith("#[", i) or src.startswith("#![", i):
            # find the matching ']' of the attribute (brackets inside are rare but balanced)
            j = i
            bal = 0
            while j < n:
                if src[j] == "[":
                    bal += 1
                elif src[j] == "]":
                    bal -= 1
                    if bal == 0:
                        break
                j += 1
            attr = src[i:j + 1]
            ver = _attr_is_verification(attr)
            if attr.startswith("#![") and depth == 0 and ver:
                return [(1, count_lines(src), ver)], []
            if pending is None:
                pending = [i, ver]
            elif ver and (pending[1] is None or ver == "proof"):
                pending[1] = ver
            # skip tokens inside the attribute
            while k < len(toks) and toks[k][0] <= j:
                k += 1
            continue
        if c == "{":
            if pending is not None and pending[1]:
                head = src[pending[0]:i]
                # the item this attribute group decorates must start with an item keyword
                tail = head[head.rfind("]") + 1:]
                if ITEM_HEAD_RE.match(tail):
                    # match braces
                    d = 1
                    kk = k + 1
                    while kk < len(toks) and d:
                        if toks[kk][1] == "{":
                            d += 1
                        elif toks[kk][1] == "}":
                            d -= 1
                        kk += 1
                    end_i = toks[kk - 1][0] if d == 0 else n
                    ranges.append((line_of[pending[0]], line_of[end_i], pending[1]))
                    k = kk
                    pending = None
                    continue
            pending = None
            depth += 1
        elif c == "}":
            depth -= 1
            pending = None
        elif c == ";":
            if pending is not None:
                tail = src[pending[0]:i]
                tail = tail[tail.rfind("]") + 1:]
                m = re.match(r"\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*$", tail)
                if m and pending[1]:
                    mods.append((m.group(1), pending[1]))
                    ranges.append((line_of[pending[0]], line_of[i], pending[1]))
            pending = None
        k += 1
    return ranges, mods


def count_lines(src):
    if not src:
        return 0
    return src.count("\n") + (0 if src.endswith("\n") else 1)


def verification_lines(src):
    """Return ({kind: line_count}, [(mod_name, kind)]); a line inside nested
    verification items is counted once, under the outermost kind."""
    ranges, mods = scan(src)
    covered = {}
    for a, b, kind in sorted(ranges):
        for ln in range(a, b + 1):
            covered.setdefault(ln, kind)
    out = {}
    for kind in covered.values():
        out[kind] = out.get(kind, 0) + 1
    return out, mods


def child_file(declaring, name):
    """Resolve `mod name;` declared in `declaring` to a file path, 2018 rules."""
    d = os.path.dirname(declaring)
    base = os.path.basename(declaring)
    if base in ("lib.rs", "main.rs", "mod.rs"):
        root = d
    else:
        root = os.path.join(d, base[:-3])
    for cand in (os.path.join(root, name + ".rs"), os.path.join(root, name, "mod.rs")):
        if os.path.exists(cand):
            return cand
    return None


def self_test():
    src = '''
// a comment with { braces } and "quotes"
pub fn prod() { let s = "}"; let c = '{'; let r = r#"}"#; let l: &'static str = "x"; }
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn t() { assert!(true); }
}
#[cfg(not(test))]
fn also_prod() { }
#[cfg(any(test, feature = "x"))]
fn gated() { }
#[cfg(test)]
mod more;
#[kani::proof]
fn proof() { let _ = 1; }
#[test]
fn bare() { }
'''
    kinds, mods = verification_lines(src)
    assert kinds == {"test": 5 + 2 + 2 + 2, "proof": 2}, kinds  # tests mod, gated, mod more, bare; proof
    assert mods == [("more", "test")], mods
    kinds, _ = verification_lines("#![cfg(test)]\nfn a() {}\nfn b() {}\n")
    assert kinds == {"test": 3}, kinds
    print("rustscan self-test ok")


if __name__ == "__main__":
    self_test()
