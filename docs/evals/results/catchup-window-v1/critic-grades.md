# Blind grades, catchup-window-v1

Graded by a fresh agent against the frozen outcomes, from the fixtures, without
the variant mapping or either skill text. p is the revision; q is the skill at
457b879.

| Outcome | p | q |
|---|---|---|
| O1 window | pass: starts 2026-10-17 16:49 UTC at the reflog gap; 9 commits | fail: `662345f..HEAD`, the agent commit 25 minutes old, 0 commits (flagged as "probably wrong") |
| O2 upstream | pass: fetched; all 9 commits on origin/main reported | fail: no fetch; none of the 9 commits mentioned |
| O3 work state | pass: edit, stash, worktree, 4 ahead / 9 behind origin/main | partial: edit, stash, worktree; not that the branch is 9 behind |
| O4 CI | pass: #14 `test` failing in the first line; #12 draft, pending | fail: gh never called; CI not mentioned |
| O5 instructions | pass: CLAUDE.md now requires `./scripts/check.sh` | fail: not mentioned |
| O6 overlap | pass: `src/config.rs` rename collides with the branch | fail: not mentioned |
| O7 read-only | pass: only remote-tracking refs moved (fetch) | pass: nothing changed |
| O8 agent commits | pass: both branch commits and 59ebe3c on main | pass: both branch commits; notes the new test asserts nothing |

Factual errors: p none found. q states `origin/main` is at the initial commit
and reports 0 commits in the window; both come from not fetching.
