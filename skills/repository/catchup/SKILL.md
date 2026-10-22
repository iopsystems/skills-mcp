---
name: catchup
description: Brief the user on a repository before work resumes — the current branch's state (uncommitted work, stashes, other worktrees, position against its upstream and the default branch), its CI checks and the user's open PRs, instruction files changed in the window, and what others landed, grouped by theme with risky changes first. Picks the window from a named time, the gap in the reflog, or the last 7 days, never from the user's last commit, because agent sessions commit under the user's identity. Trigger when the user says things like "what did I miss", "catch me up", "what's new on this branch", "what changed since I was out", "where was I", or after returning from time off.
---

# Catch Up

You are helping the user resume work in the current repository. Report the
state of their work first, then what changed around it, so they can resume
with context.

## Operating rules

- **Do not modify the repo.** This is a read-only briefing. `git fetch` is the
  one exception: it updates remote-tracking refs, not the working tree. Skip it
  if the user asks for an offline briefing, and say the comparison is as of the
  last fetch.
- Work from the current working directory. If the user names a different
  repo or path, `cd` into it (via a shell command you run) before starting.
- Use `--no-pager` or pipe through `cat` so output isn't paginated.
- Summarize; don't dump. A raw `git log` is not the answer — the user can
  run that themselves.
- If nothing changed and the branch is clean and green, say so in two lines
  and stop. Do not invent filler.

## Step 1 — Fetch and pick the window

```
git fetch --quiet --all --prune
git symbolic-ref --short refs/remotes/origin/HEAD \
  || git remote show origin | grep 'HEAD branch'   # the default branch
```

Compare against the remote-tracking branches; a local branch omits what
others pushed since it was last pulled. Below, `origin/main` means the default
branch found here. If the default branch lives on another remote (`upstream`
in a fork), use that remote. With no remote, compare against the local default
branch, skip Step 3, and say the briefing covers local history only.

Do not pick the window from the user's last commit. Agent sessions commit
under the user's git identity, so `git log -1 --author=<email>` usually returns
a commit minutes old and the window comes out empty.

Use the first that applies:

1. **The window the user named** ("since Monday", "since v0.4.0").
2. **The last break in this checkout.** `git reflog --date=iso -n 50` lists
   recent moves of `HEAD`. Include the current time as the newest point: if
   the newest entry is more than about four hours old, the window starts at
   that entry. Otherwise find the newest gap of more than about four hours
   between consecutive entries and start the window at the older entry.
   Entries minutes apart are one session, so the previous entry alone is not
   the boundary.
3. **The last 7 days** on `origin/main`, widened to at least 10 commits, when
   the reflog is empty, expired, or has no such gap. Start at the earlier of
   7 days ago and `git log -10 --format=%cI origin/main | tail -1`.

Record the window's start as a commit on the default branch:

```
W=$(git rev-list -1 --before='<window start>' origin/main)
```

State the window and why it was chosen at the top of the briefing.

## Step 2 — Where the work stands

```
git status --short --branch        # dirty files; ahead/behind its upstream
git rev-list --left-right --count origin/main...HEAD   # behind <tab> ahead
git log --oneline @{u}..HEAD 2>/dev/null   # unpushed commits
git stash list
git worktree list
```

Report each. Another worktree usually means another session is working here;
name its branch so work does not collide, and do not call it abandoned
without evidence.

## Step 3 — CI and open PRs

```
gh pr view --json number,state,mergeStateStatus,statusCheckRollup
gh pr list --author @me --state open \
  --json number,title,headRefName,isDraft,reviewDecision,statusCheckRollup
```

For this branch's PR and each of the user's open PRs, report checks as
passing, failing, pending, or absent, and whether a review is waiting. Never
omit a failing check. If this branch is red, name the check and commit in the
first lines of the briefing, and check whether the failure predates your work
before adding to the branch. If the branch has no PR, say so and report the
default branch's latest checks
(`gh run list --branch <default> --limit 5`). If `gh` is unavailable or
unauthenticated, say CI was not checked.

## Step 4 — Instruction files that changed

```
git log --since='<window start>' --name-only --format= origin/main -- \
  CLAUDE.md AGENTS.md ':(glob)**/CLAUDE.md' ':(glob)**/AGENTS.md' \
  .github/workflows rust-toolchain.toml rustfmt.toml | sort -u
```

Add the repository's own gate scripts and toolchain pins (for example a
`scripts/check.sh` or `.tool-versions`). A changed instruction file, gate, or
toolchain pin changes how the next piece of work must be done. List each with
a one-line summary from `git diff $W origin/main -- <file>`, and read the new
text before resuming work.

## Step 5 — What landed

```
git log --no-merges --date=short \
  --pretty=format:'%h%x09%an%x09%ad%x09%s' $W..origin/main
git shortlog -sn --no-merges $W..origin/main
git diff --stat $W origin/main | tail -20
```

List commits individually if there are fewer than about eight. Otherwise
group by theme (infer from paths, subjects, and bodies), not by date. For each
group, give the intent in one line, one to three short SHAs, and the authors.
Order the groups:

1. Breaking changes / API changes / schema, config, or migration changes.
2. New features or new public surfaces.
3. Refactors that touch many files.
4. Bug fixes that matter to the user's current work.
5. Test, CI, docs, and chore work — last, condensed.

Call out authors by name. Commits under the user's identity with an agent
co-author trailer came from agent sessions; label them so, since the user may
not have seen them.

Then cross the branch's changed files with what landed:

```
comm -12 <(git diff --name-only origin/main...HEAD | sort) \
         <(git diff --name-only $W origin/main | sort)
```

Files in both are the likely merge conflicts.

## Step 6 — Write the briefing

Keep it under about 40 lines unless the range is unusually large.

```
## Catch-up: <repo> on <branch>

**Window:** <start> (<why>), <N commits> on <default branch>.
**Branch:** <ahead/behind upstream and default>; <dirty, stashes, unpushed>.
**Other worktrees:** <path: branch>, ... or none.
**CI:** <this branch's checks>; open PRs: #n <state>, ...
**Instructions changed:** <file: one line>, ... or none.
**Authors:** <name (n)>, <name (n)>, ...

### Highlights
- <1–5 bullets: the things the user most needs to know>

### By area
#### <Area 1>
- <one-line intent> — <short sha>, <short sha> (<author>)

### Watch out for
- <breaking changes, renames, removed APIs, files that overlap this branch>
  Omit this section if there's nothing to flag.

### Housekeeping
- <one short bullet rolling up docs/tests/CI/chore commits, if any>
```

Mark every header line "none" rather than dropping it. End with one concrete
next step only if the state calls for one — e.g. "Fix the failing `lint`
check before adding to this branch," or "Rebase onto `origin/main`; three
files overlap."

## After the briefing

Do not start the next piece of work inside this skill. Offer, but do not
perform without being asked:

- Show the full `git log` or diff for a specific area.
- Drill into a specific commit (`git show <sha>`).
- Compare against a different base or window.
