---
status: open
opened: 2026-10-22
updated: 2026-10-22
prs: []
---

# Skill text that contradicts other skills

## Goal

Correct eight places where a skill contradicts another skill, a rule in the
same skill, or the repository it is served to. Each was checked against
`origin/main` at `457b879` before editing.

## Evidence and Changes

`plan-feature`:

- Problem brief: it named the file `briefs/<id>-problem.md`. `frame-problem`
  defines the id as `<date>-<slug>-problem` and the file as `briefs/<id>.md`,
  so following both produced a name ending `-problem-problem.md`. It now names
  `briefs/<id>.md`.
- Repo design doc template: "Ordered work items from Area 6" referred to
  numbering that `d9bb433` removed. It now names the Coordination section.

`dataflow-diagram` and `architecture-diagram`:

- `dataflow-diagram` numbered compute nodes with circled digits from U+2460;
  `architecture-diagram` lists those as a dead end because they fail in font
  fallback. `dataflow-diagram` now puts a plain digit in a layout engine's label
  and draws a circle plus a digit when the generator emits geometry.
- Single-use charts: "Skip the charter and the review gate" contradicted "Human
  review gates every chart", the red flag for a chart shipped without human
  review, and the README rule that human review is mandatory for diagrams. The
  section now skips only the charter. The existing trigger eval for this case
  already expected only the charter and CI freshness to be skipped.
- The `architecture-diagram` description listed "dataflow chart" as a trigger
  while its body sends a pipeline to `dataflow-diagram`. The trigger is removed.

Both diagram skills ship a template copy whose body
`diagram_skills_active_and_template_bodies_stay_in_sync` requires to match, so
the same edits went into `templates/`, with new `SKILL.md` digests and patch
versions: `architecture-diagram-skill` 0.2.4, `dataflow-diagram-skill` 0.1.4.

`review-guide`:

- Its review-status example ended "independent re-review of the fix remains
  pending". The 2026-10-18 PR description evaluation found that "remains
  pending" implies a future review the evidence did not establish. The example
  now reads "no independent review of the fix is recorded".
- The citation paragraph named this repository's guard by its test file, which
  means nothing in the other repositories the skill is served to. It now
  describes the check.

`rust-source-layout`: validation covered "targets already supported by the
environment", which accepts a macOS run for a split that moved Linux-only code
the host never compiles. The skill now requires the gated target's CI job or a
`cargo check --target <triple>` to pass, reports the gate as unverified when
neither ran, and notes that a cross check runs no tests.

## Evaluation

Seven of the eight changes remove a contradiction between two texts. Their
check is that the contradicting phrase is gone from `skills/` and `templates/`
and that the surviving text agrees with the rule it conflicted with; a grep for
each removed phrase returns nothing. No forward simulation was run for them.

The `rust-source-layout` change alters what an agent requires before calling a
split done, so it got a baseline, a forward simulation and a blind critic.
Outcomes frozen before either arm ran:

1. States that the macOS run does not compile the Linux-gated code.
2. Requires a passing Linux compile, naming the CI job or a cross-target check.
3. Does not report Linux as verified from macOS results; marks it unverified
   when no Linux check ran.
4. Proposes no unrelated refactors or file-name and line-count tests.

Task: a user split a reactor module into timer, kqueue (macOS) and io_uring
(Linux) files, ran fmt, clippy and tests on a MacBook, and asked whether the
change was done and for a PR validation section. One fresh agent read the
`origin/main` skill, another the revised skill; neither saw the outcomes.

A separate critic graded both under neutral labels. Both arms passed all four
outcomes. Both said `uring.rs` was not compiled, both offered a cross-target
check and the Linux CI job, and both left the Linux line as a placeholder. The
critic rated the baseline marginally better for separating the cross check from
the CI test run, and the revised arm marginally better for labelling the Linux
gate "unverified". Both wrongly said `cargo fmt` did not cover `uring.rs`.

This is a null result for one case and one response per arm. The baseline
model inferred the requirement from the user's description; the revision makes
it explicit instead of depending on that inference. The sentence on cross
checks running no tests was added after the critic's note.

## Validation

`cargo fmt --all -- --check`, `cargo clippy --all-targets --locked -- -D
warnings`, `cargo test --locked` (158 tests) and `./scripts/mcp-smoke.sh`
passed. No diagram source changed, so `render-diagrams.sh --check` was not run.

## Open

- The `architecture-diagram` single-use trigger eval does not yet assert that
  human review still applies.
