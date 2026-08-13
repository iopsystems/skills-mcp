---
status: shipped
opened: 2026-08-12
updated: 2026-08-12
---

# Release v0.2.0 and Homebrew tap automation

## Goal

Publish the four skills authored since v0.1.0 to installed users, and remove the
manual step that let the gap open in the first place by having a `skills-mcp`
release propagate to the Homebrew formula without hand editing.

## Decision Criteria

Ship when `brew install iopsystems/iop/skills-mcp` yields a binary serving
twenty-one skills, and when the formula bump that delivered it was produced by a
workflow rather than by hand. A hand-edited formula would satisfy the first
criterion and leave the actual defect in place.

## Scope

A crate version bump and tag in this repository, and a new
`update-skills-mcp.yml` workflow in `iopsystems/homebrew-iop`. Out of scope: the
release cadence policy, any change to `release.yml`, and the question of whether
tap publication should require human review.

## Evidence

The distribution machinery works and had simply not been exercised:

- One release exists. `v0.1.0` was tagged 2026-07-15; `main` is thirty-nine
  commits ahead as of 2026-08-12.
- Four skills are undistributed: `architecture-diagram`, `dataflow-diagram`,
  `sweep-comments`, and `technical-prose`. A released binary carries seventeen
  skills; `main` carries twenty-one. Because `src/main.rs` embeds the tree with
  `include_dir!`, a stale binary is a stale skill library, and no runtime
  loading softens that.
- Reach is small enough that the gap went unreported: the v0.1.0 assets show two
  downloads for `aarch64-apple-darwin`, one for `x86_64-unknown-linux-gnu`, and
  zero for the rest.
- `.github/workflows/release.yml` triggers on any `v*` tag and builds four
  targets with checksums, so the repository side needs no change.
- The tap automates everything after a pull request exists. `tests.yml` runs
  `brew test-bot --only-formulae` on pull requests, uploads bottles, and adds
  the `pr-pull` label once tests pass; `publish.yml` reacts to that label by
  running `brew pr-pull` and pushing to `main`. The single missing piece for
  this formula is the job that opens the pull request. `update-rezolus.yml`
  already does exactly that for the other formula.

## Design and Implementation

The version becomes `0.2.0` rather than `0.1.1`: four new skills are added
capability, not a fix.

The tap workflow mirrors `update-rezolus.yml` — a daily cron plus
`workflow_dispatch`, a version comparison against the upstream latest release,
and `peter-evans/create-pull-request` using the existing `API_TOKEN` secret.
Mirroring the proven path matters more than improving it, because the bottle and
publish chain downstream is sensitive to formula shape.

One adaptation is forced by a difference between the two formulae. `rezolus.rb`
points at a source tarball and carries its `sha256`, so its updater recomputes a
checksum. `skills-mcp.rb` uses a git `url` with `tag:`, so the bump rewrites the
tag and there is no source checksum to recompute. The stale `bottle do` block is
left untouched, matching the rezolus updater, because `brew pr-pull` rewrites it
from the bottles that `test-bot` produced.

A cron rather than a `repository_dispatch` from `release.yml` means up to a day
of lag between tag and formula. That is accepted: it needs no new cross-repository
secret in this repository, and it matches the cadence the tap already runs.

The sequence is chosen to prove the automation rather than assume it: bump and
tag here first, land the tap workflow second, then trigger it manually so that
the formula bump reaching users is machine-generated. Hand-editing the formula
would have shipped the release while leaving the defect that caused the gap.

## Outcome

Shipped. `brew install iopsystems/iop/skills-mcp` now serves twenty-one skills,
and the formula bump that delivered them was machine-generated, so both decision
criteria are met.

The version bump merged as `d455d47` (pull request #24). Tag `v0.2.0` triggered
release run `31670572508`, which published at 2026-08-13T05:33:53Z with all eight
assets across the four targets; `repos/iopsystems/skills-mcp/releases/latest` now
returns `v0.2.0`, which is the endpoint the tap workflow queries.

The tap updater landed as `b307828` in `iopsystems/homebrew-iop`. A
`workflow_dispatch` of it, run `31670784740`, produced pull request 117 without
manual editing, changing exactly one line:

```diff
-    tag: "v0.1.0"
+    tag: "v0.2.0"
```

The downstream chain then ran unattended. `test-bot` run `31670798345` built
bottles on `macos-14`, `macos-15`, and `ubuntu-24.04` and succeeded at 05:41 UTC;
`autotag` applied the `pr-pull` label; `publish.yml` pushed the bottled formula to
the tap's `main` by 05:42 UTC, roughly eight minutes from dispatch. The published
formula carries `tag: "v0.2.0"`, `root_url` ending `skills-mcp-0.2.0`, and fresh
checksums for `arm64_sequoia`, `arm64_sonoma`, and `x86_64_linux`.

One observation worth keeping: pull request 117 shows as closed rather than
merged. `brew pr-pull` pushes the commits to `main` directly and closes the pull
request behind them, so closed-not-merged is the expected result of a successful
publication in this tap, not a failure.

No measurement was taken of how many users the release reaches. The v0.1.0 asset
download counts that motivated the effort were read before the release and are
recorded under Evidence; the equivalent numbers for v0.2.0 will not be meaningful
for some time.

## Derived Documents

`docs/roadmap.md` Stage 2 describes v0.1.0 distribution accurately and is not
rewritten by this effort. Revisit only if release cadence becomes a stated goal.

## Deferred or Reopen Items

- Whether tap publication should require human review. The chain from an
  automated formula pull request to a published bottle ran unattended and was
  observed doing so during this effort: nobody approved pull request 117 between
  its creation and its publication. This effort inherits that policy rather than
  deciding it, and the observation raises rather than settles the question.
- Release cadence. Nothing yet prevents a second month-long gap; the automation
  removes the formula step, not the decision to tag.
- The skills-invoked appendix is omitted here because no skill was invoked,
  which is what `Record Skill Use` requires. That omission is indistinguishable
  from a writer who ignored the convention. Reopen if a survey needs to tell the
  two apart; an explicit `None.` roster would, at the cost of a heading on every
  entry.
