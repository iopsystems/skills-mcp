---
status: shipped
opened: 2026-09-01
updated: 2026-09-01
beta_skills: [review-guide]
---

# A guide explains before it names

## Goal

Make `review-guide` produce pull-request bodies a reader outside the subsystem
can act on. It ruled on what a guide answers, in what order, and at what length,
and said nothing about the register those answers are written in — so guides
came out in the vocabulary of the change's own design discussion.

## Scope

`skills/repository/review-guide/SKILL.md` and its corpus. No change to the
publish test, the always-present answers, the decision ordering, the mental
model, or the proportionality rule.

## Evidence

Reported twice by the reader of this skill's output, on consecutive pull
requests. Both guides were rewritten from the technical register into plain
language and both became materially clearer: one went from 404 words to 729,
the other from 404 to 771.

The defect is specific. A guide would write "required PR checks were part of
the clean-run gate" — accurate, and it asks the reader to unpack a phrase to
recover a fact the sentence could have handed over. The rewrite says that a
pull request runs automatic checks before a change lands, and that those checks
were one of four conditions the skill required. That version is better for the
reader who already knows the term, because they no longer pay the unpacking.

Nothing in the skill covered this. `Boundaries` defers word choice to
`technical-prose`, which owns modality, empty vocabulary, and one-name-per-thing
— all about which words are chosen, none about whether the reader holds them.
The `No identifiers` rule guards the summary only, and the gloss rule at
`Before, problem, change` covers identifiers rather than terms of art.

## Design and Implementation

A section, `Explain it, do not name it`, placed immediately before `Boundaries`
so it reads next to the skill it does not belong to.

The rule is to say what a thing does and then name it, on the argument that a
name is a handle and a guide that offers the handle without the thing has given
the reader a lookup. It applies hardest to vocabulary the change itself
invented — a phrase coined in a design discussion is jargon the rest of the
team does not hold yet, and it is the phrase an author is least likely to
notice using.

Two guards, because the rule invites two opposite failures.

Against vagueness: every number, identifier, path, measurement, and quoted
error stays exactly as precise as it was. The register changes and the claims
do not. A guide that reads easily and can no longer be checked has failed twice
— once as prose and once as evidence.

Against a collision with proportionality, shipped six days ago in
`2026-08-25-review-guide-proportional.md`: plain language runs longer, and both
observed rewrites grew by about eighty percent. That rule counts answers, never
words, so three plain sentences replacing one dense clause remain one answer.
Padding is material with nothing in it, not material a reader can follow.

The test is whether a competent engineer outside the subsystem would know what
to do after reading the guide. Three red flags and two rationalization rows name
the shapes. Corpus goes from fifty-five cases to fifty-nine.

The section was 376 words on first writing, and the reviewer asked whether a
rule about not over-explaining needed that much prose. It did not: the opening
paragraph established an audience the rule implies, one metaphor restated the
rule twice, and two sentences restated their own guards. Cut to 151 words with
the example, both guards, and the test intact. The example survived every cut,
because it is the only part that shows rather than asserts.

## Applying the ladder to the file that states it

`2026-09-01-prose-ladder.md` merged first, so the rule existed before this
change landed and the file was swept to it rather than only extended by it.

Ten paragraphs were cut, none of which lost a rule: a bolded-rule and list-item
diff before and after shows the same set, with one bold marker reflowed across a
line break. What went was restatement. The two mental-model sections each stated
that the model sits after the TL;DR and before any identifier, so the ordering
section keeps the placement and the content section keeps the concrete reason. A
144-word decisions paragraph carried an example restating the sentence after it
and a closing clause restating the ordering rule it cited. A stack paragraph
carried the section's own motivation, which belongs at the section's opening.

6,815 words to 6,588 — 227 cut, about 3%. Paragraphs over eighty words went from
seven to five, and the five that remain are dense rather than padded: each
carries several distinct facts. That is the limit the ladder names and no check
resolves, since length is not the defect and restatement is.

## Outcome

Shipped. `cargo test --locked`, `cargo fmt --all -- --check`, and clippy pass.

The evidence is two rewrites of two guides by one reader, and both rewrites were
requested rather than produced by the rule. Whether the rule makes a first draft
land there is unmeasured.

## Derived Documents

None.

## Deferred or Reopen Items

- The two rewrites both grew by roughly eighty percent. If that is typical, the
  proportionality rule and this one are in more tension than the text admits,
  and the honest fix would be a worked example of a guide that is both plain and
  short rather than a sentence asserting they compose.
- Nothing measures whether a reader outside the subsystem could act on a guide.
  The test is stated for an agent to apply to its own draft, which is the reader
  least able to judge it.

## Skill Feedback

### review-guide (beta)

- **Friction** — the skill had no rule about register. Asked for a pull-request
  body, it produced accurate sentences in the vocabulary of the change's own
  design discussion, twice in a row, and both times the reader asked for a
  plain-language rewrite that was clearly better. `Boundaries` had delegated
  word choice to `technical-prose`, which rules on which words are chosen and
  not on whether the reader holds them, so the gap sat between two skills that
  each looked complete.
- **Friction** — the section that adds this rule was itself written at 376
  words and cut to 151 after the reviewer asked whether a rule about
  over-explaining needed that much prose. Everything removed was elaboration the
  skill's own bar forbids: an audience the rule implies, a metaphor restating
  the rule, sentences restating their own guards. The skill states the bar for
  the guides it produces and nowhere applied it to its own text, and this was the
  second edit this month where the fix was cutting what the skill would have cut
  from a guide. Now closed generally rather than here:
  `2026-09-01-prose-ladder.md` puts the ladder in `technical-prose`, which every
  skill defers to, with the rule that a document stating a bar is subject to it.
- **Confirmation** — the proportionality rule did not have to be weakened to fit
  this. It counts answers rather than words, which turned out to be the formulation
  that survives a change making every answer longer. A word-count rule would have
  had to be rewritten.

## Appendix: Skills Invoked

- `review-guide` (beta) — this change's own pull-request body.
- `engineering-journal` — this entry.
