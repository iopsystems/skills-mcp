Draft mode. I’ve prepared the paired briefs, Reader tracking issue, and solo session note below using `plan-feature` and its brief and note workflows. The repo design doc is skipped as requested. Nothing has been written to Reader or the vault, and no issue has been opened.

The supplied snapshot reports no matching briefs. Live search and graph traversal were not run. These are new draft IDs, with no verified live-vault uniqueness. Imani is the author and Reader owner; other implementation or approval roles remain unspecified.

Proposed addition: `briefs/2026-10-20-reader-repeat-views-problem.md`

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
+# Repeated historical usage chart waits
+
+## What is the actual problem?
+
+During account reviews, analysts wait for the same historical usage chart to reload whenever they switch tabs. Repeated viewing interrupts their review workflow. The desired outcome is faster repeat views while ensuring that one account's data is never shown in another account's context. The number of affected analysts, request frequency, current latency, and severity have not been measured. The current behavior requires waiting for each reload; no additional workaround was supplied.
+
+## Why does it matter now?
+
+The repeated wait has prompted this planning request. No deadline, recent traffic change, or quantified cost of delay has been supplied, so urgency beyond the reported workflow friction is unknown.
+
+## What constraints are we operating under?
+
+Account isolation is a hard requirement. Imani owns Reader, which is a single repository. Request volume, an acceptable staleness bound, and the delivery budget are unknown. No cache backend has been chosen. Existing authorization behavior, data-update patterns, deployment topology, dependency policies, and retention requirements need investigation before implementation decisions. There is no evidence that a particular storage technology is mandatory.
+
+## What have we tried or considered?
+
+A caching layer has been proposed, but no implementation, benchmark, or cache experiment has been reported. Continuing to reload preserves the known wait. It is not yet established whether repeated chart computation, data access, or client reload behavior causes most of the delay. Measuring the repeated-view path can distinguish those possibilities before committing to a backend.
+
+## What are we explicitly not trying to solve?
+
+The draft focuses on repeat views of historical usage charts during account reviews. As a proposed boundary, it does not expand into general caching of Reader endpoints, a shared cross-service cache platform, or changes to the chart's meaning. A repo design doc is excluded by request. Implementation and rollout are outside this draft-only session.
+
+## What would change our analysis?
+
+Measurements could show that repeat requests are rare, cache reuse is negligible, or the delay is primarily outside the data-serving path. A strict freshness requirement could make reuse inappropriate without reliable invalidation. Any inability to enforce account and authorization isolation would rule out the affected approach. Delivery constraints, once known, may change the smallest useful scope.
```

Proposed addition: `briefs/2026-10-20-reader-repeat-views.md`

```diff
--- /dev/null
+++ briefs/2026-10-20-reader-repeat-views.md
@@ -0,0 +1,50 @@
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
+Propose a bounded reuse path for historical usage chart results, subject to measurement and a freshness contract. Before choosing a backend or shipping result reuse, establish where repeated-view time is spent and how account authorization is enforced. This draft frames [the repeat-view problem](2026-10-20-reader-repeat-views-problem.md); caching is a candidate approach rather than a validated implementation decision.
+
+## Why this approach?
+
+The problem brief identifies repeat viewing as the useful slice and account isolation as a hard constraint. A narrowly scoped reuse path could reduce repeated work without changing unrelated endpoints. Unknown volume, freshness tolerance, and budget prevent a defensible backend choice, TTL, or delivery estimate. The next work therefore resolves those uncertainties before creating an operational dependency.
+
+## What does it look like?
+
+Proposed request flow: authenticate and authorize the requested account and chart operation; derive a reuse identity from trusted account context and all inputs that affect the chart result; look up an eligible entry; otherwise compute through the existing path. Store or serve reused data only once the permitted freshness and data-handling rules are established. Authorization must still apply on every hit. Do not use a client-supplied account identifier alone as proof of access.
+
+An illustrative identity includes account, chart kind, time range, filters, and result/schema version. If results differ by user or permissions within an account, include that authorization scope or avoid sharing those entries. Actual input fields and permission semantics have not been inspected, so this is a review checklist rather than a final key schema.
+
+Account switching must not reuse data from the previous account. Trace both server and client behavior, including late responses from an earlier tab/account context. Expired, invalid, or ineligible entries should use the existing authorized path. No TTL, invalidation policy, cache size, or backend is selected. The thin slice remains the repeated historical chart view; no public API change is currently proposed, but compatibility requires repository inspection.
+
+## What does it cost?
+
+The delivery budget and staffing are unknown, so there is no justified estimate or schedule. Likely work includes request-path measurement, result identity and authorization review, the selected reuse implementation, freshness handling, tests, and operational visibility. Imani owns Reader; implementation assignees and any additional reviewers are unassigned. Shared storage would add deployment, retention, and operational work that is not yet authorized or justified by evidence.
+
+Reader is the only identified repository. Cross-repository coordination is skipped. The affected route, chart consumer, data-access code, infrastructure, and documentation paths must be identified before final issue sizing. If investigation discovers another repository or service must change, update the scope and authorization rather than assuming permission to change it.
+
+## What alternatives did we consider?
+
+Client-local retention could avoid repeat requests when tabs switch, but requires careful account switching, authorization-change, and freshness behavior. It is attractive only if this addresses the measured delay and fits the client boundaries. No client ownership or storage capability has been established.
+
+Reader process-local caching avoids selecting a shared backend, but duplicates entries across instances, loses entries on restart, and requires bounded memory. A shared cache may improve reuse across instances and support common invalidation, but adds an operational dependency and data-retention decisions. Request volume, topology, and measured reuse should decide between them; neither is selected.
+
+Optimizing the uncached request path or doing nothing avoids cache consistency complexity. Path optimization may be preferable if repeat reuse is low or freshness rules are strict. Doing nothing preserves the observed delay. Baseline measurements and an agreed latency target are needed to compare the benefit and cost fairly.
+
+## Validation and rollout proposal
+
+Measure repeat-view latency and request frequency during representative account-review flows, separating client behavior, data access, and chart computation. Agree a measurable repeat-view latency outcome and freshness limit with the affected workflow owner; neither value is currently known. Inspect whether historical data can change and whether reliable invalidation signals exist. Use a small bounded experiment to compare reuse with the uncached path before selecting the backend.
+
+Test account A and B with otherwise identical chart parameters, account switches with requests in flight, permission revocation, within-account permission differences, and all result-affecting filters. Validate freshness under data changes and expiration once a policy exists. Measure latency, hit rate, resource use, and fallback behavior under representative load. None of these checks has run.
+
+Propose a controllable opt-in rollout for the chart path after isolation and freshness checks pass. Keep an authorized uncached path available for bypass and rollback. Define capacity limits and failure behavior before rollout so cache failure does not overload the underlying service. If implementation introduces a backend, include entry cleanup and retention in the rollback plan. This is a proposal; the repository's rollout mechanism is unknown.
+
+## What would change the analysis?
+
+Abandon or revise result reuse if it cannot preserve account isolation, if freshness requirements cannot be enforced, or if measured latency improvement is not useful. Unknown request volume, staleness bound, budget, target latency, deployment topology, and backend choice remain explicit implementation gates. Resolve them through measurement and owner decisions; this draft does not invent defaults.
```

Reader tracking issue draft — not opened:

```markdown
Title: reader-repeat-views: scope and validate faster chart revisits

