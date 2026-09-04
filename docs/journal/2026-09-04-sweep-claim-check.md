---
status: shipped
opened: 2026-09-04
updated: 2026-09-04
---

# A comment can pass every rule and be false

## Goal

Give `sweep-comments` a step that checks what a comment claims against the
code it sits on. The sweep decided whether a comment was derivable, where it
belonged, how tersely it was worded, and whether it named anything deleted.
It never asked whether the comment was true.

## Scope

`skills/repository/sweep-comments/SKILL.md` and its corpus. No change to the
tiers, the retrieval test, the model home, the two-pass structure, or the
one-context requirement. `technical-prose` is untouched.

## Evidence

A reviewer read a change that had been through both the prose pass and the
comment sweep and found three comments the code contradicted. None named
anything deleted or renamed, none was an echo, and each was well placed and
tersely worded.

- A test note said one direction of a round trip could not be tested yet
  because no producer attached metadata to its message stream. Four did, and
  a sibling test in the same crate asserted one of their message streams was
  forwarded. The claim was inherited from the old note, which had said the
  same of a different message stream, and the prose pass reworded it without
  checking it.
- A field documented as the hosts that read a message stream named hosts on
  which nothing read it. Each consumed the same data over a different
  transport; the list named where they would move to, in the present tense.
- A fixture helper was documented as producing metadata the real producer
  could have written. It took one field from the registry entry and hardcoded
  two others, one of them invented.

All three share a shape. The sweep's truth check reads "check each survivor
against the current design, not the design it was written for": a check
against the author's model, aimed at pivots. A claim about what exists, what
is absent, or what matches can only be checked against the code, and nothing
in the sweep said to look. The prose pass could not have caught it either, by
its own statement: tightening finds needless words, not false ones.

## Design and Implementation

Pass 1 gains a third step after classification and the staleness check: list
every claim a reader could verify and verify each with one grep or one test
name, against the code as it is. Three claim shapes are named because they
are the ones that fail: absence and exclusivity, which age fastest;
equivalence, where every field the claim covers must be checked; and
present-tense statements of who does what, where a design that has not landed
reads as a fact.

Inherited comments get the same check as new ones. Rewording carries the old
claims forward under new authority.

The pass-1 report gains a `correct` tag for a claim the code contradicted.
Three examples join the corpus of comments that do not survive, anonymized
from the review. Two rationalizations and three red flags name the shapes.
Two evals cover the inherited false absence and the present-tense claim over
a future path; the corpus goes from thirty-two to thirty-four.

## Outcome

Shipped. `cargo fmt --all -- --check`, clippy, and `cargo test --locked` pass.

Unmeasured. The step is written from three findings in one review. Whether a
grep per claim is the right cost, or whether some claim shapes are common
enough to deserve a checklist of their own, is unknown until sweeps run under
it.

## Derived Documents

None.

## Deferred or Reopen Items

- `technical-prose`'s cold read asks who the reader is, not whether a
  sentence is true, and must not. A pointer from its "content before style"
  paragraph to this step would tell an author running only the prose pass
  that a check exists which it does not perform.

## Skill Feedback

### sweep-comments

- **Friction** — the sweep reported a clean pass 1 on this change, and the
  reviewer found three false comments in it. A pass that completes without
  reading the code the comments describe measures form.

## Appendix: Skills Invoked

- `sweep-comments` — the sweep whose gap this closes.
- `technical-prose` — the pass that reworded the inherited claim.
- `engineering-journal` — this entry.
