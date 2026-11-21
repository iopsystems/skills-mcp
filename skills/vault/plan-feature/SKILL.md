---
name: plan-feature
description: |
  Scope, frame, and design a feature end-to-end — especially one that spans more than one repo. Runs a multi-turn Q&A covering problem, user value, constraints, surface area, key decisions, and cross-repo coordination, then fans the output out into the right places: paired problem/design briefs in the knowledge-iop vault (always), an optional design doc in the feature repo (`docs/design/<slug>.md`), and draft per-repo tracking issues. Use this when the user says things like "plan a feature", "scope this work", "I need a design for X", "let's kick off Y", "we need an RFC for this", or any time work needs both centralized knowledge capture and repo-local grounding. If the user only wants to capture a problem with no solution in mind, use `frame-problem` directly instead.
---

# Plan feature

Coordinate paired problem/design briefs in the separate `knowledge-iop` vault,
an optional feature-repo design doc, per-repo tracking issues, and a vault
session note. The vault briefs are authoritative.

## 1. Establish authorization

Use **draft mode** unless the user has authorized external actions. State the
mode and targets; ask only about missing authorization needed for the next
write. Do not ask the user to repeat permissions already supplied.

- **Draft mode:** prepare diffs and issue bodies. Obtain approval before each
  external write; hand off issue drafts for the user to open.
- **Autonomous mode:** perform the authorized vault commits, feature-branch
  writes and pushes, and issue creation without per-step approval. Authorization
  must cover each target and action; a repo named in the plan is not permission
  to write to it or open its issue.

If a target, access, or tool is unavailable or unclear, draft that step and
explain the limit. Continue independent work within the established scope.
Carry these permissions into dependent skills; their approval steps do not
require reconfirming an already authorized action.

## 2. Resolve planning gaps

Use supplied facts and repository evidence before asking questions. Assess the
six areas below in order, skipping questions already answered. Ask one focused
question at a time when the answer affects the plan. Confirm an interpretation
when it is uncertain or consequential; do not require a recap after every
answer or an announcement for every area.

Separate user problems from proposed solutions and engineering preferences.
If the user asks for a draft before gaps are resolved, write it with explicit
unknowns. Do not invent answers, constraints, owners, or decisions.

### Problem

Capture the user's task, failure, affected people, frequency and context;
why it matters now; success from the user's perspective; the workaround and
its limits; scope exclusions; and what would change the analysis.
`frame-problem` turns this into the problem brief's six questions.

“We need a caching layer” names a solution. Establish the user-visible problem
before treating that solution as decided.

### User value

Identify the beneficiary and concrete benefit, affected population and severity,
cost of delay, smallest useful slice, and a measurable post-ship outcome.
“Cleaner architecture” alone does not explain user value.

### Constraints

Capture delivery budget, deadlines, staffing and on-call load; technical and
performance limits; compliance, privacy, security and licensing requirements;
compatibility contracts; ownership and approval boundaries; fragile assumptions;
and excluded approaches with reasons.

Summarize the constraints and resolve material uncertainty. Distinguish hard
limits from preferences: preferring Postgres does not rule out alternatives.

### Surface area

Identify affected interfaces, data, infrastructure, docs and SDKs; primary and
incidental repos or services; changing public contracts and their consumers;
migrations or backfills; and exclusions. Summarize the repo list because it
determines coordination and issue scope.

### Key design decisions

Surface the two or three hard choices. For each, record realistic options,
constraints that eliminate options, reversibility and consequences, the chosen
direction or unresolved question, and the deciding factor. Identify a cheap
way to resolve unknowns, such as a prototype, load test or user interview.
These choices and alternatives belong in the design brief.

### Coordination

For a single-repo change, state that cross-repo coordination is skipped.
Otherwise capture each repo's work and owner, one overall accountable person,
landing order, compatibility measures, staged rollout, rollback, and required
communication or sign-off.

## 3. Prepare and land artifacts

Follow this order within the authorization established above. In draft mode,
prepare concrete proposals for approval; do not describe them as landed.

### Problem brief

Invoke `frame-problem` with the problem evidence. Let it handle vault search,
frontmatter, `briefs/<id>.md`, supersession and commit conventions.
If it finds an accepted problem brief that already frames this plan, reuse its
id and report the reuse. Do not duplicate or edit the accepted brief.

### Design brief

Invoke `propose-design` with the remaining planning evidence and problem id.
It handles `briefs/<id>.md` with `frames: <problem-brief-id>`, transition checks
and commits. Keep alternatives and cross-repo coordination in the design brief;
the repo doc summarizes and links to it. Do not bypass either brief skill.

### Optional repo design doc

Offer `docs/design/<slug>.md` in the primary feature repo. Skip it when the user
asks or the repo does not keep design docs, and report why. When written, use:

```markdown
# <feature title>

**Vault:** `<problem-brief-id>` (problem) · `<design-brief-id>` (design)
**Status:** <draft | accepted> · **Accountable:** <person>

<One paragraph: what we're doing and why, in the team's voice.>

## Scope
- In: <bullets>
- Out: <bullets>

## Design
<Short — the design brief has the detail. Summarize and link.>

## Plan
<Ordered work items from Coordination. Reference per-repo issues once opened.>

## Open questions
<Anything unresolved. Do not invent answers to fill gaps.>
```

Commit on a feature branch and offer a draft PR. Do not write to the feature
repo's `main` branch or force-push.

### Tracking issues

Draft a tracking issue for each repo's work, including a single-repo plan:

```
Title: <feature slug>: <this repo's piece>

Body:
Part of <feature title>.
Vault: <problem-brief-id> · <design-brief-id>
Design doc: <link if written>

Scope for this repo:
- <bullet> …

Depends on: <other repo issues, if any>
Owner: <person>
```

In autonomous mode, use GitHub MCP tools to open issues only in explicitly
authorized repos and report their URLs. Leave other issues as drafts. Never
invent issue links or treat the coordination plan as authorization.

### Session note

Invoke `discuss` in session-note mode to capture participants, planning results,
briefs, issues and unresolved work. Prepare or commit the note according to the
established authorization. Do not omit it; reconciliation needs this record.

## 4. Report the handoff

Account for every output, using its actual state:

- Problem brief: id and status; reused, drafted or committed.
- Design brief: id, its problem frame, and draft or commit state.
- Repo design doc: path and state, or explicit skip reason.
- Tracking issues: repos with draft bodies or opened URLs.
- Session note: id and draft or commit state.
- Open questions and any blocked or unfinished actions.

Keep unknowns in the briefs or repo doc's open questions. Do not declare work
complete while a required artifact or action is missing; name what remains.
