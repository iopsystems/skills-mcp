# Engineering Journal Skill Design

## Purpose

Add a single `engineering-journal` skill served by this repository. The skill
standardizes durable, repository-local records of non-trivial engineering and
research efforts while supporting both intent-first and single-PR workflows.

Journal entries complement issues, pull requests, and the knowledge vault:

- Issues and pull requests remain the task and review layer.
- Journal entries are the in-repository narrative, evidence, and decision layer.
- Journal entries may be identified as useful inputs to future problem or design
  briefs, but this skill never creates or updates briefs.

The design draws on the engineering-journal practices in `iopsystems/rezolus`
and `iopsystems/hpc-research`. It combines Rezolus's coordination-oriented
lifecycle and treatment of measured negative results with hpc-research's concise
entry shape and selective reconciliation of derived documents.

## Skill Architecture

Add one served skill at `skills/engineering-journal/SKILL.md`. It has four
operations behind one entry point:

1. Open an entry.
2. Update an open entry.
3. Close an entry as `shipped`, `no-go`, or `superseded`.
4. Reconcile existing entries, their index, and affected derived documents.

The skill detects the repository's existing conventions before acting. When no
journal exists, it defaults to `docs/journal/` and creates an index at
`docs/journal/README.md`. Existing paths, headings, metadata, and derived-document
conventions take precedence over the defaults.

## Lifecycle

Frontmatter is authoritative for lifecycle state:

```yaml
---
status: open
opened: 2026-07-12
updated: 2026-07-12
---
```

Allowed statuses are:

- `open`: active, paused, or blocked. A paused or blocked entry states the
  blocker and restart condition in its body.
- `shipped`: the effort landed, with implementation and verification evidence.
- `no-go`: the effort was rejected or falsified, with the mechanism and a
  concrete reopen condition.
- `superseded`: another entry replaces this one. `superseded_by` is required and
  must identify an existing journal entry.

Optional metadata appears only when applicable:

```yaml
superseded_by: 2026-07-20-replacement-effort
issues: [123]
prs: [456]
```

The skill supports two workflows:

- **Intent-first:** land an `open` entry before implementation when early
  visibility and coordination are valuable. Close it in the implementing or
  follow-up pull request.
- **Single-PR:** create and close the entry alongside implementation when work is
  already underway or tightly scoped.

The skill selects a mode from the work and repository context, tells the user
which mode it is following, and honors an explicit user choice.

## Entry Shape

The default entry template is:

```markdown
# Effort title

## Goal
## Decision Criteria
## Scope
## Evidence
## Design and Implementation
## Outcome
## Derived Documents
## Deferred or Reopen Items
```

`Decision Criteria` may be omitted when there is no meaningful GO/NO-GO gate.
While an entry is `open`, `Outcome` states the current result or explicitly says
that the outcome remains open.

Claims must cite durable evidence such as source paths, commit SHAs, pull
requests, measurements, or canonical datasets. The skill never invents evidence,
dates, or measurements. If a fact cannot be verified, the entry says so or omits
the claim.

Temporary design or implementation-plan documents are absorbed into the journal
only when the repository has adopted the journal as its durable design record.
The skill does not remove historical design records merely because a journal was
introduced later.

## Journal Index

`docs/journal/README.md` lists each entry's opened date, title, and
frontmatter-derived status. Reconciliation treats entry frontmatter as the source
of truth and repairs stale, missing, or mismatched index rows.

## Derived Documents

For every journal change, the skill discovers and selectively reconciles existing
derived documents, including backlogs, roadmaps, assumptions, limitations,
canonical fact sheets, datasets, and bibliographies. It edits only documents
materially affected by the entry.

Deferred and reopen items are synchronized with an existing backlog. The skill
does not introduce a project-wide backlog unless the user explicitly requests
one.

## Brief Candidate Reporting

During reconciliation, the skill reports journal entries that may be useful
inputs to the existing brief workflow:

- **Problem framing candidate:** the entry contains a durable problem statement,
  constraints, and evidence.
- **Design reasoning candidate:** the entry contains a proposed approach,
  alternatives, and rationale.

This reporting is conservative and advisory. The skill does not invoke
`frame-problem`, `propose-design`, or write vault artifacts. Brief creation remains
a separate, user-directed workflow.

## Reconciliation and Validation

Reconciliation scans journal markdown files except the index and validates:

- `status`, `opened`, and `updated` are present.
- `status` is one of the four allowed values.
- `updated` is not earlier than `opened`.
- `superseded_by` is present only when needed and resolves to an existing entry.
- `shipped` entries contain implementation and verification evidence.
- `no-go` entries contain the failure or rejection mechanism and a reopen
  condition.
- Index rows agree with entry frontmatter and titles.
- Deferred and reopen items agree with an existing backlog.

Malformed or incomplete entries are reported precisely. The skill preserves
repository-specific headings and additional frontmatter instead of mechanically
rewriting every entry into the default template.

## Failure Handling

- Missing or unverifiable evidence is surfaced, never fabricated.
- Invalid lifecycle transitions stop the update until corrected.
- A missing supersession target blocks `superseded` status.
- An unrecognized repository convention prompts a narrow user question rather
  than broad restructuring.
- Reconciliation reports proposed changes before applying changes that require
  judgment.

## Testing

Skill evaluations and repository tests cover:

- Triggering on non-trivial engineering and research efforts.
- Intent-first and single-PR lifecycles.
- Opening, updating, and each terminal status.
- Paused and blocked work remaining `open`.
- Malformed frontmatter and invalid dates.
- Missing or stale journal-index rows.
- Backlog additions, updates, and removals.
- Missing evidence in `shipped` and `no-go` entries.
- Valid and invalid supersession references.
- Problem-framing and design-reasoning candidate reporting.
- The prohibition on automatic brief creation.
- Embedding and exposure of the new skill through the MCP server.

## Non-Goals

- Automatically creating or updating vault briefs.
- Replacing issues or pull requests.
- Imposing a repository-wide backlog, roadmap, or documentation generator.
- Retrospectively rewriting every historical effort.
- Introducing a separate reconciliation skill.
