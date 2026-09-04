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
one-context requirement. `technical-prose` is untouched: it already disclaims
content, and this change is the step that disclaimer points at.

## Evidence

A reviewer read a change that had been through both the prose pass and the
comment sweep and found three comments the code contradicted. None named
anything deleted or renamed, none was an echo, and each was well placed and
tersely worded.

- A test's direction note said the reverse direction was untestable because
  no producer-side stream was enveloped at origin yet. Four such streams
  existed, and a sibling test in the same crate asserted one of them was
  forwarded. The claim was inherited from the old note, which had made the
  same kind of claim about a different stream, and the prose pass reworded it
  without checking it.
- A field documented as the hosts that read a stream named hosts on which
  nothing read it. Each consumed the same data over a different transport;
  the list was staging the path they would move to, stated in the present
  tense.
- A fixture helper was documented as producing an envelope the real producer
  could have minted. It took one field from the registry entry and hardcoded
  two others, one of them invented.

All three share a shape. The sweep's truth check reads "check each survivor
against the current design, not the design it was written for", which is a
check against the author's model, aimed at pivots. A claim about what exists,
what is absent, or what matches is checked against the code, and nothing said
to go and look. The prose pass could not have caught it either, by its own
statement: tightening finds needless words, not false ones.

## Design and Implementation

Pass 1 gains a third step after classification and the staleness check: list
every claim a reader could verify and verify each with one grep or one test
name, against the code as it is. Three claim shapes are named because they
are the ones that fail: absence and exclusivity, which age fastest;
equivalence, where every field the claim covers must be checked; and
present-tense statements of who does what, where a design that has not landed
reads as a fact. A claim about the future is written as one.

Inherited comments get the same check as new ones. Rewording carries the old
claims forward under new authority, so a rewrite that skips the check makes a
false claim worse.

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

- `technical-prose`'s cold read asks who the reader is and what they lack. It
  does not ask whether a sentence is true, and it should not; but a pointer
  from its "content before style" paragraph to this step would tell an author
  running only the prose pass that a check exists which it does not perform.

## Skill Feedback

### sweep-comments

- **Friction** — the sweep ran to completion on the change in question and
  reported a clean pass 1 and a populated pass 2, and the reviewer then found
  three false comments in the same diff. A pass that can complete without
  reading the code the comments describe is a pass that measures form. This
  entry is the fix.

## Appendix: Skills Invoked

- `sweep-comments` — the sweep whose gap this closes.
- `technical-prose` — the pass that reworded the inherited claim.
- `engineering-journal` — this entry.
