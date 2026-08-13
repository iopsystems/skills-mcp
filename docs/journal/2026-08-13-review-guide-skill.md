---
status: shipped
opened: 2026-08-13
updated: 2026-08-13
---

# Review guide skill

## Goal

Give the repository a skill that drafts a pull-request body as a guide for the
human reviewing it: where to spend attention, what was tested and what was not,
which calls were judgment rather than deduction, and what will only appear in
production.

## Decision Criteria

Ship when the served skill states an attention-ranking rule that does not reduce
to file order, binds every uncertainty item to cited evidence, and draws its
boundary against `engineering-journal` explicitly. `cargo fmt --check`, the full
suite, a release build, and an MCP list/call smoke test must pass.

## Scope

One active skill with its own trigger corpus. It owns the pull-request body:
reading order, test reporting, judgment calls, production-only risk, and when a
diagram is earned. It does not own the diff, the durable record, or word choice,
each of which has a skill already. No template counterpart is created.

## Evidence

The repository carries twenty-one skills before this change, each a single
`SKILL.md` served as instruction text, and skills defer to each other by name in
prose rather than through any import mechanism —
`skills/architecture-diagram/SKILL.md:27` and `:37` are the precedent this
follows for pictures.

Two adjacent skills bound the design. `engineering-journal` already records
durable effort narrative, so a review guide that also claims durability would
create a second record nobody maintains. `sweep-comments` already holds comment
quality inside the diff, so guide prose has no business ruling on it.

The user settled four questions during design: the guide is the pull-request
body rather than a checked-in file unless explicitly asked otherwise; a diagram
is checked in when it already exists or is worth keeping, and inline mermaid
when it is scaffolding for one review; uncertainty is expressed through cited
evidence rather than confidence ratings; and the skill runs on every change
while publishing proportionally, rather than gating on a size threshold at
invocation. The user also stated that overlap with the journal is acceptable
rather than something to engineer around.

## Design and Implementation

The skill's premise is that the reviewer already has the diff, so summarizing it
adds nothing. What a reviewer cannot reconstruct alone is the author's attention
budget and the author's doubts, and those are what the body carries.

Three rules do the work.

The first governs whether a guide exists at all. An earlier draft gated
invocation on a substantial-change threshold borrowed from
`engineering-journal`. That was replaced: the skill now runs on every change and
decides afterward whether the result is worth publishing. The assessment is
cheap, and its outcome is information either way — a change with nothing to say
is a fact about the change rather than a reason to have skipped looking. A guide
publishes when it carries at least one item the reviewer could not get from the
diff: a reading order differing from the diff's own order, a test gap, a
judgment call, or a production-only risk. When all four are empty the skill
publishes one sentence and states that it checked those four and found nothing,
because silence is indistinguishable from not having looked. Padding to clear
the bar is named as a defect: an inflated guide costs the reviewer more than no
guide, since it teaches them to skim the next one.

The attention ranking orders by the cost of a missed defect multiplied by the
chance the reviewer misses it: code carrying a judgment call first, then code
whose correctness depends on something outside the diff, then untested code,
then code the author is least sure of. Naming the skimmable half — generated
output, renames, formatting — is treated as half the value, because a reviewer
who spends attention on a rename has none left for the invariant. Citations
point at a line, not a module.

Evidence-bound uncertainty is the second rule and the harder one, because the
section fails in both directions. Manufactured uncertainty hedges to look
careful and buries the real item; suppressed uncertainty presents one reading of
an ambiguous requirement as the only reading, which silently transfers a choice
the reviewer should have owned. Both are named as defects. Every item must cite
a concrete artifact and carry the call made, the alternative rejected, and what
evidence would change it; an item that cannot be stated that way is dropped.
Confidence ratings are refused outright: a self-reported number invites trust
that the reviewer cannot check.

Testing is discovered rather than assumed. The skill lists candidate surfaces —
unit, integration, smoke, benchmark, property, evaluation corpora, manual — and
requires, for each that applies, whether it ran, what it actually said, and what
it does not cover. Coverage percentages are refused because they answer a
question nobody asked and hide the gap that matters, and a skipped check must be
named rather than implied to have passed.

Production-only risk is required to name what would surface it, so the reviewer
can judge observability rather than only plausibility, and the section ends by
asking the reviewer either to weigh in or to accept the risk explicitly. A risk
nobody accepted is a risk nobody owns.

The boundary against `engineering-journal` is stated as audience and lifespan
rather than content: durable and effort-scoped against transient and
change-scoped. Content overlap is permitted, and the guide may restate a
decision rather than making the reviewer click through. The single firm rule is
that the guide never becomes the durable record.

Artifacts:

- `skills/review-guide/SKILL.md`
- `skills/review-guide/evals/trigger-evals.json`, twenty-one cases covering the
  publish test in both directions, proportional output, refused padding, ranking
  against path order, test-reporting honesty, both uncertainty failure modes,
  refused confidence ratings, diagram earning, and the journal and file-output
  boundaries
- `review_guide_evals_cover_key_scenarios` in `src/main.rs`

## Outcome

Shipped as an embedded MCP skill. `cargo fmt --check` passed, 137 tests passed
across the seven test binaries, `cargo build --release` succeeded, and a raw
JSON-RPC smoke test confirmed that `tools/list` exposes twenty-nine tools
including `review-guide` and that `tools/call` returns the served body with its
ranking, certainty, production-risk, and red-flag sections intact.

The pull request body for this change was written with the skill itself, which
is the only test available before a human reads one. It caught a real defect:
the first draft cited three line ranges that were each off by a section, because
the model estimated them rather than checking. The rule at
`skills/review-guide/SKILL.md` requiring a specific place rather than a module
was violated on first use by the model that wrote it. The mistake was corrected,
kept in the body as an uncertainty item, and produced a risk that had not been
listed: line-precise citations rot as soon as a file is edited.

The trigger corpus asserts intended behavior rather than measured behavior. No
guide has yet been produced by the skill and judged by a human reviewer, which
is the only evidence that would show whether the ranking rule survives contact.

## Derived Documents

None. The design was settled in conversation and recorded here, consistent with
the convention this repository has followed since the `technical-prose` entry.

## Deferred or Reopen Items

- A template counterpart under `templates/`. Deferred until the skill proves
  itself; the same open question recorded for `technical-prose` applies, namely
  how a seeded template reaches an MCP-served skill in a repository without
  `skills-mcp` installed.
- Forward-testing the corpus against independent agents.
- Whether the skill should read the diff itself or rely on the caller's summary.
  It currently assumes whoever invokes it has the change in context, which is
  true for an authoring agent and false for a reviewer invoking it cold.

## Appendix: Skills Invoked

- `superpowers:brainstorming` — context exploration, the three design questions
  asked before implementation, and the design-approval gate before any file was
  written. The fourth decision recorded under Evidence, always-run with
  proportional publication, came from review feedback afterward and did not pass
  through this skill.
