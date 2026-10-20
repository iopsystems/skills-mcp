Authorization is established for the new vault design brief and session-note commits, an export-api design doc on a feature branch and branch push, and an export-api tracking issue. The export-web issue will remain a draft for Bea. No repeat approval is needed. This simulation prohibits external execution, so the artifacts and authorized actions below are proposed, not committed, pushed, or opened.

I used `plan-feature` with its brief, search, and session-note workflows. From the supplied snapshot:

MATCHES (1)

- `2026-10-01-export-retries-problem` — accepted problem brief at `briefs/2026-10-01-export-retries-problem.md`; reused unchanged as the frame.
- No design brief frames it in the supplied snapshot. No open arc is supplied.

Live vault search and graph traversal were not run. No new problem brief or supersession is proposed. Unknown optional metadata is omitted.

Proposed addition: `briefs/2026-10-20-resumable-exports.md`

```diff
--- /dev/null
+++ briefs/2026-10-20-resumable-exports.md
@@ -0,0 +1,52 @@
+---
+id: 2026-10-20-resumable-exports
+type: design-brief
+status: draft
+author: Arun
+created: 2026-10-20
+frames: 2026-10-01-export-retries-problem
+---
+
+# Resumable exports
+
+Add backward-compatible resume support using opaque account-bound tokens and server-side verified offsets, followed by web capability detection and fallback to existing downloads. Resume reads from an existing export without regenerating it. The accepted [problem brief](2026-10-01-export-retries-problem.md) remains the authoritative frame and is reused unchanged. This draft proposes the design; it does not record acceptance or completed implementation.
+
+## Why this approach?
+
+Opaque tokens and server-side state support revocation and account binding while addressing the frame's no-cross-account-access constraint. Retaining old downloads and gating the new capability preserve compatibility. Resume metadata must expire within the export's existing lifetime so the design does not add retention. The three-engineer-week budget favors a bounded API and web change; feasibility still depends on the range-request prototype and the existing storage layout.
+
+## What does it look like?
+
+Proposed sequence: an authorized caller receives an opaque token tied to the account, export identity/version, existing expiry, and server-side progress state. The web client retains the token for the current authorized download context. On interruption, it requests continuation against the same immutable export representation. The API rechecks authorization, validates token/account/export binding and expiry, and resumes only at an offset verified against its stored state and export bounds.
+
+Bytes transmitted by the server are not by themselves proof of bytes durably received by the client. The prototype must establish how client progress is acknowledged and verified, or how the server safely retransmits from a conservative checkpoint and the client avoids duplicate bytes. Exact checkpoint granularity, acknowledgement format, routes, headers, and error responses remain to be specified; no undocumented API is assumed here.
+
+The range transport must keep byte offsets consistent with the stored representation, including any compression or intermediary transformations. On expired or invalid tokens, changed export identity, or unsupported resume transport, fail safely or offer the established old-download path. Never resume another account's export. Avoid exposing token values through URLs, logs, or telemetry if the existing transport permits a protected alternative; finalize handling against the existing authentication design.
+
+Resume state is bounded by existing export expiry and deleted with the export; it must not extend export retention or create a new archival copy. An expired export cannot be promised a no-regeneration resume. Confirm the current export lifecycle and storage support for offset reads before implementation. The success target applies to an interruption while the original export remains available under existing policy.
+
+## What does it cost?
+
+Three engineer-weeks are available across export-api and export-web. Arun is overall accountable and owns the API; Bea owns the web client. Work includes the transport/checkpoint prototype, API token and offset state, lifecycle cleanup, client resume handling, fallback, compatibility and isolation tests, and staged rollout. The budget is supplied; detailed allocation and estimate are not yet validated. New state and race handling add operational complexity, and proxy behavior is an explicit unresolved risk.
+
+Affected surfaces are the API download contract and export lifecycle, the web download/retry flow, capability detection, tests, operational visibility, and export-api documentation. Existing downloads and their consumers remain supported. Repository inspection must establish whether storage schema changes or cleanup hooks are required. No historical-export backfill, retention extension, or additional repository is proposed.
+
+## What alternatives did we consider?
+
+Opaque account-bound tokens with server-side offsets are the preferred direction because the API can revoke tokens and validate progress centrally. The cost is bounded server-side state and cleanup. Signed client offsets reduce state requirements but complicate revocation and still require account, export, expiry, and bounds checks. Revocation and verification needs decide the preference; no backend for progress storage is selected without inspecting existing facilities.
+
+Restarting remains the existing workaround and the compatibility fallback. It avoids resume state but does not deliver the framed user benefit. Buffering or retaining an additional export copy beyond current expiry is outside the no-added-retention constraint. The rollout flag makes capability exposure reversible; it does not by itself remove state, so cleanup remains required during rollback.
+
+## Coordination, validation, and rollout
+
+First, Arun and Bea prototype range requests through representative client proxies and establish a verified checkpoint contract before rollout. Test interrupted transfers at multiple offsets and page/chunk boundaries, compare final bytes with the original export, and verify that successful resumes do not invoke regeneration. Include cross-account tokens, permission changes, expiry, token revocation, malformed or out-of-bounds offsets, concurrent attempts, changed export identity, and cleanup under existing retention.
+
+Arun deploys backward-compatible API support behind a disabled capability flag first. Bea then deploys capability detection and resume support with old-download fallback. After prototype and compatibility/isolation checks pass, enable the flag for a small cohort. Observe successful resumes, restarts, error rates, and byte-integrity failures without logging token secrets. Exact cohort size and rollout acceptance thresholds remain to be set by the owners.
+
+Open the export-api tracking issue before any export-web issue. Its actual URL is a handoff dependency for Bea's draft; no URL or issue number exists yet. Share the checkpoint/error contract and prototype result with Bea before client integration. Arun owns overall rollout readiness; Bea owns web compatibility verification. The vault brief is authoritative, and export-api/docs/design/resumable-exports.md summarizes it. No export-web write or issue creation is authorized in this session.
+
+Rollback disables the capability flag while retaining existing downloads. Verify that the web falls back when capability is absent or resume is rejected, including interruption during rollback. Keep existing token/state cleanup active until expiry; disabling the feature must not increase retention. Any protocol or storage incompatibility must be resolved before enabling the cohort.
+
+## What would change the analysis?
+
+Revisit the transport or scope if representative proxies cannot support reliable range behavior, if offsets cannot be verified without corruption, or if existing storage cannot resume without regeneration. Any requirement to extend retention or weaken account isolation conflicts with the accepted frame and is not authorized by this design. Unknowns are proxy behavior, checkpoint verification details, storage and lifecycle integration, exact API contract, and rollout thresholds. No prototype or implementation checks have run.
```

