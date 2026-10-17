# cache-rs weighted source inventory

Revision `5cf04dd1b47e4bdda1a7a4a5872648b87743d42e` on `feature/ziplist-chain` (2026-10-16). Tracked files modified: 2. Untracked paths present: 0. Inventory digest `6114c1d21e1e734f…`.

**Scope.** all tracked *.rs files in Cargo workspace members, one owner each (longest manifest directory); not the code compiled into any one binary.

**Counting.** physical LOC: every line including comments and blanks; a final unterminated line counts as one line.

**Breakdown.** verification = files under tests/, benches/, examples/, fuzz/ (kind by directory); files declared by `mod x;` under #[cfg(test)] or #[cfg(kani)] and everything beneath them; and, inside any other file, items carrying #[cfg(test)], #[cfg(kani)], #[test], or #[kani::proof] (an any()/all() predicate containing test or kani counts; a file-level #![cfg(test)] makes the whole file verification), measured by a brace scanner that honors comments and literals (rustscan.py). cfg(not(test)) is production. Unclassifiable lines count as production..

**Excluded.** tracked non-.rs files (md: 31, toml: 8, yml: 3, lock: 3, svg: 3, gitignore: 2, json: 1, yaml: 1); target/ and vendored dependencies (not tracked).

**Root total: 27,363 LOC.**

| Layer | LOC | Share |
| --- | ---: | ---: |
| Engine | 16,658 | 60.9% |
| Codec | 8,602 | 31.4% |
| Foundation | 2,103 | 7.7% |

**Verification source: 13,082 LOC, 47.8%** (tests, benches, examples, fuzz, proofs; see breakdown policy).

| Crate | Layer | Path | Files | LOC | Share | production | test | bench | example | fuzz | proof |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| segcache | Engine | `crates/segcache` | 40 | 15,487 | 56.6% | 8,010 | 6,971 | 173 | 0 | 333 | 0 |
| ziplist | Codec | `crates/data-structure/ziplist` | 20 | 8,602 | 31.4% | 3,612 | 3,818 | 180 | 250 | 479 | 263 |
| keyvalue | Foundation | `crates/keyvalue` | 7 | 1,309 | 4.8% | 1,089 | 220 | 0 | 0 | 0 | 0 |
| cuckoo-cache | Engine | `crates/cuckoo` | 7 | 1,171 | 4.3% | 892 | 279 | 0 | 0 | 0 | 0 |
| datatier | Foundation | `crates/datatier` | 1 | 794 | 2.9% | 678 | 116 | 0 | 0 | 0 | 0 |
