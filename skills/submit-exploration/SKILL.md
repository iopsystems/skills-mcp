---
name: submit-exploration
description: |
  Author a single **exploration** within an open inquiry — one explorer's independent take on the question, drafted in isolation from other explorers. Use this when the user says things like "I want to contribute to inquiry X", "let me write my take on this", "add my exploration", "submit my draft". An exploration is intentionally drafted without seeing others' work — that's the barrier-mode coordination pattern. Submission freezes the content; synthesis comes later via the `synthesize` skill once the inquiry's barrier condition is met.
---

# Submit exploration

You are authoring **one explorer's draft** within an inquiry. Other
explorers' drafts are hidden from you (and vice versa) until the
barrier releases. That isolation is the feature — treat it as such.

## Before drafting — verify the inquiry

1. Ask the user for the inquiry id if not given.
2. Call `vault_search` with `type: inquiry` and the id's keywords,
   or call `vault_edges` on the inquiry id to confirm it exists.
3. Check the inquiry's status:
   - `open` → good, proceed.
   - `ready_for_synthesis` → the barrier already released; new
     explorations don't count. Ask the user whether to abandon or
     invoke `synthesize` instead.
   - `resolved` / `abandoned` → the inquiry is over; do not submit.
4. Read `inquiries/<inquiry-id>/inquiry.md` for the framing,
   constraints, out-of-scope, and evaluation criteria. Do NOT read
   any existing exploration files — isolation invariant.

## Frontmatter shape

```yaml
---
id:         <explorer-name>-<short-slug>   # unique within the inquiry
type:       exploration
status:     draft
author:     <explorer name>
inquiry:    <inquiry-id>
created:    <YYYY-MM-DD>
submitted:  null
---
```

Leave `submitted: null` until the user says the draft is ready. When
they do, set `status: submitted`, `submitted: <today>`. Submission
freezes the content — subsequent edits become an anti-pattern.

## Filename

Write to `inquiries/<inquiry-id>/explorations/<id>.md`. Conventional
id format: `<author>-<slug>` (e.g. `alex-token-bucket.md`). One file
per explorer.

## Body — answer the framing, on your own

Structure is flexible; clarity over form. Cover:

1. **Your proposed approach.** One paragraph up front.
2. **Why this approach.** Reason against the framing's constraints.
3. **What it looks like.** Concrete enough that a reader can evaluate it.
4. **Cost and risk.** Honest assessment. Cheap wins lose credibility.
5. **What you considered and set aside.** Two or three alternatives,
   briefly.
6. **What you're unsure about.** Naming uncertainty helps the
   synthesis weigh takes against each other.

Write as if no one else is writing an exploration — don't try to
carve a niche against what you imagine others are doing. Good
exploration presumes independence.

## Drafting workflow

1. Verify the inquiry (see above). Do not peek at other explorations.
2. Read the framing. Ask clarifying questions of the user if the
   framing is ambiguous — but don't ask about other explorers.
3. Propose frontmatter + body as a diff with the filename.
4. Write on approval.
5. Commit: `git add inquiries/<inquiry-id>/explorations/<id>.md &&
   git commit -m "Explore: <one-line>"`.

## Submission

When the user says the draft is ready to submit (not just save):

1. Update frontmatter: `status: submitted`, `submitted: <today>`.
2. Commit separately: `git commit -m "Submit exploration: <id>"`.
3. Tell the user that submission freezes the content (content hash
   recorded); further edits become a withdraw+resubmit operation.

## Withdraw

If the user decides an exploration shouldn't be included in the
synthesis:

1. Update frontmatter: `status: withdrawn`, add a short reason in
   the body.
2. Commit.

This is an explicit, logged act. The vault has no `force_withdraw`
that hides the attempt — withdrawals stay in history.

## Do not

- Do not read other explorations while the inquiry is `open`. Not
  even "just to see". The vault reader filters these for you; don't
  go around it.
- Do not submit a "meta exploration" that synthesizes others' work
  before the barrier releases. That's `synthesize`, and it comes
  later.
- Do not edit a submitted exploration. Withdraw + resubmit if
  needed; the history stays visible.
