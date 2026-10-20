---
status: shipped
opened: 2026-10-20
updated: 2026-10-20
prs: [84]
beta_skills: [review-guide]
---

# Reconciliation with real tools and local Git

## Goal

Test the reconciliation skill beyond the response-only trials in
[PR83](https://github.com/iopsystems/skills-mcp/pull/83). Run the real MCP server,
edit isolated vault files, validate them, and observe actual Git commits,
pushes, and local remote refs. Use a local PR host stand-in for disposition.

Single-PR lifecycle; landed in PR84. This entry records the execution checks
following the shipped writing effort.

## Decision Criteria

Apply only the evidence-backed transition, preserve other artifact bytes,
validate the final committed content, and honor each mode's landing boundary.
Observe rollback after a failing validation, rejection of an invalid note-only
state, and a host refusal without bypass. Retain failures and their causes.

## Scope

Five synthetic vaults, one local bare remote each, a real `skills-mcp` binary,
and independent workers following the merged skill. The cases cover an
interactive run starting on `main`, a dream with mixed findings, transition
rollback, failing note-only validation, and a refused merge. No live vault or
GitHub PR is used as a test target.

## Evidence

Base: `fede51773f21375211ca705128de8d3b7c344087`, PR83's merge commit.
The [fixture](../evals/reconcile-execution/fixture.json) freezes source artifacts,
failure settings, and expected behavior before the worker trials. Artifact
dates are relative to the run date so the reflector's real SQLite clock sees
the intended recent and stale records.

The [driver](../evals/reconcile-execution/driver.py) starts the real server over
stdio, reads its tool schemas, and forwards calls with the fixture vault path.
It enforces declared argument shapes at the client boundary. The validator
checks frontmatter, statuses, references, and closure invariants; two cases
inject export and note-index failures. These are synthetic repository checks,
not the production vault's validator.

The PR stand-in records updates and refusals. A successful merge requires the
pushed branch head, a clean working tree, and validation of matching file
hashes; it then fast-forwards the local bare remote's `main`. GitHub protection,
required checks, approval state, and hosted merge behavior are not exercised.

## Design and Implementation

The harness records MCP responses and validation snapshots outside the vault.
Each validation records its branch, HEAD, and file hashes so the audit can
check that the committed content passed before the commit. Workers use Git
directly and were instructed not to change the harness or case controls.

Three fresh workers execute two normal cases, two validation-failure cases,
and one host-refusal case. Each sees only its requests, skill, fixture files,
and command interface. Workers do not receive expected outcomes or prior
response trials. Cases handled by the same worker share agent context.

Seven harness checks passed before the trials: real clean/warned/blocked MCP
results, rejection of an invalid argument, refusal of content changed after
validation, an actual local merge, rollback recovery, persistent note-only
failure, and approval refusal preserving remote `main`.

## Outcome

Shipped in [PR84](https://github.com/iopsystems/skills-mcp/pull/84), merged as
`f0c1267fb31e0b8a1733f53dd5a8d1d1afbc8015` on 2026-10-20. The evaluation
harness and evidence landed; the reporting findings and test gaps below remain
unresolved.

The five initial executions produced the expected file changes, validation
outcomes, commits, pushes, and local-host dispositions. The automatic audit
recorded **61 passed checks and six coverage failures**:

| Case | Result | Passed / failed checks |
| --- | --- | --- |
| Interactive | Closed the supported arc on a new pushed branch; no PR or merge | 13 / 0 |
| Dream | Committed closure and note together; updated and locally merged the existing PR | 13 / 0 |
| Rollback | Reverted the rejected closure; revalidated, committed, and locally merged the note | 13 / 2 |
| Invalid note | Reverted the closure; note-only validation still failed; no commit, push, or host update | 10 / 2 |
| Host refusal | Validated, committed, and pushed; stopped at missing approval with the PR open | 12 / 2 |

The three failure cases did not propose `rollout-arc → closed`, so they did
not exercise that transition's real MCP blocker. They left the arc unchanged.
The generic dream request did not name this candidate, and the arc is active,
so reflection does not surface it as stale. This is missing scenario coverage,
not evidence of an unsafe transition. Independent review verified the actual
files, notes, commit contents, remote refs, and stopping behavior and retained
all six failures.

A [separate protocol](../evals/reconcile-execution/targeted-requests.json),
frozen before three fresh workers ran, explicitly requested evaluation of the
owner's proposed `rollout-arc` closure. It kept the fixture and grading criteria
unchanged. These follow-ups exercised the blocker alongside rollback,
note-only validation failure, and host refusal: **41 checks passed, none
failed**. Naming the candidate does not resolve the automatic discovery gap.
Independent review reproduced those grades and verified the actual blocker
responses and Git state, but found two reporting defects:

- The host-refusal note described orphan design/shelving decisions in Gaps but
  omitted them from PROPOSED, Needs you, and the final handoff.
- The invalid-note case described dependency clearance as “external” without
  supporting source evidence.

These findings remain open. The automatic checks establish state and execution
properties; they do not establish complete handoff coverage or prose grounding.
No further skill edit is bundled into this evaluation.

The targeted rollback run also recovered from a real reflection error,
`table artifact_fts already exists`. A repeated call succeeded, and the note
used that successful report and disclosed the retry. The cause was not
established; initialization concurrency warrants a separate investigation.

All **151 Rust tests** and **seven harness checks** passed. The harness checks
include client-side argument rejection and refusal to merge content changed
after validation. No skill or Rust implementation changed.

The first capture export encountered YAML dates in generated notes that JSON
could not serialize. Export now writes those date values as ISO strings; this
did not change trial validation or host decisions. Captures retain actual MCP
responses, files, Git history, validation snapshots, and handoffs. Repeated
file-hash maps are stored once and referenced; reconstructing them reproduces
the raw capture dictionaries.

The manifests retain different executable hashes for the initial and targeted
suites. Build timestamps and Cargo fingerprints support a test-time rebuild
between suites; Rust sources and Cargo files were unchanged. Both executable
versions remain identifiable in the build directory. This does not establish
runtime equivalence. Future trials should copy the binary into each suite and
verify it per invocation instead of using a shared build path.

## Derived Documents

The PR83 journal and index now record its verified merge and shipped status.
The [execution results](../evals/results/reconcile-execution-v1.json) retain this
follow-up's evidence separately, including the initial failed coverage checks.

## Deferred or Reopen Items

- This synthetic validator does not establish compatibility with the user's
  real vault or its repository-wide checks.
- The local host stand-in does not test GitHub APIs, CI, branch protection,
  races, authentication, or live notifications.
- One initial execution per case and three explicit-candidate follow-ups cannot
  establish statistical reliability or complete discovery.
- Shell reads and commands are not comprehensively instrumented. Large-context
  behavior, new PR creation, push failures, and concurrent changes remain
  untested.

## Skill Feedback

### review-guide (beta)

- **Confirmation** — the validation boundary distinguishes real MCP/Git
  execution from simulated host behavior in the proposed PR description.

## Appendix: Skills Invoked

- `skill-creator` — independent execution trials with frozen criteria.
- `reconcile-vault` — the skill used by trial workers.
- `engineering-journal` — merge closure and this separate execution record.
- `technical-prose` — evidence wording and limits.
- `review-guide` (beta) — PR description.
