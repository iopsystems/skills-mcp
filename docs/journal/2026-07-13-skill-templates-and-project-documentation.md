---
status: open
opened: 2026-07-13
updated: 2026-07-13
---

# Skill templates and project documentation

## Goal

Make the repository useful to internal engineers both as an installable MCP skill
bundle and as a place to turn development experience into reusable organizational
practice. Add inert, versioned skill templates without exposing them as invocable
skills, support project-specific installation and customization with provenance,
and establish a feature-documentation method that works for human and agent users
and developers.

The first end-to-end use will install a customized feature-documentation skill in
this repository and use it to redesign the repository README.

## Decision Criteria

Ship the initial system when:

- `skills/` contains only intentionally invocable workflows and `templates/`
  contains inert skill bases.
- A read-only catalog describes active skills and templates without duplicating
  template contents in active skill prompts.
- Read-only MCP tools can list the catalog and retrieve only manifest-declared
  template files.
- `recommend-skills` can inspect a project and distinguish active skills to use,
  templates to seed, irrelevant choices, and missing coverage without mutating the
  project.
- `seed-skill-template` can customize a selected template, preserve an established
  agent-directory convention, and record enough provenance for later comparison
  and upgrade.
- Engineering-journal and feature-documentation templates both validate and carry
  trigger evaluation cases.
- A project-local feature-documentation instance works in Codex and Claude Code
  from one canonical copy.
- The installed instance produces a materially improved README whose commands,
  diagram, links, and agent comprehension checks pass, and whose human-oriented
  presentation receives human approval.
- Rust tests, formatting, linting, release build, and MCP list/call smoke tests pass.

## Scope

### Initial deliverables

- `skills/recommend-skills/SKILL.md`, a read-only adoption adviser.
- `skills/seed-skill-template/SKILL.md`, the mutating installation and upgrade
  workflow.
- `skill_catalog`, a read-only combined active-skill and template catalog tool.
- `skill_template_get`, a read-only tool restricted to files declared by a valid
  template manifest.
- `templates/catalog.yaml` and versioned manifests for two templates.
- `templates/engineering-journal-skill/`, adapted from the existing generic MCP
  skill for project-local conventions.
- `templates/document-feature-skill/`, covering README files, code documentation,
  CLI help, and DOT-authored visual documentation.
- A customized feature-documentation instance under `.agents/skills/`, with
  Claude Code compatibility from `.claude/skills` when verified.
- A redesigned repository README created through the installed instance.
- Trigger evaluations, contract tests, forward tests, and rendered-document checks.

### Exclusions

- Publishing on coding-agent plugin marketplaces.
- Automatic or unapproved cross-repository surveys.
- Hidden telemetry from installed instances.
- Automatic promotion of a project customization into a base template.
- A general remote template registry beyond the two embedded read-only tools.
- Homebrew packaging and release automation in the initial implementation.

## Evidence

