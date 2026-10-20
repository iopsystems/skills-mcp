---
status: shipped
opened: 2026-10-20
updated: 2026-10-20
prs: [86]
beta_skills: [review-guide]
---

# Atomic vault index initialization

## Goal

Fix the concurrent SQLite schema initialization failure observed during
[PR84's execution trials](2026-10-20-reconcile-execution.md). Single-PR lifecycle.

## Decision Criteria

Concurrent callers must not interleave schema replacement. A failed replacement
must preserve the prior tables and version. Existing indexing and vault-tool
tests must continue to pass.

## Evidence

Base: `f0c1267fb31e0b8a1733f53dd5a8d1d1afbc8015`.
The recorded trial returned `table artifact_fts already exists`; a retry
succeeded. The original event alone did not establish the cause.

A new regression test releases eight connections together against a new index
or an outdated schema. Against the original code it reproduced the same FTS
error along with other duplicate-table/index errors. Schema version inspection,
table replacement, and version storage were separate autocommit operations:
multiple callers could decide to initialize and interleave their DDL.

A second test injects a failure while storing the new version. The original
code had already discarded the prior search data; the test failed when querying
that data after the error.

## Design and Implementation

An immediate transaction acquires SQLite's writer lock before inspecting the
stored version. It includes metadata creation, table replacement, and version
storage. Waiting callers read the version after the first transaction completes.
An error rolls back the entire replacement, including FTS tables.

An MCP stress test then found a separate startup boundary: the transaction-only
build returned `set journal_mode=WAL: database is locked` for one of 40 calls.
WAL setup can return BUSY without waiting through SQLite's busy handler.
The final implementation retries only that result for up to five seconds,
sleeping at most 10 milliseconds between attempts. Other errors return
immediately. Normal SQLite lock waiting is restored after WAL setup succeeds.
A held-reader test covers both timeout and success after the reader releases.

The schema version and vault Markdown are unchanged.

## Outcome

Shipped in [PR86](https://github.com/iopsystems/skills-mcp/pull/86), merged as
`7d9b5992a269e94fdf33a9496bdd3b5bde142c0d` on 2026-10-20. The transaction,
bounded WAL retries, regression tests, and stress evidence landed.

Both schema regressions fail against the original implementation and pass with
the transaction. All **154 Rust tests**, **Clippy with warnings denied**, and
format checks pass on the final code.

The [MCP stress runner](../evals/check-index-concurrency.py) starts eight
processes together against each of five fresh synthetic vaults. The original
binary had two FTS schema collisions in 40 calls. The transaction-only binary
had one WAL setup failure. The final binary completed **40/40 calls** with
consistent reflection reports. The
[captured evidence](../evals/results/vault-index-initialization-v1.json) retains
both earlier failures, executable hashes, regression logs, and final results.

The first Clippy run exhausted the temporary disk quota before broader tests
ran. Removing the disposable incremental build cache and disabling incremental
compilation allowed validation to finish. This was an environment failure.

## Deferred or Reopen Items

The focused concurrency test configures WAL before releasing connections, so it
isolates schema replacement. The MCP run and held-reader test cover WAL setup
as well, but sustained contention can still exhaust its deadline. Five stress
rounds are not a reliability estimate. The server's separate in-memory
initialization lifecycle and index refresh policy are unchanged.

## Skill Feedback

### review-guide (beta)

- **Confirmation** — the observed schema error and a failing regression support
  a specific concurrency claim; they do not justify promising error-free startup
  under all contention.

## Appendix: Skills Invoked

- `engineering-journal` — reproduced failure, mechanism, and validation record.
- `technical-prose` — precise failure and transaction explanation.
- `review-guide` (beta) — PR description and validation limits.
