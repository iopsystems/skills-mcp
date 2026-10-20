---
name: architecture-diagram
description: Use when creating or materially revising system architecture diagrams — a build-time structure chart (units, layers, product composition), a weighted source-size view (LOC alongside source structure), or runtime charts (thread model, life-of-a-request flow). Triggers include "architecture diagram", "threading diagram", "dataflow chart", "life of a request", "diagram the system". Symptoms include a hand-maintained diagram drifting from the code, a chart that cannot say whether a thread or queue still exists, or structure and runtime claims tangled in one unreadable picture.
---

# Architecture Diagrams: Structure, Source Size, and Runtime

## Purpose

A system's architecture uses separate views for structure, source footprint,
and runtime behavior, because their claims have different ground truths:

- The **build-time chart** claims what the code *is*: its units (crates,
  packages, modules), their layering, and what each shipped artifact is
  composed of. Ground truth is the build manifest, queried through the
  build system's own interface.
- The **weighted source view** shows where the source lives and how much of
  it each component owns. Ground truth is a counted source inventory mapped
  to those components, not a dependency closure.
- The **runtime charts** claim what the program *does*: the threads it
  spawns, the queues connecting them, the path one request travels. Ground
  truth is the source, held to the chart by explicit assertions.

Keep the views distinct: dependency, source-containment, and runtime arrows
make different claims. Bind them visually through shared component names
and colors (see "Chips bridge the halves"). Add the weighted companion when
source size is part of the request; it does not replace the dependency chart.

## What this is, and what it is not yet

This is the `dataflow-diagram` skill's principles carried into a domain it
was not written for. The principles survived the trip; the duo-specific
conventions are younger — derived against a multi-binary cache framework,
then carried to a second system, an io_uring runtime library, whose reader
supplied the layout rules below — so treat this skill as **beta**. Adopt the
conventions wholesale, and when one fights your system, the override with its
stated reason is the most valuable thing the effort can produce: record it in
the charter and bring it back here. Where the two skills deliberately disagree
(arrows dropped from the build chart, provenance demoted from a channel,
geometry emitted directly rather than through a layout engine), the reason
appears in the section that overrides it. For a pipeline or DAG chart of
one running program, use `dataflow-diagram` directly. The weighted source
companion adds quantitative conventions; its scenario corpus is guidance
for evaluation, not evidence that a rendered view is readable.

The skill itself is a complete set of working defaults, usable with no
configuration (including single-use charts, below). The
`architecture-diagram-skill` template adds a charter: a *delta* recording
the bindings no default can supply, plus any deviation with its reason. A
convention absent from the charter means the default applies.

## Shared principles (all views)

**Derive, never draw.** Nodes, edges, labels, and composition come from the
program's own structures at generation time. A hand-maintained diagram is a
second source of truth: right the day it is drawn, wrong on the first
refactor, silent about the divergence. A generated chart is regenerated; a
drawn one is renegotiated.

**Fail loudly on the unclassified.** An element no classification table
covers must abort the run — silent omission still renders a
complete-looking chart. So must a table entry naming an element the program
no longer has. Curated display orders are legitimate, but validate them
against derived facts at run time so curation cannot drift.

**Assert absences too.** Some of a chart's strongest claims are negative:
this variant spawns *no* signal-handler thread. Encode them as absence
checks that abort on drift, so the missing box stays honestly missing. A
failing negative claim is often a finding about the system, not the chart.

**Freshness is enforced, not hoped.** One command regenerates every chart;
CI fails on any diff against the committed output. Every source assertion
becomes a standing CI check: a refactor that changes the thread model fails
the build instead of quietly invalidating a picture.

**One visual language across the set.** A single shared module owns the
palette, type scale, and drawing primitives for all charts — cross-chart
consistency is what makes them read as one system. Intentional geometry is
itself a message: uniform sizes and computed grids show structure;
proportional areas in the weighted view show source size.

**Chips bridge the halves.** Runtime containers carry small chips naming
the build-time modules that execute there, in the build chart's layer
colors, so a reader can follow one module from the layer diagram to the
thread that runs it. Differences between variants should read as chip
migration.

