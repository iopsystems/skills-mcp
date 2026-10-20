I’ll reuse accepted problem brief `2026-10-01-export-retries-problem` unchanged. The proposed design, API repository document, both tracking issues, and session note are below.

I used plan-feature with propose-design, vault-search, and discuss. Your authorization covers committing the new design and session note to the vault, creating and pushing an `export-api` feature branch with its design document, and opening the API issue. The `export-web` issue is a draft for Bea; there will be no web writes or issue creation. No repeat permission request is needed. This simulation permits only proposed artifacts and actions, so none of those live actions was executed.

MATCHES (1)

- ID: `2026-10-01-export-retries-problem`
- Type/status: problem brief, accepted
- Path: `briefs/2026-10-01-export-retries-problem.md`
- Supplied framing: customers restart hour-long exports after connection drops; account-bound tokens, existing-download compatibility, unchanged retention, and account isolation are required.
- Connected designs: none in the supplied snapshot. No open arc was supplied.

Live vault search and graph traversal were not run. The supplied snapshot is the search evidence; no accepted brief needs replacement or modification. Repository conventions and the full implementation have not been inspected.

Discovery areas 1–5 are covered by the supplied context: interrupted exports waste completed work; success is resuming from a verified offset without regeneration; three engineer-weeks are available; API and web changes are required; opaque account-bound tokens with server-side offsets are preferred. Area 6 requires explicit coordination: Arun owns the API and overall delivery; Bea owns the web. API support and its issue precede web integration; capability detection and fallback precede the small-cohort rollout.

Proposed vault addition: `briefs/2026-10-20-resumable-exports.md`. Authorized real action: commit only this new brief with message `Design: Resume interrupted exports across API and web`.

