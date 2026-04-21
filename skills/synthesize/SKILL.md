---
name: synthesize
description: |
  Write the **synthesis** that closes an inquiry — read every submitted exploration, reconcile them against the framing, and produce the inquiry's resolved take. Use this when the user says things like "the barrier has released", "let's synthesize the inquiry", "we have all the explorations now", "resolve inquiry X". This skill assumes the barrier condition is met and all intended explorations have been submitted; it will refuse to synthesize a `status: open` inquiry. Synthesis is the only artifact that reads *across* explorations — everything before it is isolated.
---

# Synthesize

You are writing the **synthesis** that closes an inquiry. This is
the moment the isolation invariant releases: read every submitted
exploration, reconcile them against the framing, and produce the
inquiry's resolved take.

## Before writing — verify the barrier has released

1. Ask the user for the inquiry id if not given.
2. Call `vault_search` with `type: inquiry` to locate it, or
   `vault_edges` on the id.
3. Check status:
   - `open` → **do not synthesize.** The inquiry owner needs to
     transition it to `ready_for_synthesis` (after verifying the
     barrier condition) before synthesis is legitimate. Tell the
     user and stop.
   - `ready_for_synthesis` → proceed.
   - `resolved` → a synthesis already exists; offer to read it
     instead.
   - `abandoned` → the inquiry was explicitly set aside; do not
     synthesize.
4. List all explorations under `inquiries/<id>/explorations/`. Group
   by status:
   - `submitted` → included in synthesis
   - `withdrawn` → mention in synthesis (explain why they were
     withdrawn if the author noted it), but do not weigh their
     content
   - `draft` → **problem.** The inquiry should not be in
     `ready_for_synthesis` while any participant is still in draft.
     Flag to the user and stop unless they explicitly confirm.

## Frontmatter shape (synthesis.md)

```yaml
---
id:           <inquiry-id>-synthesis
type:         synthesis
status:       draft                   # transitions to accepted on sign-off
author:       <who wrote it>
inquiry:      <inquiry-id>
created:      <YYYY-MM-DD>
cites:        [<exploration-id>, <exploration-id>, ...]
---
```

`cites:` must include every **submitted** exploration (not the
withdrawn ones). The synthesis is accountable to each one — it
doesn't have to agree, but it must engage.

## Filename

Write to `inquiries/<inquiry-id>/synthesis.md`.

## Body

A synthesis is not a tournament bracket. Structure:

1. **The resolved take.** Lead with the answer — what the inquiry
   concludes, in a paragraph. A reader who stops here knows where
   the inquiry landed.
2. **How the explorations lined up.** For each submitted
   exploration, one paragraph: what it argued, where it agreed with
   others, where it diverged. Cite by id.
3. **Where the synthesis differs from each.** The take isn't any
   one exploration — say where it borrows, where it rejects, where
   it goes beyond. Be specific; if the synthesis matches one
   exploration wholesale, ask whether the inquiry was actually
   needed.
4. **Withdrawn or absent voices.** Name explorations that were
   withdrawn and why. Name participants who never submitted. The
   synthesis should make the gaps visible, not paper over them.
5. **What this resolves — and what it doesn't.** Explicitly close
   the inquiry's question. Note any follow-on problems that the
   inquiry surfaced and couldn't resolve — those may warrant new
   briefs or inquiries.

Quote sparingly. The explorations are in the vault; reference them
rather than excerpt.

## Drafting workflow

1. Verify the barrier has released (above). Abort if not.
2. Read every `submitted` exploration. Take notes — this step is
   irreversible; you'll have read them whether you commit or not.
3. Draft the synthesis. Start with the resolved take, then fill in
   the exploration-by-exploration discussion.
4. Propose frontmatter + body as a diff.
5. Write on approval.
6. Commit: `git add inquiries/<id>/synthesis.md && git commit -m
   "Synthesize: <one-line>"`.

## After the synthesis

1. Offer to transition the inquiry from `ready_for_synthesis` →
   `resolved`. Invoke `vault_check_transition` on the inquiry id
   with `new_status: resolved` to check for blockers.
2. On transition, update the inquiry's frontmatter: `status:
   resolved`, `closed: <today>`. Commit.
3. If the synthesis points at follow-up artifacts (new problem
   brief, new inquiry, a design brief to pursue), name them as
   candidate next steps for the user — do not auto-draft them.

## Do not

- Do not synthesize an `open` inquiry. The isolation invariant
  depends on the barrier being explicit.
- Do not produce a synthesis that is identical to one of the
  explorations. If that's the outcome, the inquiry didn't need
  parallel exploration — note that honestly.
- Do not silently drop withdrawn explorations from the narrative.
  Their withdrawal is itself information.
