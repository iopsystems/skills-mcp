---
name: reconcile-vault
description: |
  Reconcile the knowledge-iop vault after a phase-skill commit or discussion,
  around a named artifact, or across the vault in dream mode. Use for
  "reconcile the vault", "what needs attention", "dream over the vault",
  "check for blocked transitions", "what's the state of the project", or
  "run reconciliation". Apply clean transitions; route blockers, warnings,
  ambiguity, and failed validation to human review. Interactive mode commits
  and pushes; it opens or merges a PR only on user request. Dream mode writes
  a session-note and merges it with validated edits through a PR by default.
---

# Reconcile vault

## Mode selection

Use **interactive** after a phase skill commits, for a named artifact, or
when a discussion needs triage. Scope it to the touched artifacts and their
immediate graph neighbors.

Use **dream** for a scheduled task without a pivot, or an explicit request
such as "dream over the vault", "what's been stalling", or "give me the state
of things". It scans the full vault and writes one session-note.

If unclear, ask once and default to interactive. Dream reads widely and
requires an explicit request or scheduled invocation.

| Mode | Default disposition | Findings needing judgment |
|---|---|---|
| Interactive | Validate, commit, push; branch owner handles landing. Open or merge a PR only on user request. | Report in the existing PR, or in the session if none. Do not open a PR just for a finding. |
| Dream | Validate, commit note and edits together, push, open or update a PR, merge. | Put in PROPOSED and Needs you; deliver Needs you in the run notification. Proposals do not hold the merge. |

Both modes use the same transition criteria below. Host rules and failed
checks still prevent merging.

## Mode A: Interactive

### Step 1 — Identify the pivot

Get the pivot artifact id(s) from the phase skill's output or user message.
If missing, ask which artifact changed.

### Step 2 — Walk the edges

Call `vault_edges` with `id: <pivot>`, `direction: both`. Group neighbors by
`frames`, `supersedes`, `superseded_by`, `relates_to`, `depends_on`,
`derived_from`, `arc`, `scopes`, and `inquiry`.

### Step 3 — Propose transitions

Consider whether the pivot's new state warrants a neighbor's status change:

| Pivot change | Neighbor pattern | Proposed transition |
|---|---|---|
| problem-brief → `obsolete` | design-briefs that `frames: <pivot>` | warn (not block); user may want to supersede |
| design-brief → `accepted` | decisions with `derived_from: <pivot>` previously blocked | now unblocked; user may promote to `accepted` |
| inquiry → `resolved` | parent arc (via `arc` edge) | if this was the last open inquiry, arc may be ready to close |
| design-brief → `superseded` | framed problem-brief | may still be relevant for the new design; no change |
| synthesis written | inquiry | transition inquiry `ready_for_synthesis` → `resolved` |
| new problem-brief drafted | existing design-briefs on the topic | may need `supersedes:` pointer |

### Step 4 — Pre-flight each proposed transition

For every proposed transition, call `vault_check_transition` with
`id: <neighbor>`, `new_status: <candidate>`. Record `allowed`, `blockers`,
and `warnings` with the proposal.

### Step 5 — Choose the disposition

A transition is clean only when it:

- is `allowed` with no blockers or warnings;
- cites direct, sufficient evidence and has exactly one justified next state; and
- passes repository validation after application.

Apply eligible transitions even when other candidates need review. Keep
blocked, warned, ambiguous, or unsupported transitions as proposals. A
passing pre-flight alone does not establish evidence or resolve ambiguity.

Repository validation gates the commit. Checks available only in CI do not
run before an interactive commit unless separately invoked; do not claim
that local validation covers them.

### Step 6 — Apply or request review

If on `main`, create a branch before editing and say so. Apply eligible
transitions by editing frontmatter, run repository validation, and inspect
the diff. Commit and push validated edits without asking for confirmation.
Do not commit invalid transitions.

Update an existing PR rather than duplicating it. Open a new PR only on user
request; otherwise leave landing to the branch owner. Unresolved findings
go in that PR, or in the session if none exists:

