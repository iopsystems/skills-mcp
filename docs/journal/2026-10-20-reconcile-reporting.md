---
status: shipped
opened: 2026-10-20
updated: 2026-10-20
prs: [85]
beta_skills: [review-guide]
---

# Complete reconciliation handoffs and grounded claims

## Goal

Address the two reporting findings from the
[PR84 execution trials](2026-10-20-reconcile-execution.md): unresolved orphan
design decisions omitted from the handoff, and an unsupported “external”
dependency qualifier in strategic reflection. Single-PR lifecycle.

## Decision Criteria

Carry unresolved work decisions into PROPOSED, Needs you, and the notification
without inventing status transitions. Support factual qualifiers with the
cited evidence while retaining useful strategic judgment. Preserve transition,
validation, rollback, and landing behavior.

## Evidence and Implementation

Base: PR84 merge `f0c1267fb31e0b8a1733f53dd5a8d1d1afbc8015`.
The revised skill distinguishes work decisions from status proposals and makes
the citation requirement apply to the whole factual claim. It replaces existing
wording instead of adding a separate checklist. The PR84 journal closure is
included here.

The [frozen comparison](../evals/fixtures/reconcile-reporting-v1.json) uses
the existing host-refusal and invalid-note fixtures. One fresh worker per
variant executes both cases with the real MCP binary and local Git. Each suite
uses a dedicated binary copy. An independent reviewer sees neutral labels and
the actual outputs before the variant mapping.

## Outcome

Shipped in [PR85](https://github.com/iopsystems/skills-mcp/pull/85), merged as
`f7c7e7ec82a36875dac0a60f059160b951ed83c0` on 2026-10-20. The clarification
and comparison evidence landed; the limits below still apply.

Both variants passed all **26 automatic execution checks** and all **six manual
criteria in each of two cases**. The independent reviewer saw neutral labels
without either skill or the mapping. Both variants carried the orphan and
paused-design decisions into the worklist and handoff, kept factual claims
grounded, and retained strategic recommendations and execution boundaries.
The [results](../evals/results/reconcile-reporting-v1.json) preserve complete
captures, the frozen protocol, and the independent review.

The revised skill is **2,028 words**, down from **2,046**, counting whitespace-
delimited words including frontmatter and templates. Its structural validator
and all **151 Rust tests** pass. This comparison demonstrates no behavioral
advantage: the current version also passed. The wording change clarifies the two previously observed
failure points; PR84's failures remain part of the evidence.

Both workers were interrupted by a temporary disk quota failure. Removing the
disposable Rust build cache allowed them to resume. Their handoffs record the
interruption; failed event writes limit reconstruction of every attempted
command. Binary hashes matched before and after both runs.

The shared Cargo target initially reused the runtime-fix test binary. That
result is excluded; cleaning the package forced the reporting branch to rebuild
before the recorded 151-test run.

The SQLite initialization fix is a separate runtime change.

## Deferred or Reopen Items

These reused fixtures do not test unprompted candidate discovery, production
vault checks, GitHub behavior, or reliability across repeated runs.

## Skill Feedback

### review-guide (beta)

- **Confirmation** — separating automatic state checks from manual content
  review keeps the planned PR's claims within the evidence.

## Appendix: Skills Invoked

- `skill-creator` — focused revision and independent behavioral comparison.
- `reconcile-vault` — evaluated workflow.
- `engineering-journal` — PR84 closure and follow-up record.
- `technical-prose` — concise wording with preserved obligations.
- `review-guide` (beta) — PR body.
