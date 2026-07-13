# Engineering Journal Skill Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Serve one `engineering-journal` skill that supports intent-first and single-PR journal lifecycles, validates frontmatter-driven state, reconciles derived documents, and reports journal material that may inform future briefs without creating briefs.

**Architecture:** Add one embedded instructional skill plus a machine-readable evaluation corpus. Extend the existing loader unit tests to prove the skill is embedded and that its contract covers lifecycle, reconciliation, and brief-reporting boundaries. Adopt the journal in this repository with a self-contained entry and index, absorbing the temporary design and plan records once their durable content is captured.

**Tech Stack:** Markdown Agent Skill, JSON evaluation fixtures, Rust unit tests, `include_dir!`, Cargo.

---

## File Map

- Create `skills/engineering-journal/SKILL.md`: the served workflow and templates.
- Create `skills/engineering-journal/evals/trigger-evals.json`: realistic trigger and behavioral evaluation cases.
- Modify `src/main.rs`: unit tests for embedded exposure and required contract markers.
- Create `docs/journal/README.md`: journal convention and index.
- Create `docs/journal/2026-07-13-engineering-journal-skill.md`: durable record of this effort.
- Delete `docs/superpowers/specs/2026-07-12-engineering-journal-skill-design.md`: content absorbed into the journal entry.
- Delete `docs/superpowers/plans/2026-07-13-engineering-journal-skill.md`: execution record absorbed into the journal entry after completion.

### Task 1: Add Failing Embedded-Skill Contract Tests

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Add a test module that specifies the new skill contract**

Append this module after `main`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engineering_journal_skill_is_embedded() {
        let skills = load_skills().expect("embedded skills should load");
        let journal = skills
            .iter()
            .find(|skill| skill.name == "engineering-journal")
            .expect("engineering-journal should be served");

        assert!(journal.description.contains("non-trivial"));
        assert!(journal.body.contains("status: open"));
        assert!(journal.body.contains("status: shipped"));
        assert!(journal.body.contains("status: no-go"));
        assert!(journal.body.contains("status: superseded"));
    }

    #[test]
    fn engineering_journal_contract_covers_reconciliation_boundaries() {
        let skills = load_skills().expect("embedded skills should load");
        let body = &skills
            .iter()
            .find(|skill| skill.name == "engineering-journal")
            .expect("engineering-journal should be served")
            .body;

        for required in [
            "intent-first",
            "single-PR",
            "docs/journal/README.md",
            "Problem framing candidate",
            "Design reasoning candidate",
            "Do not invoke `frame-problem`",
            "Do not invoke `propose-design`",
        ] {
            assert!(body.contains(required), "missing contract marker: {required}");
        }
    }
}
```

- [ ] **Step 2: Run the focused tests and verify RED**

Run:

```bash
cargo test engineering_journal -- --nocapture
```

Expected: both tests fail because no embedded skill named `engineering-journal` exists.

- [ ] **Step 3: Commit the failing tests**

```bash
git add src/main.rs
git commit -m "Test engineering journal skill contract"
```

### Task 2: Add the Served Engineering Journal Skill

**Files:**
- Create: `skills/engineering-journal/SKILL.md`

- [ ] **Step 1: Create the skill with this complete contract**

```markdown
---
name: engineering-journal
description: >-
  Use when starting, continuing, handing off, or closing a non-trivial engineering or research effort in a shared repository; when preserving a measured negative result; when reconciling journal status, index, or backlog drift; or when identifying journal material that may inform future problem or design briefs.
---

# Engineering Journal

## Purpose

Keep one durable, repository-local record per non-trivial effort: its goal,
evidence, decisions, implementation, outcome, and unresolved work. Issues and
pull requests remain the task layer; the journal is the narrative and decision
layer. Do not use it for self-evident one-line changes.

## Discover the Repository Convention

Before writing, inspect repository instructions and search for `docs/journal/`,
`journal/`, an index, backlog, roadmap, assumptions, canonical facts or data,
and existing entry frontmatter. Preserve established paths, headings, extra
metadata, and derived-document conventions.

If no convention exists, use `docs/journal/YYYY-MM-DD-short-slug.md` and
`docs/journal/README.md`. Bootstrap both from the templates below. Do not create
a project-wide backlog unless the user requests one.

