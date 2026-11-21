## Catch-up: work on feature/export

**PR #14 (CSV export) is red: `test` fails on c452a18 ("Test quoted export fields").** `lint` passes. c452a18 and its parent 4ea4b98 were committed today, 2026-10-22, by agent sessions under your identity (Co-Authored-By: Claude), so you may not have seen them.

**Window:** 2026-10-17 16:49 UTC, the start of the last gap in this checkout's reflog (nothing between 10-17 16:49 and 10-22 17:19). That gives 9 commits on origin/main since 978d576.
**Branch:** even with origin/feature/export, nothing unpushed. 4 ahead of and 9 behind origin/main. One file modified and uncommitted: `src/export.rs` (comment changed from "csv, quoted" to "csv, quoted, bom"). One stash: `stash@{0}: On feature/export: wip: header row`.
**Other worktrees:** `../work-hotfix` on `hotfix/timeout` (at 978d576, the initial commit). This may be another session's checkout, so leave it alone.
**CI:** this branch (#14) has lint passing and test failing; it is BLOCKED and needs review. Your other open PR, #12 "Lock timeout retry" (hotfix/timeout), is a draft with `test` still in progress and no review requested.
**Instructions changed:** `CLAUDE.md`: run `./scripts/check.sh` before pushing, where it used to say `cargo test`. `scripts/check.sh` is new and runs `cargo clippy -- -D warnings && cargo test`. `.github/workflows/ci.yml` is new and runs CI on every push.
**Authors:** Ada Park (8), Sam Lee (1, an agent session)

### Highlights
- The `test` check on #14 fails at the head commit, which an agent session wrote today. Find out why before you add anything on top.
- `Config` was renamed to `Settings` and `timeout` to `timeout_ms` (bce581a, Ada). Your branch adds `delimiter` to the old `Config` in `src/config.rs`, so expect a conflict and a rename when you rebase.
- The push gate is now `./scripts/check.sh` (clippy with `-D warnings` plus tests). Before, it was only `cargo test`.
- "Retry ledger writes on lock timeout" (59ebe3c) landed on main from an agent session on 10-20. Draft PR #12 "Lock timeout retry" seems to cover the same work. Check whether #12 is still needed.

### By area
#### Config API (breaking)
- Rename `Config` to `Settings` and `timeout` to `timeout_ms`: bce581a (Ada Park)
#### JSON import (new feature)
- Add JSON import, and parse nested accounts: 8fa6c4b, 95a3126 (Ada Park)
#### Ledger
- Retry ledger writes on lock timeout (`src/retry.rs`): 59ebe3c (Sam Lee, agent session)
#### Gates and CI
- Require `scripts/check.sh` before pushing, and run CI on every push: 469b267, a7cc695 (Ada Park)

### Watch out for
- `src/config.rs` changed both on your branch and on main. It will conflict when you rebase, and the export code has to use `Settings`.
- The new clippy `-D warnings` gate applies to your next push.

### Housekeeping
- Import docs, the README update, and a nested-import test: 527e8a8, a7a0192, 21caf02 (Ada Park)

Next step: find out why `test` fails on c452a18, then rebase onto `origin/main`, where `src/config.rs` overlaps. Keep the uncommitted `src/export.rs` edit and the stash in mind while you rebase.
