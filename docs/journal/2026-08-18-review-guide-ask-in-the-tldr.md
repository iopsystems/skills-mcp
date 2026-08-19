---
status: shipped
opened: 2026-08-18
updated: 2026-08-18
beta_skills: [review-guide]
---

# The look-out list, and the structure that replaced it

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

The first attempt folded the asks into a prose paragraph closing the TL;DR,
numbered, each marked again at its evidence. The reader rejected it, and the
rejection is the finding: the defect was never that the items were a list. It
was that a decision stated without its context is not yet a decision, and no
amount of moving or reformatting supplies context that the summary structurally
cannot carry.

So each decision now gets its own subsection, in a fixed order: the context
that makes it a choice, the `path:line` reference so the reviewer can look
rather than trust the description, then the question. Context first is the rule;
the reference in the middle is what stops the context from being a story about
code nobody can check. The section holding them is `## Decisions`, which is the
old certainty section renamed to what it is.

The summary keeps one line, demoted to a pointer: how many decisions want the
reviewer, whether any block, and where they are. Naming them there costs the
context that makes them decisions, which is the same failure in a shorter form.

The section then moved above the ranked reading order, on the same reader's
call, and the reading order was renamed `Where to look more closely`. Both
follow from the first change: a reviewer who has met the questions can read the
ranking for those answers rather than evenly, an item in the ranking can point
at a decision by its heading, and a section arriving fourth cannot honestly be
called what to look at first — what it ranks is attention, not order.

The last change came from the same reader naming what the section was mixing.
Two different things earn attention: what the change is *for*, and whatever the
author is unsure about. They overlap, but they are not equal, and a flat list of
subsections presented them as though they were — so a reviewer answers whichever
is cheapest. The section is now ordered by centrality: the decision the change
exists to make, then a line demoting the rest, then the leftover uncertainties,
then the calls recorded rather than asked. Three tiers, in that order.

The empty case survives both revisions. "Nothing here needs a decision; the four
checks came back empty" is what the pointer says when the checks are empty,
because an absent ask and an empty one still read identically.

The stack duplication is a second rule, in the mental-model section: write the
model in the base guide, link it by pull-request number from each guide above,
and state only the delta. The skill already said this about diagrams and not
about the prose the diagram sits in.

## Outcome

Shipped. The corpus went from forty-four cases to forty-nine, `cargo fmt
--check` passed, 150 tests pass, and the citation guard stayed green.

Whether a reviewer answers more decisions when each has a subsection is
unmeasured. Two of the three shapes tried here were rejected by the one reader
who has looked at the output, which is the only evidence any of this rests on.

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
- **Friction** — the first fix was to move and reformat the asks, not to give
  them context. The skill states that a reader reporting confusion is evidence
  the claim is wrong rather than the wording, and the same trap has a second
  form the skill does not name: a structural complaint answered structurally
  when what was missing is material.
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
