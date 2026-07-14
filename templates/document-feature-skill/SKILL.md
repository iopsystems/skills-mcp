---
name: document-feature
description: Use when creating or materially revising a project's README, code documentation, CLI help, diagrams, onboarding, or other feature documentation for users or developers.
---

# Document a Feature

## Purpose

Produce source-backed documentation that helps humans and agents complete real
tasks. Synchronize the relevant README, code documentation, actual rendered CLI
help, and visual models without hiding interface or architecture defects behind
prose. Agent checks never prove human usability.

## Project Contract

Read and fill the [audience charter](references/audience-charter.md) before
changing documentation. Discover repository instructions and established
terminology, layout, generation, validation, and review conventions. Recheck the
charter at the start of every material documentation effort; stop for one narrow
question when conflicting conventions would change the work.

The charter ranks human users, agent users, human developers, and coding agents
independently. Do not collapse them into a single audience or assume that success
for one establishes success for another.

## Required Algorithm

Follow these steps in order:

1. Discover project conventions and recheck the audience charter.
2. Map authoritative code, parser, schema, test, configuration, and design
   sources.
3. Enumerate modes, lifecycle states, and failure paths.
4. Freeze representative user and developer tasks with their expected outcomes.
5. Write or update the required surfaces with shared terminology.
6. Perform deterministic verification against actual rendered output and
   authoritative code.
7. Run blind task simulations plus a separate structured critic.
8. Apply risk-based human review.
9. Revise only specific findings, for at most three rounds.
10. Report evidence and unresolved interface or architecture problems.

Keep frozen tasks and expected outcomes stable during evaluation. If
authoritative evidence shows that frozen ground truth is wrong or inconsistent,
stop and report a product or interface issue. Do not silently change the expected
answer to make documentation pass.

Blind simulations receive the documentation and frozen tasks, not authoring
history or hints. The separate structured critic receives the output, frozen
expectations, and relevant documentation. Do not let the author self-certify both
roles. Fix concrete findings only. After three unsuccessful revision rounds,
stop and report the recurring ambiguity as a design smell rather than continuing
to polish prose.

## Map the Ground Truth

For every documented claim, identify its authoritative source and a verification
method. Typical sources include:

- command parsers and actual rendered CLI help for syntax and defaults;
- schemas, configuration loaders, and examples for value formats;
- code and tests for contracts, modes, lifecycle, side effects, and failures;
- design records for intentional architecture; and
- generation configuration for synchronized or generated surfaces.

Prefer observed behavior and executable checks over copied prose. Record a
traceability map when claims span several sources. Never make one documentation
surface authoritative merely because it is easier to read.

## Freeze Representative Tasks

Freeze tasks before drafting so evaluation cannot move with the prose. Cover
representative human and agent user goals plus human and coding-agent developer
goals. Each task states the starting context, question or action, expected
outcome, relevant failure or recovery path, and evidence that would count as a
pass. Include negative and boundary behavior, not only the happy path.

## Surface Checklists

### README

- Orient readers: what the project is, who it is for, and when to use it.
- Provide a verified quick start using commands actually run or checked.
- Organize task paths for common user goals and contribution paths for
  developers.
- Explain recovery from likely failures and link to deeper references.
- Use prominent visual models when relationships, nested ownership, or workflow
  are complex; keep a concise textual equivalent nearby.
- Preserve established navigation unless a major restructure passes its human
  review gate.

### Code documentation

- Document contracts, invariants, ownership, side effects, failure behavior,
  lifecycle, and safe extension points at the boundary they govern.
- Explain why a non-obvious constraint exists and what callers may rely on.
- Keep terms synchronized with README and CLI surfaces.
- Do not narrate obvious code, restate syntax, or duplicate implementation detail
  that will drift.

### CLI help

- Show canonical invocations and required value formats.
- State defaults, modes, alternatives, examples, and deprecations.
- Explain consequential failure behavior and recovery without hiding unsafe side
  effects.
- Verify actual rendered CLI help from the built or project-standard command;
  source annotations or parser definitions alone are not sufficient.
- Add or update deterministic tests when the project treats help output or CLI
  behavior as a contract.

## Visual Documentation

DOT is authoritative for diagrams. Commit the `.dot` source and rendered `.svg`
beside it. Prefer clusters for meaningful nesting such as subsystem ownership;
do not cluster merely for decoration. Validate DOT parsing and rendering, then
verify SVG freshness against the source using the project's recorded commands.
Keep a concise textual equivalent—a component table, ordered workflow, or both—
next to every diagram so meaning is not visual-only.

Diagrams must encode claims already verified from authoritative sources. Do not
use a diagram to guess architecture. Record tooling and exact render commands in
the audience charter so contributors can reproduce the SVG.

## Deterministic Verification

Run the charter's commands and compare every material claim with authoritative
code and rendered output. At minimum, verify changed links, command examples,
defaults, modes, failure paths, generated surfaces, DOT parse/render success,
committed SVG freshness, and any project-specific formatting or documentation
tests. Capture exact commands and results; do not claim a check you did not run.

## Evaluation and Review

Run blind task simulations independently for the frozen tasks. Then give the
simulation output, frozen expectations, and relevant documentation to a separate
structured critic. Classify concrete misses by audience, source, surface, and
risk. Revise only supported findings and rerun affected deterministic and blind
checks. Count a revision after a completed simulation-and-critic cycle; cap the
effort at three rounds.

Use risk-based human review. The following are hard human review gates:

- architecture diagrams and workflow diagrams;
- visual hierarchy and navigation;
- major README restructures;
- onboarding narratives; and
- other features whose success is primarily perceptual or subjective.

Human review is not replaceable by agent consensus. Agent checks never prove
human usability. Record the requested review, decision, and unresolved comments;
if the gate has not occurred, report it as pending and do not claim usability.

## Final Report

Report changed surfaces, authoritative sources checked, frozen tasks and
outcomes, exact deterministic commands and results, blind-simulation findings,
separate critic findings, revision count, human review status, and unresolved
interface or architecture problems. Distinguish verified correctness, agent
comprehension evidence, and human judgment. Never claim human usability without
the required human review.
