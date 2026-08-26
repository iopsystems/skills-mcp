---
status: shipped
opened: 2026-08-25
updated: 2026-08-25
beta_skills: [review-guide]
---

# A guide the size of its change

## Goal

Make the skill produce a short guide for a small change without being told to.
Reported in `2026-08-25-readme-library-overview.md`: a one-file, additive,
32-line README change drew a 733-word guide, which took three drafts to bring
down to 216.

## Scope

`skills/review-guide/SKILL.md` and its corpus. No change to the publish test,
the TL;DR rules, the mental-model content, the centrality ordering, or the
prose bar.

## Evidence

The skill already carries the rule. "What you publish is proportional to the
change and to its complexity. A one-file mechanical edit earns a sentence."
Two other passages overrode it.

The body section opened "Always present, in this order:" over a numbered list of
seven, then permitted dropping one only when "the change genuinely has nothing
in it". Read together, that is a template of seven headings, and a heading draws
at least a short paragraph out of whoever fills it. The published guide had
seven headings for a change that carried two items. Its testing section said the
suite passes and no test covers README content — a sentence true of every
documentation change this repository will ever make.

The decisions rule split on whether an item wants the reviewer's answer. It said
nothing about whether an item is worth what a subsection costs. One item — should
the map's group labels be abbreviated so no line passes 75 columns — wanted an
answer in the sense the rule tested for, and got a context paragraph, a
reference, and an italic question. Decisions came to 323 words, forty-four
percent of the body, for a question one edit would reverse.

Both failures are invisible from inside the guide. Each section is defensible on
its own; only the ratio between the guide and the change shows the defect.

## Design and Implementation

The seven become the answers a guide owes its reviewer rather than a list of
headings to fill, and **a heading is earned by its content**. An answer needing a
clause is a clause in the opening paragraph; an answer needing a paragraph gets
its heading. A change crossing subsystems still grows all seven. A one-file
addition answers most of them in its first sentence and carries the two with
something in them.

The distinction that keeps this from becoming license to drop material is stated
where the drop rule already lives: losing a heading is not dropping an answer. A
one-sentence testing answer stays; the heading over it does not. The two failures
look identical from outside the guide and are opposites — one publishes a guide
that cannot say why it exists, the other pads a guide to look thorough, and the
padding costs the reviewer of a small change most.

For decisions, the missing test is added directly: **wanting an answer is not
enough to earn a subsection.** Two kinds belong in the recorded list however
curious the author is — one whose context is already visible in the diff, so the
paragraph would narrate what the reviewer is looking at, and one that turns on a
value rather than an approach, where the alternatives are obvious and one edit
reverses the choice. The reason given is the one the ordering rule already
protects: asking a label width in full costs the reviewer the same attention as
the decision the change is for.

Four red flags name the shapes: a Decisions section longer than the rest of the
body, a subsection asking about a value, a heading over a single sentence or
more headings than items that cleared the publish test, and a section that would
read the same on every change of its kind.

Corpus goes from fifty-two cases to fifty-five.

## Outcome

Shipped. `cargo test --locked`, `cargo fmt --all -- --check`, and the MCP smoke
test pass; the citation guard stayed green.

Unmeasured. The rules were derived from one overshoot on one small change, and
no guide has been written under them yet. The first real test is the next
documentation-sized change.

## Derived Documents

None.

## Deferred or Reopen Items

- A third change was proposed and held: giving the mental model an omission
  reason for a reviewer who established the model themselves, by writing or
  merging the change this one extends. The rule it would amend was set eight
  days ago in `2026-08-17-review-guide-mental-model.md` in response to a reader's
  report, and the proposed wording risks reinstating the author-predicts-the-
  reviewer gate that entry removed. Reopen on a second case, and only with a
  form that names a verifiable event rather than a judgment about what the
  reviewer knows.
- Nothing checks a guide against these rules. As with every rule in this skill,
  they are stated for an agent reading them.

## Skill Feedback

### review-guide (beta)

- **Friction** — the two frictions this effort fixes are recorded in
  `2026-08-25-readme-library-overview.md`, where they were observed. They are not
  restated here.
- **Confirmation** — the skill's own proportionality rule was correct and
  specific enough to name the defect once someone looked. What it lacked was any
  rule that would have stopped the two passages overriding it, which is a
  different failure from a wrong rule and wanted a different fix.
- **Confirmation** — the report arrived from a reader of the output, for the
  third time in this skill's history, and again the defect was invisible to the
  author and to every check the skill already required.

## Appendix: Skills Invoked

- `review-guide` (beta) — read as the subject of the change, and followed for
  this change's own pull-request body.
- `engineering-journal` — this entry.
