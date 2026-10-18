---
status: open
opened: 2026-10-18
updated: 2026-10-18
beta_skills: [review-guide]
---

# PR descriptions agree with current behavior

## Goal

Make PR descriptions consistent with their detailed behavior and current
dependency status. Preserve repository conventions when historical commit
messages describe behavior that later commits replaced.

This uses the single-PR lifecycle. The changes are prepared for review;
broader behavioral evaluation remains open.

## Evidence

Practical description review exposed three checks that were implicit in
`review-guide`: compare an opening's lifetime claim with retry exceptions,
refresh dependency blockers, and distinguish the final implementation from
retained commit history. Classification of review concerns and removal of
duplicate explanations were already covered by the skills, so those receive
new scenarios rather than more instructions.

The [previous prose revision](2026-10-17-prose-relevance-and-review-guides.md)
established selective publication and preservation of claim strength. This
revision keeps those boundaries. The public fixtures use fictional examples
and contain no private repository links or project identifiers.

## Design and Implementation

- `review-guide` checks the opening against retry, replacement, and failure
  paths. Material qualifications belong in the opening.
- Existing dependency and draft-blocker claims are checked against current
  status or supplied records. Unknown status stays unverified; a merged
  dependency does not establish integration, validation, or merge readiness.
- A synopsis explains final behavior before required historical messages
  when later commits supersede their claims. Required verbatim text survives;
  improving a description does not call for rewriting Git history.
- Five new review scenarios cover those checks, unavailable status, and
  distinct treatment of decisions, rationale, and gaps. Two prose scenarios
  cover duplicate explanations and a useful rollback cross-reference.
  Corpus counts become 66 review scenarios and 36 prose scenarios. The
  `technical-prose` instructions are unchanged.

## Evaluation

The [frozen requests](../evals/fixtures/pr-description-consistency-v1.json)
contain five fictional cases. Separate fresh agents produced responses with
the baseline instructions at `85bbed7ff0c311005efd4701ecbe6ec3c2ce63f1` and
with the revised instructions. Neither responder received the scenario
expectations or the other response. A third agent read both responses and
the raw evidence without inspecting the implementation changes or scenario
expectations. No model override was used.

The [observation record](../evals/fixtures/pr-description-consistency-observation-v1.json)
retains the instructions given to the agents, source digests, full response
artifacts, and the critic's assessment. Both runs substantially satisfied
all five requests. The critic found modest clarity gains in the revised
history and decision descriptions, with no material factual failure in
either run. Connection qualifications and repetition were essentially ties.

The critic also found a shared wording weakness: both responses changed
"no independent re-review has occurred" to "remains pending," implying a
future review the source did not establish. Preserve the original negative
observation in authored status text; the stored responses remain unedited.

These are five cases with one response per condition, not a measured
improvement in general reliability or human review outcomes. The unavailable
status and rollback cross-reference scenarios were added but not executed
in this comparison. Supplied test results in fictional cases are inputs to
the writing task, not commands executed by the responders.

## Validation and Outcome

Passed locally:

- `cargo test --locked`: 151 tests across eight binaries.
- `cargo fmt --all -- --check` and `cargo clippy --all-targets --locked -- -D warnings`.
- Debug and release builds with `--locked`.
- `./scripts/render-diagrams.sh --check`, `./scripts/mcp-smoke.sh`, and
  `./scripts/review-bridge-test.sh`.
- Skill-creator validation for both skills, unique scenario names, recorded
  input and skill SHA-256 verification, and `git diff --check`.

Repository checks establish integration and corpus structure, separately from
the limited writing evaluation above. The outcome remains open pending review
and broader behavioral evaluation.

## Deferred Work

Exercise unavailable status and useful cross-references with fresh responders.
Compare repeated generations and obtain human reader feedback before claiming
better reliability. Do not infer improvement from output length alone.

## Skill Feedback

### review-guide (beta)

- **Friction:** description review found a lifetime claim contradicted by its
  retry exception, a stale dependency blocker, and final behavior buried in
  historical messages. The revision makes those checks explicit.
- **Confirmation:** both evaluation runs preserved material test limits and
  distinguished settled rationale from an actual review decision. Existing
  selective-publication guidance remains useful without added categories.

## Appendix: Skills Invoked

- `skill-creator` — scoped changes, validation, and independent forward testing.
- `review-guide` (beta) — description assessment and PR-writing guidance.
- `technical-prose` — relevance and preservation of claim strength.
- `engineering-journal` — this record and the limits of the evaluation.
