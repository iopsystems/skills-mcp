---
status: shipped
opened: 2026-09-01
updated: 2026-09-01
---

# A ladder for how much to write

## Goal

Give the repository a rule against its own verbosity. Every skill defers to
`technical-prose` for word choice, and nothing ruled on how much material earns
space, so skills grew by paragraphs nobody could point at as wrong.

## Scope

`skills/repository/technical-prose/SKILL.md`, and the duplication the new
section exposed in `sweep-comments`. No change to modality, empty vocabulary,
one-name-per-thing, spelling, or the untouchables.

## Evidence

Reported after a 376-word section was added to `review-guide` to say "use plain
words" — a rule against over-explaining, written as an over-explanation. Cutting
it to 151 lost nothing: what went was an audience the rule implied, a metaphor
restating the rule, and sentences restating their own guards.

A survey of the twenty-four skills found the pattern is concentrated rather than
uniform. Four skills — `review-guide`, `sweep-comments`, `dataflow-diagram`,
`architecture-diagram` — hold 18,977 of the 41,480 words of skill body, and 22
of the 30 paragraphs longer than eighty words. Every vault skill is under 900
words. The heavily-iterated skills are the verbose ones, which is what accretion
looks like when each addition is individually defensible.

## Design and Implementation

The ladder comes from [ponytail](https://github.com/DietrichGebert/ponytail),
the same source `sweep-comments` borrowed its comment-form ladder from. Six
rungs — nothing, a word, a clause, a sentence, a paragraph, a section — walked
before writing and again when cutting, stopping at the first that carries the
fact intact.

Placing it required amending the skill's own boundary. `technical-prose` opened
by claiming word choice and disclaiming sentence shape, with an argument from
what travels between readers: a word carries the same fact everywhere, while a
doc comment and a README need different grammar. The ladder is neither. It
passes the skill's own test — a paragraph restating the paragraph above it
carries no fact in any context — so the boundary now reads as **what carries a
fact**, at the scale of a word and of the material around it, still disclaiming
shape.

Three rules travel with the ladder. **Be lazy about the prose, never about the
thinking** is ponytail's own, and guards against terseness substituting for
understanding. **Cut assertions before examples** is the observation from the
`review-guide` cut: the example survived every pass because it shows where a
paragraph asserts. And the never-cut list — modality, the subject of a claim,
negations, scope qualifiers, quoted evidence, untouchables — is ponytail's
non-negotiables in this domain.

The last rule is the one the repository needed: **a document that states a bar
is subject to it.**

Writing it exposed a duplication. `sweep-comments` had stated the upward-drift
rationale and the be-lazy-about-form rule in its own words, and both are now
general. Its ladder points at `technical-prose` for the reason and keeps only
its comment-specific rungs, which is the echo rule that skill itself enforces.

## Outcome

Shipped. Full suite, clippy, formatter, and the three CI scripts pass.

The section is 278 words and the deduplication removed 65 from
`sweep-comments`, so the change is roughly cost-neutral on total prose. It is a
rule, not a cut. Whether the four heavy skills come down is a separate effort
with its own review.

## Derived Documents

None.

## Deferred or Reopen Items

- The four heavy skills are unswept. `review-guide` at 6,385 words with seven
  paragraphs over eighty is the obvious first, and a cut there risks losing
  rules rather than prose, so it wants reviewing skill by skill rather than in
  one pass.
- Nothing measures a skill against the ladder. A test could flag paragraphs over
  a length, but length is not the defect — restatement is, and no check
  distinguishes a long paragraph carrying four facts from a short one carrying
  none.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
