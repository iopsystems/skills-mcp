---
status: open
opened: 2026-10-20
updated: 2026-10-20
beta_skills: [review-guide]
---

# Reuse context in design briefs and session notes

## Goal and Scope

Shorten `propose-design` and make clarification depend on missing information.
Update `discuss` to use known authorship and authorization. Preserve framing,
accepted-brief immutability, transition checks, and the boundary between
recording a note and acting on its implications.

Single-PR lifecycle; includes the verified PR88 journal closure. Base:
`883904f10b7f8ede0a125fb7d75a19d3238f8a1d`.

## Evidence and Decision Criteria

PR88's contract review identified unconditional clarification and authorship
steps in these dependencies. Its planning simulations showed no resulting
failure, so this is an instruction-consistency change rather than a repair
for a demonstrated execution regression.

`propose-design` repeats pairing and transition rules and adds rhetoric about
unframed designs and how much effort questioning deserves. The revision keeps
one framing decision tree, the six design questions and the commit workflow.
Questions and approval requests now depend on what is missing from the supplied
context. Existing authorization carries through from a calling skill.

The `discuss` change is limited to two workflow steps. It asks for missing
identity or participant details and carries existing note authorization.
That authorization does not extend to proposed followups.

Accept no lost artifact or permission requirement and no observed regression
against the frozen cases. Activation descriptions and fenced schemas remain
unchanged.

## Evaluation

The [frozen protocol](../evals/fixtures/design-discuss-concision-v1.json) has
25 expectations across six cases per variant:

- Write a successor to an accepted design, then encounter a promotion blocker.
- Ask about a consequential missing design requirement.
- Handle obsolete framing.
- Begin problem framing when no frame exists.
- Commit an authorized solo note and propose related followups.
- Ask for missing meeting attendees.

The [harness](../evals/design-discuss-eval.py) creates isolated Git vaults with
no remotes and randomizes neutral labels. Workers receive raw artifacts and
requests without the rubric. One worker per variant executes all six cases
in order. Search, edges and transition checks use a local fixture tool;
file writes and commits are real. Fixture calls are logged with results and
the current HEAD. The audit captures artifact text, Git history and final state.
It does not instrument every shell command or transient file write.

Preflight checks exercised search, incoming frame edges, unquoted YAML dates,
an unresolved-dependency blocker, empty search and obsolete framing before
worker execution. Independent blind grading precedes revealing the skill
mapping for a separate contract review.

## Outcome

`propose-design` decreases from 812 to 603 words
(26%). The two focused `discuss` clarifications increase it from 743 to 790
words. Both generic skill validators pass.

Both versions passed all **25 behavioral expectations** under blind review.
Each created and committed a successor design and a solo session note. The
design remained draft after the fixture checker reported its unresolved
dependency. Other cases asked for missing information without writing to the
vault. The reviewer corroborated final artifact bytes and committed changes
against the initial state; existing artifacts remained unchanged.

Each variant passed 42 mechanical checks across six vaults. Both workers
reported no execution errors. These results support the scoped revision; they
do not demonstrate a behavioral advantage over the original.

The separate contract review found no unintended lost requirements. It verified
all framing branches, the explicit no-frame exception, six design questions,
immutability, commit paths, transition gates and discussion followup boundaries.
Descriptions and fenced schemas remain byte-identical. The
[results](../evals/results/design-discuss-concision-v1.json) preserve all twelve
captures, four proposed diffs, hashes, frozen blind grades, contract review and
validation evidence.

All 154 Rust tests passed after cleaning the package in the shared target and
rebuilding with incremental compilation disabled.

## Deferred or Reopen Items

The fixture tool is not the production MCP server or validator. These small
repositories do not test live integration, repeated-run reliability or human
comprehension. Multiple matching frames, the explicit `frames: none` exception
and successful promotion receive static contract review rather than execution.
Calling-skill authorization inheritance and missing design authorship also
remain untested in live integration.

## Skill Feedback

### review-guide (beta)

- **Confirmation** — separating real artifact/Git execution from simulated
  lookup and transition results keeps the PR's validation claim scoped.

## Appendix: Skills Invoked

- `skill-creator` — scoped revisions and independent forward-testing.
- `technical-prose` — remove repeated rules and rhetoric.
- `propose-design`, `discuss` — evaluated local workflows.
- `frame-problem`, `vault-search` — dependency instructions read by both workers;
  fixture tools supplied the lookup and transition results.
- `engineering-journal` — evaluation record and PR88 closure.
- `review-guide` (beta) — PR description and validation limits.
