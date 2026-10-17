# cache-rs weighted source inventory

Revision `5cf04dd1b47e4bdda1a7a4a5872648b87743d42e` on `feature/ziplist-chain` (2026-10-16). Tracked files modified: 2. Untracked paths present: 0. Inventory digest `6114c1d21e1e734f…`.

**Scope.** all tracked *.rs files in Cargo workspace members, one owner each (longest manifest directory); not the code compiled into any one binary.

**Counting.** physical LOC: every line including comments and blanks; a final unterminated line counts as one line.

**Breakdown.** src/tests/benches/examples by target directory within the crate; inline #[cfg(test)] modules stay in src (inline mixed, not measured).

**Excluded.** tracked non-.rs files (md: 31, toml: 8, yml: 3, lock: 3, svg: 3, gitignore: 2, json: 1, yaml: 1); target/ and vendored dependencies (not tracked).

**Root total: 27,363 LOC.**

| Layer | LOC | Share |
| --- | ---: | ---: |
| Engine | 16,658 | 60.9% |
| Codec | 8,602 | 31.4% |
| Foundation | 2,103 | 7.7% |

| Crate | Layer | Path | Files | LOC | Share | src | tests | benches | examples |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| segcache | Engine | `crates/segcache` | 40 | 15,487 | 56.6% | 14,776 | 538 | 173 | 0 |
| ziplist | Codec | `crates/data-structure/ziplist` | 20 | 8,602 | 31.4% | 5,927 | 2,245 | 180 | 250 |
| keyvalue | Foundation | `crates/keyvalue` | 7 | 1,309 | 4.8% | 1,309 | 0 | 0 | 0 |
| cuckoo-cache | Engine | `crates/cuckoo` | 7 | 1,171 | 4.3% | 1,171 | 0 | 0 | 0 |
| datatier | Foundation | `crates/datatier` | 1 | 794 | 2.9% | 794 | 0 | 0 | 0 |