The generic documentation loop is grounded in Rezolus's
[`document-feature`](https://github.com/iopsystems/rezolus/tree/main/.claude/skills/document-feature)
skill and its
[design journal](https://github.com/iopsystems/rezolus/blob/main/docs/journal/2026-07-02-document-feature-skill.md),
and in SystemsLab's project-specific
[`document-feature`](https://github.com/iopsystems/systemslab/tree/main/.claude/skills/document-feature)
variant. Together they demonstrate a stable method with project-specific adapters:
freeze representative tasks, cover behavioral modes, verify claims against code,
render actual help, run blind task simulations and a structured critic, and treat
recurring ambiguity as a design finding rather than endlessly revising prose.

Current harness documentation shows `.agents/skills` as the broadest portable
repository location. Codex uses `.agents/skills`; Gemini CLI, GitHub Copilot,
Cursor, Windsurf, and OpenCode also recognize it. Claude Code uses
`.claude/skills` and documents support for symlinked skill directories. The
installed instance will use the common Agent Skills subset and verify the chosen
compatibility layout rather than assume it works.

## Design and Implementation

### Repository model

Use three distinct artifact roles:

| Role | Location | Behavior |
| --- | --- | --- |
| Active skill | `skills/<name>/SKILL.md` | Embedded and exposed as an MCP tool |
| Skill template | `templates/<template-id>/` | Embedded but inert; retrieved only through read-only template tools |
| Installed instance | Project agent-skill directory | Customized, project-local workflow with provenance |

The active seeder is not the template, and an installed instance never edits its
base merely because it is used. Template improvement is a separate, reviewed
upstream action.

Each template has a stable manifest containing its ID, version, purpose, entry
point, declared files, compatibility, and digests. `templates/catalog.yaml`
indexes both initial templates. Active skill metadata remains authoritative in
each active skill's frontmatter; catalog loading combines the two sources.

### Read-only catalog and retrieval tools

`skill_catalog` returns structured metadata for active skills and templates. It
supports recommendation without loading every skill or template body.

`skill_template_get` accepts a template ID and returns only the files declared by
that template's validated manifest. It rejects unknown IDs, undeclared paths,
invalid manifests, and digest mismatches. Neither tool writes files or performs
installation.

### Recommendation and seeding

`recommend-skills` inspects the project and recommends a minimal set. It classifies
each result as:

- use through MCP;
- seed and customize locally;
- do not adopt; or
- missing capability.

It explains the project evidence for each recommendation and stops before any
mutation.

`seed-skill-template` begins only after the user approves a template. It discovers
existing project conventions, asks when none exist, gathers the template-specific
project profile, retrieves the template, writes the installed instance, records
provenance, and runs structural and behavioral validation. It never overwrites an
existing real directory, symlink, or customized file without showing the proposed
merge and obtaining approval.

### Installed-instance provenance

Use `template-state.yaml` rather than a duplicated baseline directory. Record:

- a stable instance ID;
- template ID and semantic version;
- public source repository and immutable source commit;
- aggregate and per-file base digests;
- installation and last-upgrade dates;
- file merge strategies; and
- declared local customizations and their rationale.

An upgrade retrieves the old base at the recorded commit, verifies the stored
digests, retrieves the new base, and compares old base, current instance, and new
base. It preserves local-only files and asks for human review of conflicts. If the
old base or its expected hash is unavailable, the upgrade stops rather than
guessing. An optional vendored baseline may be used for offline projects, but is
not the default.

Surveys are explicit, authorized operations. They classify instances as unchanged,
intentionally customized, undeclared drift, behind the current template, invalid
provenance, or no longer comparable. Repeated customizations become candidates for
base changes, never automatic upstream edits.

### Cross-harness installation

Use `.agents/skills` as the default canonical repository location because it has
the broadest current harness support. Preserve an existing project convention
instead of relocating it.

For this repository, start with `.agents/skills` as canonical. When
`.claude/skills` is absent and all project skills are portable, prefer a relative
directory symlink to `.agents/skills`; otherwise preserve the directory and add
per-skill compatibility links. Test discovery in both Codex and Claude Code. Keep
installed skills within the common Agent Skills subset: `name`, `description`,
Markdown instructions, and relative supporting-file references.

### Feature-documentation template

Each installed feature-documentation skill records a project-specific audience
charter. The charter ranks four audiences independently: human users, agent users,
human developers, and coding agents. It also records project type, expected prior
knowledge, authoritative sources, synchronized documentation surfaces, validation
commands, diagram tooling, and risk-based review gates.

The workflow is:

1. Discover conventions and recheck the audience charter.
2. Map authoritative code, parser, schema, test, configuration, and design sources.
3. Enumerate distinct modes, lifecycle states, and failure paths.
4. Freeze representative user and developer tasks with expected outcomes.
5. Write or update the relevant surfaces using shared terminology.
6. Run deterministic verification against rendered output and authoritative code.
7. Run blind task simulations and structured clarity critics.
8. Obtain human review for human-oriented or otherwise high-risk changes.
9. Revise specific findings for at most three rounds.
10. Report evidence and unresolved interface or architecture problems.

README guidance emphasizes orientation, verified quick starts, task paths,
contribution paths, recovery guidance, and visual models where relationships are
complex. Code documentation emphasizes contracts, invariants, ownership, side
effects, failure behavior, lifecycle, and safe extension points rather than
narrating obvious implementation. CLI help emphasizes canonical invocations,
value formats, defaults, modes, alternatives, examples, deprecations, and the
actual rendered help seen by users and agents.

DOT is the authoritative diagram source. Commit a rendered SVG beside it, use
clusters for meaningful nesting, validate rendering, and provide a concise textual
equivalent such as a component table or numbered workflow. Require human review of
architecture diagrams, workflow diagrams, visual hierarchy, navigation, major
README restructuring, onboarding narratives, and other features whose success is
primarily perceptual or subjective.

### Engineering-journal template

Adapt the existing generic engineering-journal method into an inert project-local
template. Preserve its four-state lifecycle, evidence requirements, intent-first
and single-PR modes, reconciliation rules, and advisory brief inputs. Add a project
profile for repository-specific paths, frontmatter, index shape, derived documents,
validation commands, and policies such as whether a project-wide backlog exists.

The existing active `engineering-journal` skill remains available through MCP. It
and the inert template share core invariants but need not be byte-identical because
their roles differ.

### README dogfood case

This repository's primary audience is internal humans who want to install useful
skills and turn development experience into reusable practice. The README will
lead with why the repository exists, then provide two prominent paths:

- install, ask which skills fit a project, and use the bundle; and
- contribute a new skill or improve an existing skill or template.

The README will make clear that installing the binary exposes all active skills,
while templates are selectively seeded into projects. It will feature a
DOT-authored feedback-loop diagram from development experience through validation,
organizational reuse, observed customization, and base improvement. Build, raw MCP,
and maintainer reference material remains available later in the document.

### Distribution direction

The current supported installation is a source build. The first packaging target
is Apple Silicon macOS, followed by occasional Linux use; Intel macOS is deferred
until demand is demonstrated. Release automation should precede a private or
organizational Homebrew tap so users receive checksummed binaries or bottles rather
than compiling Rust during every install. Coding-agent plugin platforms remain
deferred until the skill and template lifecycle is stable.

### Error handling and stop conditions

- Reject malformed or duplicate catalog IDs and paths outside a template manifest.
- Stop recommendation when project evidence is insufficient for a defensible match.
- Stop seeding when the destination convention conflicts or an existing instance
  cannot be safely distinguished from unrelated content.
- Stop upgrades when provenance, the historical base, or digest verification fails.
- Stop documentation work when frozen ground truth proves wrong; report a product
  or interface issue rather than rewriting expectations to match output.
- After three failed documentation revision rounds, report the recurring ambiguity
  as a design smell.
- Never claim human usability without the required human review.

### Verification strategy

Follow test-driven development for code and skill authoring. Add failing Rust
contract tests before catalog or retrieval implementation. Establish baseline
skill behavior before writing the new active skills and templates, then run the
same scenarios with the skills present and close observed gaps.

Verification includes:

- manifest, catalog, path-boundary, digest, duplicate-ID, and serialization tests;
- MCP tool schema and call-result tests;
- active-skill and template trigger evaluations;
- recommendation cases for use, seed, skip, overlap, and missing coverage;
- seeding cases for new, existing, conflicting, dual-harness, and upgrade layouts;
- blind user and developer task simulations for feature documentation;
- DOT parse/render and SVG freshness checks;
- README link, command, source-build, and comprehension checks;
- `cargo fmt`, Clippy, unit tests, release build, and MCP smoke tests; and
- explicit human review of the README and its rendered visual features.

## Outcome

Open. Design decisions are approved in discussion; implementation and verification
have not started.

## Derived Documents

- [Roadmap](../roadmap.md)
- [Assumptions and limitations](../assumptions-and-limitations.md)
- [Backlog](../backlog.md)

These documents are canonical for their named concerns. This journal entry records
the reasoning and evidence and will reconcile their material changes at closure.

## Deferred or Reopen Items

- Exact Linux release architectures remain undecided.
- Homebrew release engineering begins after the initial template system ships.
- Cross-project surveys require an explicit authorization and repository set.
- Plugin marketplace distribution requires demonstrated demand and a stable local
  lifecycle.
- A broader template registry or additional retrieval API requires evidence from
  more than the initial two templates.
