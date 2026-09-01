---
name: reconcile-vault
description: |
  Reconcile the knowledge-iop vault. Two modes, same skill. **Interactive** — invoked after a phase skill commits an artifact; scoped to the touched artifact and its immediate graph neighborhood; applies and commits clean transitions by default. **Dream** — invoked by a Claude Max scheduled task; runs across the whole vault; writes a session-note with Part A (graph hygiene — deterministic) and Part B (strategic reflection — judgment), then commits it by default. Use whenever the user says things like "reconcile the vault", "what needs attention", "dream over the vault", "check for blocked transitions", "what's the state of the project", "run reconciliation". Route blockers, warnings, ambiguity, and failed validation to human review. A pull request is opened only when the user asks for one or a finding needs human review; the clean path commits on the current branch and stops. The vault has no force_accept and neither does this skill.
---

# Reconcile vault

You are reconciling the knowledge-iop vault. Pick your mode first.

## Mode selection

**Interactive** if any of the following applies:
- You were just invoked by a phase skill (frame-problem, propose-design,
  record-decision, synthesize, close-arc, ...) after a commit.
- The user names a specific artifact ("reconcile around <id>").
- The user just finished a discussion whose implications need triage.

**Dream** if:
- You were invoked by a scheduled task with no specific artifact in focus.
- The user says something like "dream over the vault", "what's been
  stalling", "give me the state of things".

If unclear, ask once — but default to interactive. Dream is expensive
(reads widely) and should be explicit.

---

## Mode A: Interactive

Scope: the artifact(s) just touched and their immediate graph neighborhood.

### Step 1 — Identify the pivot

- From the phase skill's output or the user's message, get the pivot
  artifact id(s).
- If unclear, ask: "which artifact did we just change?"

### Step 2 — Walk the edges

- Call `vault_edges` with `id: <pivot>`, `direction: both`. This returns
  everything connected to the pivot.
- Group neighbors by edge kind: `frames`, `supersedes`, `superseded_by`,
  `relates_to`, `depends_on`, `derived_from`, `arc`, `scopes`, `inquiry`.

### Step 3 — Propose transitions

For each neighbor, ask: "given the pivot's new state, does this neighbor
need a status change?" The common patterns:

| Pivot change | Neighbor pattern | Proposed transition |
|---|---|---|
| problem-brief → `obsolete` | design-briefs that `frames: <pivot>` | warn (not block); user may want to supersede |
| design-brief → `accepted` | decisions with `derived_from: <pivot>` previously blocked | now unblocked; user may promote to `accepted` |
| inquiry → `resolved` | parent arc (via `arc` edge) | if this was the last open inquiry, arc may be ready to close |
| design-brief → `superseded` | framed problem-brief | may still be relevant for the new design; no change |
| synthesis written | inquiry | transition inquiry `ready_for_synthesis` → `resolved` |
| new problem-brief drafted | existing design-briefs on the topic | may need `supersedes:` pointer |

### Step 4 — Pre-flight each proposed transition

For every transition you want to propose:

1. Call `vault_check_transition` with the neighbor's id and the
   candidate new status.
2. Attach the result (`allowed` + any `blockers` / `warnings`) to the
   proposal.

### Step 5 — Choose the disposition

Applying is the default. A run is clean only when every proposed
transition:

- is `allowed` with no blockers or warnings;
- cites direct, sufficient evidence;
- has exactly one justified next state; and
- passes repository validation.

Require human review if any transition is blocked or warned, evidence is
incomplete or ambiguous, multiple outcomes are plausible, or validation
fails.

Repository validation is the whole gate on the clean path. It used to
share the job with a pull request's required checks, and dropping the
pull request drops those, so a check that only ever ran in CI no longer
runs before the commit lands. Nothing else moves: an uncertain
transition is still not applied, and the review package below is still
produced.

### Step 6 — Apply or request review

For a clean run:

1. Apply every transition with a direct frontmatter edit.
2. Run the repository's validation and inspect the diff.
3. Commit and push the current branch, then stop.

Whatever opened the branch owns landing it — this skill rides along with
the phase skill's own change rather than opening a second one. Open a
pull request only when the user asked for one, and update rather than
duplicate an existing one for the branch.

Do not ask for confirmation on this path. Do not commit to `main`
directly, bypass branch protection, or force-push. When the current
branch is `main`, branch first and say so; a reconciliation that lands
unreviewed on the default branch is the ceremony being removed for the
wrong reason.

