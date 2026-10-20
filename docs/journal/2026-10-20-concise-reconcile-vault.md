---
status: shipped
opened: 2026-10-20
updated: 2026-10-20
prs: [83]
beta_skills: [review-guide]
---

# Shorter reconciliation instructions with explicit mode boundaries

## Goal

Shorten `reconcile-vault` while preserving its transition rules and resolving
conflicting instructions about PR creation and merging.

Single-PR lifecycle; landed in PR83. It follows the writing-concision work
and PR82's `sharable-artifact` rewrite.

## Decision Criteria

Preserve pre-flight checks, evidence requirements, validation, scope limits,
note contents, and prohibitions on bypassing checks. Check both modes against
frozen response cases, with no new failures attributable to the rewrite.
Resolve contradictions from recorded intent rather than word-count targets.

## Scope

The skill, a public synthetic evaluation fixture and its observations, and this
journal entry. No Rust behavior, real vault artifacts, or live reconciliation
PRs change.

## Evidence

The baseline is `50e82c8b2fc87253262d820565644147f8ae1fc6`. Its description
allows a new interactive PR for review findings, while Mode A allows one only
on request. Step 5 requires every candidate to be clean, but Step 6 allows
clean edits from mixed runs. The final shared rule leaves any PR open for
ambiguity, contradicting dream mode's instruction to merge with proposals.

The [interactive PR decision](2026-08-31-reconcile-vault-pr-opt-in.md)
reserves opening a PR for user requests. The later
[dream landing decision](2026-08-31-dream-applies-clean-transitions.md)
changes dream mode alone: land validated edits and the note despite unresolved
judgment calls. Its historical motivation remains in that entry.

[Eight frozen cases](../evals/fixtures/skill-concision-reconcile-v1.json)
cover mixed findings in each mode, rollback, failed note-only validation,
host refusal, interactive branching from `main`, an explicit PR request, and
a missing pivot. Cases and expectations were frozen before editing the skill.
Workers receive the cases without expectations.

## Design and Implementation

A mode table separates interactive branch ownership from dream landing.
Eligibility is per transition; both modes retain the same checks. The rewrite
removes repeated rationale and historical narration while preserving the
transition examples and all three fenced templates byte-for-byte.

Dream mode now explicitly revalidates after reverting transition edits and
stops if the note-only state still fails. It also stops and reports a host
refusal. These clarify the existing prohibitions on failed checks and host
bypasses. The frontmatter retains the invocation phrases while correcting its
interactive PR exception.

Independent static review found that “about five” weakened the existing
context cap. The final wording restores “at most five” before the candidate
trial. Step 5 also retains the explicit requirement to cite evidence.

## Outcome

Shipped in [PR83](https://github.com/iopsystems/skills-mcp/pull/83), merged as
`fede51773f21375211ca705128de8d3b7c344087` on 2026-10-20. The
[execution follow-up](2026-10-20-reconcile-execution.md) tracks the remaining
operational validation separately.

The final skill is 2,046 words, down from 2,773 (26%), counting whitespace-
delimited words including frontmatter and templates. Independent static review
has no remaining findings. [Results and full responses](../evals/results/skill-concision-reconcile-v1.json)
retain all trials, hashes, grades, and minor note-content observations.

The first blinded decision comparison passed both variants on all 22
expectations. A separate schema audit then found eight invalid baseline search
calls and 32 invalid initial-candidate calls. The candidate used
`candidate_status` instead of `new_status`; both variants invented search
arguments. Decision grades alone missed these executable-call defects.

The next revision named `id` and `new_status` and specified separate
`vault_search` calls for discussions, session-notes, and decisions with
`limit: 5`. Search already orders by creation date without a topic filter;
there is no sort argument. The reading cap remains five combined discussions/
session-notes and five decisions. Static review verified this against the
implementation. All 47 proposed calls in a fresh trial passed the captured
input schemas.

A fresh blinded grader gave that trial 20 passes and two partials: two dream
responses finalized the note after validation without revalidating it. The
final skill explicitly requires validation after such edits. A fresh worker
reran all four dream cases; the same grader passed their 12 expectations.
The ten interactive expectations retain their earlier passes because their
instructions are unchanged. This is combined coverage, not a fresh eight-case
run of the final text. All 47 calls in the combined response pass schema checks.

The initial failures remain in the results. Valid arguments and consistent
action plans do not establish that the workflow executes successfully.

`cargo test --locked --quiet` passes all 151 tests using an existing target
cache. The first fresh-target build hit disk quota; removing this run's build
cache and reusing the existing cache resolved it. The skill validator passes;
fixture hashes, eight-case response coverage, final review hash, and unchanged
templates also check out.

No production execution is claimed. The paired trials use supplied tool
results and proposed action traces; they do not execute Git operations,
repository validation, MCP tools, or host merges. The Rust tests are separate
repository checks, not execution evidence for this skill.

## Derived Documents

The skill is the operational source. The fixture and saved responses provide
the evaluation record; historical journal entries remain unchanged.

## Deferred or Reopen Items

- Actual vault validation, MCP result handling, Git mutations, and host
  enforcement remain untested. Proposed arguments were checked against captured
  schemas, but the protocol supplies tool outcomes. A downstream isolated
  execution trial is needed before claiming operational reliability.
- Workers share context across their assigned cases. The final dream rerun and
  retained interactive responses are a small sample, not a statistical estimate
  of reliability or a skill-selection test.
- The templates retain their existing prose. Shortening them can be evaluated
  separately; this pass preserves their output contract.

## Skill Feedback

### review-guide (beta)

- **Confirmation** — distinguishing validation evidence from deployment behavior
  keeps the proposed PR from presenting response cases as successful merges.

## Appendix: Skills Invoked

- `skill-creator` — scoped rewrite, static validation, and independent forward tests.
- `technical-prose` — wording for an agent using the skill without its author history.
- `reconcile-vault` — baseline and candidate instructions used in isolated dry runs.
- `engineering-journal` — this single-PR effort record and index.
- `review-guide` (beta) — PR description and evidence limits.
