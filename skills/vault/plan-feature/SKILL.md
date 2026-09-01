---
name: plan-feature
description: |
  Scope, frame, and design a feature end-to-end — especially one that spans more than one repo. Runs a multi-turn Q&A covering problem, user value, constraints, surface area, key decisions, and cross-repo coordination, then fans the output out into the right places: paired problem/design briefs in the knowledge-iop vault (always), an optional design doc in the feature repo (`docs/design/<slug>.md`), and draft per-repo tracking issues. Use this when the user says things like "plan a feature", "scope this work", "I need a design for X", "let's kick off Y", "we need an RFC for this", or any time work needs both centralized knowledge capture and repo-local grounding. If the user only wants to capture a problem with no solution in mind, use `frame-problem` directly instead.
---

# Plan feature

You are running a planning pass for a feature. Your output is not one
artifact — it's a **coordinated fan-out** across two places:

- **The vault** (`knowledge-iop`, always a separate repo) gets the
  paired problem-brief and design-brief. This is the centralized
  knowledge the reconciler reads.
- **The feature repo** (where the code change lands) optionally gets
  a `docs/design/<slug>.md` that links back to the vault briefs.
- **Per-repo issues** are drafted from the coordination plan, and
  either handed to the user to open or opened directly — depending on
  the operating mode you agreed on up front.

The coordination is the point. A plan that lands the vault briefs but
misses the repo doc, or opens issues without a paired brief, is a
failure mode this skill exists to prevent.

## Step 1 — Agree on operating mode and privileges

Before any Q&A, tell the user what this skill will need and ask which
mode to run in. Default to **draft mode** unless the user pre-authorizes.

**Draft mode (default).** You propose every external write as a diff or
a draft issue body; the user approves each before anything lands. Needs
no permissions beyond what the user already granted the session.

**Autonomous mode.** You commit vault briefs, write the repo doc, and
open per-repo issues without pausing for approval at each step. Ask the
user explicitly at the start:

> Autonomous mode will commit to the vault, push a branch to the
> feature repo, and open issues in repos A, B, C. Confirm you want
> me to proceed without per-step approval, and that I have write
> access to each of those targets.

If anything is ambiguous — unknown repo, missing vault, missing GitHub
MCP tool for a target — fall back to draft mode for that step and say
so.

## Step 2 — Discovery Q&A

Act as a combined user researcher and product manager. Work through the
areas below in order. Rules:

