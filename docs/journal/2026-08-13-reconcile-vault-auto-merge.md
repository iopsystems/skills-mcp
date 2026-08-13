---
status: shipped
opened: 2026-08-13
updated: 2026-08-13
prs: [27]
---

# Reconcile-vault auto-merge

## Goal

Record what pull request 27 changed in `reconcile-vault` and whether it holds
against the vault schema it cites. The change inverted the skill's default from
proposing every transition to applying and merging clean ones, routing only
risky cases to a human.

This entry was written after the merge by someone who did not author the change.
It is grounded in the merged artifact and the schema, not in the author's
intent, which was not recorded.

## Decision Criteria

None were stated before the work. This entry does not invent them. The question
it does settle is whether the shipped behavior contradicts `SCHEMA.md`, because
the rule that was replaced cited that document as its justification.

## Scope

`skills/reconcile-vault/SKILL.md` only, 63 insertions and 20 deletions. No Rust,
no evals, no template. The skill's two modes, its token budget, and its
pre-flight step through `vault_check_transition` are unchanged.

## Evidence

Before the change, the skill's description read "Propose changes; never
auto-apply", and its shared rule "Never auto-apply" quoted `SCHEMA.md` — "You
can get unstuck; you can't pretend something is resolved when it isn't" — as the
reason, concluding "This skill proposes. The user decides."

The vault's `SCHEMA.md` is version 1, `status: draft`, updated 2026-07-20, which
matches `SUPPORTED_SCHEMA_VERSION` in `src/vault/schema.rs`. Its lines 339 to
342 place that sentence in a specific context:

> The escape hatches are `withdraw_exploration` and `abandon_inquiry` —
> explicit, logged acts that require a reason. There is deliberately no
> `force_accept` or `override_blocking`. You can get unstuck; you can't pretend
> something is resolved when it isn't.

The sentence governs **bypassing a blocker**, not **who applies a clean
transition**. `SCHEMA.md` states derived-status invariants — a decision cannot
be `accepted` while its design brief is `draft`, an arc cannot be `closed` while
an inquiry is `open` — and removes the override that would let anyone pretend
otherwise. It does not require human confirmation for a transition that
satisfies every invariant.

The old rule therefore rested on a quotation that did not support it. The
citation was real; the inference from it was not.

## Design and Implementation

The shipped skill applies and merges only a run where every proposed transition
is `allowed` with no blockers and no warnings, cites direct and sufficient
evidence, has exactly one justified next state, and passes repository validation
and required checks. Anything else — a blocker, a warning, incomplete or
ambiguous evidence, more than one plausible outcome, a failed check, or a merge
the host refuses — becomes human-review-required, and the skill leaves the pull
request open and names the exact reason per item.

The prohibitions that carry the schema's actual constraint were kept and
strengthened. `force_accept` remains absent, and the skill now also forbids
bypassing branch protection, force-pushing, force-merging, and reporting a
refused merge as success. A merge refusal is classified as a review result
rather than an obstacle to route around.

Dream mode keeps its one-file boundary: it writes a session-note and does not
mutate other artifacts. What changed is that a clean note now merges on its own
rather than waiting for triage.

The shipped text kept the quotation but attached it to the new default, so it
read as though the schema endorsed auto-merge. This entry's effort corrected
that: the quotation now sits with the prohibition it actually supports, and the
skill states that the sentence governs bypassing an invariant rather than who
applies a clean transition, and should not be cited for either side of the
auto-merge question. The behavior is unchanged.

Against the schema, the result holds. The behavior the schema forbids is
bypassing an invariant, and `vault_check_transition` remains the gate: a
transition with any blocker cannot reach the auto-merge path at all. Warnings
are treated more conservatively than the schema requires, since a warning
propagates advisory state rather than blocking, and this skill sends it to a
human anyway.

One boundary is worth watching rather than settling here. `SCHEMA.md` line 266
says "Agents write briefs. Humans (with agent help) write decisions. Keep that
boundary firm." Transitioning a `decision` to `accepted` is a status edit rather
than authorship, so it is not obviously a violation, but it is the one place
where an unattended agent edits an artifact the schema assigns to humans. No
evidence was found that this case was considered.

## Outcome

Shipped in pull request 27, merged 2026-08-13T09:02:34Z as `38ef909`. The branch
was based on an unmerged commit of `yao/review-guide-skill` rather than on
`main`, so it carried `skills/review-guide/` and a `src/main.rs` test as
inherited files; because pull request 26 merged first, the merge took the newer
copies and nothing was reverted. This was verified after the fact: the beta
marker, the publish test, the emptiness rule, and the 22-case corpus with its
matching assertion are all present on `main`, and 137 tests pass across seven
binaries.

The skill's own behavior has not been exercised against a real vault in this
repository, and the vault is not checked out here. Whether the four cleanliness
conditions admit runs that should have stopped is unmeasured.

## Derived Documents

None. `docs/backlog.md` and `docs/roadmap.md` describe the template and
distribution stages and are not affected by a change to one skill's default.

## Deferred or Reopen Items

- Whether an unattended agent should transition a `decision`, given that
  `SCHEMA.md` assigns decisions to humans. Reopen with the vault checked out,
  or when the first decision transition is auto-applied.
- No trigger-eval corpus exists for `reconcile-vault`, so the new disposition
  rules are asserted only in prose. Four of the repository's skills carry a
  corpus and a count assertion; this one does not.
- The skill still has no evidence from use. Reopen after the first unattended
  reconciliation run against the real vault, whether it merges or stops.

## Appendix: Skills Invoked

The skill usage of pull request 27 was not recorded and is not recoverable from
the merged artifact. This entry does not reconstruct it. The only invocation
known is the one that produced this entry: `engineering-journal`, followed from
its source file rather than invoked as a tool.
