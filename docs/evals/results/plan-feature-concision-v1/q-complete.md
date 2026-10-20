Draft mode: below are the proposed paired briefs, tracking issue, and solo session note. Nothing has been written to the vault or project, committed, or opened as an issue. I used plan-feature with frame-problem, propose-design, vault-search, and discuss to prepare these drafts.

The supplied context covers discovery areas 1–5: twenty support engineers need reliable spreadsheet transfer; the thin slice is CSV to stdout; the limit is two engineer-days with no new runtime dependencies; only `lantern-cli` changes; and the proposed approach reuses the existing writer and streams query pages. Existing output and flags, filtering, authorization, and retention must remain compatible. Area 6, cross-repository coordination, is skipped because this is a single-repository change. Mina is accountable.

MATCHES (0)

The supplied local vault snapshot reports no relevant briefs or open arc for Lantern incident CSV export. Live vault search and graph traversal were not run. There is no existing frame or competing design to reuse in the supplied evidence; the proposed design below frames the new problem draft. Optional scope identifiers are omitted because none were supplied.

GAPS

- Supported spreadsheet application/version and import settings are unspecified; value preservation remains unverified.
- Exact incident subcommand, exported columns/order, header behavior, and null/empty representation need confirmation against the CLI.
- Pagination/write failure behavior and the benchmark environment need definition. The two-engineer-day budget is a constraint, not a validated estimate.
- Neither spreadsheet round-trip validation nor the 10,000-row timing probe has run.

Proposed addition to the vault: `briefs/2026-10-20-lantern-csv-export-problem.md`.

```diff
--- /dev/null
+++ briefs/2026-10-20-lantern-csv-export-problem.md
@@ -0,0 +1,33 @@
+---
+id: 2026-10-20-lantern-csv-export-problem
+type: problem-brief
+status: draft
+author: Mina
+created: 2026-10-20
+---
+
+# Reliable transfer of filtered incidents to spreadsheets
+
+## What is the actual problem?
+
+Twenty support engineers copy filtered incident rows from terminal output into spreadsheets about four times per day each. The workaround takes five minutes per transfer and loses commas or embedded newlines in customer notes. They need to transfer the rows they are authorized to see, with the selected filters and original values preserved. Success means one-command export of filtered rows in under 30 seconds for 10,000 rows, with values preserved through spreadsheet import.
+
+## Why does it matter now?
+
+A quarterly audit next month makes reliable transfer useful now. The reported workload amounts to about 80 transfers and 400 minutes of manual work per day across the team; these are estimates from the supplied usage figures. Leaving the workaround unchanged preserves both the manual burden and the known data errors during audit preparation.
+
+## What constraints are we operating under?
+
+Two engineer-days are available before the audit. There can be no new runtime dependencies. Existing table output and flags must remain compatible, and filtering and authorization must be preserved. The CLI supports UTF-8, already has a CSV writer dependency, and paginates incident queries. There is no data retention change. Mina is accountable, and lantern-cli is the only affected repository.
+
+## What have we tried or considered?
+
+Manual copying is the current workaround and does not reliably preserve values. Doing nothing retains those errors. Bulk buffering risks memory growth, while handwritten quoting risks incorrect delimiters. Reusing existing capabilities and checking spreadsheet round-trip behavior and 10,000-row timing have been considered, but those checks have not run.
+
+## What are we explicitly not trying to solve?
+
+This effort does not cover scheduling, a GUI, saved exports, remote storage, or changes to retention. It does not broaden access to incident data or redesign existing table output.
+
+## What would change our analysis?
+
+Revisit the proposal if the supported spreadsheet cannot round-trip values or streaming cannot meet the time target. The spreadsheet and import settings, exported field contract, and timing environment remain unspecified. Validation may show that the promised result cannot be delivered within two engineer-days; in that case Mina must reconsider scope or timing rather than treating the target as proven.
```

Proposed addition to the vault: `briefs/2026-10-20-lantern-csv-export.md`.

