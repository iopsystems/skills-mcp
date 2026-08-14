---
status: shipped
opened: 2026-08-13
updated: 2026-08-14
prs: [30, 31]
beta_skills: [review-guide]
---

# Review exchange bridge

## Goal

Generalize the one-way Codex-to-Claude review handoff into a mechanism by which
two independent local agents trade threaded review notes in any repository,
with the recording format guaranteed by a command rather than by prose.

## Decision Criteria

Ship when a repository with no bridge can install one from the skill text alone,
a repository with the old bridge can upgrade without losing its existing
reviews, two agents writing at the same moment cannot collide, and the tested
implementation cannot differ from the one the skill distributes. The full suite,
`cargo fmt --check`, `clippy -D warnings`, a release build, and the MCP smoke
test must pass.

## Scope

One active skill, `review-exchange`, carrying both the protocol and its
installer. One integration test. One eval corpus and its count assertion. No
change to any existing skill. The store, the commands, and the threads stay
local and untracked; nothing about pull-request review changes.

## Evidence

The mechanism being replaced was a repo-local git alias reading
`<git-common-dir>/review-feedback/latest.md`, installed by hand-running a
`PLAN.md` in each repository. Three observed problems drove the change.

It was one-directional. The bridge README instructed Codex what to write and
gave Claude a way to read it. Nothing let an author write back, so a finding the
author disagreed with had nowhere to live: it was either silently accepted or
silently dropped, and the next review round raised it again.

It had drifted. `agrippa-core` alone carried two timestamp formats —
`20260813-125352.md` and `2026-08-13T0029-0700.md` — and its newer rounds
carried a `Reviewed:` header line that this repository's rounds lacked. Neither
difference was decided by anyone.

It was not installed uniformly. At the start of this effort `skills-mcp` and
`agrippa-core` had the alias and the store; `pelikan` had neither.

There was also no durable place for an agent to learn the protocol. No
repository carries an `AGENTS.md`, and the README documenting the format sits
inside `.git/`, where an agent will not find it unprompted. The protocol
survived only by being restated each session.

Separately, `~/.cargo/bin/skills-mcp` — the binary Codex reaches through its
`config.toml` — was still v0.1.0, the original seventeen-skill build. Codex had
never seen any skill added since.

## Design and Implementation

Four decisions shaped the result, all settled with the user before
implementation.

**Threaded rounds rather than symmetric outboxes.** One thread per branch, each
round its own file. A reviewer round posts findings; an author round answers
them; the reviewer re-checks. The existing `agrippa-core` reviews already opened
with "the two prior findings are resolved", which is a thread maintained by
hand; this makes that continuity structural. The rejected alternative — one
outbox per agent — is simpler but cannot link a reply to the finding it answers,
which is the whole point.

**Both directions are commands.** `git review-feedback` reads, `git review-note`
writes. The skill governs what to say; the commands govern how it is recorded.
The header fields — reviewed commit, base, merge base, timestamp — are derived
from git rather than typed by the agent, which removes a class of defect this
repository has hit repeatedly. The rejected alternative was thin plumbing with a
thick skill, where writing is plain file creation guided by prose; it is less
code, but the observed failure of the old mechanism was convention drift, and
prose is what drifted.

**Explicit identity.** Each writer names itself with `--as`. Environment
sniffing mislabels silently when an agent shells out or runs under a wrapper,
and a wrong label is worse than a prompt. Identity cannot default from git
config either, because both agents share one `.git/config`.

**Per-agent read cursors, keyed by branch.** The design first drew them as
`cursors/<agent>`, which cannot track two threads — the second would clobber the
first's read position. Corrected during planning.

The bridge is a script written to `<store>/bin/review-bridge`, with two thin git
aliases pointing at it. The approved design had the logic inside the alias
values; a shell function of this length inside a `git config` string is
unquotable in practice, and a file on disk is what makes the next property
possible.

That property is the load-bearing one. The script exists nowhere in this
repository except as a heredoc inside `skills/review-exchange/SKILL.md`, between
`<!-- INSTALLER-BEGIN -->` and `<!-- INSTALLER-END -->` markers.
`scripts/review-bridge-test.sh` extracts that block with `awk` and executes it,
so the bytes CI tests are the bytes the skill distributes. The skill text is the
implementation, which is the structural answer to the drift that motivated the
effort.

Artifacts:

- `skills/review-exchange/SKILL.md`
- `skills/review-exchange/evals/trigger-evals.json`, fourteen cases covering
  install and upgrade detection, both roles, disputed and deferred findings,
  convergence and refusing to declare it for the other agent, refused
  manufactured findings, missing identity, and the durability boundary
- `review_exchange_evals_cover_key_scenarios` in `src/main.rs`
- `scripts/review-bridge-test.sh`, wired into `.github/workflows/ci.yml`

### The defect the test found

The implementation plan predicted the concurrency assertions would pass on
first run. They did not: three concurrent writers produced six files carrying
only four distinct round numbers.

The claim used `set -C` noclobber on the output path,
`<NNN>-<agent>-<short-sha>.md`. Because two agents have different names, they
produce different paths — so nothing collided and numbers silently duplicated.
The exclusion was wrong in precisely the case it existed to handle.

