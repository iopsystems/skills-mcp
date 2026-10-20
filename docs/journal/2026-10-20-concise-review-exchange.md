---
status: shipped
opened: 2026-10-20
updated: 2026-10-20
prs: [87]
beta_skills: [review-guide]
---

# Shorter review exchange instructions

## Goal and Scope

Shorten repeated explanations around `review-exchange` while preserving
installation approval, read-before-write, review scope, evidence, finding
dispositions, convergence, identity, and durable-record rules.
Single-PR lifecycle. This also carries the verified PR85 and PR86 journal closures.

The embedded installer, activation description, and existing 14-case trigger
corpus remain byte-identical. This effort does not change bridge behavior.

## Evidence and Decision Criteria

Base: `7d9b5992a269e94fdf33a9496bdd3b5bde142c0d`.
The skill repeats explanations after stating the rule: skipped findings return
in later rounds, untracked threads need durable summaries, and empty findings
do not establish correctness. The revision keeps each requirement and cuts its
repeated justification.

Whitespace-delimited counts, including frontmatter: **1,964 → 1,817 words**.
Excluding the installer block: **692 → 545 words**, a 21% reduction. Size alone
does not establish better behavior.

Accept no lost requirement or observed workflow regression. Reviewers must
ground findings in the branch's changes; authors must disposition every finding
with evidence and leave convergence to a reviewer round.

## Evaluation

The [frozen protocol](../evals/fixtures/review-exchange-concision-v1.json) has
five cases per variant: review a branch after its base moves, answer three
findings, inspect a missing bridge, check an already-read thread, and respond
without an agent identity.

The [setup script](../evals/setup-review-exchange-eval.py) creates separate Git
repositories with concrete Python source and seeded review rounds. Each uses
the real unchanged installer. A wrapper records bridge arguments, results, and
outputs; arbitrary shell reads are not comprehensively logged. One fresh worker
per variant performs all five cases in order, sharing context within its variant.
Workers receive requests, raw source, thread state, and their assigned skill,
without the rubric or the other variant's outputs.

An independent reviewer assessed actual rounds, handoffs, event logs, and Git
state under neutral labels. Both versions passed all 14 frozen expectations.
The reviewer independently reproduced the source probes and verified that the
workers preserved source, configuration, HEAD, and existing rounds. Grades were
frozen before revealing the variants for a separate contract review.
That review found no lost constraints across the existing 14 scenarios
(42 expectations) or the remaining instructions. The requirement for distinct
agent names remains implicit in both versions; this inherited gap was not
changed by the rewrite.

## Outcome

[PR87](https://github.com/iopsystems/skills-mcp/pull/87) merged on
2026-10-20 at `f74070b84635a8042a4509a74bd84ba243e7c58d`.
GitHub's merged state and merge commit were verified before closing this entry.

Both versions completed the five tasks with the expected findings, dispositions,
and permission boundaries. Each passed 35 mechanical checks. This supports the
scoped rewrite; it does not show a behavioral advantage over the original.
The [results](../evals/results/review-exchange-concision-v1.json) retain all ten
captures, independent reviews, hashes, and validation logs.

The structural skill validator, 29 existing bridge checks, and all 154 Rust
tests passed. The package was cleaned before rebuilding in a shared Cargo
target, with incremental compilation disabled.

Two evaluation issues are retained in the evidence:

- The environment exhausted its disk quota before the first bridge operations
  could be logged. Removing disposable incremental build caches allowed both
  workers to resume. Failed event writes limit reconstruction of those attempts.
- The initial audit incorrectly rejected valid reads with an explicit
  `--branch`. The corrected matcher accepts the matching branch and still
  excludes peek/list operations. Both workers' outputs were unchanged; the
  independent reviewer verified the reads from actual events.

## Deferred or Reopen Items

These small synthetic repositories do not establish broad review quality,
activation behavior, or reliability across repeated trials. The installer is
covered by its existing test suite, not by a claim that these five cases cover
every bridge operation.

## Skill Feedback

### review-guide (beta)

- **Confirmation** — separating the prose reduction from workflow evidence
  keeps the planned PR from treating fewer words as proof of better reviews.

## Appendix: Skills Invoked

- `skill-creator` — focused revision and independent forward-testing.
- `technical-prose` — concise wording with retained requirements.
- `review-exchange` — evaluated local Git workflow.
- `engineering-journal` — closures and this evaluation record.
- `review-guide` (beta) — PR description.
