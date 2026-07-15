# Assumptions and Limitations

This document records the conditions under which the current roadmap and design
are expected to work. Revisit an assumption when evidence contradicts it; do not
silently convert a limitation into an implied guarantee.

## Audience and documentation

- Internal humans are the primary audience for this repository. They need a short
  path to install and use the bundle and a compelling path to contribute lessons
  from development experience.
- Human and agent readers should share authoritative documentation surfaces.
  Separate agent-only copies of README facts, CLI contracts, or code behavior are
  expected to drift.
- Human users benefit from prominent visual explanations of architecture and
  workflow in complex projects. Agents and accessibility tools need equivalent
  lists, tables, and explicit contracts.
- Agent evaluation can reveal ambiguity and missing operational information but
  does not prove human usability. Human-oriented and high-risk features require
  human review.

## Template identity and upgrades

- The public `iopsystems/skills-mcp` Git history remains available at immutable
  commits recorded by installed instances.
- Template versions and file digests identify the exact base used for installation.
  Rewritten or unavailable history prevents automatic three-way comparison.
- Installed instances preserve `template-state.yaml`. Deleting or rewriting the
  instance ID or provenance may make an instance untrackable.
- A duplicated `template-base/` is not stored by default. Offline projects may opt
  into vendoring the baseline.
- Project customizations are expected. Undeclared changes can be detected by diff,
  but their intent may require human interpretation.
- Upgrades stop on missing history, digest mismatch, or unresolved merge conflict;
  they never infer or overwrite silently.
- The active seeder uses descriptor-relative no-follow operations, exclusive
  creation or staging, immediate identity and digest checks, and a project-scoped
  lock honored by cooperative project writers. An uncooperative local writer can
  ignore the lock and remains a residual threat; detected changes stop safely, but
  the workflow does not claim linearizable exclusion of every local process.

## Harness portability

- `.agents/skills` is the default canonical project location because it currently
  has the broadest support among common coding harnesses.
- Claude Code currently requires `.claude/skills`. This repository preserves it
  as a real directory and links the individual `document-feature` skill to the
  canonical `.agents/skills` instance. Claude Code 2.1.202 bare mode did not
  discover the skill through either a directory-level or per-skill link during
  the dogfood run, while normal mode required authentication; the layout is
  therefore compatibility scaffolding, not verified discovery.
- Vendor behavior and discovery paths may change. Installation validates the
  harnesses available in the project environment instead of treating documentation
  claims as proof.
- Directory symlinks may be unsuitable on Windows or in repositories with
  Claude-specific skill extensions. The seeder must fall back to preserved
  directories and per-skill compatibility links or explicit copies with sync
  tracking.

## Catalog and security

- `skill_catalog` and `skill_template_get` are read-only. Seeding is a separate,
  approval-gated workflow.
- Template retrieval exposes only manifest-declared files and rejects path
  traversal, duplicate IDs, invalid manifests, and digest mismatches.
- Cross-project surveys have no hidden telemetry. They require explicit authority
  and an explicit repository set.
- Installed skills and template scripts remain executable instructions that users
  must review before trusting.

## Evaluation

- Model-based trigger tests, blind simulations, and critics are nondeterministic
  development-time evidence, not deterministic CI gates.
- Frozen representative tasks prevent documentation evaluation from moving its
  ground truth to match an agent's answer.
- Three revision rounds are a diagnostic cap, not proof that every interface can
  be made clear through prose.
- Skill forward tests require genuinely isolated context. Leaked source or expected
  answers invalidate the result.

## Visual documentation

- DOT is the authoritative source; a committed SVG is the human-consumable output.
- Graphviz must be available to validate or regenerate diagrams.
- Syntactically valid DOT and a current SVG do not prove visual clarity. Rendered
  output requires human review.
- Every operationally necessary fact in a diagram must have a concise textual
  equivalent.

## Distribution

- There is currently no prebuilt bundle or binary. Users build the Rust binary from
  source.
- Apple Silicon macOS is the primary packaging target. Linux is used sometimes,
  but its initial architecture matrix is not yet specified. Intel macOS is not an
  initial commitment.
- A Homebrew tap should consume trustworthy release artifacts or bottles; a formula
  that merely makes every user compile Rust is not the desired end state.
- Coding-agent plugin platforms are intentionally deferred.
