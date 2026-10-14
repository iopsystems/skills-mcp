---
status: open
opened: 2026-10-14
updated: 2026-10-14
---

# Weighted source alongside architecture

## Goal

Add `weighted-source-at-a-glance` to the architecture-diagram workflow so a
reader can see source footprint alongside code structure. The user requested
area proportional to LOC, following the Rust source-layout draft.

## Design

The new companion is a hierarchical treemap, with containment showing source
ownership and leaf area proportional to counted LOC. The dependency chart
retains layering and topology; the treemap shares its component identifiers
and colors without making dependency claims through adjacency.

Physical LOC, including comments and blank lines, is the default footprint
measure. Code-only counting is an explicit, reproducible alternative. Shared
libraries count once, a module's facade file has its own leaf, and parent
totals aggregate children without adding another weighted tile. Tests remain
in the handwritten-source scope; generated and vendored source are excluded
by default and reported, or shown as distinct groups when requested.

Small labels must not inflate areas. External labels or exact-sum groups
preserve the weights; zero-size components remain listed. Counts, ownership,
subtotals, geometry ratios, provenance, and comparison scales become checked
claims. Machine-readable inventory and a textual equivalent accompany the
view and participate in regeneration and freshness checks.

## Scope and Integration

The active skill and installable template share the new conventions. The
template's charter gains source-counting and geometry bindings, and its
manifest moves to `0.2.0` with updated file digests. Five new scenario cases
cover source weighting, shared/parent ownership, tiny and zero weights,
comparison normalization, and a runtime-only request that must not gain a
treemap. The README describes the companion without introducing a separate
active skill. Existing uncommitted Rust source-layout work is preserved.

## Verification

Validation passed:

- Skill creator frontmatter validation on both active and template skills.
- All 151 repository tests, including active/template body equivalence,
  manifest completeness/digests, public catalog integration, and citations.
- `cargo fmt --all -- --check` and Clippy with warnings denied.
- Debug and release builds with the lockfile, diagram freshness, and MCP smoke.
- Direct MCP retrieval of the active architecture instructions and template
  bundle: both contain the weighted source guidance, and the template reports
  version `0.2.0`.
- JSON parsing and unique scenario names: 12 active scenarios and 11 template
  scenarios, preserving their previous cases and adding five to each.
- `git diff --check`.

The first full test run caught historical source-line citations shifted by the
new section. The repository's `CITATIONS_FIX=1` test mode refreshed them, and
all tests passed afterward. Existing rendered diagrams are unchanged.

The scenario corpus specifies expected agent behavior; it is not an independent
execution of those scenarios. No diagram generator or rendered architecture
artifact is produced by this skill edit. Prepared for pull-request review at
the user's request; behavioral evaluation remains open.

## Deferred or Reopen Items

Run independent behavioral evaluation and a real repository rendering before
claiming the quantitative conventions improve comprehension. A generated
chart must still pass its project's human-review gate. This local edit does
not approve a future chart or update an installed MCP binary.

## Skills Used

- `skill-creator` — update shared instructions and validate packaging.
- `review-guide` — prioritize judgment calls and state the behavioral test gap
  in the pull-request description.

## Review Guide Feedback

The guide's publish test is useful here because passing integration tests can
look like behavioral evidence for a skill. The PR description separates those
claims and directs attention to the LOC policy and ownership accounting. No
reviewer comprehension result has been measured yet.