```
PROPOSED TRANSITIONS (N)

1. <neighbor-id> (<type>): <current-status> → <new-status>
   reason: <one line>
   check: allowed / blocked by [<rule>: <message>]
   evidence: <ids cited>

2. ...

NO ACTION NEEDED
- <neighbor-id>: <why touched, why unchanged>
```

For each proposed transition, state its exact review reason: blocker,
warning, ambiguous evidence, multiple plausible outcomes, or failed
validation. Ask the user to decide only those items. Clean edits from the
same run still commit; an existing PR stays on its owner's schedule.

### Step 7 — Stop

Stop after committing the clean edits or presenting the review package.
Do not cascade beyond the immediate neighborhood; the next phase skill
invocation triggers its own reconciliation.

## Mode B: Dream

Apply only transitions that pass Mode A Step 5. Write one session-note
with deterministic graph hygiene in Part A and strategic judgment in Part B.

### Step 1 — Run the reflection report

Call `vault_reflect` with these defaults unless the user specifies windows:

- `window_days: 30` — recent activity.
- `min_days_stale_design: 14` — age of draft/proposed designs to flag.
- `min_days_stale_arc: 60` — inactivity of open arcs to flag.

### Step 2 — Gather context for Part B

Call `vault_search` separately with `type: discussion`, `type: session-note`,
and `type: decision`, each with `limit: 5`. Without a `topic` filter, results
are newest first; there is no sort argument. Keep the five newest combined
discussions/session-notes and five decisions. Read titles and first paragraphs,
plus no more than two full artifact bodies when Part B needs to cite them.
Load the full reflection report; keep other context within these bounds.

### Step 3 — Write the session-note

Filename: `discussions/<YYYY-MM-DD>-reconciler-dream.md`

Frontmatter:

```yaml
---
id:       <YYYY-MM-DD>-reconciler-dream
type:     session-note
author:   reconciler
created:  <YYYY-MM-DD>
---
```

Body structure:

