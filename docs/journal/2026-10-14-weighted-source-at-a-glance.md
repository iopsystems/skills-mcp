---
status: open
opened: 2026-10-14
updated: 2026-10-16
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

## Real repository rendering, 2026-10-16

The conventions were run against Pelikan at `46b98c9`, a 23-crate workspace
whose structure chart already follows this skill. The generator, inventory,
textual equivalent, and rendered view are in
`docs/evals/renderings/weighted-source/pelikan/`. It is a standalone
script rather than an addition to `cargo xtask diagrams`, because this is a
trial of the conventions, not a change to Pelikan.

What the conventions produced without trouble: a 31,551-line inventory from
`cargo metadata` and `git ls-files`, each file owned by exactly one crate,
classified by the structure chart's own layer table and failing on drift;
physical LOC stated in the caption; one area scale across nested layers, with
headers and gutters outside the quantitative area and the ratio asserted
before rounding; a src/tests/benches breakdown by target directory, with
inline test modules declared as unmeasured; the revision, dirty state,
exclusions, and an inventory digest stamped.

What the first rendering caught that the assertions had not:

- The footer caption ran past the canvas. The generator bounds-checked every
  rectangle and no text. The skill says bounds checks include external
  labels; a generator that reads that as "rectangles" will pass its own
  checks and fail the raster. Text now has an estimated-width check.
- A 71-line crate rendered as a sliver 345 by 5 pixels, and the numeral
  placed in it collided with the tile above. The skill says to label tiny
  tiles outside; it does not say where, and at a panel edge there is
  nowhere. The generator now reserves a callout strip under any layer whose
  tiles are too small for a numeral, and draws a leader to it.
- A fixed minimum label width let `protocol-admin` run to both edges of its
  tile. The in-place decision now uses the same width estimate as the check.

All three were visible in the raster and invisible to every numeric check,
which is the case the skill's human-review gate makes. The rendered view is
posted on skills-mcp#67 for that review; this entry does not record it as
passed.

## Second repository, 2026-10-16

cache-rs at `5cf04dd`, five crates, 27,363 physical LOC, rendered as checked
out: branch `feature/ziplist-chain`, two tracked test files modified, both
disclosed in the caption and the inventory. The generator now takes its
layer table and palette from a bindings file, one per project, under
`docs/evals/renderings/weighted-source/`; the Pelikan geometry was
byte-compared across the refactor and only the caption lines moved.

cache-rs has no structure chart, so there was no layer table or palette to
reuse. The bindings follow the project's own description: engines depend on
`keyvalue`, `ziplist` is a byte-format codec, `keyvalue` and `datatier` are
foundation. The Engine layer is validated against the dependency graph at
generation time, which is the skill's rule for curated tables. The colors
are the shared palette's tints by role, stated in the bindings as a choice.

Two more things the raster caught: a project footer that ran off the canvas
once a longer note was bound, and the keyed legend's third row sitting
eighteen pixels above the footer. The text check now fails on any two texts
whose estimated boxes overlap, and the footer note has its own line.

The Pelikan checkout turned out to be on a feature branch too. The first
record stamped the revision and said nothing about the branch; the caption
now names any branch other than `main`.

## Verification source as texture, 2026-10-16

The crate tiles folded tests into source footprint. The maintainer's
objection: with proof harnesses coming, a tile that grows when a Kani
harness lands is a tile that misreports the engine. Two designs were
weighed. A separate verification group keeps the weighted total honest and
breaks containment, since tests belong to the crate they test. Texture
keeps containment and spends the one channel the view had not used: each
tile splits into a solid part and a hatched part, partitioned by weight, so
the hatched area is the same `k` times verification LOC. The maintainer
chose texture, and a second rule with it: every component is drawn, and
significance decides only whether a tile's hatch is worth drawing. A strip
under four pixels is tabled, not drawn, and the legend says so.

The number under the hatch is measured, not guessed. The skill's warning
against regexes over filenames is right and was being read as a reason to
leave inline test modules in production. A brace scanner (`rustscan.py`)
that honors comments, string, raw, byte and char literals, and lifetimes
finds the extent of any item carrying `#[cfg(test)]`, `#[cfg(kani)]`,
`#[test]`, or `#[kani::proof]`; `cfg(not(test))` is production; a
`mod x;` declared under such an attribute makes the file and everything
beneath it verification; files under `tests/`, `benches/`, `examples/`,
and `fuzz/` are classified by directory. The inventory now lists every file
with its classification so the figure can be audited.

What that measured: Pelikan is 18.1% verification; cache-rs is 47.8%,
with `segcache` at 7,477 of 15,487 lines and `ziplist` at 4,990 of 8,602.
The crate-level view had called all of that production.

The raster caught one more thing: a label that spans the hatched strip is
hard to read on the hatching. Labels that cannot sit in the solid part now
sit on a halo in the tile's own fill.

## Deferred or Reopen Items

- Independent behavioral evaluation of the scenario corpus remains unrun.
- The skill text still says to count handwritten source "including tests"
  with no way to tell them apart. The texture rule and the parse method
  belong in it; this trial establishes both work and does not yet move them.
- Both views are normalized to their own total. Comparing them shows
  proportions, not absolute size: `segcache` at 56.6% of cache-rs is 15,487
  lines, `protocol-resp` at 26.0% of Pelikan is 8,191.
- The Pelikan view awaits its maintainer's review; a rendering that passed
  its own checks is not a rendering that reads.
- Whether a project adopting the companion should extend its existing
  regeneration command or carry a second generator is unresolved; the trial
  carried a second one and the skill's freshness rule assumes the first.
- This entry does not update an installed MCP binary.

## Skills Used

- `skill-creator` — update shared instructions and validate packaging.
- `review-guide` — prioritize judgment calls and state the behavioral test gap
  in the pull-request description.

## Review Guide Feedback

The guide's publish test is useful here because passing integration tests can
look like behavioral evidence for a skill. The PR description separates those
claims and directs attention to the LOC policy and ownership accounting. No
reviewer comprehension result has been measured yet.