**Verify the rendering, not the source.** Bounds-check every element into
its panel (exclude intentional margin elements explicitly — don't loosen
the check). After refactoring the generator, byte-compare the output;
"looks the same" is not a check. Renderers silently ignore attributes they
don't support: confirm geometry actually moved. A bounds check proves
containment and nothing beyond it — see "Bounds are not balance".

**Human review gates every chart.** Layout quality and whether the chart
communicates are perceptual judgments no assertion covers. Every new chart
and visual change goes through the project's designated reviewer; approval
of an earlier revision does not cover a later one. When the raster preview a
review depends on is unavailable, say so and name what stood in for it —
markup validity, deterministic regeneration compared by hash, bounds and
collision checks, a reviewer reading the committed artifact. A gate skipped
without that record reads afterwards as a gate passed.

Keep a compact check record in the existing journal, test output, or beside
the artifact. Identify the source revision and any dirty inputs, the rendered
path and content hash, and the regeneration command. Retain a source snapshot
or diff when the inputs are uncommitted. Record source/claim checks, rendering
checks, actual visual inspection, and required human approval separately:
passed, failed, or not run, with the reason for any skipped or inapplicable
check. Name the renderer or preview inspected and the reviewer for approval.
Results apply to that source/artifact pair; after an edit, rerun affected checks
and obtain any required review for the new artifact. In the handoff, link the
diagram and state remaining gaps; leave the check detail in the record.

## The build-time chart

- **Blocks, not units.** Unit-level arrows at whole-system scale are
  spaghetti regardless of layout tuning. Group units into a handful of
  blocks and make the block the visual element.
- **Arrows should not drive the layout — and may not be needed.** Stacked
  full-width bands encode layering by position, and *nesting* (each product
  box contains bars for its protocol/storage/core choices) encodes
  composition, eliminating arrows entirely. Any block-level arrows that
  remain are aggregates — never per-unit.
- **Position claims are verified claims.** Arrange each block so row-major
  reading order is a topological order of the real dependency subgraph, and
  assert that at generation time. Weaker than "sits above its
  dependencies", but checkable, and it survives any grid aspect.
- **Square-ish grids beat truthful-but-tall stacks.** Prefer near-square
  grids (6 units as 3×2, never 5+1) under the row-major-topological
  constraint.
- **Block-level cycles are real.** A strict stack that hides the back edge
  claims an acyclicity the code does not have. Draw both directions, or
  band the mutually-dependent blocks at the same level.
- **Provenance is not a category.** Units place by *role*: an external
  storage engine sits in the storage band, external TLS in the foundation.
  Externals enter through a curated whitelist with a validity check, marked
  quietly (italic, a registry link) — not with a loud channel.
- **Composition is derived, including what manifests can't record.** A
  product's bars come from direct dependencies plus targeted source greps
  for wiring choices (which storage engine it instantiates). Fail loudly
  when a product links a facade without wiring a concrete choice.
- **Distinctiveness = direct dependencies** minus those all products share.
  Transitive closures drag shared facades' dependencies into everything and
  erase the signature.

## Weighted source at a glance

`weighted-source-at-a-glance` is the source-footprint companion to the
build-time chart. Use it when the request includes code size, source balance,
large modules, or size alongside architecture. Do not force it into an
unrelated runtime-only diagram request.

Draw a **hierarchical treemap**: nesting shows source containment, and a
leaf's area is proportional to its measured LOC. Reuse component names and
role colors from the structure chart. Place the views alongside each other
or link them by those identifiers. Keep dependency arrows and topological
position in the structure chart; treemap adjacency claims neither dependency
nor execution order. This view overrides the uniform-box geometry used by
the other charts, while keeping their palette and typography.

### Derive an auditable source inventory

- Start from the same revision and component inventory as the structure
  chart. Enumerate repository-owned source through the project's build and
  source interfaces, including relevant files outside the obvious `src/`
  directory. Declare whether the scope is all checked-in source or one
  target/feature configuration. Never imply that an all-source inventory
  measures the code compiled into a particular binary.
- Default to **physical LOC**: every line in the selected source files,
  including comments and blank lines. State that definition in the caption.
  An existing language-aware counter can supply code-only LOC instead when
  requested; record its version and options and use the same definition
  throughout. Do not label a newline count as code-only LOC. Handle a final
  unterminated line consistently. LOC measures source footprint, not runtime
  cost, complexity, quality, or the need for a refactor.
