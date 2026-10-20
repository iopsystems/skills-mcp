---
name: propose-design
description: |
  Draft a **design brief** — the "how do we solve it" half of the paired briefs convention in knowledge-iop. Every design brief must declare `frames:` pointing at a problem brief (or `frames: none` with a reason, discouraged). Use this whenever the user says things like "I need a design brief", "let's design a solution for X", "propose a design", "draft the design half", "how should we approach this". If no matching problem-brief exists in the vault, this skill will direct you to `frame-problem` first — solutions without framing are the anti-pattern the vault is built to prevent.
---

# Propose design

Draft a design brief in the knowledge-iop vault, paired with a problem brief
through `frames:`.

## Find the frame and related designs

Invoke `vault-search` with topic keywords before drafting:

- **No matching problem:** invoke `frame-problem` first, then resume here.
- **One draft or accepted problem:** use it as the frame.
- **One obsolete problem:** explain that the framing was set aside. Ask whether
  to supersede it through `frame-problem` or choose another frame.
- **Multiple matches:** ask which applies, listing ids, statuses and titles.

`frames:` is required. Use `frames: none` only at the user's explicit request
with a free-text reason; push back once before accepting that exception.

Find related designs with `vault_edges` on the problem id, `kind: frames`,
`direction: incoming`. Determine whether the proposed design is an alternative,
a successor, or different work. Do not edit an accepted design brief; write a
new one with `supersedes: <old-id>`.

## Frontmatter and filename

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

Write to `briefs/<id>.md`. Use `<date>-<slug>` with the creation date and no
`-design` suffix; `type:` identifies the brief.

## Body

Answer these six questions for a reader who has the problem brief available.
Reference its constraints instead of repeating the problem statement.

1. **Approach:** open with one paragraph explaining the proposed solution
   before any subsections.
2. **Rationale:** connect the choice to named constraints in the problem brief.
3. **Concrete design:** show APIs, schemas, workflows, file layouts or sequences.
   Prefer examples over specifications.
4. **Cost:** give effort, risks, compatibility effects and required participants.
   If an estimate is unknown, say what would make estimation possible.
5. **Alternatives:** discuss two or three and why each was set aside. Include
   doing nothing when it is a viable option.
6. **Reconsideration:** state what would cause abandonment or supersession.

## Draft, commit and report

1. Complete the framing and related-design search above. Use supplied answers
   before asking questions. Ask only about missing information that affects
   the design, including unknown authorship; do not require another discovery
   round when the context is sufficient. If the user requests a draft with
   gaps, mark them explicitly rather than inventing answers.
2. Present the filename and frontmatter plus body as a diff. Write only within
   the user's authorization. If writing and committing this brief are already
   authorized, proceed without asking again; otherwise obtain approval first.
   Carry applicable authorization from a calling skill such as `plan-feature`.
3. Commit with `git add briefs/<id>.md && git commit -m "Design:
   <one-line>"`.
4. If the user requests promotion to `proposed` or `accepted`, invoke
   `vault_check_transition` with the new status first. Resolve its blockers
   before promoting; writing a draft does not authorize promotion.
5. Report the actual draft or commit state and any unresolved questions or
   promotion blockers. If a decision follows from this design, mention
   `record-decision` with `derived_from:` pointing at the brief.
