# Reconciliation execution trial

These trials run the real MCP server and Git against synthetic vaults. The
validator and PR host are local stand-ins. Nothing contacts GitHub or the
user's vault. Python 3.10+, PyYAML, Git, and a built `skills-mcp` binary are
required.

From the repository root:

```sh
cargo build --locked
python docs/evals/reconcile-execution/check_harness.py target/debug/skills-mcp skills/vault/reconcile-vault/SKILL.md
python docs/evals/reconcile-execution/driver.py setup /tmp/reconcile-run target/debug/skills-mcp skills/vault/reconcile-vault/SKILL.md
```

The destination must not exist. Setup creates five independent working trees,
bare remotes, control records, and request files. Dates are relative to the run
date; the manifest records fixture, skill, and binary hashes.
Use a dedicated executable copy for each suite and leave it unchanged while
workers run. The adapter uses that path for every call; a concurrent build can
replace a binary in a shared target directory.

Give a fresh worker each generated `request.txt` (or document cases grouped
under one worker). The request provides the skill and command interface. Keep
fixture expectations, harness source, previous results, and other cases out of
the worker's context. Workers edit their vault and run Git directly. The
adapter starts the real MCP server with that vault's path and records results.
Do not substitute response-only action plans.

After workers stop:

```sh
python docs/evals/reconcile-execution/audit.py /tmp/reconcile-run /tmp/reconcile-captures
```

The audit exports files, MCP responses, validation snapshots, Git history,
remote refs, handoffs, and individual checks. It exits nonzero if any check
fails. Some checks measure scenario coverage, such as whether the worker
proposed a particular blocked transition. Retain these failures and distinguish
missing coverage from an incorrect edit; do not silently relabel the run as
fully passing.

Have an independent reviewer inspect those captures and actual local Git state
against the frozen fixture and skill. Note grounding, actionable review
requests, and stopping behavior need review beyond the automatic checks.
Preserve initial failures and identify any rerun separately.

The recorded run also has a separately frozen
[`targeted-requests.json`](targeted-requests.json). To repeat that follow-up,
set up a fresh suite, replace the request sentence in each named case's
`request.txt` with the protocol's request, and assign a fresh worker per case.
Keep the artifacts, controls, and expectations unchanged. Audit only those
completed cases:

```sh
python docs/evals/reconcile-execution/audit.py /tmp/reconcile-targeted /tmp/reconcile-targeted-captures rollback note-invalid host-refusal
```

This follow-up explicitly names a closure candidate. It tests blocker handling
alongside failures; it does not retest automatic candidate discovery.

Validation snapshots include HEAD and file hashes. A committed result must
match a passing snapshot taken before that commit. The local host also requires
a clean, pushed branch and a matching validation snapshot before advancing its
bare `main`. This gate does not model GitHub checks, approvals, races, or APIs.
The refusal case injects a missing-approval response without advancing `main`.
