---
status: shipped
opened: 2026-10-20
updated: 2026-10-20
beta_skills: [review-guide]
prs: [78, 80]
---

# Shorter writing instructions with behavioral checks

## Goal and Scope

Shorten `technical-prose`, `sweep-comments`, and `review-guide` without losing
requirements, exceptions, examples that distinguish failures, or evidence limits.
The edits shipped in [PR78](https://github.com/iopsystems/skills-mcp/pull/78);
the completed follow-up evaluation is recorded below. Descriptions and existing
scenario corpora are unchanged.

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

## Initial Evaluation

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
  rerun before attributing it to the edit. Subsequent concrete trials below did
  not reproduce the omission; the original observation remains in the record.
- Shared partials include missing requested explanations and insufficiently
  explicit review directions. Raw grades are retained without adjusting the
  rubric to make the candidate pass.

A separate contract reviewer compared all three skill diffs against all 140
existing expectation sets. It caught one lost destination instruction: useful
derivations belonged in the PR description. The author restored a standalone
bullet, and the reviewer verified it. No further concrete contract loss was
reported. That restoration happened after response generation; the result file
records both evaluated and final candidate hashes. The execution follow-up below
exercised the restored instruction.

Workspace credit errors surfaced after complete response and writing-grade
files were saved. Their JSON and exact case coverage were checked. Further
independent trials could not run. This is limited response evidence, not proof
of equivalence or improved reader comprehension.

## Execution Follow-up

The [execution fixture](../evals/fixtures/skill-concision-execution-v1.json)
supplies concrete Python source, two tests, and a PR description. Its criteria
were frozen before separate agents received the baseline and merged skills.
Neither agent received the criteria or earlier findings. Each performed the
whole sweep in its own context.

Both agents saved edits before stopping with workspace credit errors. The
[result record](../evals/results/skill-concision-execution-v1.json) preserves
their files, selected command logs, and author checks. Both edited Python files
are byte-identical across variants:

- Test property labels and the capacity-two eviction rationale remain.
- Append and assertion narration is removed.
- The bucket contract and persisted-level constraint remain in code.
- The algebra derivation moves from the function docstring into the PR body.

Author reruns passed both tests for each variant. Comparing syntax trees after
removing docstrings found no executable-code changes. The candidate saved a
two-pass sweep report; the baseline did not save its report before interruption.
No independent execution grader ran.

This concrete case did not reproduce the earlier cleanup omission and exercised
the restored derivation instruction. It does not erase the original response
warning or establish equivalence on other inputs.

## Completed Downstream Evaluation

The [task fixture](../evals/fixtures/skill-concision-tasks-v1.json) freezes a
bounded comment sweep and PR rewrite against cache-rs commit
`5cf04dd1b47e4bdda1a7a4a5872648b87743d42e`. Both workers edited comments around
the split and merge APIs, stride-rejection test, and conservation test in
`crates/data-structure/ziplist/src/chain.rs`. The
[results](../evals/results/skill-concision-tasks-v1.json) retain their patches,
PR bodies, sweep reports, checks, and independent grades.

Both variants preserved all executable Rust tokens and assertions. Each passed
the 23 selected chain tests. Both retained header-excluding byte accounting,
caller conversions, the odd-entry fixture rationale, and the need to measure
conservation independently. Both removed mechanics narration and corrected an
existing description of the two forward walks.

The independent grader received hidden A/B identities and the frozen criteria.
The shorter skills passed all seven criteria; the baseline passed six, with a
partial on the split contract. Its new comment said a target fitting the whole
body forces a split before the last unit, overlooking the half-target budget.
A subsequent executable probe confirmed the error: four 2-byte entries and
target 8 split 2|2, not 3|1. The result record includes the probe and preserves
the evaluated comments unchanged.

Both PR bodies accurately reported their scope and validation. Strict rustdoc
failed on existing links outside the baseline's edited regions; the candidate's
nonstrict run completed with 23 warnings outside its edited regions. Neither
claimed clean documentation or exhaustive input validation.

There was one trial per variant. Workers retained their variant and context
across tasks; interrupted work resumed as the same trial. The grader independently
checked the comment-only diffs and semantic claims, while token comparison and
fresh test execution were author checks. Single-worker sweep chronology rests
on worker reports. These cases found no concision regression and do not establish
general equivalence or reader comprehension.

## Validation and Outcome

Before PR78 merged:

- `cargo test --locked --quiet`: 151 tests passed.
- `cargo fmt --all -- --check` and `git diff --check`: passed.
- Skill-creator `quick_validate.py`: all three edited skills passed.
- Fixture/result JSON, case coverage, source hashes, and unchanged existing
  corpora were checked.

Both evidence-only follow-ups passed all 23 citation and repository-documentation
tests, JSON/hash/coverage checks, and `git diff --check`. The completed task record
also verifies saved patches, handoff manifests, and checked-in rendering hashes.

PR78 is merged. The completed downstream trial supplies the missing final reports
and independent assessment. The scoped evaluation is complete; no further
instruction change is supported by these findings.

## Deferred Work

Whole-repository sweeps, repeated trials, and reader comprehension remain
unmeasured. Descriptions were unchanged, so this pass makes no activation claim.
A reproducible omission on a concrete input would justify another investigation.

## Skill Feedback

### review-guide (beta)

- **Confirmation:** both variants preserved the retry exception, expanded final
  scope, public-API decision, and validation limits in the corresponding cases.
  Both downstream PR bodies also preserved the malformed-input rationale,
  independent test oracle, and actual test and rustdoc limits.
- **Friction:** both received partial grades for failing to make rendered
  proportionality an explicit review priority in the treemap case. Retain that
  limitation rather than reporting the run as a general pass.

## Appendix: Skills Invoked

- `skill-creator` — scoped edits and independent forward-testing.
- `technical-prose` — editing and response evaluation.
- `sweep-comments` — response evaluation, an isolated executable fixture sweep,
  and a bounded downstream Rust sweep.
- `review-guide` (beta) — response evaluation and this PR's description.
- `engineering-journal` — method, findings, and closure of the scoped evaluation.
