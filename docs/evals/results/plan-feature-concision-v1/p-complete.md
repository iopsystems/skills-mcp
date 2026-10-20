Draft mode. Below are proposed vault additions and the `lantern-cli` issue body; nothing has been written to the vault or project, and no issue has been opened. The optional repo design doc is skipped as requested and because this repo does not keep design docs.

I used `plan-feature` with `frame-problem`, `propose-design`, `discuss` in solo session-note mode, and their `vault-search` workflow, limited to the supplied snapshot.

MATCHES (0)

The supplied local vault snapshot reports no relevant briefs and no open arc. Live search and graph traversal were not run. These are new draft IDs, with no claim of live-vault uniqueness. Optional scope IDs are omitted because none were supplied.

GAPS

- The supported spreadsheet and its import settings are unknown; round-trip preservation must be checked there.
- The exact incident command, CSV columns, header names, order, and null-versus-empty representation need confirmation against the existing CLI.
- Partial-output behavior on a later page failure needs an explicit contract.
- The spreadsheet round-trip check and 10,000-row timing probe have not run. The two engineer-days are a budget, not a validated estimate.

Proposed addition: `briefs/2026-10-20-lantern-csv-export-problem.md`

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
+# Reliable incident values in spreadsheets
+
+## What is the actual problem?
+
+Twenty support engineers copy filtered incident rows from terminal output into spreadsheets about four times per engineer per day. Each transfer takes five minutes, and the workaround drops commas or newlines from customer notes. Engineers cannot reliably carry the selected values into their spreadsheet workflow. At the stated frequency, this consumes roughly 400 engineer-minutes per day. Success means transferring the filtered rows in one command, preserving values, and completing an export of 10,000 rows in under 30 seconds.
+
+## Why does it matter now?
+
+A quarterly audit next month increases the value of reliable transfers. Continuing the workaround until then preserves both repeated manual work and the risk of incorrect customer-note values.
+
+## What constraints are we operating under?
+
+Mina is the author and accountable owner. Two engineer-days are available before the audit. No new runtime dependencies may be added; existing table output and flags must remain compatible. Filtering and authorization must be preserved, and data retention must not change. The CLI supports UTF-8, paginates incident queries, and already has a CSV writer dependency. The performance target is under 30 seconds for 10,000 rows. The supported spreadsheet and exact export schema have not been specified. No additional staffing or on-call assumptions are established.
+
+## What have we tried or considered?
+
+The current manual copy process takes five minutes and loses punctuation or embedded line breaks. Doing nothing preserves these failures. Reusing the existing CSV writer and consuming query pages have been considered; handwritten quoting risks delimiter errors, and buffering every row risks memory growth. Neither the spreadsheet round-trip check nor the timing probe has run, so these considerations are not validation results.
+
+## What are we explicitly not trying to solve?
+
+The useful slice is CSV output to stdout. Scheduling, a GUI, saved exports, and remote storage are outside scope. This work does not change retention or the existing default table output. It is confined to the lantern-cli repository and requires no cross-repository coordination.
+
+## What would change our analysis?
+
+Revisit the scope or approach if the supported spreadsheet cannot round-trip the values or if streaming cannot meet the time target. Clarifying the spreadsheet, column contract, and handling of interrupted exports may expose additional constraints. Validation that exceeds the two-day budget would require an explicit scope or schedule decision before the audit.
```

Proposed addition: `briefs/2026-10-20-lantern-csv-export.md`

```diff
--- /dev/null
+++ briefs/2026-10-20-lantern-csv-export.md
@@ -0,0 +1,50 @@
+---
+id: 2026-10-20-lantern-csv-export
+type: design-brief
+status: draft
+author: Mina
+created: 2026-10-20
+frames: 2026-10-20-lantern-csv-export-problem
+---
+
+# Lantern CSV export
+
+Add opt-in `--format csv` to the existing incident-query workflow, route rows through the existing CSV writer, and write successive query pages to stdout. Preserve the existing filtering and authorization path and leave table output as the default. This is a proposed design paired with [the problem brief](2026-10-20-lantern-csv-export-problem.md); implementation and validation have not begun.
+
+## Why this approach?
+
+Reusing the installed writer respects the no-new-runtime-dependencies constraint and avoids custom delimiter handling. Streaming pages addresses the memory-growth risk while targeting the 10,000-row latency requirement. Opt-in formatting respects existing table output and flags, keeps the change bounded by the two-engineer-day budget, and gives Mina a narrow rollback mechanism.
+
+## What does it look like?
+
+An engineer adds `--format csv` to their existing filtered incident command and redirects stdout into a spreadsheet input file. The exact command name is not supplied. Filters and authorization apply before serialization using the existing query path. Fetch and serialize pages incrementally; do not accumulate the complete result set. Emit UTF-8 values through the existing writer without manual quoting or silent value rewriting.
+
+Proposed output contract: emit one header row in a stable column order, including for an empty result, and reserve stdout for CSV. Confirm field selection, header names, null-versus-empty handling, and compatibility with any existing format flag before implementation. These details are proposals, not discovered repository facts.
+
+Proposed failure contract: send diagnostics to stderr and return a nonzero status if fetching or writing fails. Streaming may already have emitted rows when a later page fails; document that consumers must discard output from a failed command. Confirm existing CLI error conventions before finalizing this behavior. Fully atomic output would require additional storage or buffering and is outside the chosen thin slice.
+
+Only lantern-cli is affected: argument parsing and help, incident output formatting, pagination integration, tests, and user-facing export/error documentation. No new service, infrastructure, migration, backfill, SDK change, or retention change is proposed. CSV adds an opt-in output contract for spreadsheet users and scripts. Cross-repository coordination is skipped because there is one repository.
+
+## What does it cost?
+
+Mina owns delivery within the available two engineer-days. Proposed allocation is one day for checking the output contract and implementing the writer/pagination integration, then one day for compatibility, data-preservation and performance checks plus help text. This allocation must be reassessed after checking the code and spreadsheet. No runtime dependency addition is needed. Risks are spreadsheet coercion, page-boundary omissions or duplication, partial output on failure, and elapsed time dominated by query retrieval. Existing behavior should remain compatible, but that requires verification rather than assumption.
+
+## What alternatives did we consider?
+
+Reuse the installed writer instead of handwritten CSV quoting: the writer avoids a new implementation of delimiter and newline escaping and satisfies the dependency constraint. A replacement library has no demonstrated need and would violate the runtime-dependency limit if added. This choice can be revised internally without changing the output contract if validation finds a writer limitation.
+
+Stream pages instead of buffering all rows: streaming bounds retained row data to the page and writer buffers, while full buffering grows with result size. The consequence is potentially partial stdout on failure. The deciding factors are bounded memory and the narrow delivery budget; changing to atomic output would reopen the storage and error contract.
+
+Ship opt-in CSV instead of doing nothing or replacing default table output: doing nothing preserves manual errors and delay, while a default-format replacement violates compatibility. The new option is reversible by removing it without changing the default; consumers that adopt it would lose that option during rollback.
+
+## Validation and rollout
+
+First confirm the CSV columns, target spreadsheet, and error conventions. Then integrate the writer with the existing filtered and authorized paginated query. Verify quoted commas, embedded newlines, Unicode, empty results, and multiple pages, including exact row coverage and a single header. Check filtering and authorization, default table output and existing flags, and failure after an earlier successful page.
+
+Perform a spreadsheet round-trip using the supported application and compare values, explicitly checking text coercion and newline preservation. Run an end-to-end 10,000-row timing probe with representative query conditions and record elapsed time; the acceptance target is strictly under 30 seconds. Neither check has run. Unit-level serialization correctness alone does not establish spreadsheet fidelity or end-to-end speed.
+
+Roll out through `--format csv` only after validation. Success is one-command export of the filtered rows with preserved values and the stated performance target. Rollback removes the option while retaining default table behavior. Mina owns resolving validation failures and deciding whether the proposal fits the available budget before the audit.
+
+## What would change the analysis?
+
+Revisit this design if the supported spreadsheet cannot round-trip values or if streaming cannot meet the timing target. Unknowns remain the spreadsheet/import settings, final CSV schema and null semantics, existing format/error conventions, and measured performance. If those require extra dependencies, persistent storage, or work beyond the two-day budget, return to scope and constraints before expanding the implementation.
```

Draft tracking issue for `lantern-cli` — not opened:

```markdown
Title: lantern-csv-export: add streaming CSV output

