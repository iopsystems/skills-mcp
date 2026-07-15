# Backlog

This backlog owns actionable work. The [roadmap](roadmap.md) owns direction, and
[assumptions and limitations](assumptions-and-limitations.md) owns design
boundaries.

## Now: Initial template system

- [x] Add failing Rust contract tests for template manifest loading, catalog
  combination, digest validation, duplicate IDs, and path-boundary enforcement.
- [x] Add failing MCP contract tests for `skill_catalog` and
  `skill_template_get`.
- [x] Define `templates/catalog.yaml` and a versioned `template.yaml` schema.
- [x] Create and validate the engineering-journal skill template.
- [x] Establish baseline scenarios, then create and validate the
  feature-documentation skill template.
- [x] Establish baseline scenarios, then create and validate `recommend-skills`.
- [x] Establish baseline scenarios, then create and validate
  `seed-skill-template`.
- [x] Implement the two read-only template tools and keep mutation in the seeder
  workflow.
- [x] Install a provenance-tracked feature-documentation instance under
  `.agents/skills` in this repository.
- [ ] Verify live Codex and Claude Code discovery of one canonical installed
  instance; current attempts record the approved per-skill fallback but no
  successful discovery in either available harness.
- [x] Use the installed instance to redesign the repository README.
- [x] Author the README workflow diagram in DOT, commit its SVG, and provide a
  textual counterpart.
- [x] Run deterministic README, diagram, CLI, and link checks.
- [x] Run isolated user and developer comprehension tests and structured critics.
- [x] Obtain human approval for the README structure, narrative, and rendered
  diagram.
- [x] Run formatting, linting, unit tests, release build, and MCP list/call smoke
  tests.
- [ ] Close the engineering journal entry with implementation and verification
  evidence and reconcile this backlog.

## Next: Distribution

- [ ] Select the initial Linux architecture and runtime compatibility target.
- [ ] Add Apple Silicon macOS and selected Linux release CI.
- [ ] Publish versioned, checksummed release artifacts.
- [ ] Create an organizational Homebrew tap and Apple Silicon bottle.
- [ ] Test install, upgrade, uninstall, and MCP startup from the packaged binary.
- [ ] Update the README to lead with packaged installation when it is available.

## Later: Adoption and evolution

- [ ] Define an explicit repository-list input and read-only report format for an
  authorized installed-instance survey.
- [ ] Prototype comparison of recorded base, customized instance, and current base.
- [ ] Classify unchanged instances, declared customizations, undeclared drift,
  outdated templates, invalid provenance, and unavailable historical bases.
- [ ] Define the human review process for promoting recurring customizations into a
  base template.
- [ ] Test three-way upgrades across multiple real project customizations.
- [ ] Evaluate whether more templates justify catalog search, version selection, or
  a broader template registry API.
- [ ] Evaluate coding-agent plugin and marketplace distribution only after local
  adoption and upgrades are stable.