Part of Faster repeat chart views in Reader.
Vault: 2026-10-20-reader-repeat-views-problem · 2026-10-20-reader-repeat-views
Both briefs are proposed drafts.

Scope for this repo:
- Identify the historical chart request path and causes of repeated-view latency.
- Measure request frequency, baseline repeat latency, and likely reuse.
- Establish the acceptable staleness bound, target latency, budget, and implementation owner before selecting a backend.
- Review account authorization, permission-sensitive results, account switching, and late responses; derive a complete result identity.
- Compare client-local reuse, process-local caching, shared caching, and uncached-path optimization against the evidence.
- Build a bounded experiment for the selected approach only after its isolation and freshness contract is defined.
- Verify isolation across accounts and permission scopes, freshness, invalidation, filter correctness, resource limits, and failure behavior.
- Define a controlled rollout, uncached bypass, rollback, and operational ownership.

Depends on: No other repository issues identified. Measurements and freshness/budget decisions precede implementation commitment.
Owner: Imani (Reader owner; implementation assignee not yet specified)
Open: Request volume, staleness bound, delivery budget, latency target, backend, deployment topology, and data-handling constraints.
Out of scope: General Reader caching, a cross-service cache platform, and a repo design doc.
```

Proposed addition: `discussions/2026-10-20-reader-repeat-views-session.md`

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
+Imani, Reader's owner, worked with an assistant to frame the repeated historical chart wait during account reviews. Analysts see the chart reload after tab switches. The desired benefit is faster repeat viewing with account isolation preserved. Imani requested a draft with explicit gaps and no further questions or repo design doc.
+
+The session drafted a problem brief and a paired design brief, plus the Reader issue “reader-repeat-views: scope and validate faster chart revisits.” No files were published and no issue URL exists. The supplied vault snapshot had no matching briefs; live search and graph traversal were not run.
+
+No cache backend, TTL, budget, or performance target was decided. The proposed next work is measurement and account/freshness contract review, followed by a bounded experiment. Request volume, staleness bound, and delivery budget are explicitly unknown. Cross-repository coordination is skipped because Reader is the only identified repository.
+
+Imani owns resolving the open product and delivery constraints and arranging the implementation ownership. Planned verification covers cross-account and within-account authorization, account switches and in-flight responses, freshness, performance, resource limits, and rollback. No validation has been performed.
+
+Artifact implications: the new problem and candidate design are covered by the paired drafts. Based on the supplied snapshot, no matching brief needs supersession. If uncertainty persists after measurement, a bounded inquiry into freshness and reuse could be useful. A decision record should wait for an actual approach selection; none is claimed here. Those optional artifacts are not created in this draft session.
```

The requested drafts are complete: problem `2026-10-20-reader-repeat-views-problem` and design `2026-10-20-reader-repeat-views` both carry `status: draft`; the design frames that problem. Session note `2026-10-20-reader-repeat-views-session` is drafted without a status field. The Reader issue remains a body for handoff, and the repo doc is skipped. Implementation planning remains provisional until the stated gaps are resolved. Optional inquiry or decision artifacts can follow when there is evidence or a decision to capture; no additional answer is needed for this draft.
