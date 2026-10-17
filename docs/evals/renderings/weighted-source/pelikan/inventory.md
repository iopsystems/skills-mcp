# Pelikan weighted source inventory

Revision `46b98c9dc6fb70848808b903ba1d60a9655150be` on `spec/ziplist-collections` (2026-10-16). Tracked files modified: 0. Untracked paths present: 5. Inventory digest `658b74baa86ccb01…`.

**Scope.** all tracked *.rs files in Cargo workspace members, one owner each (longest manifest directory); not the code compiled into any one binary.

**Counting.** physical LOC: every line including comments and blanks; a final unterminated line counts as one line.

**Breakdown.** verification = files under tests/, benches/, examples/, fuzz/ (kind by directory); files declared by `mod x;` under #[cfg(test)] or #[cfg(kani)] and everything beneath them; and, inside any other file, items carrying #[cfg(test)], #[cfg(kani)], #[test], or #[kani::proof] (an any()/all() predicate containing test or kani counts; a file-level #![cfg(test)] makes the whole file verification), measured by a brace scanner that honors comments and literals (rustscan.py). cfg(not(test)) is production. Unclassifiable lines count as production..

**Excluded.** tracked non-.rs files (toml: 37, md: 30, gitignore: 6, sh: 4, yml: 4, dict: 4, json: 3, svg: 3); target/ and vendored dependencies (not tracked).

**Root total: 31,551 LOC.**

| Layer | LOC | Share |
| --- | ---: | ---: |
| Product | 1,646 | 5.2% |
| Core | 5,486 | 17.4% |
| Protocol | 17,007 | 53.9% |
| Storage | 1,028 | 3.3% |
| Foundation | 4,110 | 13.0% |
| Tooling | 2,274 | 7.2% |

**Verification source: 5,710 LOC, 18.1%** (tests, benches, examples, fuzz, proofs; see breakdown policy).

| Crate | Layer | Path | Files | LOC | Share | production | test | bench | example | fuzz | proof |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| protocol-resp | Protocol | `src/protocol/resp` | 56 | 8,191 | 26.0% | 5,954 | 2,216 | 0 | 0 | 21 | 0 |
| protocol-memcache | Protocol | `src/protocol/memcache` | 76 | 6,963 | 22.1% | 5,601 | 1,105 | 74 | 0 | 183 | 0 |
| config | Foundation | `src/config` | 22 | 2,300 | 7.3% | 2,209 | 91 | 0 | 0 | 0 | 0 |
| xtask | Tooling | `xtask` | 6 | 2,274 | 7.2% | 2,274 | 0 | 0 | 0 | 0 | 0 |
| server | Core | `src/core/server` | 7 | 1,606 | 5.1% | 1,606 | 0 | 0 | 0 | 0 | 0 |
| proxy | Core | `src/core/proxy` | 5 | 1,418 | 4.5% | 1,418 | 0 | 0 | 0 | 0 | 0 |
| pelikan-net | Foundation | `src/net` | 9 | 1,372 | 4.3% | 1,294 | 78 | 0 | 0 | 0 | 0 |
| session | Core | `src/session` | 4 | 1,265 | 4.0% | 1,054 | 211 | 0 | 0 | 0 | 0 |
| pelikan-segcache | Product | `src/server/segcache` | 6 | 794 | 2.5% | 198 | 500 | 96 | 0 | 0 | 0 |
| admin | Core | `src/core/admin` | 1 | 775 | 2.5% | 775 | 0 | 0 | 0 | 0 | 0 |
| protocol-http | Protocol | `src/protocol/http` | 7 | 714 | 2.3% | 549 | 165 | 0 | 0 | 0 | 0 |
| entrystore | Storage | `src/entrystore` | 6 | 587 | 1.9% | 587 | 0 | 0 | 0 | 0 | 0 |
| protocol-ping | Protocol | `src/protocol/ping` | 16 | 579 | 1.8% | 408 | 120 | 35 | 0 | 16 | 0 |
| pelikan-rds | Product | `src/server/rds` | 6 | 565 | 1.8% | 193 | 269 | 103 | 0 | 0 | 0 |
| protocol-admin | Protocol | `src/protocol/admin` | 5 | 489 | 1.5% | 340 | 99 | 34 | 0 | 16 | 0 |
| queues | Core | `src/queues` | 1 | 422 | 1.3% | 282 | 140 | 0 | 0 | 0 | 0 |
| bloom | Storage | `src/storage/bloom` | 2 | 317 | 1.0% | 209 | 57 | 51 | 0 | 0 | 0 |
| common | Foundation | `src/common` | 7 | 242 | 0.8% | 212 | 30 | 0 | 0 | 0 | 0 |
| logger | Foundation | `src/logger` | 1 | 196 | 0.6% | 196 | 0 | 0 | 0 | 0 | 0 |
| pelikan-pingproxy | Product | `src/proxy/ping` | 2 | 171 | 0.5% | 171 | 0 | 0 | 0 | 0 | 0 |
| storage-types | Storage | `src/storage/types` | 1 | 124 | 0.4% | 124 | 0 | 0 | 0 | 0 | 0 |
| pelikan-pingserver | Product | `src/server/pingserver` | 1 | 116 | 0.4% | 116 | 0 | 0 | 0 | 0 | 0 |
| protocol-common | Protocol | `src/protocol/common` | 1 | 71 | 0.2% | 71 | 0 | 0 | 0 | 0 | 0 |
