---
name: frame-problem
description: |
  Draft a **problem brief** — the "what actually is the problem" half of the paired briefs convention in knowledge-iop. A problem brief is written before any solution is proposed. Use this whenever the user says things like "we need a problem brief", "let's frame the problem", "what are we even solving", "capture the constraints", "I want to document a problem we can't solve yet", or any time they're about to design something but the problem hasn't been stated in one place. A problem brief can be orphan-of-solution — it's valuable even without a design. After framing a problem, the natural next step is `propose-design` (which `frames:` this brief).
---

# Frame problem

You are drafting a **problem brief** in the knowledge-iop vault. A
problem brief answers "what is actually going on here, and why does it
resist easy solution?" — before anyone proposes a fix.

## Before drafting — search first

1. Invoke the `vault-search` skill (or call `vault_search` directly)
   with keywords from the user's topic to find any existing
   problem-brief on this subject.
2. If a match exists with `status: draft`, offer to continue editing
   it instead of starting over — paired-brief convention favors
   amending a draft over proliferating near-duplicates.
3. If a match exists with `status: accepted`, ask the user whether
   they mean to **supersede** it (new understanding of the same
   situation) or whether this is a different problem that just shares
   keywords. Accepted briefs are immutable; new ones supersede.
4. If a match exists with `status: obsolete`, warn the user — that
   framing was already set aside; offer to supersede and surface the
   reason.

## Frontmatter shape

```yaml
---
id:            <YYYY-MM-DD>-<short-slug>-problem
type:          problem-brief
status:        draft
author:        <person>
created:       <YYYY-MM-DD>
arc:           <arc-id | omit if orphan-of-arc>
scopes:        [<scope-id>, ...]
supersedes:    <id | null>
relates_to:    [<id>, ...]
conflicts_with: [<id>, ...]
depends_on:    [<id>, ...]
tags:          [<tag>, ...]
---
```

Required: `id`, `type`, `status`, `author`, `created`.
Recommended: `scopes`, and `arc` if one is open that this belongs to.

The `id` is the stable handle. Convention: `<date>-<slug>-problem`.
`slug` is 2–4 hyphenated words, lowercase, no filler.

## Filename

Write to `briefs/<id>.md`. The `briefs/` directory is flat; don't nest.

## Body — the 6 questions

A problem brief answers, in this order:

1. **What is the actual problem?** State it before any solution is in
   mind. If you can't say it without saying "so we should …", keep
   refining.
2. **Why does it matter now?** What has changed that makes this worth
   writing down today?
3. **What constraints are we operating under?** Hard limits — budget,
   compatibility, team capacity, deadlines. Soft limits — aesthetic,
   strategic, cultural.
4. **What have we tried or considered?** Include dead ends. Documenting
   a rejected alternative is as valuable as proposing a new one.
5. **What are we explicitly not trying to solve?** Adjacent problems we
   acknowledge but are setting aside. Name them so the scope doesn't
   drift.
6. **What would change our analysis?** If the answer is "nothing", the
   brief is over-claimed. Name the conditions that would reopen the
   question.

Write in complete sentences. No bullet-ridden skeletons. A future
reader — human or agent — should be able to infer the situation
without further context.

## Drafting workflow

1. Search the vault (above).
2. Ask the user the 6 questions if answers aren't already in the
   conversation. Don't ask all at once — work through them.
3. Propose frontmatter + body as a diff. Show the filename you plan
   to write.
4. On user approval, write the file.
5. Commit via `git add briefs/<id>.md && git commit -m "Frame: <one-line>"`.
6. Tell the user the next step: "invoke `propose-design` now to
   propose a solution that `frames:` this brief." Mention that if
   no design is forthcoming, a standalone problem brief is still
   valuable.

## After commit

- Do **not** run `vault_reindex` — the index auto-refreshes on next
  query.
- If the user wants to check downstream impact (are there drafts
  elsewhere that relate?), invoke `vault-search` with the new id.

## Do not

- Do not include a "proposed solution" section. That's what
  `propose-design` is for. Keep the two halves separate.
- Do not write "TBD" fields. Either answer a question or drop the
  frontmatter field.
- Do not edit an accepted problem brief — supersede it instead.