- Default to repository-owned handwritten source, including tests, and
  measure the verification share of every component: tests, benches,
  examples, fuzz targets, and proof harnesses. A tile that grows when a
  proof harness lands misreports the component, and the share grows as
  verification does. Measure it, never guess it from filenames: files under
  the target directories classify by directory; a module declared under
  `#[cfg(test)]` or `#[cfg(kani)]` is verification with everything beneath
  it; inside any other file, the extent of each item carrying such an
  attribute, `#[test]`, or `#[kani::proof]` comes from a parser or a brace
  scanner that honors comments and literals, with `cfg(not(test))` as
  production. State the method in the caption. Unclassifiable lines count
  as production, so the error runs toward undercounting. Exclude vendored
  dependencies, generated output, build artifacts, and assets from the
  weighted total; report their exclusions. If generated or vendored source
  is requested, give it a distinct labeled group and subtotal.
- Assign each counted file to exactly one source owner. Count a library once
  even if several products depend on it. Validate curated ownership against
  the inventory: fail on an unmapped file, duplicate ownership, or stale
  entry. Use logical crate/module containment when source tooling resolves
  it; otherwise label the hierarchy as directories/files. Filesystem nesting
  alone does not establish Rust module ownership.
- Keep a parent's own source as a leaf beside its children. A Rust module's
  `mod.rs` or facade file counts once; the parent label reports the sum of
  descendant leaves, not an additional weighted tile. For shared files,
  feature-gated modules, macros, or `include!`, declare the ownership rule
  rather than counting dependency reachability or expanded code repeatedly.

### Make area a truthful quantitative channel

For each leaf, `area = k * LOC` using one scale within the view. Assert the
ratio on the allocated data rectangles before pixel rounding, with an
explicit rounding tolerance for rendered geometry, and assert it again on
the rectangles as serialized: a gutter subtracted after the check passes
shrinks every tile by a fixed border, and small tiles most. Partition the
available data area by weight; leave headers, borders, gutters, and legend
outside the quantitative area. Parent totals equal the sum of their children, and
the root equals the counted inventory total.

Do not enlarge small tiles to fit labels, apply a minimum area, use log
scaling, or size both width and height directly by LOC (which squares the
ratio). Place labels outside tiny tiles or group them into an explicit
`Other` tile with their exact summed weight and a member list. Zero-LOC
components get a listed zero, not an invented area. An empty inventory gets
an empty-state explanation rather than a fabricated treemap.

Draw the verification share as texture inside the tile, not as a separate
tile: a hatched partition by weight, so its area is the same `k` times
verification LOC and containment is unchanged. Texture is the channel
color and area have not spent, and it reads for color-blind readers. Every
component is drawn; significance decides only whether a tile's hatch is
worth drawing. A strip narrower than a stated pixel threshold is tabled,
not drawn, and the legend names the threshold and the tiles it applies
to. A label that must span the hatch sits on a halo in the tile's own
fill.

Use deterministic ordering and tie-breaking. Every tile identifies its
component, LOC, and share of the scoped total, either directly or through a
keyed legend/table; do not rely on color or hover alone. If views compare
revisions or variants, keep the counting policy and scope comparable and
state whether area uses a shared absolute scale or each panel's own total.
Independently normalized panels show proportions, not absolute growth.

### Verify and ship the view with its evidence

Emit a machine-readable inventory and an adjacent textual equivalent with
component paths, categories, LOC, percentages, exclusions, root total, and
counting policy. Stamp the source revision and disclose working-tree changes;
retain a content digest when the chart is derived from uncommitted source.
Reconcile every parent subtotal, root total, and area ratio. Exercise unequal
weights (for example, 100 and 400 LOC must allocate areas in a 1:4 ratio),
tiny and zero weights, and a file shared by two products before trusting the
generator. Bounds and collision checks still apply, including external labels.

Use the existing regeneration command and CI freshness check for both the
source inventory and rendered view, so added, removed, renamed, or resized
files invalidate stale output. Review the rendered view under the chart's
human-review gate; numerical correctness alone does not prove readability.