Authorized vault action, proposed only: add the design brief above and commit with `Design: Plan resumable exports`. Leave the accepted problem brief unchanged. Keep the new design at `draft`; no transition check or promotion is proposed.

Proposed feature branch: `plan/resumable-exports` in `export-api` (availability not checked). Proposed addition: `export-api/docs/design/resumable-exports.md`:

```diff
--- /dev/null
+++ export-api/docs/design/resumable-exports.md
@@ -0,0 +1,35 @@
+# Resumable exports
+
+**Vault:** `2026-10-01-export-retries-problem` (problem) · `2026-10-20-resumable-exports` (design)
+**Status:** draft · **Accountable:** Arun
+
+We will add opt-in resume support so interrupted downloads can continue from a verified offset without regenerating the export. The API will use opaque account-bound tokens and server-side progress state, and the web client will detect the capability while preserving existing downloads. The vault briefs remain authoritative.
+
+## Scope
+
+- In: Backward-compatible API resume support, account and authorization checks, bounded progress state, client capability detection and fallback, proxy prototype, and staged rollout.
+- Out: Added export retention, cross-account access, removal of old downloads, and changes outside export-api and export-web.
+
+## Design
+
+See the authoritative vault design at `knowledge-iop/briefs/2026-10-20-resumable-exports.md`, framing `knowledge-iop/briefs/2026-10-01-export-retries-problem.md`. These are vault locators; a browsable vault URL has not been supplied.
+
+Bind each opaque token to the authorized account, export identity/version, expiry, and verified server-side progress. Reauthorize resume requests. Reuse the existing export representation and expire progress state within the existing export lifecycle. Define an acknowledgement/checkpoint contract that distinguishes bytes sent from bytes actually received. Signed client offsets were considered but complicate revocation; the design brief contains the full tradeoff.
+
+## Plan
+
+1. Arun and Bea prototype range behavior through representative proxies and specify verified checkpoints and fallback errors before rollout.
+2. Arun implements and deploys backward-compatible API support behind a disabled capability flag, with account isolation, expiry, cleanup, and byte-integrity tests.
+3. Bea implements capability detection and resume behavior with old-download fallback after the API support is available.
+4. Both owners verify interrupted downloads resume without regeneration, preserve exact bytes, and respect authorization and existing retention.
+5. Arun enables a small cohort after validation; Bea verifies web behavior. Rollback disables the flag while retaining old downloads and normal state cleanup.
+
+The export-api tracking issue must be opened first. Its real URL will be added here and supplied to Bea before she opens the export-web issue. Neither issue has a number or URL yet. The total delivery budget is three engineer-weeks; detailed estimates await the prototype and repository inspection.
+
+## Open questions
+
+- Do representative client proxies preserve usable range semantics and byte offsets?
+- What checkpoint acknowledgement and retransmission scheme verifies durable client progress?
+- Which existing storage facilities support offset reads and progress expiry without added retention?
+- What are the exact API responses, cohort size, and rollout thresholds?
+- What are the eventual issue and vault browser URLs?
```