The fix claims an atomic `mkdir` on `.claim-NNN`, which is name-independent.
`next_round` counts in-flight claims as well as written files, so a writer that
loses the race advances to the next number instead of recomputing the same one
and spinning until the winner finishes. The bounded retry is a backstop, not the
mechanism.

## Outcome

Shipped in pull request 30, merged 2026-08-14T01:42:10Z as `264b2648`. Five
files, 792 insertions, six commits. `cargo test --locked` reports 138 tests
across eight test binaries; `cargo fmt --check`, `clippy --all-targets -D
warnings`, the release build, `./scripts/review-bridge-test.sh`, and
`./scripts/mcp-smoke.sh` all pass. A `tools/list` handshake returns thirty
tools including `review-exchange`.

The integration test was run five times, including three consecutive runs to
check the concurrency assertions for flakiness, and once under
`GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_SYSTEM=/dev/null` to confirm it does not
depend on the runner's git identity.

Released as v0.4.0 in pull request 31, merged 2026-08-14T01:49:23Z as
`42647142`, tagged and published 2026-08-14T01:54:10Z with eight assets.

Rolled out to all three target repositories: `skills-mcp` and `agrippa-core` as
upgrades, each reporting its preserved legacy reviews (three and four files),
and `pelikan` as the fresh case, which had nothing before. `~/.cargo/bin/skills-mcp`
was rebuilt from v0.1.0 to current, and a handshake against that binary confirms
it now serves thirty tools.

**The protocol has no evidence from use.** Every test drives the bridge from one
bash script, which proves the mechanism and says nothing about whether an agent
reading `SKILL.md` produces a well-formed round, dispositions every finding, or
reads before writing. Pull request 30 was itself reviewed and merged without
going through the bridge. The eval corpus asserts intended behavior in prose and
executes none of it, the same limitation the other five corpora carry.

## Derived Documents

The design and implementation plan were written to `docs/superpowers/`, which is
gitignored in this repository by standing instruction. Their durable content is
absorbed above; the files are not committed and are not the record.

`docs/backlog.md` and `docs/roadmap.md` are unaffected: they describe the
template system and distribution stages, and this change adds a skill without
altering either.

## Deferred or Reopen Items

- No two-agent exchange has occurred. Reopen after the first real round trip:
  one agent posting findings through `git review-note`, the other reading,
  answering, and re-reading. This is the only evidence that would settle whether
  the protocol survives an agent that did not write it.
- Whether `review-exchange` should be marked beta in its own instruction text.
  It is as unproven as `review-guide` was at the same stage, but nothing declares
  it beta today, so the journal contract does not treat it as such.
- Timestamps use `date +%Y-%m-%dT%H:%M:%S%z` for macOS portability, which yields
  `-0700` rather than `-07:00`. They are ISO-adjacent, not ISO 8601. Nothing
  parses them today; reopen when something does.
- Linked worktrees are untested. `store_dir` resolves through
  `git rev-parse --git-common-dir` so all worktrees share one store, which is
  intended, but every test repository is a main worktree.
- The claim mechanism assumes POSIX `mkdir` atomicity. All three target
  repositories are on local filesystems; reopen before putting one on a network
  filesystem.
- Two questions left open by design: whether a thread should key on branch plus
  base — a branch reviewed against two bases would collide today, and no such
  case exists — and whether convergence should be recorded rather than left
  implicit in an empty Findings section.
- The Homebrew tap still serves v0.3.0 as of this entry. The updater runs on an
  08:15 UTC cron or by dispatch; the v0.3.0 chain completed unattended in about
  eight minutes.

## Skill Feedback

### review-guide (beta)

- **Friction** — Asked to draft the pull-request body for this change. The rule
  "Point at a specific place, not a file", with a line-number example, is
  correct for a body read this week against one commit, but the skill gave no
  guidance for a citation in a durable document, where a line number rots the
  next time the cited file is edited. Pinned the body to a commit SHA and stated
  so at the top, then opened a separate effort to make citation form follow the
  artifact's lifespan.
- **Friction** — The body shipped one wrong citation: `:194` for a line reading
  `done`, where the attempt cap is `:193`. The skill's instruction to cite a
  specific line was followed; what failed was that four anchors were read and
  the fifth was estimated. The skill had no rule against partial verification.
  Corrected before merge and recorded as an uncertainty item in the body.
- **Confirmation** — The publish test cleared on all four items, and the ranking
  rule put the round-number claim first, which is where the real defect had
  been. Ranking by cost-of-missed-defect rather than by path order named the
  right file without hindsight.
- **Confirmation** — "What none of this covers" forced the statement that the
  protocol had never been exercised by two agents. That is the most important
  sentence in the body and would not have been written without the rule.

## Appendix: Skills Invoked

- `superpowers:brainstorming` — settled the exchange shape, identity, and
  notification model before any code.
- `superpowers:writing-plans` — produced the seven-task implementation plan.
- `superpowers:executing-plans` — drove the tasks with their test cycles.
- `superpowers:finishing-a-development-branch` — verification and integration.
- `review-guide` (beta) — drafted the pull-request body for change 30.
- `engineering-journal` — this entry.

The roster covers the whole effort. The release in change 31 was mechanical and
invoked no skill beyond this one.