```diff
--- /dev/null
+++ briefs/2026-10-20-resumable-exports.md
@@ -0,0 +1,50 @@
+---
+id: 2026-10-20-resumable-exports
+type: design-brief
+status: draft
+author: Arun
+created: 2026-10-20
+frames: 2026-10-01-export-retries-problem
+---
+
+# Resumable exports across API and web
+
+Add backward-compatible resume support to export-api using opaque account-bound tokens and server-side offset state, then teach export-web to detect support and resume interrupted downloads with an old-download fallback. Gate support behind an initially disabled capability flag and enable it for a small cohort after prototype and integration checks. This design frames the accepted [export retries problem brief](2026-10-01-export-retries-problem.md), which remains unchanged.
+
+## Why this approach?
+
+Opaque tokens let the server control validity, revocation, account association, and permitted offsets. They support the accepted brief's account-bound-token and no-cross-account-access requirements without trusting a caller-supplied offset as authority. Keeping old downloads available satisfies compatibility and provides rollback. Token state and exported bytes must expire within existing retention; this proposal does not extend export availability.
+
+## What does it look like?
+
+Proposed flow: an authorized client begins a download, receives an opaque resume token when capability support is available, and retains the token and locally verified progress for that export. On interruption it requests resume with its current authentication, token, and candidate offset. The server validates account access, token validity, export identity/version, retained-file availability, and agreement with permitted progress before returning bytes from a verified offset. Invalid, expired, or mismatched requests must fail without returning another account's data or silently joining inconsistent bytes.
+
+The exact wire contract remains to be designed. In particular, bytes written by the server are not necessarily bytes received by the client. The prototype must establish how client progress is acknowledged or reconciled with server-side state, define offset units and boundaries, and prevent gaps or duplicate bytes. File identity, length, and integrity checks must bind the resumed response to the original export. Account binding is required even for an unguessable token, and current authorization must be checked on resume.
+
+The web detects the advertised capability and uses the existing download path when it is unavailable. A failed or unsupported resume offers the existing restart behavior with a clear user outcome. Range-request behavior through client proxies is unknown and must be tested before rollout. API endpoint names, response headers, token transport, and error codes are proposed design work, not existing contracts.
+
+## What does it cost?
+
+Three engineer-weeks are available across the work. Arun owns API delivery and overall coordination; Bea owns web integration. A provisional allocation is one week for the proxy/progress prototype and contract, one for API state/lifecycle support and web integration, and one for compatibility, isolation, integrity checks, and rollout preparation. This is a planning allocation, not a validated estimate; existing file retention and state-storage capabilities have not been inspected.
+
+Risks include proxy behavior, inaccurate progress accounting, offset/file mismatches, authorization regressions, stale token state, and incompatibility with existing clients. Persisting resume metadata adds lifecycle work but must not add retention. If the export no longer exists within current retention, resume cannot succeed and must use the established fallback behavior.
+
+## What alternatives did we consider?
+
+Signed client offsets reduce server-side progress storage but complicate revocation and still need account, file, and offset validation. The preferred opaque-token design keeps those controls on the server. A token protocol can be replaced behind the capability gate, but incorrect resume behavior may corrupt downloaded files or expose data before rollback, so validation must precede rollout.
+
+Restarting the export is the existing workaround. It preserves compatibility and avoids new protocol/state costs but fails the desired outcome of resuming without regeneration. It remains the fallback when capability is absent or resume is impossible.
+
+## Coordination, validation, and rollout
+
+First create the export-api tracking issue, owned by Arun. Its eventual URL must be inserted into Bea's export-web issue draft before Bea opens that issue. No URL or issue number exists yet. The authoritative design is this vault brief; the summary document belongs at export-api/docs/design/resumable-exports.md.
+
+Prototype client/proxy range behavior and progress reconciliation before rollout. Deploy backward-compatible API support with the capability flag disabled. Then land web capability detection, resume behavior, and old-download fallback. After joint validation, enable the capability for a small cohort. Arun coordinates API readiness; Bea verifies web behavior and receives the API issue URL for dependency tracking.
+
+Test interruption and reconnection at representative offsets, compare the resumed file with an uninterrupted export, and verify that resumption does not regenerate the export. Test invalid/out-of-range offsets, mismatched export identity, account switches, unauthorized tokens, revoked or expired tokens, permission changes, and exports removed by existing retention. Check old clients against the new API and new web against capability-disabled API. Verify proxy behavior and behavior when flags change during a download. These checks have not run.
+
+Rollback disables the capability flag while retaining old downloads. Define how in-flight resume attempts fail or fall back when the flag is disabled; do not extend retention to support rollback. Propose monitoring resume success/fallback rates, integrity failures, and authorization failures during the cohort; exact rollout thresholds remain to be agreed by Arun and Bea.
+
+## What would change the analysis?
+
+Revisit the transport or scope if the prototype shows proxies cannot support reliable range requests or if client/server progress cannot be reconciled safely. Revisit if existing retained exports cannot be reused without regeneration or retention changes, or if the work cannot fit three engineer-weeks. Open details are the wire contract, progress acknowledgment, storage/lifecycle mechanism, in-flight rollback behavior, cohort size, and rollout thresholds. No status promotion or final decision record is claimed.
```

Proposed API repository addition: `export-api/docs/design/resumable-exports.md`. Authorized real action: create a feature branch proposed as `feature/resumable-exports-plan`, commit this document there, and push the branch. Branch availability and remote details remain unverified; no branch or URL is claimed to exist. No main-branch change or force push is proposed.

