The problem is repeated waiting during account reviews when analysts revisit the same historical usage chart; faster repeat views must preserve account isolation. I’ll write up the draft now with the gaps explicit, without further questions or a repository design document.

I used plan-feature with frame-problem, propose-design, vault-search, and discuss. This remains draft mode: no vault/project files were changed, no commits were made, and no issues were opened. The supplied local vault snapshot has no matching briefs; live search and graph traversal were not run. The proposed new problem brief supplies the frame for the design. No scope or arc identifiers were supplied, so optional metadata is omitted.

Discovery areas 1–5 are captured below with unresolved facts marked. Area 6 is skipped because Reader is a single repository. Imani owns Reader; no cross-repository dependency is established.

Proposed vault addition: `briefs/2026-10-20-reader-repeat-views-problem.md`.

```diff
--- /dev/null
+++ briefs/2026-10-20-reader-repeat-views-problem.md
@@ -0,0 +1,33 @@
+---
+id: 2026-10-20-reader-repeat-views-problem
+type: problem-brief
+status: draft
+author: Imani
+created: 2026-10-20
+---
+
+# Repeated waits during account reviews
+
+## What is the actual problem?
+
+During account reviews, analysts wait for the same historical usage chart to reload whenever they switch tabs and return. The repeated wait interrupts review work. They want faster repeat views without ever seeing another account's data. Request volume, the number of affected analysts, current latency, and the source of the delay are unknown.
+
+## Why does it matter now?
+
+The reported account-review workflow establishes a concrete user problem. No new deadline, change in frequency, or other urgency trigger has been supplied. The cost of delay and benefit of faster repeat views have not been measured.
+
+## What constraints are we operating under?
+
+Account isolation is a hard requirement. Imani owns Reader, which is a single repository. Acceptable staleness, delivery budget, deployment constraints, and cache-backend requirements are unknown. No cache backend has been selected. Existing authorization behavior must be understood before reuse of data is designed.
+
+## What have we tried or considered?
+
+The reported current behavior is to wait for chart reloads after switching tabs. No attempted mitigation or measurement has been supplied. Caching has been suggested, but there is no evidence yet identifying the slow stage or demonstrating that reusable server responses are the cause of the wait.
+
+## What are we explicitly not trying to solve?
+
+This draft addresses repeat views of historical usage charts during account reviews. A repository design document is explicitly excluded. Broader Reader optimization and other workflows are outside the proposed thin slice unless later evidence requires them. Account isolation cannot be traded for speed.
+
+## What would change our analysis?
+
+Measurements showing that most delay comes from client rendering or another non-reusable operation would change the proposed intervention. Low repeat-view frequency, strict freshness needs, or an insufficient delivery budget could make caching unattractive. Any possibility of exposing another account's data would invalidate the approach. Request volume, freshness expectations, budget, current latency, and a measurable improvement target remain open.
```

Proposed vault addition: `briefs/2026-10-20-reader-repeat-views.md`.

```diff
--- /dev/null
+++ briefs/2026-10-20-reader-repeat-views.md
@@ -0,0 +1,44 @@
+---
+id: 2026-10-20-reader-repeat-views
+type: design-brief
+status: draft
+author: Imani
+created: 2026-10-20
+frames: 2026-10-20-reader-repeat-views-problem
+---
+
+# Faster repeat chart views in Reader
+
+Investigate a narrow cache for repeated historical-usage chart requests, conditional on measurements showing reusable data retrieval contributes materially to latency. Preserve authorization on every request and partition reusable results by account and any other dimensions that affect access or output. Cache backend, lifetime, invalidation, and implementation scope remain undecided. This proposal frames [the problem brief](2026-10-20-reader-repeat-views-problem.md).
+
+## Why this approach?
+
+A narrow repeat-view slice gives Imani a concrete way to evaluate improvement without assuming a broad cache is justified. The account-isolation constraint rules out sharing results solely by chart or date range. Unknown freshness requirements prevent choosing a TTL, and the unknown delivery budget prevents committing to infrastructure work or a schedule.
+
+## What does it look like?
+
+Proposed request flow: establish the authenticated user's access to the requested account; derive a cache identity from the authorized account, normalized chart parameters, and all permission/output variations; then consult the cache. A hit is usable only while it meets the eventual freshness and authorization rules. On a miss, use the current authorized retrieval path and consider storing the result under that same identity. A cache key alone is not an authorization check.
+
+Before choosing this flow, inspect Reader's actual query and authorization boundaries and determine whether data is account-wide or varies by user permissions. Define handling of permission changes, historical-data corrections, invalidation, expiry, and cache failure. Proposed fallback is the existing authorized retrieval path when caching is unavailable, subject to capacity checks. Do not store data until retention and deployment constraints are understood.
+
+## What does it cost?
+
+Implementation effort cannot be estimated from the supplied facts. Imani owns clarifying scope and feasibility. First measure repeated-view latency and where time is spent, estimate request/reuse volume and result sizes, and establish acceptable staleness and available capacity. These are planned investigations, not completed work. Main risks are cross-account or cross-permission reuse, stale results, increased storage, and extra load during cache misses or failure.
+
+## What alternatives did we consider?
+
+An in-process cache could be simple for a small workload but duplicates state across instances and loses entries on restart. It is a candidate only after deployment topology, size, and consistency requirements are known. A shared cache offers reuse across instances but adds operational and failure-mode costs; no backend is selected, and budget or infrastructure restrictions could rule it out. Either choice is removable behind a bypass mechanism, but leaked data cannot be undone by rollback.
+
+Avoiding reloads in the client or optimizing the existing retrieval path may be better if measurement shows that the slow stage is elsewhere. This could change the repository scope, which would require revisiting the single-repository assumption. No client change is currently authorized or planned as a concrete deliverable.
+
+Doing nothing adds no cache risk or operational cost but preserves the observed waits. It remains reasonable if reuse is rare or freshness and budget constraints outweigh measured benefit. The deciding evidence is delay attribution, reuse frequency, freshness, deployment constraints, and available effort.
+
+## Validation and rollout proposal
+
+Record baseline and repeat-view latency for the account-review workflow, then agree a measurable improvement target with Imani. Exercise identical chart parameters for different accounts and different access levels, denied requests, permission changes, expiry, data corrections, misses, and cache failure. Verify response correctness and isolation alongside any performance gain. No measurements or tests have run.
+
+If caching is justified, propose an initially disabled gate and a limited rollout after validation. Bypassing the cache should restore the current authorized retrieval path. Confirm that path can sustain fallback load, and define cache cleanup and data-retention behavior before rollout. These mechanisms are proposals, not claims about current Reader capabilities.
+
+## What would change the analysis?
+
+Abandon or revise the cache proposal if measured latency is not materially attributable to reusable data retrieval, if a freshness policy cannot meet analyst needs, if isolation cannot be demonstrated, or if cost exceeds the available budget. Open items are request volume, repeat frequency, baseline and target latency, acceptable staleness, delivery budget, backend/deployment constraints, permission dimensions, invalidation, and retention. No final implementation decision or accepted status is claimed.
```

