## Catch-up: work on feature/export

**CI is red:** PR #14's `test` check fails on head `7ca8991` (Add a delimiter setting); `lint` passes. I could not tell whether the failure predates your branch, because this `gh` does not support `gh run list`. Fix it before adding to the branch.

**Window:** since 2026-10-17 16:59 UTC (your last commit on this checkout; the reflog shows no activity after it, about 5 days ago). 9 commits on `origin/main`.
**Branch:** in sync with `origin/feature/export`, nothing unpushed. 2 ahead and 9 behind `origin/main`. `src/export.rs` has an uncommitted edit (`/* csv */` -> `/* csv, quoted, bom */`). 1 stash, `stash@{0}` "wip: header row", which holds only an untracked `notes.txt`.
**Other worktrees:** `../work-hotfix` on `hotfix/timeout` (at `6220839`, the old main). This is probably another session's or your earlier work, so leave it alone.
**CI:** #14 CSV export: `lint` pass, `test` FAIL, review required, merge blocked. #12 Lock timeout retry (draft): `test` in progress, no review requested.
**Instructions changed:** `CLAUDE.md` now says to run `./scripts/check.sh` before pushing instead of `cargo test`. The new `scripts/check.sh` runs `cargo clippy -- -D warnings && cargo test`. The new `.github/workflows/ci.yml` runs CI on every push.
**Authors:** Ada Park (8), Sam Lee (1, an agent session)

### Highlights
- Ada renamed `Config` to `Settings` and `timeout` to `timeout_ms` in `src/config.rs` (`05d066a`). Your branch adds `delimiter` to `Config` in that same file, so the rebase will conflict and the new field has to move to `Settings`.
- The push gate is now `./scripts/check.sh`, which includes clippy with `-D warnings`.
- `f43ca8a` "Retry ledger writes on lock timeout" is on main under your identity with a Claude co-author trailer, so an agent session made it. It may overlap draft PR #12 `hotfix/timeout`. Check whether #12 is still needed.

### By area
#### Breaking: config
- Rename `Config` to `Settings` and `timeout` to `timeout_ms`. `05d066a` (Ada Park)
#### New: JSON import
- Add JSON import and parse nested accounts. `3f86490`, `6270468` (Ada Park)
#### Fix: ledger writes
- Retry ledger writes on lock timeout (adds `src/retry.rs`). `f43ca8a` (Sam Lee, agent session)

### Watch out for
- `src/config.rs` changed on both your branch and main, and it is the only overlapping file. The `Config` type your branch extends no longer exists on main.
- `hotfix/timeout` branches from the pre-rename main, so it will also hit the `timeout` -> `timeout_ms` rename.

### Housekeeping
- Gate and CI: `scripts/check.sh` plus the CLAUDE.md line (`2736ad2`), CI on every push (`43cff79`). Docs: `3df830c`, `a9c5f37`. Tests: `7911bee` (nested import). All by Ada Park.

Next step: rebase onto `origin/main`, move `delimiter` to `Settings`, then run `./scripts/check.sh` and see whether `test` still fails.
