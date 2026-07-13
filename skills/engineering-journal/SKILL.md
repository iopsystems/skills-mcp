---
name: engineering-journal
description: >-
  Use when starting, continuing, handing off, or closing a non-trivial engineering or research effort in a shared repository; when preserving a measured negative result; when reconciling journal status, index, or backlog drift; or when identifying journal material that may inform future problem or design briefs.
---

# Engineering Journal

## Purpose

Keep one durable, repository-local record per non-trivial effort: its goal,
evidence, decisions, implementation, outcome, and unresolved work. Issues and
pull requests remain the task layer; the journal is the narrative and decision
layer. Do not use it for self-evident one-line changes.

## Discover the Repository Convention

Before writing, inspect repository instructions and search for `docs/journal/`,
`journal/`, an index, backlog, roadmap, assumptions, canonical facts or data,
and existing entry frontmatter. Preserve established paths, headings, extra
metadata, and derived-document conventions.

If no convention exists, use `docs/journal/YYYY-MM-DD-short-slug.md` and
`docs/journal/README.md`. Bootstrap both from the templates below. Do not create
a project-wide backlog unless the user requests one.

Search the journal and Git history for prior work before opening a new entry.

## Choose the Lifecycle

Support both modes:

- **intent-first** — land an `open` entry before implementation when early
  visibility, coordination, or a GO/NO-GO probe matters; close it later.
- **single-PR** — create and close the entry in the implementation PR when work
  is already underway or tightly scoped.

State which mode you are using. Honor an explicit user choice.

## Frontmatter State

Frontmatter is authoritative:

```yaml
---
status: open
opened: YYYY-MM-DD
updated: YYYY-MM-DD
---
```

Allowed states:

- `status: open` — active, paused, or blocked. Record the blocker and restart
  condition in the body; do not invent a paused state.
- `status: shipped` — landed with implementation and verification evidence.
- `status: no-go` — rejected or falsified with the mechanism and a concrete
  reopen condition.
- `status: superseded` — replaced by an existing entry; require
  `superseded_by: <entry-slug>`.

Optional fields such as `issues: [123]` and `prs: [456]` appear only when known.
Never invent lifecycle dates, links, measurements, or evidence.

Valid transitions are `open` to any terminal state, and `shipped` or `no-go` to
`superseded` when a replacement entry exists. Do not reopen terminal entries in
place; open a new entry that links the prior result. Use `superseded_by` only on
superseded entries.

## Entry Template

```markdown
---
status: open
opened: YYYY-MM-DD
updated: YYYY-MM-DD
---

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

Omit `Decision Criteria` only when there is no meaningful GO/NO-GO gate. While
open, state the current result or say explicitly that the outcome remains open.

Ground claims in durable source paths, commit SHAs, pull requests, measurements,
or canonical datasets. Mark unverifiable gaps; never fabricate them. A NO-GO is
a first-class result. Record what was tried, the mechanism, and what evidence or
changed condition would reopen it.

When the repository has adopted the journal as its durable design record, absorb
the durable goal, decisions, rationale, dead ends, and verification from temporary
specification or plan documents into the entry, then remove those temporary files
in the same PR. Do not rewrite unrelated historical design records.

## Operations

### Open

Create the entry and index row. Capture goal, scope, evidence already known,
decision criteria where useful, intended approach, and current unresolved items.
For intent-first work, land this record before implementation.

### Update

Re-read code, Git history, and durable evidence. Update `updated`, current
implementation, evidence, outcome, and unresolved items. Paused or blocked work
remains open and names its blocker and restart condition.

### Close

- For shipped work, set `status: shipped` and record what landed, where, and how
  it was verified.
- For a negative result, set `status: no-go` and record the mechanism and reopen
  condition.
- For replacement, set `status: superseded`, add `superseded_by`, verify the
  target exists, and explain the replacement.

Update `updated`, the index, and affected derived documents in the same change.

### Reconcile

Scan journal markdown files except the index. Treat entry frontmatter as source
of truth and check:

- `status`, `opened`, and `updated` exist.
- Status is `open`, `shipped`, `no-go`, or `superseded`.
- `updated` is not earlier than `opened`.
- A superseded entry names an existing replacement.
- A shipped entry carries implementation and verification evidence.
- A no-go entry carries its mechanism and reopen condition.
- Index date, title, status, and membership match the entries.
- Deferred and reopen items match an existing backlog.

Repair mechanical index drift. For changes requiring judgment, report the exact
proposed edit before applying it. Preserve repository-specific headings and extra
frontmatter rather than rewriting mechanically.

Selectively reconcile existing backlog, roadmap, assumptions and limitations,
canonical fact sheets, datasets, and bibliographies only when materially affected.
Remove or close backlog items completed or deprecated by the outcome.

## Report Possible Brief Inputs

Reconciliation may report, without creating vault artifacts:

- **Problem framing candidate** — durable problem, constraints, and evidence.
- **Design reasoning candidate** — approach, alternatives, and rationale.

For each candidate, name the entry and summarize why it qualifies. This is an
advisory handoff only. Do not invoke `frame-problem`. Do not invoke `propose-design`.
Do not create or update briefs unless the user separately asks
to start that workflow.

## Index Template

```markdown
# Engineering Journal

One durable, repository-local record per non-trivial effort. Entry frontmatter
is authoritative for lifecycle status.

| Opened | Effort | Status |
| --- | --- | --- |
| YYYY-MM-DD | [Effort title](YYYY-MM-DD-short-slug.md) | open |
```

## Stop Conditions

- Stop a lifecycle update when required evidence or a supersession target is
  missing; state the gap.
- Ask one narrow question when repository conventions conflict and the answer
  changes where or how records are written.
- Never use a GitHub issue, transient plan, or unmerged scratch file as the only
  durable record of a non-trivial effort.
- Never silently create briefs during journal reconciliation.
