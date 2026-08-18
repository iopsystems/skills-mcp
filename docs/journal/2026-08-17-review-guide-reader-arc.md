---
status: shipped
opened: 2026-08-17
updated: 2026-08-17
beta_skills: [review-guide]
---

# Review guide opening, rebuilt by its reader

## Goal

Record the sequence of changes a single reader drove through `review-guide` in
one sitting, by reading guides it produced for a six-pull-request stack in
another repository and reporting what failed.

## Scope

`skills/review-guide/SKILL.md` and its corpus, plus the six guides in
`agrippa-industries/agrippa-core` that were rewritten as each rule landed. The
publish test, the attention ranking, the certainty rules, and the testing rules
are untouched — every change here is to the opening and to how explanations are
built.

The prior entry for the mental model covers the first of these; this one covers
the arc and supersedes nothing.

## Evidence

Eight reports, each against a specific guide rather than against the skill:

1. The opening had no summary. A reader wanting the gist read a paragraph.
2. Sections said what they contained and never what they asked of the reviewer.
3. The mental model sat at position six of seven, marked optional, gated on the
   author predicting what the reviewer knew.
4. The opening quoted a field name, which failed for a reader who does not know
   what the thing holding it is.
5. The two-sentence ceiling fought the no-identifiers rule.
6. A checklist of questions at the top asked the reviewer to answer things
   before they had read anything.
7. "Skip to the reading order" named no section that exists.
8. An explanation named a pre-existing identifier with no gloss, then made the
   same point three times from three angles.

## Design and Implementation

The opening is now: TL;DR, then what to look out for, then the mental model. Each
position was argued for by a failure rather than by symmetry.

The TL;DR became a short essay answering why, what the key idea is, and what is
true once it lands, with the ask closing it as its own paragraph. It carries no
type, field, function, flag, or file path: an identifier in the opening is the
strongest available signal that the author described the edit instead of the
reason. The two-sentence ceiling was removed when it collided with that rule, and
the collision resolved toward accessibility, which is now stated as the general
priority — the rule against padding forbids material with nothing in it, not
material longer than the author would like.

What to look out for replaced a list of questions. A reviewer who has read one
paragraph cannot answer anything, so the list stopped asking for answers and
became numbered flags to carry, each marked again by number where its evidence
appears. The reviewer meets each item twice.

`How to review this` was added and then removed within two days, both times on
this reader's report. Once the look-out list carried the asks and the reading
order already named what was skimmable, the routing block had no remaining job.

Explanations took a single shape: where things stood before, the new problem or
condition that motivates the change, then the change. That shape absorbed the
separate rule about glossing pre-existing identifiers, because the before beat is
where such a gloss naturally belongs. An identifier the change adds introduces
itself; one that already existed has no such sentence, which is why it is the one
left undefined.

Two rules came from the skill failing its own standards. A cross-reference must
name a section exactly as its heading reads, because "the reading order" is a
description and the section is headed Where to look first — the one-name-per-thing
rule failing inside the skill that imposes it. And a beat appears once: the first
four-beat version of the explanation shape said "what changed in the world" and
"what that broke", which is one thing said twice.

## Outcome

Shipped across nine commits on `yao/review-guide-prose-bar`. The corpus went from
twenty-two cases to forty-one, `cargo fmt --check` passed, the suite passed with
no failures, and the citation guard stayed green — including one failure it
caught, where a worked example cited a path from the other repository.

The six guides were rewritten after each rule landed, so they are the only
evidence that any of this works, and they have not been read back since the last
four changes.

## Derived Documents

None.

## Deferred or Reopen Items

- No mechanism checks any of these rules. The citation guard covers line
  citations; nothing covers an undefined identifier, a missing before beat, an
  unreferenced numbered item, or a cross-reference naming a description.
- The routing block was added and retired inside two days. Nothing in the skill
  records which of its current rules are similarly provisional, and the beta
  marker covers all of them equally.

## Skill Feedback

### review-guide (beta)

- **Friction** — every one of the eight reports came from reading output, none
  from reading the skill. Two rounds of automated review over the same period
  found count drift and a rule contradiction, and none of the eight.
- **Friction** — the skill twice broke its own rules in text written the same
  day: a cross-reference by description rather than heading, and an explanation
  shape that stated one beat twice. Both were reported by the reader rather than
  caught by the author.
- **Friction** — a rule added on one report was retired on a later one, which
  cost two rounds of rewriting six guides. Faster feedback would have been
  cheaper than a more careful first draft, but nothing in the skill says how to
  tell a provisional rule from a settled one.
- **Confirmation** — the beta channel produced its intended result. Every change
  here traces to a named failure in a specific artifact, and none of them was
  visible to the author.
- **Confirmation** — rewriting the six guides after each rule was the only way
  any of these failures surfaced. Reading the skill would not have found them.

## Appendix: Skills Invoked

- `review-guide` (beta) — used to draft this change's own pull-request body, and
  applied to six guides in another repository as each rule landed. Followed from
  its source file rather than invoked as a tool while it was being edited.
- `technical-prose` — the word-level bar, invoked as a tool earlier in the same
  branch and applied to the six guides.