- Ask **one** focused question at a time. Never dump a numbered list.
- After each substantive answer, reflect back in one sentence ("So the
  core problem is X — is that right?"). Cheap correction.
- If an answer is vague, engineering-flavored, or a solution in
  disguise, drill in before moving on.
- Announce each new area ("Moving to area 2 of 6: user value.") so the
  user knows where they are.
- If the user clearly knows an area cold, confirm the key facts and
  move on — don't pad.
- If the user says "just write it up", push back once: the briefs are
  only useful if the inputs are real. If they insist, proceed and flag
  gaps in the output.

### Area 1 — Problem

Goal: the **real** problem, separate from any proposed solution. This
feeds directly into the problem-brief's 6 questions (see `frame-problem`).

- What is the user trying to do today, and where does it break down?
- Who hits this? How often? In what situation?
- Why now — what changed or is newly blocking?
- What would "problem solved" look like from the user's seat?
- Existing workaround? Why isn't it good enough?
- What are we explicitly **not** trying to solve? (Adjacent problems
  we acknowledge and are setting aside.)
- What would change this analysis? (If the answer is "nothing", the
  framing is over-claimed.)

Watch for solutioning disguised as problems ("we need a caching layer"
is a solution). And for problems stated only in engineering terms ("the
job is slow") without the user-visible consequence.

### Area 2 — User value

Goal: **why solving it matters** and to whom.

- If this works, what does the user get? (time saved, error avoided,
  new capability, compliance, trust, revenue)
- How many users are affected? How painful today on a 1–5 scale?
- Cost of **not** doing this in the next quarter?
- If we shipped only a thin slice, which slice delivers the most
  value?
- Is there a measurable post-ship outcome we could check?

Watch for value stated in engineering terms ("cleaner architecture")
instead of user outcomes, and vague "strategic" value with no
concrete beneficiary.

### Area 3 — Constraints

Goal: the **boundary of what is possible**. Anything named here
shrinks the solution space for the next three areas — pin it down.

- Delivery: deadlines, budget, headcount, on-call load.
- Technical: stack/runtime, platform limits, performance budgets,
  latency/throughput floors, storage ceilings.
- Compliance / legal / privacy / security: data residency, PII,
  audit trail, licensing, customer contracts.
- Backward-compat: public APIs, schemas, wire formats, SDK contracts,
  on-disk formats, CLI flags.
- Organizational: team ownership boundaries, approval gates,
  vendor/contract limits, required sign-offs.
- Fragile assumptions: call these out explicitly; they're risks, not
  givens.
- Off-limits: approaches already ruled out, and why.

At the end of this area, read back a short constraint checklist and
confirm. Watch for "constraints" that are really preferences ("we
prefer Postgres" isn't a constraint unless something forbids
alternatives).

### Area 4 — Surface area

Goal: **where the change lands**.

- Which surfaces does this touch? (UI, public API, internal API, data
  model, infra, CLI, docs, SDKs)
- Which **repos** or services? Name them. Which are primary vs
  incidental?
- Any public contracts changing — APIs, schemas, CLI flags, file
  formats, wire protocols? Who depends on them?
- Migrations, backfills, or reshaping required?
- Explicitly **out** of scope?

Read back the repo list at the end; this list drives the coordination
plan and the issue fan-out.

### Area 5 — Key design decisions

Goal: surface the **2–3 hard choices** this work hinges on, early.

- What are the biggest design decisions this depends on?
- For each: realistic options, leaning?
- For each option: reversibility, blast radius if wrong.
- For each option: check against area 3's constraints — does any
  constraint eliminate it outright?
- What's unknown, and how do we find out cheaply — spike, prototype,
  load test, user interview?

For each decision, capture: options considered, chosen direction (or
"undecided — needs X to choose"), deciding factor. This becomes the
spine of the design brief.

### Area 6 — Coordination plan

**Skip this area if the change is truly single-repo.** Say so
explicitly and move on.

- Which repos/teams must change, and what is each piece?
- Required order of landing? (producer before consumer, schema
  before callers, flag before rollout)
- Compatibility seams: flags, version gates, shims, dual writes.
- Who owns each piece? Who is the **single** person accountable for
  the whole feature landing?
- Rollout: dark launch, staged rollout, percentage ramp, cutover?
- Rollback: what's the "oh no" button at each step?
- Communication: RFC, design review, stakeholder sign-off, customer
  comms?

## Step 3 — Fan out the artifacts

Once Q&A is complete, land the outputs in this order. Each step
pauses for user approval in draft mode; in autonomous mode, proceed
and report what you did.

### 3a. Vault: problem brief

Invoke the `frame-problem` skill with the Area 1 answers. That skill
handles its own vault search (for existing/superseded briefs),
frontmatter, filename (`briefs/<id>-problem.md`), and commit. Do not
duplicate its logic here.

If `frame-problem` finds an existing accepted problem brief this plan
should frame against, use that id instead of writing a new one. Note
the reuse to the user.

### 3b. Vault: design brief

Invoke the `propose-design` skill with Area 2–5 (and Area 6 if it
applies) and the problem-brief id from 3a. That skill writes
`briefs/<id>.md` with `frames: <problem-brief-id>` and commits.

Alternatives considered (from Area 5) and the coordination plan (from
Area 6) live in the design brief's body — that's the design brief's
job, not the repo doc's.

### 3c. Feature repo: optional design doc

Offer to write `docs/design/<slug>.md` in the primary feature repo.
This file is **optional**: skip it if the user says so, or if the
feature repo doesn't conventionally keep design docs. The vault briefs
are the source of truth either way.

Shape when written:

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
<Ordered work items from Area 6. Reference per-repo issues once opened.>

## Open questions
<Anything unresolved. Do not invent answers to fill gaps.>
```

Commit on a feature branch (not main) and offer to open a draft PR.
Do not force-push or touch main.

### 3d. Per-repo issues

For each repo in the coordination plan, draft an issue:

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

In draft mode, print all issue bodies and let the user open them. In
autonomous mode, open them via the GitHub MCP tools (only for repos
the user authorized at Step 1) and report URLs back. **Never** open
issues in a repo the user didn't explicitly authorize, even if it's
named in the coordination plan.

### 3e. Vault: session note

Commit a short session-note to the vault via the `discuss` skill
(session-note flavor) summarizing the plan pass: who was involved,
what briefs landed, what issues were opened, what's still open. This
is the audit trail for reconciliation.

## Step 4 — Closing checklist

Before declaring the plan complete, confirm aloud (or in output):

- [ ] Problem brief landed in vault (`<id>` · status)
- [ ] Design brief landed in vault, frames problem brief (`<id>`)
- [ ] Repo design doc: written at `<path>` / explicitly skipped with
      reason
- [ ] Per-repo issues: drafted for `<repos>` / opened at `<urls>`
- [ ] Session-note committed to vault (`<id>`)
- [ ] Open questions list handed back to the user

If any item is missing and wasn't an explicit skip, say so — don't
silently drop it. This checklist is the reason this skill exists.

## Do not

- Do not write the problem or design brief directly. Chain to
  `frame-problem` and `propose-design` so the vault conventions
  (search, frontmatter, supersession, transition checks) are honored.
- Do not open issues in repos the user didn't authorize in Step 1,
  even if the coordination plan names them. Draft and hand off
  instead.
- Do not skip the session-note. The reconciler needs the trail;
  otherwise this planning pass becomes invisible to future reflection.
- Do not write to the feature repo's `main` branch or force-push.
  Use a feature branch and a draft PR.
- Do not invent answers to unanswered discovery questions to fill
  gaps. Carry them into the brief's "what would change the analysis"
  or the repo doc's "open questions".
