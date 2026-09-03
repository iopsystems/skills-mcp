---
status: shipped
opened: 2026-09-03
updated: 2026-09-03
beta_skills: [technical-prose]
---

# The reader is not you

## Goal

Give `technical-prose` the rule every other rule in it depends on: who the
words are for.

## Scope

`skills/repository/technical-prose/SKILL.md` and its corpus. `review-guide`
keeps its own `Explain it, then name it`, which is this rule applied to one
kind of document.

## Evidence

A session outside this repository
([transcript](https://claude.ai/share/733be7db-ce8e-4445-9741-0346f1ca0248))
rewrote one commit body under The Elements of Style. Two rewrites in a row each
passed the style rules and each carried an unresolved referent through
untouched: "operator-chosen label" the first time, "this arc" the second. Both
were caught by the reader asking what the phrase meant. Neither is a word that
carries no fact; each carries a fact to the writer and nothing to anyone else.

The skill had no rule that could catch them. Its bar is applied word by word,
its Attribution calls its rules "reader-agnostic", and the ladder's first rung,
"the reader already holds it", is exactly where a writer who cannot un-know
what they know goes wrong.

## Design and Implementation

A first section, `The reader is not you`, ahead of `Modality`: name the reader
and what they lack; reread as that reader and state every referent that resolves
only for someone who was in the room; run that read before the word pass, since
tightening cannot find a missing noun; set self-sufficiency by what ships with
the text, a diff or nothing; and treat a metaphor in place of the fact as the
same failure. The ladder's first rung now names the reader rather than the
writer. Two rationalization rows, four red flags, four corpus cases, and the
references the session named: Pinker on the curse of knowledge, Google's
audience guidance.

The section was 288 words on first writing and 236 after its own read: one
example restating the sentence before it, one assertion the example already
proved, one restatement of a heading, and one flourish about vividness in the
paragraph that bans flourishes.

## Outcome

Shipped. Full CI job green locally.

## Derived Documents

None.

## Deferred or Reopen Items

- `review-guide`'s `Explain it, then name it` and this rule now overlap. If
  the overlap becomes friction, the guide's version should shrink to a pointer.

## Skill Feedback

### technical-prose (beta)

- **Friction** — the skill could tighten a sentence around a missing referent
  and report the sentence done. Every rule in it was reader-agnostic by
  design, and the one question no style rule asks, who is this for, was the
  one that found both defects.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