```diff
--- /dev/null
+++ export-api/docs/design/resumable-exports.md
@@ -0,0 +1,31 @@
+# Resumable exports
+
+**Vault:** `2026-10-01-export-retries-problem` (accepted problem) · `2026-10-20-resumable-exports` (draft design)
+**Status:** draft · **Accountable:** Arun
+
+Let customers resume interrupted downloads from a verified offset without regenerating their export. Add opaque account-bound tokens with server-side progress state in export-api, then capability detection and old-download fallback in export-web. The paired vault briefs are authoritative; these stable IDs identify them until repository URLs are available.
+
+## Scope
+
+- In: authorized resume tokens, verified offsets and file identity, compatibility, unchanged retention, web detection/fallback, staged rollout.
+- Out: retention extensions, cross-account access, removal of existing downloads.
+
+## Design
+
+Authorize every resume against the token's account and export, validate availability and progress, and return bytes only from a verified position in the same file. The design brief holds rationale, alternatives, and coordination details. Do not equate server bytes sent with client bytes received; prototype the progress contract and proxy range behavior before rollout.
+
+## Plan
+
+1. Arun opens the API issue and supplies its URL for Bea's web issue draft. Neither issue has a URL yet.
+2. Prototype range/proxy behavior and progress reconciliation; settle the API/web contract.
+3. Arun deploys backward-compatible API support behind a disabled capability flag.
+4. Bea adds web capability detection, resume, and old-download fallback.
+5. Verify integrity, no regeneration, account isolation, retention expiry, and compatibility, then enable a small cohort.
+6. Roll back by disabling the flag while preserving old downloads.
+
+## Open questions
+
+- Do client proxies preserve the required range-request behavior?
+- How will client progress be reconciled with server offsets, and which wire and storage contracts implement it?
+- What happens to in-flight resume requests during rollback?
+- Which cohort and monitoring thresholds gate expansion within the three-engineer-week budget?
```

The requested branch push is authorized. A draft PR can be proposed after the branch exists; no PR creation is included among the live actions here.

API tracking issue, to be opened first in `export-api` under the existing authorization:

**Title:** `resumable-exports: add account-bound resume support`

```markdown
Part of Resumable exports across API and web.
Vault: 2026-10-01-export-retries-problem · 2026-10-20-resumable-exports
Design doc: docs/design/resumable-exports.md on the proposed feature/resumable-exports-plan branch. A branch URL is not available yet.

Scope for this repo:
- Prototype range requests through supported client proxies with Bea, and reconcile client-received progress with server-side offsets.
- Define the resume token, offset, file-identity, error, expiry, and revocation contract.
- Implement opaque account-bound tokens; validate current access, export identity, retention availability, and offsets on every resume.
- Reuse retained export bytes without regeneration or any retention extension.
- Deploy backward-compatible support behind a disabled capability flag.
- Preserve existing downloads and implement defined behavior for in-flight resumes when the flag is disabled.
- Validate resumed-file integrity, account isolation, invalid/revoked/expired tokens, offsets, old-client compatibility, and unchanged retention cleanup.

Acceptance: interrupted downloads resume from a verified offset without regeneration; existing downloads remain compatible; no cross-account access or additional retention is introduced. Proxy/progress prototype must pass before rollout.
Coordination: open this issue first, then give its actual URL to Bea for the export-web issue. Deploy API support disabled before web integration; enable a small cohort only after joint checks.
Rollout: small cohort after web detection and fallback are ready.
Rollback: disable capability flag and retain old downloads.
Depends on: no predecessor issue; proxy/progress validation and agreed contract gate rollout.
Owner: Arun
Overall delivery budget: three engineer-weeks across API and web, subject to feasibility validation.
```

Web tracking issue draft for Bea. Do not open it or write to `export-web`.

**Title:** `resumable-exports: detect capability and resume interrupted downloads`

```markdown
Part of Resumable exports across API and web.
Vault: 2026-10-01-export-retries-problem · 2026-10-20-resumable-exports
Design doc: export-api/docs/design/resumable-exports.md on the proposed feature/resumable-exports-plan branch; branch URL pending actual creation.

Scope for this repo:
- Work with Arun on the proxy range-request and client/server progress prototype.
- Detect API resume capability and preserve the old-download flow when unavailable.
- Keep token/progress state bound to the intended account and export and follow the agreed acknowledgment/offset contract.
- Resume interrupted downloads with the API only when supported; handle expired/unavailable exports and resume failures through clear old-download fallback.
- Verify final file integrity, account switches, interruption/reconnection, disabled capability, and compatibility with the existing API behavior.

Acceptance: supported interrupted downloads resume without regeneration and produce the expected file; unsupported/disabled cases retain old-download behavior; account changes cannot reuse another account's resume state.
Depends on: export-api issue “resumable-exports: add account-bound resume support”. Its actual URL must be inserted here after Arun opens it and before Bea opens this issue. No issue number or URL exists yet.
Landing order: backward-compatible API support deployed disabled, then web detection/resume/fallback, then small-cohort enablement after joint validation.
Rollback: API capability disabled; web continues to offer old downloads.
Owner: Bea
Overall accountable owner: Arun
```