Search the journal and Git history for prior work before opening a new entry.

## Choose the Lifecycle

Support both modes:

- **intent-first** — land an `open` entry before implementation when early
  visibility, coordination, or a GO/NO-GO probe matters; close it later.
- **single-PR** — create and close the entry in the implementation PR when work
  is already underway or tightly scoped.

State which mode you are using. Honor an explicit user choice.

## Frontmatter State

Frontmatter is authoritative:

```yaml
---
status: open
opened: YYYY-MM-DD
updated: YYYY-MM-DD
---
```

Allowed states:

- `status: open` — active, paused, or blocked. Record the blocker and restart
  condition in the body; do not invent a paused state.
- `status: shipped` — landed with implementation and verification evidence.
- `status: no-go` — rejected or falsified with the mechanism and a concrete
  reopen condition.
- `status: superseded` — replaced by an existing entry; require
  `superseded_by: <entry-slug>`.

Optional fields such as `issues: [123]` and `prs: [456]` appear only when known.
Never invent lifecycle dates, links, measurements, or evidence.

Valid transitions are `open` to any terminal state, and `shipped` or `no-go` to
`superseded` when a replacement entry exists. Do not reopen terminal entries in
place; open a new entry that links the prior result. Use `superseded_by` only on
superseded entries.

## Entry Template

```markdown
---
status: open
opened: YYYY-MM-DD
updated: YYYY-MM-DD
---

# <Effort title>

## Goal
## Decision Criteria
## Scope
## Evidence
## Design and Implementation
## Outcome
## Derived Documents
## Deferred or Reopen Items
```

Omit `Decision Criteria` only when there is no meaningful GO/NO-GO gate. While
open, state the current result or say explicitly that the outcome remains open.

Ground claims in durable source paths, commit SHAs, pull requests, measurements,
or canonical datasets. Mark unverifiable gaps; never fabricate them. A NO-GO is
a first-class result. Record what was tried, the mechanism, and what evidence or
changed condition would reopen it.

When the repository has adopted the journal as its durable design record, absorb
the durable goal, decisions, rationale, dead ends, and verification from temporary
specification or plan documents into the entry, then remove those temporary files
in the same PR. Do not rewrite unrelated historical design records.

## Operations

### Open

Create the entry and index row. Capture goal, scope, evidence already known,
decision criteria where useful, intended approach, and current unresolved items.
For intent-first work, land this record before implementation.

### Update

Re-read code, Git history, and durable evidence. Update `updated`, current
implementation, evidence, outcome, and unresolved items. Paused or blocked work
remains open and names its blocker and restart condition.

### Close

- For shipped work, set `status: shipped` and record what landed, where, and how
  it was verified.
- For a negative result, set `status: no-go` and record the mechanism and reopen
  condition.
- For replacement, set `status: superseded`, add `superseded_by`, verify the
  target exists, and explain the replacement.

Update `updated`, the index, and affected derived documents in the same change.

### Reconcile

Scan journal markdown files except the index. Treat entry frontmatter as source
of truth and check:

- `status`, `opened`, and `updated` exist.
- Status is `open`, `shipped`, `no-go`, or `superseded`.
- `updated` is not earlier than `opened`.
- A superseded entry names an existing replacement.
- A shipped entry carries implementation and verification evidence.
- A no-go entry carries its mechanism and reopen condition.
- Index date, title, status, and membership match the entries.
- Deferred and reopen items match an existing backlog.

Repair mechanical index drift. For changes requiring judgment, report the exact
proposed edit before applying it. Preserve repository-specific headings and extra
frontmatter rather than rewriting mechanically.

Selectively reconcile existing backlog, roadmap, assumptions and limitations,
canonical fact sheets, datasets, and bibliographies only when materially affected.
Remove or close backlog items completed or deprecated by the outcome.

## Report Possible Brief Inputs

Reconciliation may report, without creating vault artifacts:

- **Problem framing candidate** — durable problem, constraints, and evidence.
- **Design reasoning candidate** — approach, alternatives, and rationale.

For each candidate, name the entry and summarize why it qualifies. This is an
advisory handoff only. Do not invoke `frame-problem`. Do not invoke
`propose-design`. Do not create or update briefs unless the user separately asks
to start that workflow.