Part of Lantern CSV export.
Vault: 2026-10-20-lantern-csv-export-problem · 2026-10-20-lantern-csv-export
Both vault briefs are proposed drafts, not committed artifacts.

Scope for this repo:
- Confirm the CSV column/header/null contract, supported spreadsheet, and existing format/error conventions.
- Add opt-in --format csv with the existing writer and stream incident-query pages to stdout.
- Preserve filtering, authorization, UTF-8 values, existing flags, and default table output.
- Document usage and partial-output failure behavior in CLI help or existing documentation; no separate design doc.
- Verify commas, embedded newlines, Unicode, empty results, and pagination with no missing or duplicated rows.
- Verify filtering/authorization, existing behavior, and later-page failure handling.
- Run and record a spreadsheet round-trip and an end-to-end 10,000-row timing probe. Acceptance: preserved values, one-command export, and elapsed time under 30 seconds.
- Roll out opt-in; rollback removes the option without changing the default.

Depends on: No other repository issues. Spreadsheet and schema clarification precede final implementation; validation precedes rollout.
Owner: Mina
Budget: Two engineer-days before next month's audit; feasibility not yet measured.
Out of scope: Scheduler, GUI, saved exports, remote storage, new runtime dependencies, and retention changes.
```

Proposed addition: `discussions/2026-10-20-lantern-csv-export-session.md`

```diff
--- /dev/null
+++ discussions/2026-10-20-lantern-csv-export-session.md
@@ -0,0 +1,21 @@
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
+This solo session note captures Mina's planning work with an assistant ahead of next month's quarterly audit. Twenty support engineers repeatedly transfer incident rows into spreadsheets, losing time and sometimes commas or embedded newlines. The session prepared drafts only; it did not execute implementation, validation, or publication.
+
+The proposed slice adds CSV to stdout through --format csv, reuses the existing writer, and streams query pages while preserving filtering, authorization, and default behavior. Mina is accountable for the two-engineer-day delivery budget. Cross-repository coordination is unnecessary. A repo design doc was explicitly skipped and is not customary in this repository.
+
+The paired drafts are briefs/2026-10-20-lantern-csv-export-problem.md and briefs/2026-10-20-lantern-csv-export.md. A tracking issue titled “lantern-csv-export: add streaming CSV output” was drafted for lantern-cli but not opened; no issue URL exists. This note is also a proposed addition, not a vault write.
+
+The supplied snapshot reported no relevant briefs or open arc; live vault search and graph checks were not run. The design frames the newly drafted problem brief. No status transitions or accepted decisions were recorded.
+
+Open questions concern the supported spreadsheet/import settings, CSV columns and null semantics, existing CLI formatting/error conventions, and partial output on failure. Neither the spreadsheet round-trip nor the 10,000-row probe has run. Mina owns resolving these gaps and evaluating whether the implementation fits the budget before rollout.
+
+Artifact implications: the new problem and proposed design are covered by the paired drafts, so no duplicate briefs are needed based on the supplied evidence. A decision record could follow once the design is accepted; this session does not claim acceptance. A bounded inquiry could be useful if spreadsheet fidelity or performance remains unresolved after the planned checks. Neither optional artifact is created here. No existing artifact supersession or arc closure is indicated by the snapshot.
```

All requested draft artifacts are supplied: problem `2026-10-20-lantern-csv-export-problem` (draft), design `2026-10-20-lantern-csv-export` (draft, framing that problem), the `lantern-cli` issue body, and solo note `2026-10-20-lantern-csv-export-session` (drafted, with no status field). There are no commits or opened issues. Repository inspection, live-vault checks, validation, and any later publication remain unperformed. Optional follow-ups are a decision record after acceptance or an inquiry if the proposed checks leave a material unknown; no selection is needed to use these drafts.