Authorized repo action, proposed only: create a fresh feature branch in export-api, add that doc, commit with `docs: Plan resumable exports`, and push the branch. No branch URL is claimed. After the API issue exists, replace its locator in the doc with the real URL on that branch and push the update. A draft PR can be prepared from the branch; PR creation is not part of the supplied authorization and has not been performed.

Export-api tracking issue — authorized creation proposed, not opened:

```markdown
Title: resumable-exports: implement account-bound API resume support

Part of Resumable exports.
Vault: 2026-10-01-export-retries-problem (accepted) · 2026-10-20-resumable-exports (draft)
Design doc: docs/design/resumable-exports.md on the proposed plan/resumable-exports feature branch; no pushed URL exists yet.

Scope for this repo:
- Prototype range requests with Bea through representative client proxies before rollout.
- Specify token, checkpoint verification, offset bounds, expiry, and client error/fallback contracts.
- Implement opaque account-bound tokens and server-side progress using existing export data; reauthorize every resume.
- Preserve old downloads and existing retention; keep progress cleanup tied to export expiry.
- Verify exact resumed bytes without regeneration, cross-account rejection, permission changes, expiry, revocation, malformed offsets, concurrent attempts, and cleanup.
- Deploy behind a disabled capability flag before the web rollout.
- Coordinate small-cohort enablement with Bea; rollback by disabling the flag while retaining old downloads.
- Publish this issue's actual URL to Bea and update the design doc before the web issue is opened.

Depends on: No prior web issue. Proxy/checkpoint prototype gates rollout; cohort enablement follows compatible web deployment.
Owner: Arun
Overall budget: Three engineer-weeks across API and web; allocation not yet validated.
Open: Proxy behavior, verified checkpoint details, storage integration, and rollout thresholds.
```

Export-web tracking issue — draft for Bea, with no authorized write or creation:

```markdown
Title: resumable-exports: add capability-aware web download resume

Part of Resumable exports.
Vault: 2026-10-01-export-retries-problem · 2026-10-20-resumable-exports
Design doc: export-api/docs/design/resumable-exports.md; branch URL will be supplied after the authorized API branch push.

Scope for this repo:
- Work with Arun on the proxy and verified-offset prototype before rollout.
- Detect resume capability and preserve old-download fallback when absent or rejected.
- Implement the agreed checkpoint acknowledgement/retransmission contract without duplicated or missing bytes.
- Keep token and download state in the correct account context; clear or reject stale context on account changes.
- Verify interruption recovery without regeneration, exact bytes, expired/revoked tokens, account changes, unsupported proxies, and flag-disable rollback.
- Deploy after backward-compatible API support, before small-cohort enablement.

Depends on: The export-api resumable-exports issue, which must be opened first. Insert its actual URL before opening this issue; no URL exists yet.
Owner: Bea
Open: API checkpoint/error contract, client proxy behavior, and rollout acceptance thresholds.
```