## Index Template

```markdown
# Engineering Journal

One durable, repository-local record per non-trivial effort. Entry frontmatter
is authoritative for lifecycle status.

| Opened | Effort | Status |
| --- | --- | --- |
| YYYY-MM-DD | [Effort title](YYYY-MM-DD-short-slug.md) | open |
```

## Stop Conditions

- Stop a lifecycle update when required evidence or a supersession target is
  missing; state the gap.
- Ask one narrow question when repository conventions conflict and the answer
  changes where or how records are written.
- Never use a GitHub issue, transient plan, or unmerged scratch file as the only
  durable record of a non-trivial effort.
- Never silently create briefs during journal reconciliation.
```

- [ ] **Step 2: Run the focused tests and verify GREEN**

Run:

```bash
cargo test engineering_journal -- --nocapture
```

Expected: 2 passed, 0 failed.

- [ ] **Step 3: Commit the skill**

```bash
git add skills/engineering-journal/SKILL.md
git commit -m "Add engineering journal skill"
```

### Task 3: Add a Machine-Readable Evaluation Corpus

**Files:**
- Create: `skills/engineering-journal/evals/trigger-evals.json`
- Modify: `src/main.rs`

- [ ] **Step 1: Add evaluation cases**

```json
{
  "evals": [
    {
      "name": "intent-first kickoff",
      "prompt": "Start a cross-team storage migration that needs an exploratory probe before implementation.",
      "expectations": ["Select intent-first mode", "Create an open entry before implementation", "Include decision criteria"]
    },
    {
      "name": "single-PR shipped close",
      "prompt": "The parser refactor is implemented and tested; journal it in this PR.",
      "expectations": ["Select single-PR mode", "Set status to shipped", "Cite implementation and verification evidence"]
    },
    {
      "name": "paused remains open",
      "prompt": "Pause the cache investigation until new hardware arrives.",
      "expectations": ["Keep status open", "Record the blocker", "Record the restart condition", "Do not invent a paused status"]
    },
    {
      "name": "measured no-go",
      "prompt": "The prototype regressed p99 by 18 percent, so close the effort without shipping.",
      "expectations": ["Set status to no-go", "Record the measured mechanism", "Record a concrete reopen condition"]
    },
    {
      "name": "invalid supersession",
      "prompt": "Mark this entry superseded by 2026-08-01-new-design, which has not been created.",
      "expectations": ["Reject the lifecycle update", "Report the missing target", "Do not create the target silently"]
    },
    {
      "name": "stale index",
      "prompt": "The journal entry says shipped but the README index still says open. Reconcile it.",
      "expectations": ["Treat entry frontmatter as authoritative", "Repair the index status"]
    },
    {
      "name": "backlog reconciliation",
      "prompt": "This shipped effort completed one backlog item and left a new deferred measurement.",
      "expectations": ["Remove or close the completed backlog item", "Add the deferred item with a journal link"]
    },
    {
      "name": "problem brief candidate",
      "prompt": "Reconcile an entry containing a durable problem statement, constraints, and production evidence.",
      "expectations": ["Report a Problem framing candidate", "Do not invoke frame-problem", "Do not create a brief"]
    },
    {
      "name": "design brief candidate",
      "prompt": "Reconcile an entry comparing three designs and explaining the selected tradeoff.",
      "expectations": ["Report a Design reasoning candidate", "Do not invoke propose-design", "Do not create a brief"]
    },
    {
      "name": "trivial change",
      "prompt": "Fix a typo in one comment.",
      "expectations": ["Do not create a journal entry"]
    }
  ]
}
```

- [ ] **Step 2: Add a fixture-shape test**

Add this test to the existing `tests` module:

```rust
    #[test]
    fn engineering_journal_evals_cover_key_scenarios() {
        let raw = include_str!(
            "../skills/engineering-journal/evals/trigger-evals.json"
        );
        let value: serde_json::Value =
            serde_json::from_str(raw).expect("journal evals should be valid JSON");
        let evals = value["evals"]
            .as_array()
            .expect("journal evals should contain an evals array");

        assert_eq!(evals.len(), 10);
        for eval in evals {
            assert!(eval["name"].as_str().is_some());
            assert!(eval["prompt"].as_str().is_some());
            assert!(eval["expectations"].as_array().is_some_and(|v| !v.is_empty()));
        }
    }