```diff
--- /dev/null
+++ briefs/2026-10-20-lantern-csv-export.md
@@ -0,0 +1,48 @@
+---
+id: 2026-10-20-lantern-csv-export
+type: design-brief
+status: draft
+author: Mina
+created: 2026-10-20
+frames: 2026-10-20-lantern-csv-export-problem
+---
+
+# Lantern incident CSV export
+
+Add opt-in `--format csv` output to the existing incident-query command. Reuse the installed CSV writer and consume the existing paginated query incrementally, writing UTF-8 CSV to stdout while preserving filters and authorization. Keep existing output as the default. This proposal frames [the problem brief](2026-10-20-lantern-csv-export-problem.md).
+
+## Why this approach?
+
+Reusing the writer respects the no-new-runtime-dependencies constraint and avoids introducing custom delimiter and quoting logic. Streaming pages bounds working memory relative to a page instead of the total result count. An opt-in format flag protects existing table output and CLI compatibility. Keeping the existing query path protects filtering and authorization; CSV serialization must not introduce a separate access path.
+
+## What does it look like?
+
+An engineer uses the existing incident-query command and filters, adds `--format csv`, and redirects stdout to a local file or pipes it to another tool. The exact command name is not supplied. The writer must encode commas, quotes, embedded newlines, and Unicode correctly. Diagnostics should use stderr so stdout remains valid CSV.
+
+Proposed output contract: a stable ordered header emitted once, including for empty results, with rows emitted as each page arrives. The exact columns, field mapping, and distinction between null and empty values must be settled before implementation. This is a proposed contract, not a description of existing behavior.
+
+Proposed failure behavior: return a nonzero exit status on query or write failure and report the failure on stderr. A failure after streaming begins can leave a partial output file; this limitation must be documented, and consumers must check the exit status. Atomic export is outside this stdout-only slice.
+
+## What does it cost?
+
+The available budget is two engineer-days, owned by Mina. A provisional sequence is to confirm the field contract and spreadsheet target, wire format selection to the existing writer and paginated path, and then validate correctness, compatibility, and timing. Repository inspection has not occurred, so fit within the budget is unverified. Risks include spreadsheet coercion of values, pagination errors, partial output on late failures, and unexpected writer or query behavior. No cross-repository work, migration, backfill, or retention change is proposed.
+
+## What alternatives did we consider?
+
+Buffering all rows before serialization simplifies batch handling but risks memory growth with result size. Streaming pages is the chosen direction; if it fails the target, revisit the design before rollout. The implementation can be replaced behind the opt-in flag, but incorrect exports could already have affected spreadsheet work.
+
+Handwritten CSV quoting avoids adapting the existing writer but risks delimiter and newline corruption without providing a dependency advantage. Reuse the installed writer; verify its behavior with representative data. This choice is locally reversible, but faulty serialization would undermine the value-preservation goal.
+
+Doing nothing requires no implementation effort and preserves compatibility, but leaves the reported manual errors and workload before the audit. It is the fallback if the thin slice cannot meet its checks in the available time.
+
+## Validation and rollout
+
+Verify quoted commas, embedded newlines, Unicode, embedded quotes, empty results, and multiple pages, including no missing or duplicated rows and a single header. Check that the same filters and authorization apply and that existing table output and flags behave as before. Exercise query/write failures to confirm the proposed exit status and separation of diagnostics from CSV.
+
+Round-trip representative exported values through the supported spreadsheet and compare them with the source values. Run a 10,000-row timing probe under an agreed representative environment; success is under 30 seconds. Observe memory as additional pages are consumed to check the streaming assumption. These are planned checks; none has run.
+
+Roll out only through `--format csv` after validation. Rollback removes that option while preserving default output. Mina owns implementation and validation follow-through. No cross-repository sequencing is required.
+
+## What would change the analysis?
+
+Revisit if the supported spreadsheet cannot round-trip values or the streaming path cannot meet the time target. Also revisit scope or scheduling if repository inspection shows the work exceeds two engineer-days. Open decisions are the spreadsheet/version and import settings, exact column and value contract, confirmation of failure behavior, and a representative timing environment. No status promotion is proposed before those checks.
```

The optional repository design document is explicitly skipped as requested; `lantern-cli` does not conventionally keep design documents. The paired vault briefs are the proposed source of truth.

Tracking issue draft for `lantern-cli`:

**Title:** `lantern-csv-export: add streamed incident CSV output`

