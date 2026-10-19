---
status: open
opened: 2026-10-19
updated: 2026-10-19
beta_skills: [review-guide]
---

# PR writing follows the final scope

## Goal

Keep a PR's title and explanation aligned with its final scope when later
commits broaden the change. Add coverage for behavior-preserving refactors
whose representation, visibility, or inherited comments still need judgment.

This uses the single-PR lifecycle. The change is prepared for review;
broader behavioral evaluation remains open.

## Evidence and decision

Description review found an explanation organized around the original file
split, followed by inventories and review history for later additions.
The [previous consistency revision](2026-10-18-pr-description-consistency.md)
checks behavioral qualifications, dependency status, and historical claims.
This revision adds a narrow check for scope growth across the whole review
artifact, including its title and priorities.

Existing prose guidance already removes repeated inventories while retaining
consequential rationale. Existing comment guidance already checks inherited
claims. Those receive evaluation cases rather than additional instructions.
The public fixtures are fictional and contain no private PR links or source.

## Implementation

`review-guide` gains one paragraph: when scope changes, reassess the title,
opening, review priorities, and validation against the final diff; rebuild
the explanation around the final change.

Four new scenarios cover expanded scope, unchanged behavior with visibility
and representation exceptions, redundant inventories with necessary rationale,
and an inaccurate comment moved unchanged beside its encoder. Corpus counts
become 68 review, 37 prose, and 35 comment scenarios. The prose and comment
instructions are unchanged.

## Evaluation

The [frozen requests](../evals/fixtures/pr-scope-writing-v1.json) were executed
by separate fresh baseline and revised responders. A third agent assessed
both outputs against the raw evidence without reading skill changes or
scenario expectations. No model override was used. The focused comment case
was read as one block in each responder's context, without a full-sweep claim.

The [observation record](../evals/fixtures/pr-scope-writing-observation-v1.json)
retains exact prompts and responses, source hashes, and the critic's assessment.
The baseline instructions come from `46dfe26cefc116ab252584592a8c67af73029792`.
Both runs covered the core requirements. The revised title named the expanded
scope more explicitly, but its response also made an unsupported generalization
about declarations moving into settings modules and dropped the registry-size
motivation. The response artifacts are unedited.

This comparison does not establish an overall improvement. It covers four
supplied cases with one generation per condition, without human reader
measurement or repeated-generation reliability estimates. The fictional test
results are task inputs, not commands run by the responders.

## Validation and outcome

Passed locally:

- `cargo test --locked --quiet`: 151 tests across eight binaries.
- Formatting, strict clippy, debug and release builds with `--locked`.
- Diagram freshness, MCP smoke, and review-bridge checks.
- Skill-creator validation for all three skills and unique corpus names.
- Recorded input, skill, and corpus digests and journal links verified; the
  built MCP server returns the new scope guidance through `review-guide`.

Repository checks establish corpus structure and integration, separately from
the writing comparison. The outcome remains open pending review and broader
evaluation.

## Deferred work

Repeat the expanded-scope case with varied briefs and obtain human reader
feedback. Check that ordinary validation updates do not trigger unnecessary
rewrites. Preserve the small instruction change unless evidence supports more.

## Skill feedback

### review-guide (beta)

- **Friction:** a PR that grew beyond its original refactor retained the old
  title and accumulated sections. The new paragraph makes final-scope
  reassessment explicit.
- **Confirmation:** the baseline already separated unchanged runtime intent
  from API and representation changes. Both responders preserved validation
  limits and the narrow extent of independent review.

## Appendix: Skills Invoked

- `skill-creator` — scoped instruction change and independent forward testing.
- `review-guide` (beta) — description assessment and this PR's review context.
- `technical-prose` — relevance, rationale, and preservation of claim strength.
- `sweep-comments` — verification of unchanged relocated comments.
- `engineering-journal` — the evidence record and evaluation limits.