Proposed vault addition: `discussions/2026-10-20-resumable-exports-session.md`. Authorized real action: commit the session note with message `Note: Plan resumable export coordination`. The draft accurately records simulation results; an actual execution note should record resulting issue/branch URLs only after those actions succeed.

```diff
--- /dev/null
+++ discussions/2026-10-20-resumable-exports-session.md
@@ -0,0 +1,21 @@
+---
+id: 2026-10-20-resumable-exports-session
+type: session-note
+author: Arun
+created: 2026-10-20
+relates_to: [2026-10-01-export-retries-problem, 2026-10-20-resumable-exports]
+---
+
+# Resumable exports planning session
+
+Arun supplied context for a solo planning pass with an agent. Arun owns the API and overall delivery; Bea owns the web work. Bea is named as an owner, not asserted to have attended this session. Customers need to resume interrupted downloads without regenerating hour-long exports.
+
+The accepted problem brief 2026-10-01-export-retries-problem is reused unchanged. The supplied snapshot contains no design framing it and no open arc. Live vault search and graph traversal were not run. A new draft design, 2026-10-20-resumable-exports, proposes opaque account-bound tokens with server-side offsets while retaining existing downloads and retention limits.
+
+The plan orders backward-compatible API support behind a disabled capability flag, web capability detection with fallback, and then a small-cohort rollout. Disabling the flag while retaining old downloads is the rollback direction. Client proxy handling of range requests and reliable progress reconciliation require a prototype before rollout. Three engineer-weeks are available; detailed feasibility is unverified.
+
+Authorized actions are committing the new design and this note to the vault, creating and pushing the export-api document branch, and opening the API issue. Web writes and opening its issue are not authorized; its issue is drafted for Bea. The API issue must be opened first and its actual URL passed into Bea's draft. This simulation executed none of those external actions: no brief or note landed, no document branch was created/pushed, and no issue was opened. No URLs or issue numbers exist in this output.
+
+Arun owns confirming the resume contract, state lifecycle, prototype outcome, API readiness, and URL handoff. Bea owns web integration and fallback validation. Both must settle in-flight rollback behavior and cohort criteria before rollout.
+
+Artifact triage: the accepted frame and new design draft cover the problem/design pair; no supersession is indicated. A bounded inquiry could track failed proxy/progress experiments if needed, and a later decision record could capture the validated approach. These are optional followups, not created artifacts or accepted decisions. No arc closure or status transition is proposed.
```

Session-note triage uses the same supplied snapshot: the existing accepted problem is relevant and the new design fills the missing design relationship. No additional problem brief is warranted. A proxy/progress inquiry and a later decision record remain options if their evidence warrants them; neither is created automatically.

Completion status:

- Problem brief: existing accepted `2026-10-01-export-retries-problem` reused unchanged.
- Design brief: concrete draft `2026-10-20-resumable-exports`, correctly framed; commit authorized but not executed in this simulation.
- Repository design document: proposed at `export-api/docs/design/resumable-exports.md`; feature-branch creation, commit, and push authorized but not executed.
- API tracking issue: ready to open first; authorized, not executed; eventual URL still required.
- Web tracking issue: drafted for Bea; opening and web writes remain outside authorization.
- Session note: concrete solo-note draft; commit authorized but not executed.
- Open questions: proxy/range behavior, verified client progress, wire/state/lifecycle details, in-flight rollback, cohort criteria, and feasibility within three engineer-weeks. Prototype and tests remain unrun.