```markdown
Part of Lantern incident CSV export.
Vault: 2026-10-20-lantern-csv-export-problem · 2026-10-20-lantern-csv-export
Both briefs are proposed drafts and have not been saved to the vault.

Scope for this repo:
- Confirm the existing incident command, CSV field order/header, null/empty mapping, target spreadsheet/import settings, and failure behavior with Mina.
- Add opt-in --format csv using the existing CSV writer and UTF-8 output to stdout.
- Stream the existing authorized, filtered query pages without buffering the full result set.
- Preserve default table output and existing flags; send diagnostics to stderr.
- Verify commas, quotes, embedded newlines, Unicode, empty results, pagination, filtering, authorization, and query/write failures.
- Round-trip representative values through the supported spreadsheet and compare with the source.
- Run a representative 10,000-row timing probe with a target under 30 seconds; check memory behavior as pages accumulate.
- Document usage and partial-output behavior in existing CLI help/docs. Do not add a repo design document.

Acceptance:
- One command exports all filtered, authorized rows with values preserved.
- The agreed spreadsheet round-trip check passes.
- The 10,000-row probe completes in under 30 seconds.
- Existing output and flags remain compatible; no new runtime dependency is added.

Budget: two engineer-days before next month's quarterly audit; feasibility is unverified.
Out of scope: scheduler, GUI, saved exports, remote storage, retention changes.
Rollout: opt-in --format csv after checks pass.
Rollback: remove the CSV option without changing default output.
Revisit: spreadsheet round-trip fails, streaming misses the target, or the work exceeds capacity.
Depends on: no other repository issues; agree the output contract and validation target before implementation.
Owner: Mina
```

Proposed addition to the vault: `discussions/2026-10-20-lantern-csv-export-session.md`.

```diff
--- /dev/null
+++ discussions/2026-10-20-lantern-csv-export-session.md
@@ -0,0 +1,19 @@
+---
+id: 2026-10-20-lantern-csv-export-session
+type: session-note
+author: Mina
+created: 2026-10-20
+relates_to: [2026-10-20-lantern-csv-export-problem, 2026-10-20-lantern-csv-export]
+---
+
+# Lantern CSV export planning session
+
+Mina's supplied context was used with an agent to draft a single-repository plan for reliable incident transfer before next month's audit. This is a solo session note. Mina is accountable for the plan and its follow-through.
+
+The proposed direction is opt-in CSV to stdout using the installed writer and streaming the existing filtered, authorized query pages. The available budget is two engineer-days. Existing output and flags remain compatible, with no new runtime dependency or retention change. Scheduling, GUI, saved exports, and remote storage remain out of scope.
+
+The problem brief 2026-10-20-lantern-csv-export-problem and paired design brief 2026-10-20-lantern-csv-export were drafted, together with a lantern-cli tracking issue. No briefs landed, no issue was opened, and this note has not been committed. The optional repository design document was explicitly skipped. The supplied vault snapshot contains no relevant briefs or open arc; live search and graph traversal were not run.
+
+Open questions concern spreadsheet/version and import settings, exact columns and value mapping, failure behavior, and a representative timing environment. The spreadsheet round-trip and 10,000-row timing checks have not run. Mina owns resolving these points and validating feasibility before rollout.
+
+Artifact triage: the supplied snapshot indicates no existing framing or design to amend, so the paired drafts above cover the problem and proposed solution. This plan does not establish an accepted decision or invalidate an existing brief. No arc closure is indicated. If validation fails, Mina can choose a bounded inquiry or revise the draft design; no additional artifact or status transition is created automatically.
```

Artifact implications from the note are covered by the paired drafts, using the same supplied snapshot as search evidence. A later decision record could capture the chosen approach after validation; a bounded inquiry could investigate a failed round-trip or timing result. Neither is proposed as an immediate extra write.

Completion status:

- Problem brief: drafted as `2026-10-20-lantern-csv-export-problem`, status `draft`; not landed.
- Design brief: drafted as `2026-10-20-lantern-csv-export`, status `draft`, with the required `frames` link; not landed.
- Repository design document: explicitly skipped at your request and consistent with repository practice.
- Tracking issue: drafted for `lantern-cli`; not opened.
- Solo session note: drafted as `2026-10-20-lantern-csv-export-session`; not committed.
- Remaining gaps and validation work: handed back above. All implementation and external writes remain unperformed, as requested.
