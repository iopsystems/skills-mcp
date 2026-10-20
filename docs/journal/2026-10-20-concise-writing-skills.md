---
status: open
opened: 2026-10-20
updated: 2026-10-20
beta_skills: [review-guide]
---

# Shorter writing instructions with behavioral checks

## Goal and Scope

Shorten `technical-prose`, `sweep-comments`, and `review-guide` without losing
requirements, exceptions, examples that distinguish failures, or evidence limits.
This is a single-PR effort, prepared as a draft while evaluation remains open.
Descriptions and existing scenario corpora are unchanged.

## Evidence and Design

Earlier reviews reached different conclusions about removable prose:
[the first survey](2026-09-01-sweep-heavy-skills.md) found little, while
[the subsequent editorial pass](2026-09-01-adversarial-cut.md) found justification
stacking and historical framing. This pass removes repeated explanations and
rationalizations, retaining the operational rules and final checklists.

Word counts include frontmatter and use whitespace splitting:

| Skill | Before | After |
| --- | ---: | ---: |
| technical-prose | 3,044 | 2,045 |
| sweep-comments | 6,351 | 4,919 |
| review-guide | 2,341 | 2,189 |

The 2,583-word reduction is a size result, not evidence of better behavior.
An existing test loses a rhetorical marker and updates two wording markers;
the underlying classification and placement rules remain. Historical citations
to removed or moved explanations now link to the pre-edit commit.

## Decision Criteria

Preserve modality, evidence strength, local edit constraints, test claim
bindings, solo two-pass sweeps, and the distinction between settled rationale
and open review decisions. Treat any lost requirement or worse evaluated
response as a finding to investigate. Do not accept an aggregate score as a
reason to ignore an individual regression.

## Evaluation

The [frozen fixture](../evals/fixtures/skill-concision-writing-v1.json) selects
18 existing cases before editing: six per skill. It records the original
expectations. The [result record](../evals/results/skill-concision-writing-v1.json)
contains full baseline and candidate responses, skill hashes, randomized label
mapping, and the independent grader's reasons.

One independent agent answered each variant's suite, instructed to treat cases
separately. Cases shared that agent's context. Responders received prompts and
skill text, not expectations or the other variant. A separate grader received
the frozen expectations and responses labeled A/B in randomized order per case.
There was one trial per case and variant; no live repository execution.

The strict grades were 10 pass / 8 partial for the baseline and 12 pass / 6
partial for the candidate. Three cases improved and one worsened:

- `sweep-comments-15`: the candidate preserved the test's property statement and
  odd-fixture rationale but omitted removal of mechanics narration. The baseline
  mentioned that cleanup. The rule remains in both skills, and the prompt
  supplies no concrete test body, so the observation needs a concrete-test
  rerun before attributing it to the edit. The warning remains open.
- Shared partials include missing requested explanations and insufficiently
  explicit review directions. Raw grades are retained without adjusting the
  rubric to make the candidate pass.

A separate contract reviewer compared all three skill diffs against all 140
existing expectation sets. It caught one lost destination instruction: useful
derivations belonged in the PR description. The author restored a standalone
bullet, and the reviewer verified it. No further concrete contract loss was
reported. That restoration happened after response generation; the result file
records both evaluated and final candidate hashes. Its behavioral rerun remains
pending.

Workspace credit errors surfaced after complete response and writing-grade
files were saved. Their JSON and exact case coverage were checked. Further
independent trials could not run. This is limited response evidence, not proof
of equivalence or improved reader comprehension.

## Validation and Outcome

- `cargo test --locked --quiet`: 151 tests passed.
- `cargo fmt --all -- --check` and `git diff --check`: passed.
- Skill-creator `quick_validate.py`: all three edited skills passed.
- Fixture/result JSON, case coverage, source hashes, and unchanged existing
  corpora were checked.

The draft remains open. A real comment sweep and PR rewrite have not been
evaluated with these revisions.

## Deferred Work

Rerun the test-comment case with concrete source and fixtures, exercise the
restored derivation rule, and compare both variants on a downstream repository.
The user was asked for a representative repository; none has been supplied yet.
Descriptions were left unchanged, so this pass makes no activation claim.

## Skill Feedback

### review-guide (beta)

- **Confirmation:** both variants preserved the retry exception, expanded final
  scope, public-API decision, and validation limits in the corresponding cases.
- **Friction:** both received partial grades for failing to make rendered
  proportionality an explicit review priority in the treemap case. Retain that
  limitation rather than reporting the run as a general pass.

## Appendix: Skills Invoked

- `skill-creator` — scoped edits and independent forward-testing.
- `technical-prose` — editing and response evaluation.
- `sweep-comments` — response evaluation, not a repository comment sweep.
- `review-guide` (beta) — response evaluation and this PR's description.
- `engineering-journal` — method, findings, and unresolved evaluation.
