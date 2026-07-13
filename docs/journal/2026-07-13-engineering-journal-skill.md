---
status: shipped
opened: 2026-07-12
updated: 2026-07-13
---

# Engineering journal skill

## Goal

Standardize repository-local engineering journals through a skill embedded and
served by `iop-skills`, while supporting intent-first and single-PR lifecycles.

## Decision Criteria

Ship when the served skill defines the four-state frontmatter lifecycle,
reconciliation behavior, derived-document handling, and advisory brief-candidate
reporting. Contract tests, the full repository test suite, a release build, and
an MCP list/call smoke test must pass.

## Scope

The skill opens, updates, closes, and reconciles journal entries. It does not
replace issues or pull requests and does not create problem or design briefs.

## Evidence

The workflow was informed by the engineering-journal practices in
[iopsystems/rezolus](https://github.com/iopsystems/rezolus/blob/main/.claude/skills/engineering-journal/SKILL.md)
and
[iopsystems/hpc-research](https://github.com/iopsystems/hpc-research/blob/main/.claude/skills/engineering-journal/SKILL.md).
The implementation is grounded in `skills/engineering-journal/SKILL.md`, the ten
cases in `skills/engineering-journal/evals/trigger-evals.json`, and embedded
loader contract tests in `src/main.rs`.

## Design and Implementation

Frontmatter is authoritative with `open`, `shipped`, `no-go`, and `superseded`.
Paused or blocked work remains open. The skill chooses intent-first or single-PR
mode from context, preserves repository conventions, reconciles an index and
existing derived documents, and reports possible problem-framing or design-
reasoning inputs without invoking brief skills.

The test-first implementation is recorded by commits `f2964e0` (failing
contract), `51ada7f` (served skill), and `044cd38` (evaluation corpus).

## Outcome

Shipped as an embedded MCP skill. `cargo fmt --check` passed, all 56 Rust tests
passed, `cargo build --release` succeeded, and an interactive MCP smoke test
confirmed that `tools/list` exposes `engineering-journal` and `tools/call`
returns the skill body.

## Derived Documents

This journal index was created. The temporary Superpowers design and
implementation plan were absorbed into this entry and removed from the branch;
their earlier commits preserve the review and execution history.

## Deferred or Reopen Items

Forward-testing the evaluation corpus with independent agents remains useful
when the execution environment explicitly permits delegated evaluation.