## The runtime charts

Typically two: a **thread model** (who runs, how connected) and a **request
flow** (what happens to one request, in order). Ground truth for both is
source assertions — positive and negative pattern claims against spawn
sites, queue wiring, signal registration, ports, and event-loop verbs,
checked at generation time: the runtime analog of querying the build
manifest.

Thread model:

- **Literal runtime names, monospace.** Thread boxes carry the exact names
  the code registers, grep-asserted against the spawn sites, so an operator
  can match a hot thread in `top -H` to the chart. Monospace means "this
  string is literal"; reserve it for that.
- **Thread fill is reserved.** Plain threads stay unfilled so the chips
  carry the color; fill only the genuinely unusual (non-default scheduler,
  pinning).
- **External elements are italic and dashed** — one style for everything
  outside the process. Skip the process-boundary frame if externals and
  internals share a column; a rectangle that falsely encloses an external
  is a false claim.
- **Edge weight = boundary crossing.** Process-boundary bytes draw heavier
  than internal queue traffic. Label edges by what actually travels — wire
  bytes vs. parsed objects, named by the code's own types.
- **Queues are connective glyphs, not foci** — a small segmented glyph that
  doesn't compete with the threads it connects.
- **One panel per variant, stacked vertically**, margin-annotated with the
  binaries it covers. Same-role elements align across panels so variants
  compare by scanning.

Request flow:

- **Swimlanes are threads; a stage sits in the lane that runs it.** Thread
  hops read as geometry — the dip into a storage thread, the zig-zag
  through a proxy — which is the cost the chart exists to show.
- **Number the stages** with drawn badges — a circle plus a digit; Unicode
  circled digits die in font fallback. Execution order is a real fact here,
  so numbering is honest.
- **Stage verbs are the code's verbs** (`receive`, `execute`, `send`,
  `flush`), noting what each bundles. Invented vocabulary drifts; the
  code's vocabulary is asserted. The exception is a verb the code spends on
  two different things: `poll` for readiness and `poll` for task execution,
  set side by side in one label, name neither. Use the lifecycle word —
  `schedule` — and leave the mechanism to the prose, unless the mechanism is
  the distinction the chart exists to draw. This is `technical-prose`'s
  one-name-one-thing rule, and a label is where it hides best.
- **Variants that differ in vocabulary get their own panels.** Interleaving
  two backends' operation names inside one flow makes the stages they
  genuinely share look like shared mechanism. Draw a panel per variant on an
  obvious comparison axis — top and bottom — and repeat the portable stages
  in each. Repetition costs a reader less than one node carrying two
  vocabularies, and what the variants really share belongs in the prose
  beside the chart.
- **Uniform stage pitch across panels**, lane switch or not, so panels
  column-align and the only visual difference between variants is the real
  one.
- **Scope deliberately.** Data plane and control plane rarely belong on one
  request-flow chart; pick one and let the thread model carry the other.

## A default visual language

Adopt wholesale or replace wholesale in the charter — never mix. Derived
against one system (see the beta note), but internally consistent, and
starting from it beats assembling from scratch.

- **Palette** (pastel — large filled areas read at length):
  interface/protocol `#FBB4AE`, storage/state `#B3CDE3`, runtime/core
  `#CCEBC5`, foundation `#F2F2F2`, externals white.
- **Type scale**: 14 chips, legends, and edge labels; 16 sub-labels;
  17 element labels; 20 panel titles.
- **Style channels**: monospace = literal runtime strings; italic + dashed
  = external; underline = hyperlink to an external unit's registry page
  (draw the underline as a line — text-decoration is unreliable in
  rasterizers).
- **Edges**: heavy (2.4) crossing the process boundary, thin (1.4)
  internal; orthogonal only; labels above arrows.
- **Panels**: right-margin annotations vertically centered; the runtime-chart
  rule above sets one per variant, stacked.

## Bounds are not balance

Every element inside its panel is the weakest layout claim available, and it
is the one a bounds check makes. A two-line label centered on its first
baseline, three arrows attached to a box at points chosen one at a time, a
child crossing its parent's border, a pair of arrows whose labels both sit
above their lines — each of those passes bounds, and each is what a reader
notices first. Emitting geometry directly is what buys the rigid alignment
this skill asks for; it also means the generator inherits every placement
decision a layout engine would have made, so each one below becomes a check
standing beside the bounds check.

