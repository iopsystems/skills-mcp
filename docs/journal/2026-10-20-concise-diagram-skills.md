---
status: open
opened: 2026-10-20
updated: 2026-10-20
beta_skills: [architecture-diagram, format-layout-diagram, review-guide]
---

# Shorter diagram introductions and rationale

## Goal and Scope

Shorten the architecture, dataflow, and format-layout skill prose while keeping
their domain rules and validation requirements. This single-PR effort is a draft
pending behavioral follow-up. It is independent of the writing-skill revision.

## Evidence and Design

The introductory project histories and repeated justifications can be condensed
without replacing the independently usable skills with references to each other.
The edits preserve limited-domain evidence, override reporting, source-derived
claims, single-use exceptions, proportional area, codec checks, review gates,
and the check-record and repair instructions added in PR73. Existing descriptions
and evaluation corpora are unchanged.

Word counts include frontmatter and use whitespace splitting:

| Active skill | Before | After |
| --- | ---: | ---: |
| architecture-diagram | 4,820 | 4,618 |
| dataflow-diagram | 4,648 | 4,188 |
| format-layout-diagram | 2,157 | 2,121 |

The active instructions lose 698 words. The architecture and dataflow templates
mirror the revisions, retaining their project and trust contracts. Their versions
advance to `0.2.3` and `0.1.3`, with refreshed content hashes. Historical citations
to shortened passages link to the pre-edit commit rather than requiring the
old wording to remain in the active skill.

## Decision Criteria

Keep derivation and classification mandatory where applicable; preserve the
explicit single-use exception and external-source transcription workflow.
Never exchange real topology, LOC ratios, or byte positions for a prettier
layout. A previously approved artifact cannot confer approval on a changed one.
Any response that omits a required step is a follow-up finding, even when the
instruction still appears in the shortened skill.

## Evaluation

The [frozen fixture](../evals/fixtures/skill-concision-diagrams-v1.json) selects
12 existing cases before editing, four per skill. Separate agents answered the
baseline and candidate suites using prompts and skill text without the rubric
or the other variant's outputs. Cases shared an agent context but were requested
as standalone tasks. There was one trial per case and variant.

The [result record](../evals/results/skill-concision-diagrams-v1.json) preserves
all 24 responses, source hashes, and author observations. Workspace credit errors
surfaced after both complete response files were saved; JSON parsing and exact
case coverage confirmed the artifacts were complete. No independent grader or
repeat trials completed for this suite.

Author observations, not an independent evaluation pass:

- In the conference-talk case, the candidate omitted explicit fail-loud
  classification that the baseline mentioned. Both retained derivation, a
  generator beside the artifact, a dated snapshot, and the single-use exception.
  The classification requirement remains in the skill; a concrete inventory
  exercise is needed to determine whether execution regressed.
- Both weighted-area answers preserved the 1:4 ratio, tiny-file alternatives,
  zero-area handling, and rounding checks. Neither explicitly mentioned
  decorative headers, and neither generated an artifact.
- Both external-source answers retained transcription provenance and freshness
  limits, but neither explicitly stated that classification must fail loudly.
- The six check-record answers kept missing evidence visible and required fresh
  review. Both variants distinguished inline tokens from fixed-color image exports.

These responses do not establish layout quality, correct generation, or human
readability. No renderer ran during this evaluation.

## Validation and Outcome

- `cargo test --locked --quiet`: 151 tests passed, including template parity,
  manifest hashes, and citations.
- `cargo fmt --all -- --check` and `git diff --check`: passed.
- Skill-creator `quick_validate.py`: all three active skills and both templates
  passed.
- Fixture/result JSON, case coverage, source hashes, and unchanged existing
  corpora were checked.

The PR stays in draft. Response evidence and static consistency are insufficient
to clear the behavioral guardrail.

## Deferred Work

Independently grade the saved responses and repeat the classification cases on
a real generator with an unclassified element. Run before/after generation and
check artifacts, review status, and regeneration behavior in a downstream repo.
The user was asked for a representative repository; none has been supplied yet.

## Skill Feedback

### architecture-diagram (beta)

- **Confirmation:** both responses retained quantitative area and stale-approval
  boundaries in the selected cases.
- **Friction:** the candidate's single-use answer omitted explicit classification;
  this needs an execution test before concluding that the instruction was lost.

### format-layout-diagram (beta)

- **Confirmation:** both responses retained re-encoding, reserved/shipped
  distinctions, and embedding-dependent colors. No artifact-level claim follows.

### review-guide (beta)

- **Confirmation:** the draft description separates repository checks from the
  ungraded response run and the missing real-diagram evaluation.

## Appendix: Skills Invoked

- `skill-creator` — scoped edits and independent response runs.
- `technical-prose` — prose editing; unchanged baseline guidance in this branch.
- `architecture-diagram` (beta) — response evaluation.
- `dataflow-diagram` — response evaluation.
- `format-layout-diagram` (beta) — response evaluation.
- `review-guide` (beta) — this PR's description.
- `engineering-journal` — evidence and open validation work.
