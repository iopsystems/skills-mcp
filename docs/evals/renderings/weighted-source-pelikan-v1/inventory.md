# Pelikan weighted source inventory

Revision `46b98c9dc6fb70848808b903ba1d60a9655150be` (2026-10-16). Tracked files modified: 0. Untracked paths present: 5. Inventory digest `1120183e5538e945…`.

**Scope.** all tracked *.rs files in Cargo workspace members, one owner each (longest manifest directory); not the code compiled into any one binary.

**Counting.** physical LOC: every line including comments and blanks; a final unterminated line counts as one line.

**Breakdown.** src/tests/benches by target directory within the crate; inline #[cfg(test)] modules stay in src (inline mixed, not measured).

**Excluded.** non-.rs files (toml, md, yml, sh, py, json, dict); resp-proto-kani/ (present on disk, not tracked by git); target/ and vendored dependencies (not tracked).

**Root total: 31,551 LOC.**

| Layer | LOC | Share |
| --- | ---: | ---: |
| Product | 1,646 | 5.2% |
| Core | 5,486 | 17.4% |
| Protocol | 17,007 | 53.9% |
| Storage | 1,028 | 3.3% |
| Foundation | 4,110 | 13.0% |
| Tooling | 2,274 | 7.2% |

| Crate | Layer | Path | Files | LOC | Share | src | tests | benches |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| protocol-resp | Protocol | `src/protocol/resp` | 56 | 8,191 | 26.0% | 8,191 | 0 | 0 |
| protocol-memcache | Protocol | `src/protocol/memcache` | 76 | 6,963 | 22.1% | 6,889 | 0 | 74 |
| config | Foundation | `src/config` | 22 | 2,300 | 7.3% | 2,300 | 0 | 0 |
| xtask | Tooling | `xtask` | 6 | 2,274 | 7.2% | 2,274 | 0 | 0 |
| server | Core | `src/core/server` | 7 | 1,606 | 5.1% | 1,606 | 0 | 0 |
| proxy | Core | `src/core/proxy` | 5 | 1,418 | 4.5% | 1,418 | 0 | 0 |
| pelikan-net | Foundation | `src/net` | 9 | 1,372 | 4.3% | 1,372 | 0 | 0 |
| session | Core | `src/session` | 4 | 1,265 | 4.0% | 1,265 | 0 | 0 |
| pelikan-segcache | Product | `src/server/segcache` | 6 | 794 | 2.5% | 198 | 500 | 96 |
| admin | Core | `src/core/admin` | 1 | 775 | 2.5% | 775 | 0 | 0 |
| protocol-http | Protocol | `src/protocol/http` | 7 | 714 | 2.3% | 549 | 165 | 0 |
| entrystore | Storage | `src/entrystore` | 6 | 587 | 1.9% | 587 | 0 | 0 |
| protocol-ping | Protocol | `src/protocol/ping` | 16 | 579 | 1.8% | 544 | 0 | 35 |
| pelikan-rds | Product | `src/server/rds` | 6 | 565 | 1.8% | 193 | 269 | 103 |
| protocol-admin | Protocol | `src/protocol/admin` | 5 | 489 | 1.5% | 455 | 0 | 34 |
| queues | Core | `src/queues` | 1 | 422 | 1.3% | 422 | 0 | 0 |
| bloom | Storage | `src/storage/bloom` | 2 | 317 | 1.0% | 266 | 0 | 51 |
| common | Foundation | `src/common` | 7 | 242 | 0.8% | 242 | 0 | 0 |
| logger | Foundation | `src/logger` | 1 | 196 | 0.6% | 196 | 0 | 0 |
| pelikan-pingproxy | Product | `src/proxy/ping` | 2 | 171 | 0.5% | 171 | 0 | 0 |
| storage-types | Storage | `src/storage/types` | 1 | 124 | 0.4% | 124 | 0 | 0 |
| pelikan-pingserver | Product | `src/server/pingserver` | 1 | 116 | 0.4% | 116 | 0 | 0 |
| protocol-common | Protocol | `src/protocol/common` | 1 | 71 | 0.2% | 71 | 0 | 0 |