**A label is placed against a resolved shape, not a nominal coordinate.**
Anchor explicitly, center on the box or the segment the label names, and hold
a minimum padding from borders, arrow paths, and neighboring text. Then
collision-check text bounds against shapes and connectors: text that clears
the panel can still cross the connector running past it.

**A multiline label is one layout object.** Measure the line group, center
the group in the shape, then place each baseline within it. Centered line by
line, a two-line block sits visibly high while the arithmetic reports it
centered.

**Connectors on one shape are placed as a group.** One belongs at the
midpoint of its edge; two take equal offsets either side of it; more
distribute evenly and symmetrically, holding corner padding and enough
separation for arrowheads and labels. Chosen one at a time, the first arrow
keeps the midpoint and the rest are pushed aside, and the asymmetry reads as
though it meant something.

**Route against resolved bounds, not the column a shape nominally sits in.**
Siblings that differ in width let a connector that was correct for the narrow
box penetrate the wide one. Give equivalent steps one geometry, and test each
endpoint against the border it actually lands on.

**Label the connector group, not the connector.** For a pair, place the
labels outward — the upper label above the upper line, the lower below the
lower. On the same side, the inner label sits nearer the arrow it does not
describe. Larger groups need collision-aware placement rather than a rule.

**Containment is checked at every semantic layer.** Children stay inside the
parent's content area, siblings do not collide, and a parent's header owns a
band outside its children's visual block. A hierarchy whose parent and child
labels share one text rhythm reads flat, however legal each string is on its
own.

**Label fit is measured, not estimated.** Check a label against its box using
the rendered text's width — the font's metrics, or a measurement from the
renderer — never a character count times an average glyph. A label one wide
glyph too long passes the estimate and overflows for the reader, and the
bounds check, which sees the box and not the text, passes it too.

**A visual fix that is not a test recurs.** Every rule above is a defect a
reader reported, and every one would have returned at the next layout change
had the fix been coordinates. Convert the report into a generator assertion —
the line group is centered, equivalent boxes share a width, endpoints land on
borders, paired labels face outward, children stay in their lane — so the fix
survives the edit that would otherwise undo it.

## Diagnose and repair a specific failure

Report a failed check with a stable rule name, affected component/edge IDs,
observed geometry or claim, expected constraint, and the artifact checked.
Use the existing generator's diagnostics; no new reporting framework is needed.

Repair the implicated neighborhood in the generator. Preserve containment,
runtime relationships, conditions, and labels' meaning. In weighted views,
preserve inventory totals and LOC-to-area ratios; move a label rather than
inflate its tile. Do not drop components or weaken checks to make a layout pass.
Check the rendered result and rerun affected source and geometry assertions.
Fewer diagnostics alone does not establish that the repair worked.

Keep the last passing artifact while testing a candidate separately. If the
same issue survives two focused repairs, inspect its measurements and source
contract before one further attempt. If it persists, stop and report the
unresolved constraint. Retain the previous artifact as a labeled snapshot,
not evidence that the current source passes. Existing review gates still apply.

## Single-use charts

Single use is the no-charter case, and the skill is complete without one. A
chart for a talk, or for a repository you cannot add files to, keeps the
core — derive from the program's real structures, classify through
fail-loud tables, use the default visual language — and keeps the generator
script with the artifact, wherever the work lives. What it loses is
freshness: nothing will catch drift, so stamp the chart with the commit it
was derived from and present it as a dated snapshot. Skip the charter and the
review gate. A chart worth keeping makes the script the seed of the installed
generator.

When the program is not in any repository the generator can read — a
reference page about a third-party system, a dependency's internals —
derivation itself is unavailable, and the chart rests on a model derived by
reading. `dataflow-diagram` states what that model owes the reader: a
transcription that names the source path and symbol behind every element,
a provenance table rendered beside the charts, and an evidence baseline
that says what revision was read and that freshness is not enforced. The
classification tables and geometry checks still run; the caption says the
chart was derived by reading, not generated from source.

## Workflow

