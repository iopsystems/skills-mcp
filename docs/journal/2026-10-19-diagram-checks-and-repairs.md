---
status: open
opened: 2026-10-19
updated: 2026-10-19
---

# Diagram check records and focused repairs

## Goal and Scope

Draft the small Archify methodology lessons that fit the existing architecture,
dataflow, and format-layout skills. This uses the single-PR lifecycle and is
prepared for pull-request review. Interactive HTML, a shared diagram model, and new renderer
or delivery infrastructure remain separate work.

## Evidence

The comparison used Archify revision `88d1ae4498b6a1df610badf690c5406882622d15`.
Its [delivery contract](https://github.com/tt-a1i/archify/blob/88d1ae4498b6a1df610badf690c5406882622d15/archify/references/delivery-contract.md)
distinguishes artifact checks, browser evidence, and perceptual review. Repairs
follow diagnostic identity and measured evidence, preserve semantics, and have
a stopping condition. Its [repository-authoring contract](https://github.com/tt-a1i/archify/blob/88d1ae4498b6a1df610badf690c5406882622d15/archify/references/repository-authoring.md)
also distinguishes committed evidence from dirty working-tree inputs.

Our skills already derive facts from code, assert rendered geometry, enforce
freshness, and distinguish numerical checks from readability. The additions
make check results attributable to one source/artifact pair and make repairs
diagnosable without importing Archify's publishing pipeline.

## Design

1. Keep a compact check record in an existing journal, test report, or adjacent
   record. Identify source revision, dirty-input evidence, rendered path/hash,
   and regeneration command. Separate semantic checks, rendering checks,
   visual inspection, and required human approval; explain anything not run.
2. Report failures by stable rule and element identity, observed evidence, and
   expected constraint. Fix the implicated generator logic or layout while
   preserving topology, conditions, containment, LOC ratios, or byte offsets.
3. Test candidates separately from the last passing artifact. After two failed
   focused repairs, inspect source and measurements before one final attempt.
   If the issue persists, report it and retain the earlier artifact only as a
   labeled snapshot. Fewer errors is insufficient if a new defect appeared.

The repair limit is a draft default, not a measured optimum. Existing review
requirements and explicit single-use exceptions remain unchanged. No new
receipt schema, dependency, CLI, or automated publisher is introduced.

The architecture and dataflow templates mirror the active instructions, with
versions `0.2.1` and `0.1.1` and refreshed digests. Three scenarios per skill
cover evidence after an edit, preservation of diagram meaning during repair,
and persistent failure. The same cases join the two template corpora.

## Decision Criteria

An agent must distinguish which artifact passed which checks, retain an
unavailable check as a gap, and repair layout without falsifying its source.
Repeated failures must produce an actionable report rather than more arbitrary
edits. Existing source assertions and human review remain necessary.

## Validation and Outcome

Passed locally:

- `cargo test --locked --quiet`: 151 tests, including active/template body
  equivalence, manifest digests, and source citations.
- `cargo fmt --all -- --check` and `git diff --check`.
- Skill-creator's `quick_validate.py` validation for all three active skills
  and both templates.
- JSON parsing and unique scenario names: architecture 16, dataflow 26, and
  format-layout 26; architecture template 15 and dataflow template 11.

The scenario additions specify expected behavior; no independent agent
execution or reader comparison is claimed. No renderer implementation changes.

## Deferred Work

Exercise the revised instructions against a real diagram repair before treating
the diagnostic format or retry limit as behaviorally validated. Interactive
viewer work and shared rendering infrastructure remain outside this draft.

## Appendix: Skills Invoked

- `skill-creator` — scoped instruction edits and validation.
- `technical-prose` — concise wording and preservation of evidence strength.
- `engineering-journal` — rationale, scope, and validation limits.
