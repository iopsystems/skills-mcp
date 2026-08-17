---
status: shipped
opened: 2026-08-17
updated: 2026-08-17
beta_skills: [review-guide]
---

# Review guide mental model

## Goal

Make a guide establish the concepts a reviewer needs before it names a type, a
field, or a function. Reported by a reader of guides the skill produced: the
bodies went straight to structure and field detail, leaving no picture for the
details to attach to.

## Scope

`skills/review-guide/SKILL.md` and its corpus. No change to the publish test,
the attention ranking, the certainty rules, the TL;DR, the routing block, or the
prose bar.

## Evidence

The skill listed `Mental model` as item six of seven and placed it under
"present when earned", gated on the change crossing "a boundary the reviewer
cannot hold in their head". Three things followed. Context came after the
details when it came at all. The gate asked the author to predict what the
reviewer already knows, which the author is the worst-placed person to judge.
And the default was omission, so the section rarely appeared.

The `Diagrams` section carried the same gate and the same default, so a change
with a topology described that topology in prose.

Six guides written for a stacked series in another repository showed the result:
each opened on the reading order and named types the reviewer had not been
introduced to.

## Design and Implementation

The mental model moves to position two, immediately after the claim and before
the reading order, and becomes part of the always-present set. The burden
inverts: omission now needs a reason, and the reasons are narrow — a typo, a
version bump, a rename inside one file.

Its content is ordered so that each step earns the next: what the system does at
the level this change touches, in the domain's terms rather than the code's; the
two or three concepts the change depends on, each named once and reused; and
where this change sits among them. Only after that can the reading order name a
type, because the name now has somewhere to land.

Two failure modes are named, because the section invites both. A subsystem tour
explains the subsystem rather than this change, and the test for it is whether a
paragraph would be equally true of a different pull request against the same
files. Restating the diff in prose fails differently: the mental model is what
holds before the change plus where the change lands, not the change.

Stating the assumed starting point costs one line and releases a reviewer who
already holds the model, which is what the old gate was trying to achieve by
prediction.

Diagrams are re-scoped from a rarity to the natural carrier of shape. The
section now splits on what is being shown rather than on how unusual the change
is: `architecture-diagram` for structure — what exists and what contains what —
and `dataflow-diagram` for movement — what flows where. Where the picture lives
follows how long it is worth: checked in when it outlives the review, inline
mermaid when it is scaffolding for this one.

One rule comes directly from the reported case. A stacked series shares one
shape, so the diagram is drawn once and each guide marks its own piece, rather
than a different diagram per pull request. The reviewer learns one model and
reuses it across the stack.

Corpus goes from twenty-eight cases to thirty-two: context before the first
identifier, shape drawn rather than described, the subsystem-tour failure, and
the shared diagram across a stack.

## Outcome

Shipped. `cargo fmt --check` passed, the suite passed with no failures, and the
citation guard stayed green.

Whether guides now give a reviewer somewhere to put the details is unmeasured.
The report that prompted this came from one reader; the fix has not been read
back by that reader or any other.

## Derived Documents

None.

## Deferred or Reopen Items

- The six guides that prompted the report were written before this change and
  still open on their reading orders. They are the first test of whether the fix
  works, and updating them is separate from shipping the rule.
- No mechanism checks that a guide introduces a concept before naming it. As
  with the prose bar, the rule is stated and applied by an agent reading it.

## Skill Feedback

### review-guide (beta)

- **Friction** — a reader of six guides reported that they went straight to
  struct and field detail with no higher-level context, so there was no mental
  image to attach the details to. The skill had a mental-model section, but at
  position six, marked optional, and gated on the author predicting what the
  reviewer already knew. Every one of those three choices pushed toward omitting
  it. Fixed by moving the section to position two, making it always-present, and
  inverting the burden to omission.
- **Friction** — the same report named diagrams as the missing half. The
  `Diagrams` section carried the same rarity gate, so a change with a topology
  described the topology in prose. Fixed by scoping diagrams to what carries
  shape and pointing at the two diagram skills by what they show.
- **Confirmation** — the report arrived as a structural complaint about the
  output contract rather than about one guide, which is the second time this
  channel has produced that. Both times the defect was invisible to the author
  and to automated review, and visible immediately to a reader.

## Appendix: Skills Invoked

- `review-guide` (beta) — used to draft this change's own pull-request body,
  followed from its source file rather than invoked as a tool, because this
  effort was editing that file.