1. Establish bindings: recheck the charter if one is installed (defaults
   apply where it is silent); otherwise discover the generator and reviewer
   from the repository, or run single-use on pure defaults.
2. Map the request to a chart in the inventory, or decide it is genuinely
   new.
3. Write the ground-truth extraction and claims before any geometry:
   manifest queries for structure, a counted and ownership-checked inventory
   for source size, source assertions for runtime behavior.
4. Build or extend the generator in the project's native toolchain — no new
   contributor dependency — sharing the one visual-language module and one
   regeneration command.
5. Render-verify: bounds and claims pass; weighted views reconcile LOC
   totals and area ratios; refactors byte-compare clean when no visual change
   was intended.
6. Confirm the CI freshness check covers the new or changed chart, and keep
   a textual equivalent adjacent to every embedded chart.
7. Obtain human review; re-review after any subsequent change, however
   small.
8. Record every convention that fought your domain — with the reason — in
   the charter. That record is how this skill improves.

## Known dead ends

Recorded so they are not re-attempted:

- Unit-level arrows at system scale, with any amount of layout-engine
  tuning (clustering, edge concentration, rank constraints): spaghetti.
- General-purpose layout engines for the band/nesting design: rigid
  alignment is the message, and layout engines will not deliver it — emit
  geometry directly.
- Transitive-closure intersection as the "shared skeleton": one shared
  facade drags everything into every product's closure.
- Unicode circled digits for stage numbering: font fallback.
- Text-decoration underlines for links: some SVG rasterizers ignore them —
  draw the line.
- Per-product dependency minicharts: superseded by composition nesting in
  the top chart plus a textual table. Prove a companion chart adds claims
  the main chart cannot carry before building it.

## Prose in the chart

Prose in and around a chart — node labels, the key, the caption, the charter —
follows `technical-prose`, including its default of American spelling. A
convention the user or the project states overrides that default; a mixture of
both overrides nothing and reads as several authors.

## Draw the state, do not bury it

A stateful component folded into the label of the thing that reads or writes it
is invisible: a ring named inside a process box, a queue mentioned in an edge
label, a store that exists only as a verb. Draw it as its own node.

In a structure chart this decides what a unit *is*. A process that owns a
buffer and a process that borrows one look identical until the buffer is drawn
and the ownership edge has somewhere to land. In a runtime chart it decides
where the interesting failures live: a thread cannot lose a record, but the
queue between two threads can, and a chart with no queue has nowhere to put
the loss.

The test: **if a box names both a unit and something that outlives it, split
it.**

## Red flags

- A weighted tile sized by guesswork, minimum label size, or dependency count.
- Shared source counted once per product, or parent LOC counted again as a leaf.
- A LOC caption without a counting policy, scope, exclusions, and source revision.
- A treemap used to imply dependency order, complexity, or runtime cost.

- A stateful component named inside another node's label rather than drawn.
- A node or edge list maintained by hand beside the code it describes.
- A generator that skips an element it cannot classify.
- A curated order or whitelist with no validity check against derived facts.
- A runtime chart with no source assertions — or none that assert absence.
- Diagrams regenerable only on one contributor's machine.
- Structure and runtime claims on one chart, or an edge whose kind of claim
  a reader cannot determine.
- A thread box whose name is not the literal runtime name.
- Chips or colors that disagree between the build and runtime charts.
- Variant panels whose spacing differs for reasons other than a real
  difference.
- A bounds check loosened to admit an overflowing label.
- Label fit estimated from a character count rather than measured text.
- A chart of a program outside the repository presented as generated from
  it, or shipped without a provenance table and an evidence baseline.
- A label centered by its first baseline rather than by its line group.
- Arrows attached to one shape at points chosen one at a time.
- A connector routed to a column coordinate rather than a resolved border.
- Paired labels on the same side of two parallel arrows.
- A child shape crossing its parent's border, or a parent's label sharing the
  text rhythm of its children.
- A stage label carrying a verb the code spends on two different things.
- A reported visual defect fixed in coordinates rather than in an assertion.
- A review gate skipped for an unavailable preview, with no record of what
  stood in for it.
- A generator refactor merged without byte-comparing its output.
- A chart shipped without human review, or re-shipped under a stale
  approval.
- An override applied without recording the reason.
