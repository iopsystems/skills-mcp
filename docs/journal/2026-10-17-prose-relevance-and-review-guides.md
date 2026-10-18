---
status: open
opened: 2026-10-17
updated: 2026-10-17
beta_skills: [review-guide]
---

# Relevant prose and selective PR descriptions

## Goal

Draft changes to `technical-prose` and `review-guide` after assessing the
description of [PR 67](https://github.com/iopsystems/skills-mcp/pull/67).
The reader needs the change, its rationale, review priorities, and validation
limits without reconstructing them from implementation history.

This uses the single-PR lifecycle. The changes are prepared for pull-request
review; behavioral evaluation remains open.

## Evidence

At `e7ad242`, `review-guide` required proportional output but also said complex
changes earned every section, empty asks must be stated, and the opening could
contain no identifiers. Its scenario corpus expected a 400-to-700-word rewrite
to retain its length without examining the added content. These instructions
can reward completeness of structure over review relevance.

The earlier [proportionality revision](2026-08-25-review-guide-proportional.md)
already documented a small change receiving an oversized guide. The
[prose ladder](2026-09-01-prose-ladder.md) supplied a smallest-useful-form rule;
this revision adds relevance to its omission test. This is evidence of repeated
instruction friction, not a measured claim about one model's writing habits.

## Design and Implementation

- `technical-prose` now checks whether accurate details serve the reader's task.
  It preserves consequential rationale before process history and distinguishes
  requirements, recommendations, hypotheses, observations, and attributed claims.
- `review-guide` assesses review concerns internally and publishes material
  findings. The draft removes mandatory empty categories, fixed section order,
  the identifier ban, and automatic demotion of peripheral risks. Source checks,
  precise citations, consequential rationale, and material gaps remain required.
- The evaluation corpora replace conflicting expectations. Six new prose cases
  cover relevance and claim strength; two new PR cases cover the source-layout
  and treemap change and a multi-subsystem rename with one validation gap.
  The corpora contain 34 and 61 scenarios respectively.
- The README description and existing corpus-count assertions follow those
  changes. No new PR-writing skill or template is added.

## Decision Criteria

A useful output lets a reviewer identify the behavior change, understand its
consequential rationale, locate the important review areas, and distinguish
evidence from unresolved claims. Cutting necessary context fails this test as
surely as adding irrelevant detail. Word count alone is not a success measure.

## Validation and Outcome

Passed locally:

- `cargo test --locked --quiet`: 151 tests passed.
- `cargo fmt --all -- --check` and `git diff --check`.
- Skill-creator's `quick_validate.py` for both skills. The first run found an
  oversized technical-prose description; shortening it cleared the check.
- JSON parsing and unique scenario names: 34 prose cases and 61 PR cases.

Repository tests check integration and corpus structure; they do not execute
the writing scenarios.

## Deferred Work

Independent execution of the revised scenarios and a reader comparison against
the previous instructions remain open. The PR 67 case supplies facts for a
future evaluation; no generated response or comprehension result is claimed.

## Skill Feedback

### review-guide (beta)

- **Friction:** revising the PR 67 body exposed conflicting proportionality and
  mandatory-structure rules. The draft replaces them with selective publication
  rather than adding another concision rule.
- **Confirmation:** checking the source and separating validation from review
  status kept a passing regression fixture from becoming a claim of independent
  confirmation. Those distinctions remain in the revised skill.

## Appendix: Skills Invoked

- `skill-creator` — scoped revision and skill validation guidance.
- `technical-prose` — relevance, evidence strength, and wording of the draft.
- `review-guide` (beta) — PR-writing criteria and the subject of the revision.
- `engineering-journal` — this draft's rationale and validation limits.
