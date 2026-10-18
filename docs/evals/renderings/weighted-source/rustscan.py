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
                          r"(mod|fn|impl|struct|enum|trait|union|static|const|type|use|crate|macro_rules!)\b")
MOD_HEAD_RE = re.compile(r"\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*$")
# b"..", c"..", r".." / r#".."#, br".." , cr".." — any literal prefix Rust accepts
LIT_PREFIX_RE = re.compile(r'(?P<raw>br|cr|r)(?P<hashes>#*)"|(?P<plain>[bc])"')


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


def _skip_raw_string(src, i, m):
    """`m` is a LIT_PREFIX_RE match at `i` with the raw group set."""
    hashes = len(m.group("hashes"))
    j = m.end()  # just past the opening quote
    end = src.find('"' + "#" * hashes, j)
    return len(src) if end < 0 else end + 1 + hashes


def tokens(src):
    """Yield (index, kind) for structural characters: '{', '}', '#', and ';'.
    Comments and literals are skipped. A ';' is yielded only outside
    parentheses and square brackets, so `[u8; 2]` and `for<'a>` bounds never
    look like the end of an item."""
    i, n = 0, len(src)
    nest = 0  # () and [] depth
    while i < n:
        c = src[i]
        if src.startswith("//", i):
            j = src.find("\n", i)
            i = n if j < 0 else j
        elif src.startswith("/*", i):
            i = _skip_block_comment(src, i)
        elif c == '"':
            i = _skip_string(src, i)
        elif c in "rbc" and not (i > 0 and (src[i - 1].isalnum() or src[i - 1] == "_")) and LIT_PREFIX_RE.match(src, i):
            m = LIT_PREFIX_RE.match(src, i)
            i = _skip_raw_string(src, i, m) if m.group("raw") is not None else _skip_string(src, m.end() - 1)
        elif c == "'":
            if i + 1 < n and src[i + 1] == "\\":
                j = src.find("'", i + 2)
                i = n if j < 0 else j + 1
            elif i + 2 < n and src[i + 2] == "'":
                i += 3
            else:
                i += 1  # lifetime or label
        elif c in "([":
            nest += 1
            i += 1
        elif c in ")]":
            nest = max(0, nest - 1)
            i += 1
        elif c in "{}#" or (c == ";" and nest == 0):
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
    # a quoted value such as feature = "test-support" is not a predicate
    expr = re.sub(r'"(?:[^"\\]|\\.)*"', '""', expr)
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
    Ranges are (start_line, end_line, kind) inclusive, 1-based. Mods are
    (path, kind) for every `mod name;` that is verification, either by its own
    attribute or by an enclosing verification item; `path` is the module path
    below the file (`tests/support` for `mod tests { mod support; }`), ready
    for `child_file`."""
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
    # One frame per open brace: (kind, start_index, inline_mod_name). kind is
    # set when the brace opens a verification item; inline_mod_name when it
    # opens `mod name {`, verification or not, so nested declarations resolve.
    stack = []
    pending = None  # [attr_start_index, kind, attr_end_index] for the item that follows
    seg_start = 0   # text since the last structural token or attribute
    k = 0

    def enclosing_kind():
        for kind, _, _ in reversed(stack):
            if kind:
                return kind
        return None

    while k < len(toks):
        i, c = toks[k]
        if c == "#" and (src.startswith("#[", i) or src.startswith("#![", i)):
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
            if attr.startswith("#![") and not stack and ver:
                return [(1, count_lines(src), ver)], []
            if pending is None:
                pending = [i, ver, j]
            else:
                pending[2] = j
                if ver and (pending[1] is None or ver == "proof"):
                    pending[1] = ver
            seg_start = j + 1
            while k < len(toks) and toks[k][0] <= j:
                k += 1
            continue
        tail = src[seg_start:i]
        if c == "{":
            kind = None
            start = i
            if pending is not None and pending[1] and ITEM_HEAD_RE.match(tail):
                kind, start = pending[1], pending[0]
            m = MOD_HEAD_RE.match(tail)
            stack.append((kind, start, m.group(1) if m else None))
            pending = None
        elif c == "}":
            if stack:
                kind, start, _ = stack.pop()
                if kind:
                    ranges.append((line_of[start], line_of[i], kind))
            pending = None
        elif c == ";":
            m = MOD_HEAD_RE.match(tail)
            kind = (pending[1] if pending is not None else None) or enclosing_kind()
            if m and kind:
                path = "/".join([f[2] for f in stack if f[2]] + [m.group(1)])
                mods.append((path, kind))
            if pending is not None and pending[1] and ITEM_HEAD_RE.match(tail):
                ranges.append((line_of[pending[0]], line_of[i], pending[1]))
            pending = None
        seg_start = i + 1
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

    # Finding 1: a feature name that contains "test" or "kani" is a value, not a predicate.
    src = '''#[cfg(feature = "test-support")]
pub fn helper() {
    let _ = 1;
}
#[cfg(feature = "kani-shim")]
pub fn shim() {}
#[cfg(all(test, feature = "test-support"))]
fn real_test() {}
'''
    kinds, _ = verification_lines(src)
    assert kinds == {"test": 2}, kinds

    # Finding 2: declarations inside an inline verification module inherit its
    # kind and resolve below the inline path; an inline production module
    # still prefixes the path of a test module declared inside it.
    src = '''#[cfg(test)]
mod tests {
    mod support;
    #[test] fn t() {}
}
mod inner {
    #[cfg(test)]
    mod tests;
}
'''
    kinds, mods = verification_lines(src)
    assert kinds == {"test": 7}, kinds  # the inline tests module (5) and the mod decl with attribute (2)
    assert mods == [("tests/support", "test"), ("inner/tests", "test")], mods

    # Finding 3: a `;` inside brackets does not end the item; a `;`-terminated
    # attributed item is counted.
    src = '''#[cfg(test)]
fn helper() -> [u8; 2] {
    [1, 2]
}
#[cfg(test)]
const EXPECTED: usize = 2;
#[cfg(test)]
use std::collections::HashMap;
fn prod() -> [u8; 1] { [0] }
'''
    kinds, _ = verification_lines(src)
    assert kinds == {"test": 4 + 2 + 2}, kinds

    # Finding 4: C and raw C string literals are data, as b"" and r"" already were.
    src = '''pub const A: &core::ffi::CStr = c"hi\\"";
pub const B: &core::ffi::CStr = cr#"quote " then
#[test]
fn not_code() {}"#;
pub const C: &[u8] = br"x";
fn prod() {}
'''
    kinds, _ = verification_lines(src)
    assert kinds == {}, kinds
    print("rustscan self-test ok")


if __name__ == "__main__":
    self_test()
