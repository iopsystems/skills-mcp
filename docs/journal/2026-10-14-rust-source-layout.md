---
status: open
opened: 2026-10-14
updated: 2026-10-14
---

# Rust source layout follows responsibility

## Goal

Draft a reusable skill that keeps Rust source manageable as it grows, using a
specific review finding to distinguish file length from mixed responsibilities.

## Evidence

The motivating review concerned a large Rust stream registry combining
declarative tables, schema types, host discovery, sizing constants, lookup
helpers, and extensive tests with a witness table. The requested split preserves
downstream imports through the existing public facade. The reusable lesson is
to separate responsibilities while keeping that public contract stable.

Before editing, fetched `origin` and confirmed this clean checkout matched
`origin/main` at `5cd5389`. No pull or rebase was necessary.

## Design

`skills/repository/rust-source-layout/SKILL.md` turns that finding into guidance
for choosing cohesive boundaries during substantial implementation or review.
It treats roughly 500 handwritten production lines as a prompt to inspect,
not an enforced maximum, and distinguishes production logic, declarative tables,
and tests. Schema and host-discovery concerns can move out of a registry while
a small table-specific lookup can remain beside its data.

The skill preserves public paths, type identity, values, feature gates, and
visibility. Private unit tests remain in child modules rather than forcing
implementation details into the public API. Generated source, coherent large
tables, and unsafe protocols resist mechanical fragmentation. A small unrelated
fix does not authorize a broader cleanup.

## Evaluation corpus

Eight scenarios freeze the intended decisions: a mixed registry, a coherent
long table, a short mixed module, a narrow bug fix, an unsafe protocol, private
unit tests with fixtures, feature-gated public APIs, and generated source.
The corpus uses the repository's `evals/trigger-evals.json` convention. It is
a set of behavioral expectations, not an automated demonstration that an agent
will follow them. No wording-matching Rust tests were added.

## Outcome

Drafted at the user's request and prepared for pull-request review. Validation
passed:

- Skill creator's `quick_validate.py` and parsing all eight evaluation scenarios.
- `cargo fmt --all -- --check`.
- `cargo clippy --all-targets --locked -- -D warnings`.
- `cargo test --locked`: 151 tests passed.
- `cargo build --locked` and `cargo build --release --locked`.
- `./scripts/render-diagrams.sh --check` and `./scripts/mcp-smoke.sh`.
- Direct MCP `tools/list` and `tools/call` for `rust-source-layout`: the new
  tool is discoverable and returns its instructions.
- `git diff --check`.

The first test run caught the existing inventory assertion still expecting
28 catalog entries; it now expects 29 (25 active skills and four templates),
and the README lists the same inventory. A diagram check initially could not
create its temporary file under the sandbox; rerunning with the required
filesystem access passed. No generated diagram changed.

These checks validate packaging and repository integration, not agent behavior.
The 500-line inspection prompt is an uncalibrated heuristic, not a measured
maintainability threshold. The draft and scenario corpus remain open for review.

## Deferred or Reopen Items

Independent forward simulation, baseline comparison, and a separate critic
remain open before treating the guidance as behaviorally validated. Observe
whether the soft size prompt causes needless splits, and whether the facade
guidance preserves real downstream consumers. The user requested a pull request
for this work; publishing a release or changing the installed MCP binary remains
outside this change.

## Skills Used

- `skill-creator` — scoped instructions, precise discovery, and skill validation.
