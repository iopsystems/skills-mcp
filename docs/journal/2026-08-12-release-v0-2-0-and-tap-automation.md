---
status: open
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

Open. The version bump is prepared; the tag, the release build, the tap
workflow, and the formula bump have not yet run.

## Derived Documents

`docs/roadmap.md` Stage 2 describes v0.1.0 distribution accurately and is not
rewritten by this effort. Revisit only if release cadence becomes a stated goal.

## Deferred or Reopen Items

- Whether tap publication should require human review. Today the chain from an
  automated formula pull request to a published bottle runs unattended, which
  this effort inherits rather than decides.
- Release cadence. Nothing yet prevents a second month-long gap; the automation
  removes the formula step, not the decision to tag.
- The skills-invoked appendix is omitted here because no skill was invoked,
  which is what `Record Skill Use` requires. That omission is indistinguishable
  from a writer who ignored the convention. Reopen if a survey needs to tell the
  two apart; an explicit `None.` roster would, at the cost of a heading on every
  entry.
