---
status: shipped
opened: 2026-08-18
updated: 2026-08-18
beta_skills: [review-guide]
---

# The look-out list, retired by the guides it produced

## Goal

Remove the section `review-guide` placed between the TL;DR and the mental
model, after a reader reported that its items ask about things the reader has
not been given yet.

## Scope

`skills/review-guide/SKILL.md` and its corpus. The publish test, the attention
ranking, the certainty rules, and the testing rules are untouched.

## Evidence

Six guides for one stacked series in another repository, read together rather
than one at a time.

The look-out list was legible in one of the six. In the others its items leaned
on concepts the mental model introduces two sections later: "the three ways a
stream can answer" names an enumeration the reader meets afterwards, and "with
both relay stamps the largest bus message reaches roughly its buffer's capacity"
is unreadable before the link is described.

Every item in every one of the six reappeared below, at greater length, under
`Judgment calls and low certainty` or `Production-only risks`. The section
totalled 854 words across the six against 1,335 for the certainty sections it
previewed. The rule said the reviewer meets each item twice; what the guides did
was state each item twice.

One guide's summary said "Two things want your opinion" above a list of three.

Measured on the same six: five of the six mental models are ~90% identical to
each other — the same four concepts and the same diagram, retyped per pull
request, about 1,650 words of duplicate text.

## Design and Implementation

The section is gone. The paragraph that closes the TL;DR carries the ask, which
it already did for one line; it now numbers the items when there is more than
one, and each number is marked again where its evidence appears.

That placement is what makes it readable. The TL;DR carries no identifiers by an
existing rule, so it is the one part of a guide written entirely in the domain's
terms — the only place an ask can be stated before the mental model without
depending on it. The ask therefore names each item and argues none of them: the
case is made below, under the same number, where the evidence is.

The empty case survives the move. "Nothing here needs a decision; the four
checks came back empty" now closes the summary instead of heading a section,
because an absent ask and an empty one still read identically.

The stack duplication is a second rule, in the mental-model section: write the
model in the base guide, link it by pull-request number from each guide above,
and state only the delta. The skill already said this about diagrams and not
about the prose the diagram sits in.

## Outcome

Shipped. The corpus went from forty-four cases to forty-eight, `cargo fmt
--check` passed, 150 tests pass, and the citation guard stayed green.

Whether the ask survives at the end of a paragraph rather than as a heading is
unmeasured. The failure it replaces was reported; this shape has not been read
by anyone yet.

## Derived Documents

None.

## Deferred or Reopen Items

- The six guides in the other repository were rewritten against the new shape in
  the same sitting, which is the only evidence it works, and by the author.
- Nothing checks that a numbered ask is marked again below, that a stacked guide
  links rather than retypes, or that a summary's count matches its list. The
  count mismatch found here is the third of its kind in this skill's history.

## Skill Feedback

### review-guide (beta)

- **Friction** — a section this skill introduced to make asks unmissable made
  them unreadable instead, because it placed them ahead of the vocabulary they
  are written in. The rule that produced it — meet each item twice — was stated
  without saying what the first meeting may contain.
- **Friction** — the skill has now had a section added and retired twice in five
  days: the routing block, and this. Nothing in it distinguishes a provisional
  rule from a settled one, which was recorded as a reopen item on the first
  occasion and has not been acted on.
- **Confirmation** — reading six guides together found what reading them one at
  a time did not. The duplicate mental models and the section that previews the
  certainty section are both invisible in a single guide.
- **Confirmation** — the no-identifiers rule on the TL;DR is what made this fix
  available. A rule kept for one reason turned out to be load-bearing for
  another.

## Appendix: Skills Invoked

- `review-guide` (beta) — the subject of the change, and used to draft its own
  pull-request body. Followed from its source file rather than invoked as a tool
  while it was being edited.
- `sweep-comments` — applied to the same six pull requests in the other
  repository, in the sitting that produced this evidence.
- `technical-prose` — the word-level bar for this entry and for the rewritten
  guides.
