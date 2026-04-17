---
name: catchup
description: Summarize recent development activity in the current repo so the user can catch up on what others have changed in the last hours/days/weeks. Picks the window automatically — since the user's last commit, or else the larger of the last 5 commits vs the last 7 days — and produces a grouped, prioritized summary (themes, risky changes, files touched, who did what). Trigger when the user says things like "what did I miss", "catch me up", "what's new on this branch", "what changed since I was out", or after returning from time off.
---

# Catch Up

You are helping the user catch up on recent development activity in the
current repository. Produce a grouped, prioritized summary of what has
changed since they last engaged, so they can resume work with context.

## Operating rules

- **Do not modify the repo.** This is a read-only briefing — no edits,
  commits, branches, or pushes.
- Work from the current working directory. If the user names a different
  repo or path, `cd` into it (via a shell command you run) before starting.
- Prefer running git commands via the Bash tool. Use `--no-pager` or pipe
  through `cat` so output isn't paginated.
- Summarize; don't dump. A raw `git log` is not the answer — the user can
  run that themselves.
- If the range is empty ("nothing has changed since your last commit"), say
  so plainly and stop. Do not invent filler.

## Step 1 — Determine the identity

Find the user's git identity, since you need it to detect their own commits.

```
git config user.email
git config user.name
```

If both are empty, note it and fall back to the "no prior commits" path
in step 2.

## Step 2 — Pick the range

Use this decision tree, in order:

1. **Since the user's last commit.** On the current branch, look for the
   most recent commit authored by the user's email (preferred) or name:
   ```
   git log -1 --author="<email>" --pretty=%H
   ```
   If a commit is found, the range is `<that-sha>..HEAD`. Report the SHA
   and date so the user can sanity-check.

2. **No commits by this user on this branch.** Fall back to the *larger*
   of these two windows, by commit count:
   - The last 5 commits: `HEAD~5..HEAD` (guard against shallow histories).
   - The last 7 days: `--since="7 days ago"`.

   Run `git rev-list --count` for each and pick whichever is bigger. Ties
   go to the 7-day window (more time context is usually more useful).

3. **Shallow or tiny history.** If the repo has fewer than 5 commits
   total, just summarize everything and say so.

Always state the chosen window at the top of your summary, e.g.
"Showing 12 commits from the last 6 days (since your last commit
`a91187c` on 2026-04-11)." Include both the commit range and the
calendar range when possible.

## Step 3 — Gather the data

Use a compact log format plus per-commit stats. Example commands:

```
git log --no-merges --date=short \
  --pretty=format:'%h%x09%an%x09%ad%x09%s' <range>

git log --no-merges --stat --pretty=format:'%h %s' <range>

git shortlog -sn --no-merges <range>
```

For large ranges (roughly >30 commits), also pull:

```
git diff --stat <range>
```

…to see aggregate file churn, and cap per-file detail to the top ~15
churned files.

Skip merge commits from the narrative but note if a notable merge
(e.g. a feature branch landing) happened. Don't re-summarize each
individual commit that was already part of a squashed merge.

## Step 4 — Group and prioritize

Don't list commits one-by-one unless there are fewer than ~8. Instead,
organize by **theme**, not by chronology:

- Group commits by area/feature (infer from paths, subject lines, and
  commit bodies — e.g. "skill loader", "MCP transport", "build setup").
- Within each group, note the intent in one line, cite the 1–3 key
  commits with short SHAs, and name the author(s).
- Surface **high-signal** items first:
  1. Breaking changes / API changes / schema or migration changes.
  2. New features or new public surfaces.
  3. Notable refactors that touch many files.
  4. Bug fixes that matter to the user's current work.
  5. Test, CI, docs, and chore work — last, condensed.
- Flag anything that looks like it might collide with likely in-flight
  work (e.g. renamed files, removed functions, config format changes).

Call out authors other than the current user by name — the point is
to catch up on *others'* work.

## Step 5 — Write the briefing

Produce a single markdown briefing with this shape. Keep it tight —
aim for under ~40 lines unless the range is unusually large.

```
## Catch-up: <branch> — <range summary>

**Window:** <N commits> over <date range> (<why this window was chosen>).
**Authors:** <name (n)>, <name (n)>, ...

### Highlights
- <1–5 bullets: the things the user most needs to know>

### By area
#### <Area 1>
- <one-line intent> — <short sha>, <short sha> (<author>)

#### <Area 2>
- ...

### Watch out for
- <breaking changes, renames, removed APIs, config format changes,
  anything likely to conflict with the user's in-flight work>
  Omit this section if there's nothing to flag.

### Housekeeping
- <one short bullet rolling up docs/tests/CI/chore commits, if any>
```

End with a one-line suggestion for the next concrete step — e.g.
"Pull `main` and re-run `cargo build` before resuming your branch,"
or "Worth a quick look at `src/transport.rs` before your next change."
Only offer a suggestion if something in the diff actually warrants it.

## After the briefing

Offer, but do not perform without being asked:

- Show the full `git log` or diff for a specific area.
- Drill into a specific commit (`git show <sha>`).
- List files the user's in-flight branch touches that *also* changed
  upstream (likely merge-conflict candidates).
- Compare against a different base (e.g. `origin/main` instead of the
  local branch tip).
