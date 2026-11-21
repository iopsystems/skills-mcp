---
name: catchup
description: Brief the user on a repository before work resumes — the current branch's state (uncommitted work, stashes, other worktrees, position against its upstream and the default branch), its CI checks and the user's open PRs, instruction files changed in the window, and what others landed, grouped by theme with risky changes first. Picks the window from a named time, the gap in the reflog, or the last 7 days, never from the user's last commit. Trigger when the user says things like "what did I miss", "catch me up", "what's new on this branch", "what changed since I was out", "where was I", or after returning from time off.
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
  and stop, without the Step 6 template. Do not invent filler.

## Step 1 — Fetch and pick the window

```
git fetch --quiet --all --prune
r=origin; git remote | grep -qx upstream && r=upstream   # a fork's upstream
git symbolic-ref --short refs/remotes/$r/HEAD 2>/dev/null \
  || git remote show $r | sed -n "s|.*HEAD branch: |$r/|p"   # default branch
```

Compare against the remote-tracking branches; a local branch omits what
others pushed since it was last pulled. Below, `origin/main` means the default
branch found here, which is `upstream`'s in a fork with an `upstream` remote.
With no remote, compare against the local default branch, skip Step 3, and say
the briefing covers local history only.

Do not pick the window from the user's last commit: agent sessions commit
under the user's git identity, so that commit is often minutes old.

Use the first that applies:

1. **The window the user named** ("since Monday", "since v0.4.0").
2. **The last break in this checkout.** Find the newest gap of more than four
   hours (14,400 seconds) in `HEAD`'s reflog, counting the current time as the
   newest point, and start the window at the older entry of the gap. This
   prints that entry's Unix time, or nothing when there is no such gap:

   ```
   git reflog --date=unix -n 500 --format='%gd' | sed 's/.*{\(.*\)}/\1/' \
     | awk -v now=$(date +%s) 'BEGIN{p=now} p-$1>14400{print $1; exit} {p=$1}'
   ```

   Unix times avoid time zone and daylight-saving arithmetic. Name the entry
   the window starts at, with its time and message, in the Window line. An
   agent session in this checkout while the user was away also moves `HEAD`,
   and the window then starts at the end of that session. If the entry the
   window starts at, or the entries before it back to the previous gap, come
   from such a session (commits with an agent co-author trailer, or moves the
   user does not recognise), say so and offer to start the window at the
   previous gap instead.
3. **The last 7 days** on `origin/main`, widened to at least 10 commits, when
   the reflog is empty, expired, or has no such gap. Start at the earlier of
   7 days ago and `git log -11 --format=%cI origin/main | tail -1`; `W` below
   is excluded from the range, so the eleventh commit leaves ten in it.

Record the window's start as a commit on the default branch:

```
W=$(git rev-list -1 --first-parent --before='<window start>' origin/main)
```

`--first-parent` keeps `W` on the default branch's own history. Without it, a
pull request branch commit made before the window start and merged after it
can become `W`, and that pull request drops out of every range below. For a
Unix time from the reflog, write the start as `@<seconds>`. If `W` is empty, the
window predates the default branch's first commit, and `$W..origin/main` would
silently mean `HEAD..origin/main`. Use the full history instead: set
`W=$(git rev-list --max-parents=0 --first-parent origin/main | tail -1)`, and
say the window covers all of it (the root commit itself is then not listed).

State the window and why it was chosen at the top of the briefing.

## Step 2 — Where the work stands

```
git status --short --branch        # dirty files; ahead/behind its upstream
git rev-list --left-right --count origin/main...HEAD   # behind <tab> ahead
git log --oneline @{u}..HEAD 2>/dev/null   # unpushed commits
git stash list
git worktree list
```

Report each. Another worktree may mean another session is working here;
name its branch so work does not collide, and do not call it abandoned
without evidence.

## Step 3 — CI and open PRs

```
gh pr view --json number,state,mergeStateStatus,reviewDecision,statusCheckRollup
gh pr list --author @me --state open \
  --json number,title,headRefName,isDraft,reviewDecision,statusCheckRollup
```

For this branch's PR and each of the user's open PRs, report checks as
passing, failing, pending, or absent, and whether a review is waiting. Never
omit a failing check. If this branch is red, name the check and commit in the
first lines of the briefing, and check whether the failure predates the user's
latest commits on the branch before adding to it. If the branch has no PR, say
so and report the default branch's latest checks
(`gh run list --branch <default> --limit 5`). If `gh` is unavailable or
unauthenticated, say CI was not checked.

## Step 4 — Instruction files that changed

```
git diff --name-only $W origin/main -- \
  CLAUDE.md AGENTS.md ':(glob)**/CLAUDE.md' ':(glob)**/AGENTS.md' \
  .github/workflows rust-toolchain.toml rustfmt.toml
```

Add the repository's own gate scripts and toolchain pins (for example a
`scripts/check.sh` or `.tool-versions`). List each with
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

Mark every header line "none" rather than dropping it, except in the two-line
nothing-changed case. End with one concrete next step only if the state calls
for one — e.g. "Fix the failing `lint`
check before adding to this branch," or "Rebase onto `origin/main`; three
files overlap."

## After the briefing

Do not start the next piece of work inside this skill. Offer, but do not
perform without being asked:

- Show the full `git log` or diff for a specific area.
- Drill into a specific commit (`git show <sha>`).
- Compare against a different base or window.