Proposed addition: `discussions/2026-10-20-resumable-exports-session.md`

```diff
--- /dev/null
+++ discussions/2026-10-20-resumable-exports-session.md
@@ -0,0 +1,23 @@
+---
+id: 2026-10-20-resumable-exports-session
+type: session-note
+author: Arun
+created: 2026-10-20
+relates_to: [2026-10-01-export-retries-problem, 2026-10-20-resumable-exports]
+---
+
+# Resumable export planning session
+
+Arun's solo planning session with an assistant developed a design against the accepted export-retries problem brief. Bea is the named web owner, not a claimed attendee. Arun owns the API and overall delivery; three engineer-weeks are available.
+
+The accepted brief is reused unchanged. The supplied snapshot contains no design framing it, and no open arc is supplied. Live search and graph traversal were not run. A new draft proposes opaque account-bound tokens with server-side verified offsets, no additional retention, backward compatibility, and no cross-account access. Signed client offsets remain an alternative with revocation costs; restarting remains the old-download fallback.
+
+The planned landing order is API support behind a disabled flag, web capability detection with fallback, then a small enabled cohort. Rollback disables the flag and keeps old downloads and state cleanup. Proxy range handling is unresolved and must be prototyped before rollout. Checkpoint verification, storage integration, exact API responses, and rollout thresholds also need specification.
+
+Prepared artifacts are briefs/2026-10-20-resumable-exports.md and export-api/docs/design/resumable-exports.md, an API tracking issue body, and a web issue draft for Bea. The API issue must exist first; its actual URL is required before Bea opens the web issue. No issue or branch URL exists in this simulation.
+
+The user authorized committing the new design and this note to the vault, writing the API doc on a feature branch and pushing it, and opening the API issue. The simulation permits proposals only, so none of these actions has executed. No export-web write or issue creation is authorized. Publication details must be updated with actual results if the proposed actions are later executed.
+
+Arun owns the API issue and eventual URL handoff, API compatibility, lifecycle verification, and cohort readiness. Bea owns web fallback and integration; both owners contribute to the range/checkpoint prototype. No test or prototype result is claimed.
+
+Artifact implications: the accepted problem still frames the work and does not need supersession. The proposed design covers the new solution. A decision record may follow actual acceptance; the preferred token approach is not represented as an accepted decision here. If the proxy prototype remains inconclusive, a bounded inquiry may be useful. Neither optional artifact is created. No arc closure or status transition is proposed.
```

Authorized vault action, proposed only: add the session note and commit with `Note: Plan resumable exports`. If executing after the API issue and push, update the note with their actual states and URLs before committing it; never substitute invented links.

The handoff accounts for every output:

| Output | State |
| --- | --- |
| Problem `2026-10-01-export-retries-problem` | Accepted; reused unchanged |
| Design `2026-10-20-resumable-exports` | Drafted; frames the accepted problem; commit authorized but not executed |
| `export-api/docs/design/resumable-exports.md` | Drafted; feature-branch write and push authorized but not executed |
| export-api tracking issue | Concrete body supplied; creation authorized but not executed |
| export-web tracking issue | Draft for Bea only; awaiting eventual API issue URL |
| Session `2026-10-20-resumable-exports-session` | Drafted without a status field; commit authorized but not executed |

The proposed execution order is design commit, API doc branch commit/push, API issue creation, actual URL updates and handoff to Bea, then the session-note commit with actual results. The simulation is the only reason those authorized actions remain unperformed; it does not require reconfirming permission. Proxy and checkpoint validation remain required before rollout, and no implementation or validation is claimed complete.
