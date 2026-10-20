---
status: shipped
opened: 2026-10-20
updated: 2026-10-20
prs: [88]
beta_skills: [review-guide]
---

# Planning questions follow missing information

## Goal and Scope

Shorten `plan-feature` and replace mandatory conversational checkpoints with
questions that resolve planning gaps. Preserve coordinated artifacts and
authorization by action and target. Single-PR lifecycle; includes the verified
PR87 journal closure.

## Evidence and Decision Criteria

Base: `f74070b84635a8042a4509a74bd84ba243e7c58d`.
The original repeats artifact and authorization rules in its introduction,
workflow, closing checklist and prohibitions. It also requires a confirmation
after each substantive answer, an announcement for each area, and pushback
before drafting from incomplete information.

The revision keeps six planning areas and the artifact sequence. It uses known
facts before asking questions, carries existing authorization into dependent
skills, and drafts with explicit unknowns when requested. These are deliberate
interaction changes, not a claim that the wording is behaviorally identical.
Single-repo issue drafting is explicit, resolving the original's instruction
to skip coordination followed by drafting issues from that coordination plan.

Accept the change only if artifact coverage, evidence limits and authorization
survive evaluation. Fewer words alone do not establish a better planning flow.

## Evaluation

The [frozen protocol](../evals/fixtures/plan-feature-concision-v1.json) contains
20 expectations across three cases: a fully specified draft request, an
underspecified request with a scripted followup, and a cross-repository plan
with different permissions for each target.

The [setup script](../evals/setup-plan-feature-eval.py) randomizes neutral labels
and copies the assigned skill and identical dependencies into isolated packets.
One fresh worker per variant handles the cases in order. The followup is
delivered only after the initial underspecified response is saved. Workers
receive no rubric or other variant's output. Independent grades were frozen
before revealing the mapping for contract review.

This evaluates planning and artifact proposals. The supplied local vault
snapshot substitutes for live search, and authorized external actions are
represented as proposals. It does not test actual commits, pushes, issue
creation, access enforcement or live dependency discovery. Cases share worker
context; one run per variant does not establish reliability.

## Outcome

[PR88](https://github.com/iopsystems/skills-mcp/pull/88) merged on
2026-10-20 at `883904f10b7f8ede0a125fb7d75a19d3238f8a1d`.
GitHub's merged state and merge commit were verified before closing this entry.

The draft reduces whitespace-delimited words from
1,926 to 1,148. Frontmatter, including the activation description, and the two
fenced artifact templates are byte-identical.

Both versions received **19 passes, one partial, and no failures** against the
20 frozen expectations. Both drafted immediately from complete context, asked
one user-problem question when context was missing, then drafted without more
questions after the scripted followup. Both kept the cross-repo proposals
within the supplied authorization. This run found no behavioral advantage for
either version.

The shared partial concerns vault links in the optional API repo doc: both
outputs supplied exact IDs, and the revision also supplied paths, but neither
included navigable links. The original template itself uses IDs. The partial
grade remains recorded; contract review confirmed this inherited ambiguity.
Dependency use is supported by the visible conventions and worker reports,
not an independent trace of file reads or tool calls.

The separate contract review found no substantive unintended loss across
22 checks. It confirmed identical frontmatter, templates and dependency files.
Some dependency skills still prescribe clarification or authorship questions;
their live interaction with the revised parent remains untested. No such
conflict appeared in these responses.

The [results](../evals/results/plan-feature-concision-v1.json) preserve all eight
response files, exact requests, hashes, frozen blind grades, separate contract
review and the Rust test log. The shorter instructions are accepted for this PR
with the shared partial and simulation limits retained.

All 154 Rust tests passed after cleaning the package in the shared Cargo target
and rebuilding with incremental compilation disabled.

The generic skill-creator validator rejects both the original and revision:
`Description cannot contain angle brackets (< or >)`. The unchanged description
contains `docs/design/<slug>.md`. This inherited validator incompatibility is
recorded rather than reporting a pass or changing activation text mid-trial.

## Deferred or Reopen Items

Live multi-repository execution and repeated conversational trials remain
outside this evaluation. A concrete artifact omission or unnecessary question
would justify another case. Navigable cross-repo vault links need a convention
that can be tested with real repository locations.

## Skill Feedback

### review-guide (beta)

- **Confirmation** — reporting validation at its actual strength keeps the
  shared partial grade and simulated execution boundary in the PR body.

## Appendix: Skills Invoked

- `skill-creator` — focused revision and independent forward-testing.
- `technical-prose` — concise instructions with explicit interaction changes.
- `plan-feature` — evaluated planning workflow.
- `frame-problem`, `propose-design`, `discuss`, `vault-search` — dependency
  instructions supplied to the workers; live tools were not invoked.
- `engineering-journal` — evaluation record and PR87 closure.
- `review-guide` (beta) — PR description and validation limits.