When human review is required, do not apply the uncertain or invalid
transition. Surface it where the user will see it: in an existing pull
request for the branch, left open, or in the session when there is
none. Produce a concise proposal list:

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

State the exact review reason for each item: blocker, warning, ambiguous
evidence, multiple plausible outcomes, or failed validation. Ask the user
to decide only those items.

### Step 7 — Stop

After the clean transitions are committed, or after the review package is
presented, interactive reconciliation is done. Do NOT cascade further — the next phase skill invocation will
trigger its own interactive reconciliation if needed.

---

## Mode B: Dream

Scope: the full vault. Write one session-note; do not mutate other artifacts
from this mode.

### Step 1 — Run the reflection report

Call `vault_reflect` with default windows (or windows the user
specified):

- `window_days: 30` — what counts as "recent activity"
- `min_days_stale_design: 14` — how old a `draft`/`proposed` design
  must be to flag
- `min_days_stale_arc: 60` — how quiet an `open` arc must be to flag

This returns structured data for Part A.

### Step 2 — Gather context for Part B

Part B is judgment, grounded in evidence. To ground it, additionally
call:

- `vault_search` with `type: discussion`, sorted by recency (take the
  last ~5 discussions or session-notes).
- `vault_search` with `type: decision`, sorted by recency (last ~5).

Keep token budget bounded — read titles and first paragraphs, not full
bodies, unless Part B specifically needs to cite something.

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

### Proposed transitions
For each gap that suggests a transition, call `vault_check_transition`
with the candidate new status and record the result. Do NOT propose
transitions you haven't pre-flighted.

## Part B — Strategic reflection

**This is judgment, not SQL.** The rules:

1. **Evidence-or-don't-say-it.** Every claim cites an artifact id or a
   number from Part A. Sentences like "things feel slow in X" without
   citation don't belong here.
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

- Do not commit any artifact edits from this pass. The dream pass
  writes ONE file: this session-note. Transitions are proposals for
  the user to triage.
- Do not invent offenders or fabricate citations. Every claim in
  Part B traces to a Part A row or a discussion id.
- Do not recommend force-applying a blocker. If `vault_check_transition`
  returns blocked, your proposal says "would require resolving
  <blocker>" — and the follow-up is on the user.
```

### Step 4 — Commit and choose the disposition

Commit the note and push the current branch:

`git add discussions/<id>.md && git commit -m "Reconciler dream:
<YYYY-MM-DD>"`.

Apply the shared review gate below. If the report contains no
review-required finding and repository validation passes, the commit is
the end of the run — open a pull request only when the user asked for
one. If the report contains a blocker, warning, ambiguous evidence,
multiple plausible outcomes, or failed validation, surface the exact
reason for human review, and leave open any pull request the branch
already has.

### Step 5 — Stop

After the note is committed, or after the review-required findings are
surfaced, stop. Do not chain.

---

## Shared rules

### Default to apply; gate on observable risk

Apply and commit clean, evidence-backed work by default. Human review is
required when any candidate has a blocker or warning, the evidence is
incomplete or ambiguous, more than one outcome is plausible, or validation
fails.

The default is applying, never landing. Committing on the current branch is
where a clean run ends; opening a pull request, merging one, and asking a
host to merge are all things this skill does only when the user asks.

Never bypass a blocker, warning, branch protection rule, or failed check.
Never use `force_accept`, force-push, or force-merge. A refusal from the
host is a review result, not permission to work around it.

That prohibition is what SCHEMA.md states. Its escape hatches —
`withdraw_exploration` and `abandon_inquiry` — are explicit and logged, and
there is deliberately no `force_accept` or `override_blocking`: "You can get
unstuck; you can't pretend something is resolved when it isn't." The sentence
governs bypassing an invariant, not who applies a transition that satisfies
every invariant. Do not cite it for either side of the question of who
applies.

### Token budget

Dream mode reads broadly. Cap what you load:

- Full `vault_reflect` output (~compact JSON).
- Titles + first paragraph only for the top 5 most recent
  discussions and decisions.
- Full body of no more than 2 artifacts, and only if you must cite
  them in Part B.

If the report balloons, your rubric is wrong. Cut.

### When in doubt, ask

If a proposal is ambiguous — two plausible next states, or the offender
evidence is thin — classify the run as human-review-required, name the
ambiguity, and leave open any pull request the branch already has.
Reconciliation that hallucinates certainty is worse than reconciliation
that names uncertainty.