```markdown
# Reconciler dream — <YYYY-MM-DD>

Window: last <window_days> days. Generated by the reconcile-vault
dream pass.

## Needs you

Summarize every unresolved decision from Gaps and PROPOSED, including work
that needs no status change. One line each: artifact, decision, and what
it awaits. Keep the full reason in PROPOSED and deliver this list in the
run notification. If empty, say "nothing this pass". Proposals do not hold
the merge.

## Part A — Graph hygiene

### Activity heatmap
- Scopes, ranked by recent artifact volume. For each: total, recent,
  last activity date. Call out scopes with zero recent activity
  (candidate retirement) and scopes dominating current attention.

### Arc momentum
- Open arcs, ranked by recent artifact volume. For each: total,
  recent, last activity.

### Gaps
- **Orphan problem-briefs** (no paired design): list all. Each needs
  either a design proposed, or explicit `obsolete` / `accepted`
  with reason for shelving.
- **Stale design-briefs** (draft/proposed past <min_days_stale_design>
  days): list all, with days-old. Each is either forgotten or
  correctly paused — prompt a decision.
- **Pending syntheses** (inquiries `ready_for_synthesis` without a
  synthesis): list all. The barrier released but no one wrote.
- **Stale open arcs** (no activity past <min_days_stale_arc> days):
  list all. Each is either forgotten or needs explicit pause/close.

### Transitions
For each gap that suggests a transition, call `vault_check_transition`
with the candidate new status and record the result. Do NOT list a
transition you have not pre-flighted.

Record them under two headings, because a reader acts on the two
differently — one is a changelog, the other is a worklist:

- **APPLIED** — cleared Step 5's test and was edited in this pass.
  One line each: id, type, old status, new status, the evidence cited.
- **PROPOSED** — unresolved transitions and work decisions from Gaps.
  Give each transition's exact reason: blocker, warning, thin evidence,
  more than one plausible next state, or failed validation. For work
  such as proposing a design or confirming a pause, state the decision
  without inventing a status transition or pre-flight result. Summarize
  every entry in Needs you and the run notification.

  It does not hold anything back. A PROPOSED entry sits in a merged
  note, not an open branch.

An empty APPLIED list on a run whose Part A found cleared transitions
is a failure, not a clean bill: it means the pass described edits it
was allowed to make.

## Part B — Strategic reflection

**This is judgment, not SQL.** The rules:

1. **Evidence-or-don't-say-it.** Every factual claim cites an artifact id
   or a number from Part A that supports the whole claim, including its
   qualifiers. Keep interpretations conditional; do not turn an unknown
   owner, cause, or dependency into a fact.
2. **Named alternatives.** When recommending a "highest leverage next
   move", list 2–3 candidates before picking. The user should be able
   to disagree with the ranking, not the framing.
3. **Staff-engineer's memo tone.** Not an edict. Present the read,
   argue the take, and name what would change it.

### Sub-sections

- **What's hot.** Which scopes / arcs are accumulating artifacts.
  What that suggests about current attention.
- **What's cold.** Which are stale. What that suggests — finishing
  moves needed, scope retirement, arc abandonment with handoff.
- **Open questions across discussions.** Thread any unresolved
  questions from recent discussions/session-notes. Each one cites
  the source.
- **Missing edges.** Artifacts that probably should `relates_to`
  each other (via topical overlap) but don't. Propose up to 3.
- **Highest-leverage next move.** Your recommendation. 2–3 candidates
  with tradeoffs, then your pick with a one-paragraph argument.
  Explicitly name what would cause you to change the pick.

## Do not

- Do not apply a transition that does not clear Mode A Step 5's test.
  Cleared transitions are applied in this pass and listed under
  APPLIED; everything else stays a proposal for the user to triage.
- Do not apply anything from Part B. Part B is judgment written to be
  read by a human, and none of its recommendations are transitions
  this pass may enact.
- Do not invent offenders or fabricate citations. Every claim in
  Part B traces to a Part A row or a discussion id.
- Do not recommend force-applying a blocker. If `vault_check_transition`
  returns blocked, your proposal says "would require resolving
  <blocker>" — and the follow-up is on the user.
```

### Step 4 — Apply, commit, and merge

If on `main`, branch before editing and say so. Apply eligible transitions
by editing frontmatter, run repository validation, and inspect the diff.
Record only actual, validated edits as APPLIED.
If finalizing the note changes it after validation, validate the final note
and edits again before committing.

If validation fails after applying, revert those transition edits. Move
them to PROPOSED with the validation failure and update Needs you. Validate
the resulting note-only state and inspect the diff again. If it still fails,
report the failure and stop; do not merge an unvalidated note.

Commit the note and validated edits together, then push:

`git add discussions/<id>.md <edited artifacts> && git commit -m
"Reconciler dream: <YYYY-MM-DD>"`.

Use one commit so the note records the edits it accompanies. After a
successful rollback and revalidation, commit the note alone.

Open or update the branch's PR and merge once validation and host
requirements pass. Dream runs unattended; the PR records its autonomous
writes. PROPOSED items need human decisions, but do not hold up the validated
note and edits. Deliver those items through Needs you in the run notification.

### Step 5 — Stop

After merging or reporting a validation failure or host refusal, stop.
Do not chain. If the host refuses the merge, leave the PR open and report
its requirement; do not claim it merged.

## Shared boundaries

Never bypass a blocker, warning, branch protection rule, or failed check.
Never commit directly to `main`, use `force_accept`, force-push, or
force-merge. A host refusal requires review, not a workaround.

SCHEMA.md's escape hatches — `withdraw_exploration` and `abandon_inquiry` —
are explicit and logged. It has no `force_accept` or `override_blocking`:
"You can get unstuck; you can't pretend something is resolved when it isn't."
This prohibits bypassing invariants; it does not determine who may apply
transitions that satisfy them.

Ambiguity requires review of the affected transition. In interactive mode,
leave landing to the branch owner unless the user requests it. In dream
mode, merge the validated result with unresolved proposals in the note.
