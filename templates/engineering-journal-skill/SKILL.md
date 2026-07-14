---
name: engineering-journal
description: Use when starting, continuing, handing off, reconciling, or closing a non-trivial engineering or research effort in a repository with durable project records.
---

# Engineering Journal

## Purpose

Keep one durable, repository-local record for each non-trivial effort: its goal,
evidence, decisions, implementation, outcome, and unresolved work. Issues and
pull requests remain the task layer; the journal is the narrative and decision
layer. Do not journal a self-evident one-line change.

## Establish the Project Contract

Read and fill the [project profile](references/project-profile.md) before writing
an entry. Discover repository instructions, existing journal entries and index,
Git history, frontmatter, derived documents, and validation commands rather than
guessing. Preserve established paths, headings, lifecycle extensions, and index
shape. Ask one narrow question if conventions conflict and the answer changes
where or how records are written.

The completed profile is mandatory project configuration. Recheck it before an
open, close, or reconciliation operation. If no convention exists, propose
`docs/journal/YYYY-MM-DD-short-slug.md` with `docs/journal/README.md`, but obtain
approval before bootstrapping them. Do not create a project-wide backlog by
default.

Search existing records and Git history before opening an entry. Extend the
existing record when it covers the same effort; do not create parallel history.

## Choose the Operating Mode

Honor the user's explicit choice, then the project profile's operating-mode
preference:

- **intent-first** — land an `open` entry before implementation when early
  visibility, coordination, or a GO/NO-GO probe matters; close it later.
- **single-PR** — create and close the entry in the implementation pull request
  when work is already underway or tightly scoped.

State the selected mode. Both modes use one entry for the whole lifecycle; do
not split opening and closure into separate records.

## Lifecycle Contract

Frontmatter is authoritative. Unless the project profile adds compatible
metadata fields, use:

```yaml
---
status: open
opened: YYYY-MM-DD
updated: YYYY-MM-DD
---
```

The four states are fixed. A project may add metadata fields or stricter
transition requirements, but never another status value:

- `status: open` — active, paused, or blocked. Keep blocked work open and record
  the blocker and restart condition; do not invent `paused` or `closed` states.
- `status: shipped` — landed with implementation and verification evidence.
- `status: no-go` — rejected or falsified with the mechanism and a concrete
  reopen condition.
- `status: superseded` — replaced by an existing entry and accompanied by
  `superseded_by: <entry-slug>`.

Valid transitions are `open` to a terminal state, and `shipped` or `no-go` to
`superseded` when a real replacement exists. Never reopen a terminal entry in
place. Open a new linked effort. Never invent dates, links, measurements, issue
numbers, pull requests, or evidence.

## Entry Content

Match project-specific headings from the profile. A new convention may start
with:

```markdown
# <Effort title>

## Goal
## Decision Criteria
## Scope
## Evidence
## Design and Implementation
## Outcome
## Derived Documents
## Deferred or Reopen Items
```

Ground claims in durable source paths, commit SHAs, pull requests, measurements,
canonical datasets, or exact validation commands and results. Mark unverifiable
gaps. While open, state the current result or explicitly say the outcome remains
open. A no-go is first-class evidence: record what was tried, why it failed, and
what changed condition would justify a new effort.

## Operations

### Open or update

Create or update the single entry and its project-specific index row together.
Capture the goal, scope, known evidence, decision criteria where meaningful,
intended approach, current result, and unresolved items. In intent-first mode,
land the record before implementation.

When updating, re-read authoritative code, history, and evidence. Update the
profile-defined date field, implementation, evidence, outcome, blocker, restart
condition, and unresolved items without erasing useful reasoning or dead ends.

### Close

For `shipped`, cite what landed, where it landed, and how it was verified. For
`no-go`, cite the observed mechanism and reopen condition. For `superseded`,
verify the replacement exists and explain the relationship. Update the entry,
index, and affected durable derived documents in the same change.

### Reconcile

Scan journal entries except the index and treat entry frontmatter as source of
truth. Check required fields, allowed states and transitions, date ordering,
supersession targets, closure evidence, index membership and metadata, and links
to deferred work. Repair mechanical index drift. For a change requiring judgment,
report the exact proposed edit before applying it.

Reconcile only within the boundaries recorded in the project profile. Preserve
project-specific metadata. Selectively update durable derived documents such as
the backlog, roadmap, assumptions and limitations, canonical fact sheets,
datasets, or bibliographies only when the outcome materially changes them.
Remove or close completed or deprecated items. Do not create a project-wide
backlog just because an entry contains deferred work.

When temporary specifications or plans duplicate a journal adopted as the
durable design record, absorb their durable goal, decisions, rationale, dead
ends, and evidence into the entry and remove the superseded temporary files in
the same change. Never rewrite unrelated historical records.

## Advisory Brief Inputs

Reconciliation may report two advisory candidates without creating them:

- **Problem framing candidate** — durable problem, constraints, and evidence.
- **Design reasoning candidate** — approach, alternatives, and rationale.

Name the source entry and why it qualifies. Do not create or update briefs, and
do not invoke a problem-framing or design-proposal workflow, unless the user
separately asks for that work. Advisory reporting is never silent brief creation.

## Report and Stop Conditions

Report the selected mode, entry and index paths, lifecycle change, evidence,
validation performed, derived documents reconciled, and unresolved gaps.

Stop when required closure evidence or a supersession target is missing. Stop
when profile fields are incomplete or conventions conflict materially. Never use
an issue, transient plan, or unmerged scratch file as the only durable record of
a non-trivial effort.
