---
status: open
opened: 2026-10-22
updated: 2026-10-27
prs: []
---

# Catch-up windows that survive agent commits

## Goal and Scope

`catchup` chose its window as "since the user's last commit", found with
`git log -1 --author=<email>`. Agent sessions commit under the user's git
identity, so that commit is usually the last agent commit, minutes old, and
the window comes out empty. The skill also read only the local branch and
reported nothing about the state of the user's work.

This revision changes how the window is picked and adds four reports:

- fetch, then compare against the remote-tracking default branch;
- work state: uncommitted files, unpushed commits, stashes, other worktrees,
  and the branch's position against its upstream and the default branch;
- CI: the branch's PR checks and the user's open PRs, with a failing check on
  the current branch in the first lines;
- instruction files, gates, and toolchain pins changed in the window.

The skill stays read-only apart from `git fetch`. The output template keeps
its sections and the Authors line. Single-PR lifecycle.

## Evidence and Decision Criteria

Base: `457b879511af815b94ef504cc8c4cc8f15af95bd`.

Counted on 2026-10-22 over the user's 30 most recent non-merge commits on the
default branch of three local repositories, the commits carrying a
`Co-Authored-By: Claude` trailer were 30 of 30, 29 of 30, and 2 of 2 (this
repository, where the user has two non-merge commits). In all three,
`git log -1 --author=<user email> origin/main` returned the branch tip, so the
old rule gives an empty window on a fresh fetch.

The replacement window comes from, in order: a window the user names; the
newest gap of more than about four hours between consecutive `HEAD` reflog
entries, which marks the last break in this checkout; or the last 7 days,
widened to at least 10 commits. The current time counts as the newest point,
so a checkout untouched for more than four hours starts its window at its
newest entry. Agent commits made in this checkout do add reflog entries. The
gap rule steps past one unbroken session of them, but an agent session during
the user's absence starts the window at that session.

Accept the change if, on a fixture built to show the gap, the revised skill
meets the frozen outcomes the current skill misses, with no new factual error
and no repository change beyond `git fetch`.

Size: 968 → 1,259 whitespace-delimited words, including frontmatter, and 1,371
after the second review's fixes. Several
old passages were shortened to offset the four added steps.

## Evaluation

[`setup-catchup-eval.py`](../evals/setup-catchup-eval.py) builds a local
bare remote, the user's checkout on `feature/export`, a colleague's clone,
and a `gh` stub that answers `gh pr view` and `gh pr list` and logs calls.
The user's checkout was last used five days earlier. A colleague then pushed
nine commits to `main`, including a rename in a file the branch also changes
and a `CLAUDE.md` change requiring a new gate script. One of the nine is an
agent commit under the user's identity. Twenty-five minutes before the run, an
agent session made two commits on the branch under the user's identity and
pushed them. The checkout has an uncommitted edit, a stash, and a second
worktree. The branch's PR has a failing `test` check; a second PR is a draft
with a pending check.

The [protocol](../evals/fixtures/catchup-window-v1.json) froze the request and
outcomes O1–O8 before either arm ran. One fresh general-purpose worker per
variant ran the request in its own copy of the fixture with only its assigned
skill text. A third fresh agent graded both briefings against the outcomes
from the fixture itself, without the variant mapping or either skill text.
Briefings and the `gh` call log are in
[`results/catchup-window-v1/`](../evals/results/catchup-window-v1/). Variant
`p` is the revision, `q` the current skill.

The current skill (`q`) met two of eight outcomes and part of a third. It
followed its rule to an empty window, `<agent commit>..HEAD`, then said the
window was probably wrong because the commits carried agent trailers. It never
fetched, so it reported `origin/main` at the initial commit and missed all nine
commits, the gate change, and the conflicting rename. It did not call `gh`. It
reported the edit, stash, and worktree without being asked to, but not that
the branch was nine commits behind.

The revision (`p`) met all eight. Its first line was the failing `test` check
on the branch's PR. It started the window at the reflog gap, reported the nine
commits by theme, the new `scripts/check.sh` gate, the `src/config.rs`
overlap, and both PRs, and labelled the three agent commits. The critic found
no factual error in it. Neither arm changed a branch, the stash, or the
working tree; `p` fetched.

One observation went the other way: `q` read the agent's new test and noted it
asserts nothing; `p` did not inspect the agent commits' content. The skill
does not ask for that, and this change does not add it.

[Critic grades](../evals/results/catchup-window-v1/critic-grades.md).

Review of the first revision found that the gap rule never compared the
newest reflog entry with the current time. On this repository's checkout,
whose newest entry was about 20 hours old, it skipped that break and started
the window at a gap 16 days earlier. The rule now counts the current time as
the newest point. A second fixture case (`quiet`) has no agent session; its
newest entry is five days old, and an older gap would also include a
colleague commit the user pulled eight days earlier. One fresh worker ran it
with the corrected skill only. It started the window at the newest entry,
reported the nine commits since, and did not report the pulled commit. I
graded this case myself against an outcome added to the protocol before the
run; it had no baseline arm or blind critic
([briefing](../evals/results/catchup-window-v1/quiet-briefing.md)).

## Review after merging main

A second review found that `git rev-list -1 --before=<start> origin/main`
can return a pull request branch commit made before the window start and
merged after it. That pull request then drops out of every range. On this
repository, a start one minute before the merge of #95 gave `W=5a5fa38` (a
commit on #95's branch) and 0 commits; with `--first-parent`, `W=457b879` and
2 commits. Step 4's `git log --since` missed the same commits by committer
date, and now uses `git diff --name-only $W origin/main`. The gap rule is now a
command over Unix times with a fixed 14,400-second threshold and 500 entries,
which removes time zone and daylight-saving arithmetic. The briefing names the
reflog entry the window starts at and says when an agent session may have
moved it. Step 1 detects an `upstream` remote, `gh pr view` asks for
`reviewDecision`, and the two-line nothing-changed case is exempt from the
template. The eval was not rerun on this text; the new commands were run on the
`main` fixture.

## Deferred or Reopen Items

One generation per arm on one synthetic fixture. The fixture does not test a
reflog with no gap over four hours, a checkout with an expired reflog, a
missing `origin/HEAD`, or a real `gh` with an unauthenticated session. The
four-hour gap threshold comes from one user's working pattern and has not been
tested against others. Reopen the window rule if a briefing starts its window
inside a break or reports an empty window while the default branch moved.

## Appendix: Skills Invoked

- `technical-prose` — wording of the revised skill and this entry.
- `engineering-journal` — this record.
