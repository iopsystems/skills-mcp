---
name: record-decision
description: |
  Record a **decision** — the "because" that cites one or more design briefs (each of which frames a problem brief). Three-hop traceability: why do we do this? → this design → because of this problem. Use this when the user says things like "record the decision", "capture what we decided", "let's log this decision", "add a decision", or after a design brief has been accepted and the user wants to capture the downstream commitment. Decisions are written by humans (with agent help); briefs are written by agents. Keep that boundary firm. A decision cannot be `accepted` while any of its `derived_from` design briefs is not accepted — this skill invokes `vault_check_transition` to enforce that before writing.
---

# Record decision

You are recording a **decision** in the knowledge-iop vault. A
decision cites the design briefs that informed it via `derived_from:`.
Briefs are the *what*; decisions are the *because*.

## Before writing — verify the chain is ready

1. Ask the user which design briefs this decision is derived from.
   At least one; usually one, occasionally more.
2. For each design-brief id, call `vault_search` (or `vault_edges`)
   to confirm it exists and get its status.
3. If any is not `accepted`, stop and tell the user:
   "decisions cannot be accepted while any `derived_from` design
   brief is not accepted." Offer options:
   - Accept the design brief first (via `vault_check_transition`
     pre-flight + a status-change commit).
   - Record this decision as `status: draft` for now and promote to
     `accepted` once the briefs land.
4. For each design brief, also surface its `frames:` problem brief
   so the user can confirm the full chain: problem → design →
   decision.

## Frontmatter shape

```yaml
---
id:            <YYYY-MM-DD>-<short-slug>
type:          decision
status:        accepted                   # or draft if waiting on briefs
author:        <person>
created:       <YYYY-MM-DD>
derived_from:  [<design-brief-id>, ...]   # REQUIRED, at least one
arc:           <arc-id | omit>
scopes:        [<scope-id>, ...]
relates_to:    [<id>, ...]
---
```

`derived_from:` is required. Without it, this isn't a decision —
it's an opinion. If the user truly can't name a design brief, push
back once; if they insist, the right thing is usually a session note
or discussion, not a decision.

## Filename

Write to `decisions/<id>.md`. The decisions directory is append-only
in spirit — do not edit prior decisions.

## Body

A decision is short by design. Usually under a page. Cover:

1. **What we decided.** One clear sentence.
2. **Why — grounded in the briefs.** Name the design brief, then in
   one paragraph explain why its approach is the one being adopted.
   Cite the framed problem for context.
3. **Scope of commitment.** What does this decision commit us to,
   for whom, for how long? If the commitment is revisitable,
   under what condition.
4. **What we decided against.** If the design brief considered
   alternatives, one line per rejected alternative and why.
5. **Consequences to watch.** What should change because of this
   decision. What will go wrong if it goes wrong.

If the decision is long, it's probably actually a design brief.
Decisions cite; they don't elaborate.

## Pre-flight the transition

Before committing with `status: accepted`, invoke
`vault_check_transition` with the decision's id and
`new_status: accepted` to catch:

- derived-from briefs not yet accepted
- any other cascading violations

If blockers come back, address them first. If you write the file
before transitioning, use `status: draft` and commit; promote later.

## Drafting workflow

1. Verify the chain (above). Do not proceed past a blocker.
2. Draft the decision. Short.
3. Propose frontmatter + body as a diff.
4. Write on approval.
5. Commit: `git add decisions/<id>.md && git commit -m "Decide:
   <one-line>"`.

## Reversing a decision

Decisions can be `reversed` (not deleted). If the user asks to undo
a previous decision:

1. Do not edit the original. It stays in history.
2. Write a new decision that `supersedes: <old-id>` with
   `status: accepted`, explaining what changed.
3. Update the original's frontmatter: `status: reversed`,
   `superseded_by: <new-id>`. Commit that edit separately.

The trail shows both the original and the reversal — which is the
point of an append-only decision log.

## Boundary: agents vs humans

Per SCHEMA.md: "Agents write briefs. Humans (with agent help) write
decisions. Keep that boundary firm."

When this skill runs:
- You draft the decision based on the conversation and briefs.
- The user is the one committing. Present the diff; they approve.
- If the user asks the agent to pick the decision, push back once:
  "decisions are a commitment; you should be the one making it.
  I can draft it for your review."

## Do not

- Do not write a decision without `derived_from`. Refuse politely.
- Do not edit a prior decision. Supersede + reverse instead.
- Do not expand a decision into design-brief territory. Cite the
  brief; let the reader click through.
