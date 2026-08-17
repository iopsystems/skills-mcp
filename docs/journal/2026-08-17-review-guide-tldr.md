---
status: shipped
opened: 2026-08-17
updated: 2026-08-17
beta_skills: [review-guide]
---

# Review guide opening: TL;DR and reader routing

## Goal

Give the guide an opening a reviewer can act on: one or two sentences of
summary, then a block explaining how to use the sections that follow. Both were
requested by the user after reading guides the skill produced.

## Scope

`skills/review-guide/SKILL.md` and its corpus. No change to the publish test,
the attention ranking, the certainty rules, or the citation guidance added in
pull request 32.

## Evidence

The body's first element was a paragraph stating what changed and the claim it
makes. A reviewer wanting only the gist had to read that paragraph and extract
it, which is work the author is better placed to do once. The skill already
required a single-sentence fallback when nothing cleared the publish test, so
the short form existed for the empty case and not for the ordinary one.

This is the first change to `review-guide` originating from a reader of its
output rather than from its author or from an automated review.

## Design and Implementation

The TL;DR sits above every heading and is capped at two sentences. It states
what changed and what the change asks of the reviewer, which is the part a title
cannot carry: approve on sight, look hard at one named thing, or settle one
decision. Writing it last is instructed, because which of the three it is
becomes clear only after the rest exists.

Two failure modes are named. A TL;DR restating the title spends the reviewer's
first two sentences on what they already read. A TL;DR that opens a list defers
the work rather than doing it. The section below it expands rather than repeats.

The single-sentence fallback is now identified as the TL;DR itself, so the two
short forms are one rule rather than two.

The routing block follows the TL;DR and answers a different question. The
summary says what the change is; the block says what each section wants from the
reviewer. One line per section actually present, each naming an action —
verify, decide, accept, skip — rather than the content, and closing with the
shortest useful path when a reviewer might reasonably read only part.

The block's failure mode is boilerplate: a list of section titles repeated on
every pull request becomes furniture a reviewer learns to skip, which is the
same defect the skill already names for padded sections. Three rules guard it.
It may name only sections this guide contains, every line must state an action
rather than a summary, and it must name at least one specific item the reviewer
should settle. A block of titles is a table of contents, and the reviewer has
one already. It is omitted entirely below three sections, where routing costs
more than it saves.

Corpus goes from twenty-two cases to twenty-six: the opening and its expansion,
a refused title restatement, routing rather than summarizing, and the block
omitted when there is nothing to route.

## Outcome

Shipped. `cargo fmt --check` passed, tests passed across the eight binaries
including the citation guard, and `cargo build --release` succeeded.

Whether either addition changes reviewer behavior is unmeasured. Both requests
came from one reader, and the skill's other rules remain unvalidated in the same
way. The routing block carries the higher risk of the two: it is the first
element the skill requires that says nothing about the change itself, so it pays
off only if reviewers use it rather than skip it.

## Derived Documents

None.

## Deferred or Reopen Items

- Demote the routing block once the format is familiar. The user accepted it as
  useful now while naming its expiry: readers who know the sections do not need
  to be told what each one asks, and at that point the block should move to an
  appendix or become optional. Reopen when reviewers stop reading it — visible
  as guides whose block goes unmentioned while its items are acted on anyway —
  rather than on a schedule. The argument for keeping it is that it teaches the
  format; that argument ends when the format is known.
- The prior entry for this skill is terminal, so this effort is recorded
  separately rather than reopening it. Both describe the same skill and a reader
  needs both.

## Skill Feedback

### review-guide (beta)

- **Friction** — the user, reading guides the skill produced, asked for a
  summary at the top. The body opened with a paragraph, and the gist had to be
  extracted by the reader. The skill already knew how to write one sentence, but
  only for the case where nothing was worth publishing. Fixed by requiring the
  opening in every guide.
- **Friction** — the same reader then asked how the sections were meant to be
  used. The skill specified what each section must contain and never said what
  it asks of the reviewer, so a guide could satisfy every rule and still leave
  the reader to work out which parts wanted a decision and which wanted a
  glance. Fixed with the routing block.
- **Confirmation** — both requests arrived as changes to the output contract
  rather than complaints about a specific guide, which is what the beta channel
  is for: the first feedback on this skill from someone who did not write it.
  Both gaps were invisible to its author and to two rounds of automated review.

## Appendix: Skills Invoked

- `review-guide` (beta) — used to draft this change's own pull-request body,
  followed from its source file rather than invoked as a tool, because this
  effort was editing that file.
