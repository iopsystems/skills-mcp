---
status: shipped
opened: 2026-10-20
updated: 2026-10-20
beta_skills: [architecture-diagram, format-layout-diagram, review-guide]
prs: [79, 80]
---

# Shorter diagram introductions and rationale

## Goal and Scope

Shorten the architecture, dataflow, and format-layout skill prose while keeping
their domain rules and validation requirements. The edits shipped in
[PR79](https://github.com/iopsystems/skills-mcp/pull/79); the completed follow-up
evaluation is recorded below. The diagram edits were evaluated separately from
the writing edits.

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

## Initial Evaluation

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

## Follow-up Evaluation

An independent grader assessed the 12 saved response pairs against the frozen
expectations, with A/B labels randomized per case and variant identities hidden.
The [grades](../evals/results/skill-concision-diagrams-v1-blind-grades.json)
retain the label mapping and a reason for each expectation. Both versions
received six passes and six partials, with no case-level grade changes.

The shared partials concern explicit fail-loud behavior, decorative-area
accounting, and concrete provenance or artifact links. Some prompts provide no
source or artifact from which to obtain that provenance; an honest incomplete
record is preferable to invented evidence. Equal grades do not establish
equivalent behavior.

The [execution fixture](../evals/fixtures/skill-concision-execution-v1.json)
uses tracked Pelikan source at
`46b98c9dc6fb70848808b903ba1d60a9655150be`. Before either worker ran, the author
executed its generator offline on a clean archive: all three SVGs reproduced
byte-for-byte. This establishes a working generator, not a skill comparison.

Separate agents then received the baseline or merged skill and identical
disposable source copies containing a new, unclassified `metrics-probe`
workspace library. Both ran the generator; both observed:

```text
ERROR: workspace crate 'metrics-probe' is not classified in LAYER
```

Both workers stopped with workspace credit errors before writing a diagram
handoff. Author inspection found no generator or classification changes and no
changes to the existing SVGs. Only the controlled workspace edit and the
Cargo-generated lock entry changed. The
[execution results](../evals/results/skill-concision-execution-v1.json) retain
the command outputs, source diff, hashes, and interruption status.

This exercises a real generator's failure path. It does not establish that the
agents would implement that guard, preserve provenance in the final handoff,
or withhold a stale chart after continuing. No independent execution grader or
visual comparison completed.

## Completed Downstream Evaluation

Both interrupted workers resumed and completed their failure handoffs. Each
reported the unresolved classification, labeled the retained SVG as stale for
the modified source, preserved provenance and dirty inputs, and withheld
unperformed visual and approval claims. An independent grader passed both
handoffs against all four frozen criteria. The baseline handoff requires
reconstructing source from the upstream commit and retained changes; the
candidate additionally supplied a full source archive.

The [task fixture](../evals/fixtures/skill-concision-tasks-v1.json) then asked
each variant to mark workspace library crates with no workspace dependents on
the same Pelikan architecture chart. Both derived `bloom` and `protocol-http`
from dependency declarations and added visible markers with a scoped legend.
Both preserved grouping, classification and topology checks, and byte-identical
runtime companion SVGs. Regeneration reproduced each changed SVG exactly.

The [results](../evals/results/skill-concision-tasks-v1.json) preserve patches,
handoffs, command logs, source revisions, artifact hashes, and independent
grades. The inspected previews are retained for
[baseline](../evals/renderings/skill-concision-tasks-v1/baseline.png) and
[candidate](../evals/renderings/skill-concision-tasks-v1/candidate.png).
Font measurements and visual inspection found no marker or legend clipping.
An author mutation check added a real dependency on `bloom`: both generators
removed its marker while retaining the `http` marker.

The independent grader passed both successful changes against all seven
criteria, checked source and artifact evidence, and reproduced the PNGs
byte-for-byte. Charter-required maintainer review remains explicitly pending
on these disposable evaluation artifacts. Agent inspection does not supply
that approval.

There was one trial per variant; workers retained context across tasks and
resumed after interruptions. These cases found no concision regression. They
exercise an existing generator and charter, not new-generator design or
activation, and do not establish general equivalence.

## Validation and Outcome

Before PR79 merged:

- `cargo test --locked --quiet`: 151 tests passed, including template parity,
  manifest hashes, and citations.
- `cargo fmt --all -- --check` and `git diff --check`: passed.
- Skill-creator `quick_validate.py`: all three active skills and both templates
  passed.
- Fixture/result JSON, case coverage, source hashes, and unchanged existing
  corpora were checked.

Both evidence-only follow-ups passed all 23 citation and repository-documentation
tests, JSON/hash/coverage checks, and `git diff --check`. The completed task record
also verifies saved patches, handoff manifests, and checked-in rendering hashes.

PR79 is merged. The completed handoffs, grounded diagram changes, rendering
checks, and independent assessment close the scoped evaluation. No further
instruction change is supported by these findings.

## Deferred Work

New-generator construction, artifact-level dataflow and format-layout changes,
and repeated trials remain unmeasured. A concrete failure in those domains
would justify another investigation. The live Pelikan working tree was untouched.

## Skill Feedback

### architecture-diagram (beta)

- **Confirmation:** both responses retained quantitative area and stale-approval
  boundaries in the selected cases.
- **Friction:** the candidate's single-use answer omitted explicit classification;
  the grader found both answers insufficiently explicit about failure behavior.
  Both execution workers reached the real classification guard, and their
  completed handoffs preserved its failure and withheld freshness claims.
- **Confirmation:** both successful changes derived markers from source,
  responded to a dependency mutation, preserved runtime charts, and kept
  maintainer review pending.

### format-layout-diagram (beta)

- **Confirmation:** both responses retained re-encoding, reserved/shipped
  distinctions, and embedding-dependent colors. No artifact-level claim follows.

### review-guide (beta)

- **Confirmation:** the initial draft description separated repository checks from the
  ungraded response run and the missing real-diagram evaluation.

## Appendix: Skills Invoked

- `skill-creator` — scoped edits, independent response runs, and downstream trials.
- `technical-prose` — prose editing; baseline guidance for the initial diagram
  revision and assigned variants for the execution follow-up.
- `architecture-diagram` (beta) — response evaluation and isolated Pelikan execution.
- `dataflow-diagram` — response evaluation.
- `format-layout-diagram` (beta) — response evaluation.
- `review-guide` (beta) — this PR's description.
- `engineering-journal` — evidence and closure of the scoped evaluation.