```

- [ ] **Step 3: Run the focused tests**

```bash
cargo test engineering_journal -- --nocapture
```

Expected: 3 passed, 0 failed.

- [ ] **Step 4: Commit the evaluation corpus**

```bash
git add src/main.rs skills/engineering-journal/evals/trigger-evals.json
git commit -m "Add engineering journal evaluations"
```

### Task 4: Adopt and Close the Journal Record

**Files:**
- Create: `docs/journal/README.md`
- Create: `docs/journal/2026-07-13-engineering-journal-skill.md`
- Delete: `docs/superpowers/specs/2026-07-12-engineering-journal-skill-design.md`
- Delete: `docs/superpowers/plans/2026-07-13-engineering-journal-skill.md`

- [ ] **Step 1: Create the journal index**

```markdown
# Engineering Journal

One durable, repository-local record per non-trivial effort. Entry frontmatter
is authoritative for lifecycle status.

| Opened | Effort | Status |
| --- | --- | --- |
| 2026-07-12 | [Engineering journal skill](2026-07-13-engineering-journal-skill.md) | shipped |
```

- [ ] **Step 2: Create the durable effort entry**

Create this self-contained entry:

```markdown
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
reporting; contract tests and the full repository test suite must pass.

## Scope

The skill opens, updates, closes, and reconciles journal entries. It does not
replace issues or pull requests and does not create problem or design briefs.

## Evidence

The workflow was informed by the engineering-journal practices in
`iopsystems/rezolus` and `iopsystems/hpc-research`. The implementation is grounded
in `skills/engineering-journal/SKILL.md`, its evaluation corpus, and the embedded
loader contract tests in `src/main.rs`.

## Design and Implementation

Frontmatter is authoritative with `open`, `shipped`, `no-go`, and `superseded`.
Paused or blocked work remains open. The skill chooses intent-first or single-PR
mode from context, preserves repository conventions, reconciles an index and
existing derived documents, and reports possible problem-framing or design-
reasoning inputs without invoking brief skills.

## Outcome

Shipped as an embedded MCP skill. `cargo fmt --check`, `cargo test`,
`cargo build --release`, and an MCP list/call smoke test pass.

## Derived Documents

This index was created. The temporary Superpowers design and implementation plan
were absorbed into this entry and removed.

## Deferred or Reopen Items

Forward-testing the evaluation corpus with independent agents remains useful when
the execution environment explicitly permits delegated evaluation.
```

- [ ] **Step 3: Remove the absorbed temporary design and plan**

```bash
git rm docs/superpowers/specs/2026-07-12-engineering-journal-skill-design.md
git rm docs/superpowers/plans/2026-07-13-engineering-journal-skill.md
```

- [ ] **Step 4: Run complete verification**

```bash
cargo fmt --check
cargo test
cargo build --release
git diff --check main...HEAD
```

Expected: formatting passes, all tests pass, release build succeeds, and the diff
has no whitespace errors.

- [ ] **Step 5: Commit the journal adoption**

```bash
git add docs/journal docs/superpowers
git commit -m "Journal engineering journal skill"
```

### Task 5: Final Contract and Bundle Verification

**Files:**
- Verify only; no expected file changes.

- [ ] **Step 1: Verify the MCP tool is listed and callable**

```bash
(
  printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"journal-smoke","version":"0"}}}'
  printf '%s\n' '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  printf '%s\n' '{"jsonrpc":"2.0","id":2,"method":"tools/list"}'
  printf '%s\n' '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"engineering-journal","arguments":{}}}'
) | ./target/release/iop-skills | jq -c 'select(.id == 2 or .id == 3)'
```

Expected: `tools/list` contains `engineering-journal`, and `tools/call` returns
the skill body containing `# Engineering Journal`.

- [ ] **Step 2: Verify repository state**

```bash
git status --short --branch
git log --oneline main..HEAD
```

Expected: clean worktree and the design, safety, tests, skill, evaluations, and
journal commits on the feature branch.
