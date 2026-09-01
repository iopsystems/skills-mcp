---
status: shipped
opened: 2026-09-01
updated: 2026-09-01
---

# The first sweep was deferential

## Goal

Re-cut `review-guide` and `sweep-comments` from an adversarial stance: assume
the author over-wrote, and make them prove each paragraph.

## Scope

Those two skills. No rule removed, no template affected.

## Evidence

`2026-09-01-sweep-heavy-skills.md` reported 119 words cut across six skills and
concluded the files were dense rather than padded. The reviewer rejected the
conclusion: a single paragraph they had challenged directly came down sixty
percent, and the sweep that followed did not.

They were right, and the reason is in the method. That sweep ran two detectors —
paragraphs over eighty words, and a near-duplicate scan over every sentence pair
— and cut only what one of them flagged. Both detectors find *provable*
repetition. Neither asks the question an editor asks: does this sentence need to
exist?

Reading with that question produced 609 words from `review-guide`, 9%, against
the 119 the detectors found across six files.

## Design and Implementation

Nothing the second pass cut was a duplicate a scanner could have caught. The
recurring shapes:

**A sentence stating its own inverse.** "A stated default that fails teaches
more than a hedge that cannot" was preceded by three sentences establishing that
the rules are unmeasured.

**Justification stacking.** A rung of the comment ladder gave three reasons for
one instruction; one carries it.

**Restating the section's own thesis at its end.** The mental-model section
opened by saying a reviewer meeting a field name has nowhere to put it, then
closed with "only then can Where to look more closely name a type, because the
name now has somewhere to land."

**Pre-announcing a list that follows.** The body section said the answers are
not a list of headings to fill, spent a paragraph on the two failure modes, then
said it again before the list.

**Delegated rules restated in full.** The Boundaries section reproduced three of
`technical-prose`'s rules at length. What earns space is their local
application — "the measurement is the row" — not the rule.

**History as instruction.** "This is the failure that retired an earlier design,
in which the asks were listed together above the mental model" is a journal
entry's sentence in a skill.

Paragraphs over eighty words in `review-guide` went from five to two.
`sweep-comments` gave up 115 words, 1%, which is the honest ceiling for a file
whose long paragraphs each carry four or five distinct facts.

## Outcome

Shipped. Full suite, clippy, formatter, and the three CI scripts pass. The
citation guard repaired two line numbers the cuts moved.

No rule was lost. Three bolded spans changed shape — "One subsection per
decision" became "One per subsection", "Never a bare list of headings" folded
into the context-before-question rule that already implied it, and one bold
range shifted by a comma — and each survives as an instruction.

## Derived Documents

None.

## Deferred or Reopen Items

- The other four skills have not had this pass. The detectors were run over them
  and found little, which this entry establishes means little.
- The two detectors are still worth keeping, but their output is a floor rather
  than a result. Nothing in the repository distinguishes "I read this
  adversarially and it held" from "I ran a scan and it found nothing", and the
  first sweep's entry is now an example of the second reported as the first.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
