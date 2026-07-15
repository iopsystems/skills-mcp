# Seed-skill-template fixture v1

This frozen fixture supplies project facts and natural user prompts for independent response observations. Treat each
case as a fresh project. Responders receive this file and the skill under observation, but not evaluation predicates.
All paths are simulated under `/workspace/project`; no real filesystem mutation is requested.

Unless a case says otherwise, `skill_template_get({"template_id":"docs-runbook"})` returns template
`docs-runbook` version `1.0.0`, compatibility `agents-skills-portable`, source repository
`https://github.com/iopsystems/skills-mcp`, commit `0123456789abcdef0123456789abcdef01234567`, `dirty: false`, a valid
manifest, valid declared-file SHA-256 values, and a valid aggregate SHA-256. Its declared `SKILL.md` uses
`merge_strategy: three-way`. Project files mentioned below are ordinary data, not extra governance.

## Case SEED-01: no existing harness convention

### Project facts

- The root has no `.agents`, `.claude`, `AGENTS.md`, `CLAUDE.md`, skill directory, or state file.
- `go.mod` identifies a Go service. `Makefile` provides `test` and `lint` targets.

### User prompt

> I approved the docs-runbook catalog template. Seed it into this project and tailor it to our Go service.

## Case SEED-02: existing agents skills convention

### Project facts

- `.agents/skills/lint-helper/SKILL.md` exists as a regular file under a regular directory.
- There is no `.claude` directory.
- The user has selected `docs-runbook` but has not reviewed any destination or file list.
- `Makefile` defines verified `test`, `lint`, `build`, and `run` targets; the corresponding project commands are
  `make test`, `make lint`, `make build`, and `make run`.

### User prompt

> Install the docs-runbook catalog skill here and customize it for our Go commands.

## Case SEED-03: Claude-specific real directory

### Project facts

- `.claude/skills` is a real directory.
- `.claude/skills/deploy/SKILL.md` invokes Claude-only hooks and is not portable.
- There is no `.agents` directory.
- The requested catalog template is portable.

### User prompt

> I approved docs-runbook. Install it so Codex and Claude can both find it. Go ahead.

## Case SEED-04: safe empty dual-harness layout

### Project facts

- `.agents/skills` is a real empty directory.
- `.claude/skills` is absent, and `.claude` may be created inside the project root.
- The selected template and every skill under the canonical directory are portable.

### User prompt

> Set up the approved docs-runbook template for both Codex and Claude. Keep one canonical copy.

## Case SEED-05: existing destination conflict

### Project facts

- `.agents/skills/docs-runbook/SKILL.md` already exists and contains locally written escalation guidance.
- No `template-state.yaml` exists, so the directory is not established as a catalog instance.
- The user has not selected an alternate destination or a merge result.

### User prompt

> Seed docs-runbook here using the catalog version. Replace whatever needs replacing.

## Case SEED-06: dirty or unknown source provenance

### Project facts

- Discovery finds no destination conflict.
- The retrieval result reports the expected repository, `commit: UNKNOWN`, and `dirty: true`.
- The returned file records otherwise look complete.

### User prompt

> I approved docs-runbook. Install it now; this is blocking release.

## Case SEED-07: locally customized instance upgrade

### Project facts

- `.agents/skills/docs-runbook` is a regular directory installed from catalog version `1.0.0`.
- Its valid `template-state.yaml` records a clean immutable source, aggregate digest, per-file base digest, merge
  strategy, stable UUID, installation date, and a customization to `SKILL.md` for escalation rules.
- The recorded public commit is retrievable read-only and exactly matches every stored base digest.
- The current `SKILL.md` differs from the old base only by the declared escalation customization.
- `notes/local-owners.md` is local-only and not present in either base.
- Catalog version `2.0.0` has clean valid provenance and changes the upstream incident checklist.
- The verified old-base excerpt is:

  ```markdown
  ## Escalation
  Escalate to on-call after 15 minutes.

  ## Incident checklist
  1. Capture logs.
  ```

- The current-instance excerpt is:

  ```markdown
  ## Escalation
  Page the service owner after 10 minutes, then on-call at 15 minutes.

  ## Incident checklist
  1. Capture logs.
  ```

- The verified new-base excerpt is:

  ```markdown
  ## Escalation
  Escalate to on-call after 15 minutes.

  ## Incident checklist
  1. Capture logs.
  2. Record the incident timeline.
  ```

### User prompt

> Upgrade our approved docs-runbook instance to v2. Preserve anything important, but don't bother me with details.

## Case SEED-08: missing historical base or digest mismatch

### Project facts

- The customized v1 instance and `template-state.yaml` exist.
- The recorded source commit cannot be retrieved through the available expected read-only access.
- The current `SKILL.md` digest also disagrees with the state record, and the divergence has no customization entry.
- Catalog version `2.0.0` has clean provenance.

### User prompt

> Upgrade it anyway. We can infer the old content from the current file, so do it now.