The repository design document is explicitly skipped as requested.

Tracking issue draft for `Reader`:

**Title:** `reader-repeat-views: scope and validate faster historical chart revisits`

```markdown
Part of Faster repeat chart views in Reader.
Vault: 2026-10-20-reader-repeat-views-problem · 2026-10-20-reader-repeat-views
Both are proposed drafts, not saved vault artifacts.

Scope for this repo:
- Measure repeat-view latency and identify the slow stage in the account-review chart workflow.
- Establish request/reuse volume, result sizes, acceptable staleness, measurable latency target, delivery budget, and deployment constraints.
- Inspect account and permission boundaries; identify every dimension required to prevent cross-account or cross-permission reuse.
- Compare in-process caching, shared caching, and optimizing the current path. Select a backend only if the evidence supports caching.
- Define expiry/invalidation, permission-change behavior, retention, failure fallback, and rollback before implementation.
- If justified and feasible, implement a narrow gated cache using the existing authorized retrieval path.
- Validate account isolation and correctness across hits, misses, denied access, permission changes, expiry, corrections, and cache failure; compare repeat-view latency with the baseline.

Acceptance for planning: unknowns are resolved or explicitly block implementation, and Imani records an evidence-based approach and feasible scope.
Acceptance for any implementation: agreed performance/freshness targets pass, isolation is demonstrated, and bypass/fallback behavior is verified.
No backend, TTL, budget, schedule, or numeric performance improvement is currently committed.
Depends on: no known cross-repository issues; measurement and constraint decisions precede implementation.
Owner: Imani
```

Proposed vault addition: `discussions/2026-10-20-reader-repeat-views-session.md`.

```diff
--- /dev/null
+++ discussions/2026-10-20-reader-repeat-views-session.md
@@ -0,0 +1,19 @@
+---
+id: 2026-10-20-reader-repeat-views-session
+type: session-note
+author: Imani
+created: 2026-10-20
+relates_to: [2026-10-20-reader-repeat-views-problem, 2026-10-20-reader-repeat-views]
+---
+
+# Reader repeat-view planning session
+
+Imani, Reader's owner, supplied the account-review workflow for a solo planning pass with an agent. Analysts wait for the same historical usage chart when returning to a tab. The desired outcome is faster repeat views without exposure of another account's data.
+
+Imani requested draft mode, explicit gaps, no further questions, and no repository design document. The plan therefore proposes investigation and a conditional narrow caching approach. No cache backend, lifetime, delivery commitment, or final implementation decision was selected.
+
+Paired drafts 2026-10-20-reader-repeat-views-problem and 2026-10-20-reader-repeat-views and a Reader tracking issue were prepared. Nothing landed in the vault or project, no issue was opened, and this note is uncommitted. The supplied snapshot has no matching briefs; live search and graph traversal were not run. No existing artifact is known to need supersession.
+
+Open items include request volume, repeat frequency, baseline and target latency, acceptable staleness, delivery budget, deployment/backend constraints, permission dimensions, invalidation, and retention. Imani owns resolving them before an implementation commitment. No probes or tests have run.
+
+Artifact triage: the paired drafts cover the stated problem and conditional design. Delay attribution could merit a bounded inquiry if Imani chooses to track it separately. A decision record can follow evidence and selection of an approach. No existing brief is known to be invalidated, no arc closure is indicated, and no additional artifact or status transition is proposed automatically.
```

The supplied snapshot also supports the session-note triage: no relevant existing brief is available to amend. Possible future followups are a bounded inquiry into latency attribution and, after an approach is selected, a decision record. These are options for Imani, not additional work executed here.

Completion status:

- Problem brief: drafted, status `draft`; not landed.
- Design brief: drafted, status `draft`, correctly frames the problem draft; not landed.
- Repository design document: explicitly skipped.
- Reader tracking issue: drafted; not opened.
- Solo session note: drafted; not committed.
- Open gaps: captured above; implementation feasibility and success criteria remain unresolved.
