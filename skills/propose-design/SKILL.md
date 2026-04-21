---
name: propose-design
description: |
  Draft a **design brief** — the "how do we solve it" half of the paired briefs convention in knowledge-iop. Every design brief must declare `frames:` pointing at a problem brief (or `frames: none` with a reason, discouraged). Use this whenever the user says things like "I need a design brief", "let's design a solution for X", "propose a design", "draft the design half", "how should we approach this". If no matching problem-brief exists in the vault, this skill will direct you to `frame-problem` first — solutions without framing are the anti-pattern the vault is built to prevent.
---

# Propose design

You are drafting a **design brief** in the knowledge-iop vault. A
design brief is always paired with a problem brief via `frames:`.
Without the pair, the solution loses its "why" as soon as the context
evaporates — which is exactly what this convention exists to prevent.

## Before drafting — search for the paired problem

1. Invoke `vault-search` with keywords from the topic to find the
   matching problem brief.
2. **Zero matches** → chain to `frame-problem` first. Do not write a
   design brief with `frames: none` unless the user explicitly asks
   for it and has a reason; the escape hatch is deliberately annoying
   to use. Resume here once the problem brief exists.
3. **One match, status `accepted` or `draft`** → this is your frame.
   Proceed.
4. **One match, status `obsolete`** → the problem framing was set
   aside. Ask the user whether to (a) supersede the obsolete problem
   with a new framing (invoke `frame-problem` with `supersedes:`) or
   (b) pick a different frame. Don't silently frame from an obsolete
   problem.
5. **Multiple matches** → ask the user which frame applies, listing
   them with ids, status, and one-line titles.

Also search for existing **design briefs** on the topic via
`vault_edges` on the problem-brief id with `kind: frames`,
`direction: incoming`. A problem brief may have multiple designs —
check if yours is a competing alternative, a successor (supersedes),
or genuinely different.

## Frontmatter shape

```yaml
---
id:            <YYYY-MM-DD>-<short-slug>
type:          design-brief
status:        draft
author:        <person>
created:       <YYYY-MM-DD>
frames:        <problem-brief-id>   # REQUIRED
arc:           <arc-id | omit if orphan-of-arc>
scopes:        [<scope-id>, ...]
supersedes:    <id | null>
relates_to:    [<id>, ...]
conflicts_with: [<id>, ...]
depends_on:    [<id>, ...]
tags:          [<tag>, ...]
---
```

`frames:` is required. Without it, this is not a design brief — it's
just opinions. If you truly can't frame it, the user must pass
`frames: none` with a free-text reason; push back once before
accepting that.

## Filename

Write to `briefs/<id>.md`. Convention for slugs: `<date>-<slug>`
(no `-design` suffix — the `type:` field distinguishes it from the
problem brief). The date is the creation date.

## Body

A design brief answers:

1. **What's the proposed approach?** One paragraph before any
   subsections. A reader should know the shape of the answer after
   reading this alone.
2. **Why this approach?** Cite the problem brief's constraints by
   name. If the brief's reasoning doesn't lean on the framing, the
   pairing isn't earning its keep — either the framing is wrong or
   the design is unmoored.
3. **What does it look like?** APIs, schemas, workflows, file
   layouts, sequences — whatever makes it concrete. Prefer examples
   over specs; specs age faster.
4. **What does it cost?** Effort estimate, risk, what it breaks, who
   must be involved. If you can't estimate, say so and note what
   needs to happen to enable estimation.
5. **What alternatives did you consider?** Two or three, with why
   each was set aside. Include "do nothing" if it's on the table.
6. **What would change the analysis?** The conditions under which
   this design would be abandoned or superseded.

Write assuming a reader who has the paired problem brief open next to
this one. Don't restate the problem — reference it.

## Drafting workflow

1. Search the vault; confirm or chain to `frame-problem`.
2. Ask the user clarifying questions. A good design brief is 30%
   drafting, 70% questioning.
3. Propose frontmatter + body as a diff with the filename.
4. Write the file on approval.
5. Commit with `git add briefs/<id>.md && git commit -m "Design:
   <one-line>"`.
6. If the user wants to move this to `proposed` or `accepted` next,
   invoke `vault_check_transition` with the new status first — it
   will flag any blockers (framed problem still draft, depends_on
   inquiry still open).

## After commit

- Invoke `vault_check_transition` before promoting status — it
  catches common cascading violations.
- If the design will result in a decision, mention that the user can
  invoke `record-decision` later with `derived_from:` pointing at
  this brief.

## Do not

- Do not write a design brief without `frames:`. If you catch
  yourself about to, stop and invoke `frame-problem` instead.
- Do not edit an accepted design brief. Write a new one with
  `supersedes: <old-id>`.
- Do not include the problem statement in the body — that lives in
  the problem brief. Link, don't duplicate.
