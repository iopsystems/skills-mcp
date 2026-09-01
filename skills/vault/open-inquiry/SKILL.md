---
name: open-inquiry
description: |
  Open a bounded parallel **inquiry** — the knowledge-iop vault's barrier-mode coordination primitive for cases where multiple people should explore a question independently before anyone synthesizes. Use this when the user says things like "let's explore alternatives", "I want N people to try this independently", "we need parallel drafts", "let's not converge too early", or when a problem has multiple credible approaches and premature convergence would foreclose options. An inquiry is a *directory*: framing up front, isolated exploration files, synthesis after the barrier releases. If the question has one clearly best answer already, use `propose-design` instead — inquiries are for genuine ambiguity.
---

# Open inquiry

You are opening a **bounded parallel exploration** in the
knowledge-iop vault. This is the barrier-mode coordination pattern:
multiple people (or multiple agent runs) each draft independently,
nobody synthesizes until the barrier releases.

The output is a directory:

```
inquiries/<inquiry-id>/
  inquiry.md          # this file — the framing
  explorations/       # empty initially — explorers submit here via `submit-exploration`
  synthesis.md        # written later via `synthesize` after the barrier releases
```

## Before opening — search first

1. Invoke `vault-search` with keywords from the topic. Check for
   existing inquiries on the same question — duplication defeats the
   barrier's purpose.
2. If a similar inquiry is already `open` or `ready_for_synthesis`,
   ask the user whether they mean to join it (via
   `submit-exploration`) rather than opening a parallel one.

## Frontmatter shape (inquiry.md)

```yaml
---
id:                  <YYYY-MM-DD>-<short-slug>
type:                inquiry
status:              open
initiator:           <person>
participants:        [<person>, <person>, ...]  # who is expected to submit
created:             <YYYY-MM-DD>
opened:              <YYYY-MM-DD>
barrier_condition:   <text — what must be true before synthesis>
arc:                 <arc-id | omit only with reason>
scopes:              [<scope-id>, ...]
relates_to:          [<id>, ...]
---
```

Almost every inquiry lives inside an arc. Orphan-of-arc is possible
but rare — ask the user to name the arc, and if none applies, consider
whether the arc needs opening first via `open-arc`.

## Directory creation

```
inquiries/<id>/
  inquiry.md
  explorations/       # create as an empty directory (add a .gitkeep if needed)
```

Do not pre-create exploration stub files for each participant — that
defeats the point. Each explorer creates their own via
`submit-exploration`.

## Body of inquiry.md — the framing

An inquiry's framing is problem-brief-shaped. Cover:

1. **The question.** What exactly should each explorer produce an
   answer to? Write it so two explorers reading the framing without
   talking to each other would answer the same question.
2. **Why now.** What's triggering the inquiry.
3. **Constraints.** Hard and soft, as in a problem brief.
4. **Out of scope.** What explorations should *not* attempt. Keeps
   drafts comparable.
5. **Barrier condition.** Explicit: "when all named participants have
   submitted, or by 2026-05-01 whichever is sooner", or "when at
   least 3 explorations are in and 2 weeks have passed". Vague
   barriers produce vague synthesis.
6. **Evaluation criteria.** What makes an exploration good —
   correctness, extensibility, cost, etc. This is the synthesis's
   rubric.

## Isolation invariant

**While the inquiry is `open`, explorations are isolated.** The vault
reader (this MCP server, future CI, future CLI) filters reads of
`inquiries/<id>/explorations/*` so only the exploration's own author
sees their draft. This is the *feature*, not a bug: duplicated effort
is what lets the synthesis see genuinely independent takes.

Do not preview anyone's draft. Do not share partial work between
explorers. If someone asks "what did the others come up with", the
answer is "you'll see at the synthesis".

## Drafting workflow

1. Search the vault; confirm no live inquiry already covers this.
2. Ask the user to name the participants and state the barrier
   condition. Push back on vague answers.
3. Propose frontmatter + body as a diff with the directory layout.
4. Create the directory + inquiry.md on approval.
5. Commit: `git add inquiries/<id>/ && git commit -m "Open inquiry:
   <one-line>"`.
6. Tell the user the next step: each participant invokes
   `submit-exploration` with this inquiry id.

## After commit

- If the arc didn't exist, offer to open it first via `open-arc`.
- Remind the user of the isolation invariant: don't peek at
  explorations until the barrier releases.

## Do not

- Do not open an inquiry for a question with one obvious answer —
  that's `propose-design` territory. Inquiries cost effort by design.
- Do not set `status: ready_for_synthesis` at open time. That
  transition happens when the barrier condition is met.
- Do not nest inquiries. If exploration reveals a sub-question
  worth its own inquiry, close this one or finish it, then open
  the next.
