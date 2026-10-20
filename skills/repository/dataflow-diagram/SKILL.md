---
name: dataflow-diagram
description: |
  Draw a dataflow or pipeline diagram for a program that already exists in code — a DAG, a processing chain, a stream topology, a service graph. Use when asked to "diagram the pipeline", "show the dataflow", "chart the DAG", "visualize how the data moves", "make a graph of the program", or when a design doc or PR needs a picture of a running system. Symptoms that this skill applies: a hand-maintained diagram that no longer matches the code, a chart whose legend disagrees with the chart, a glyph reused for two different kinds of thing, a diagram that silently omits a node nobody classified, a legend placed by eye that overlaps an edge, or a picture that is a screenshot rather than a source file.
---

# Dataflow diagram

A diagram of a running program is a claim about that program. Every claim it
makes must be one the code can be held to, and every claim the code makes
must appear. Both halves fail quietly, which is why this is a generator
problem before it is a drawing problem.

## Defaults and scope

Derivation, complete classification, truthful visual encodings, and measured
placement come from observed failures. The palette, shapes, edge styles, and
frames are defaults with limited domain experience. State which conventions you
adopt and ask where they conflict with the domain: missing node kinds, an
unsuitable palette, or a distinction with no visual encoding. Record overrides
and their reasons so they can inform the skill.

Use `architecture-diagram` for build-time structure and runtime thread/request
charts. This skill covers dataflow and pipelines of a running program. Its
placement, external-source, palette, and address-arrow rules incorporate lessons
from architecture charts, an io_uring runtime, and profiler-format references.

The skill supplies complete defaults, including single-use charts: keep the
generator with the artifact and stamp the source commit as a dated snapshot.
The `dataflow-diagram-skill` template adds a charter containing project bindings
and justified deviations. Defaults apply wherever the charter is silent.

## Derive, never draw

The nodes and edges come from the program's own structures — its topic
registry, its step declarations, its wiring function — read at generation
time. Not from a list beside them.

A hand-maintained list can drift while the picture still renders. Derivation
makes a renamed topic appear renamed or fail generation.

## When the program is not in the repository

A reference page about a third-party system, a chart of a dependency's
internals, a comparison of two writers of one format: the program exists,
but not where the generator runs, and nothing at generation time can read
its structures. Derivation is unavailable. What replaces it is a model
**derived by reading**, and the chart's honesty then rests on three parts:

- **A model file that says it is a transcription.** Each node and edge
  carries the source path and symbol it was read from. The generator still
  classifies every element through fail-loud tables and still checks the
  geometry; what it cannot check is whether the model matches the program.
- **A provenance table the reader can see.** One row per node — figure,
  node, read from — rendered beside the charts, not kept in the generator.
  It is the claim surface a reader audits, and the only place a wrong
  reading can be caught.
- **An evidence baseline.** The revision the reading was made at, pinned if
  the reading was pinned; the version range and a statement that it is not
  pinned if it was not. Say that freshness is not enforced and that a reader
  who depends on a row should re-derive it against the exact version in use.

Caption the charts as derived by reading, and keep every other rule: state
drawn as nodes, unknowns left as gaps, the key generated from the same
tables. A read-derived model is a second source of truth by construction;
the provenance table is what keeps it an honest one.

## Fail loudly on anything unclassified

A generator maps program elements onto visual properties through tables:
this operation is that role, this topic is that category. **An element no
table covers must stop the run, not be skipped.**

A skipped stage leaves an incomplete chart that still looks complete.

The same applies in reverse: a table entry naming an element the program no
longer has is a stale classification, and should fail just as loudly.

## Every visual property is a claim

Shape, fill, line style, decoration, position: each is a channel, and a
reader who sees two things drawn differently will look for the difference.
A channel spent on ornament teaches a distinction that does not exist.

Choose the channel from what the distinction *is*:

- **Shape** — what kind of thing it is. A container of history, a unit of
  computation, a fixed value. Shape is the strongest channel; spend it on
  the taxonomy the reader most needs.
- **Fill** — which family within a kind: the role an operation plays, the
  boundary a topic sits on.
- **Line style** — conditionality. Solid for always, dashed for sometimes,
  dotted for a dependency that is not dataflow.
- **Decoration** — a property layered on a kind, such as whether a run
  records it.

A glyph asserts its own structure. A six-celled queue asserts a history a
reader can look back over; using it for a parameter that is written once
asserts something untrue about the parameter. Match the glyph to the claim
or pick another glyph.

**Absence must be legible.** When a property is encoded, the elements
lacking it should read as lacking it rather than as unmarked. The one input
in a chart with no recording mark is a finding, and it is only a finding
because the marked ones are visibly marked.

**Never encode a value you had to guess.** An unknown that renders as a
plausible mark is worse than an unknown that renders as a gap.

## Draw the state, do not bury it

Before a shape can distinguish compute from data, both have to be nodes. The
failure comes first as a stateful component folded into the label of the thing
that reads or writes it: "recorded streams → rings", "writes to the queue",
"the cache".

A component that holds data across invocations — a ring, a queue, a cache, a
store, a cursor — is not an implementation detail of its reader. Draw it. The
properties that make it worth drawing belong to nothing else in that box: what
it holds, how much, what happens when it fills, who else writes it, and whether
a reader can fall behind and lose records.

Two facts surface the moment it becomes a node, and neither is visible while it
is a phrase:

- **Where paths converge.** Two producers meeting at one buffer is an
  architectural claim a label cannot make. In one case that fact moved the
  boundary of a chart's central claim: two paths drawn as separate pipelines
  turned out to meet at the ring, so what was "identical from the program down"
  was really identical from the buffer down, one node earlier.
- **Where records can be lost.** Loss happens at the thing with a capacity. A
  chart that draws only processes has nowhere to put the loss, so it does not
  show it, and a reader concludes the path is lossless.

The test: **if a box names both a process and something that outlives it, split
it.** A file being read and the buffer it is read into are two nodes, and only
one of them can lap.

## Compute and data must not share a silhouette

The first thing a reader resolves is *what kind of thing is this*, and they
resolve it from outline before they resolve fill or text. Give the two
halves of a dataflow different corners:

- **Compute — rounded.** Operations, steps, stages, transforms: anything
  that runs. `shape=box, style="rounded,filled"`.
- **Data — square.** Queues, topics, buffers, parameters, stores: anything
  that holds. Straight corners, whatever the shape otherwise is.

Rounded-versus-square is deliberately quiet. It should register as a
texture across the whole chart rather than as a label on each node, which
is what lets fill stay free for the distinction *within* each half.

Then split data by what it promises:

- **A history** — a segmented glyph, cells in a row, reading as a queue
  with depth. It asserts that a reader can look back over a window.
- **One value** — a solid glyph with no segmentation, such as a dog-eared
  page (`shape=note`) for a parameter fixed before the run.

Segmentation is the claim. A six-celled queue drawn on a write-once
parameter promises a history the parameter does not have.

## Vary a shape before adding one

The shape inventory is a vocabulary the reader has to learn, and it is the
one channel with no gradient — two shapes are either the same or they are
not. Every shape added costs every future reader a lookup, so **a new shape
asserts a new kind of thing, and nothing less than that earns one.**

Where the thing is the same kind carrying a property, annotate the shape it
already has:

- **Fill** for which family it belongs to — an input queue and a state
  queue are both queues.
- **A frame** for a property layered on: recorded, tapped, exported.
- **Line style** on that frame for whether the property always holds.
- **Segmentation, size, a corner** for structure within the kind.

These compose. One shape plus three annotation channels expresses more
distinctions than four unrelated shapes, and the reader decodes them
incrementally: *queue, so a history — framed, so recorded — dashed, so
only sometimes.*

The test for whether something has earned a new shape: **would a reader who
knows the base shape still recognize it?** A framed queue is a queue. A
tinted queue is a queue. A page with a folded corner is not a queue, and
should only appear because a parameter genuinely is not a history — a
different kind, not a queue with a property.

Adding a shape is also the change most likely to strand the key, since a
key that fakes shapes can approximate a fill but not a silhouette.

## A default palette

Adopt this or replace it wholesale, but do not mix: a palette works because
its members were chosen against each other.

**Operation fills** — ColorBrewer Accent (`d3.schemeAccent`), pastel rather
than saturated because these are large filled areas a reader looks at for
a long time:

| Role | Hex |
|---|---|
| sensing / ingest | `#BEAED4` |
| estimation | `#7FC97F` |
| detection / decision | `#FFFF99` |
| control / output | `#FDC086` |

**Data tints**, lighter than the operation fills so data reads as quieter
than compute:

| Kind | Hex |
|---|---|
| external input | `#E6DEF2` |
| derived | `#EDEDED` |
| state carried across ticks | `#FBE3CB` |
| parameter | `#DDE4CF` |

**Edge colors** — `#4D4D4D` for ordinary dataflow, then `#CC79A7` and
`#D55E00` for the two paths worth separating. Those two are Okabe–Ito
colors, chosen for colorblind-safe separation; keep them if you change
everything else, because edge color is the channel with no shape to fall
back on.

**Panel** — `#F2F2F2` fill with a `#9E9E9E` border, for the key.

The operation roles above are a sensing-and-control pipeline's, and a
storage or I/O path does not partition into them. When the roles do not
fit, replace the operation fills with families the chart's own claim
needs — in one writer chart: the implementation that runs, the tools that
read then write, the buffers where pressure becomes loss, the files that are
the shared contract, the values fixed before the run — and keep shape for
compute-versus-data underneath.

A chart inlined in a page that already defines a semantic palette uses that
page's tokens, so a hue means the same thing in the figure as in the prose
beside it and the theme switches together. A chart shipped as an image
bakes its ink over an opaque ground, because an image embed strips the
page's context; `format-layout-diagram` states that rule for the figure it
applies to most. How the figure is embedded decides, not preference.

## An edge vocabulary

| Style | Means |
|---|---|
| solid | dataflow that constrains evaluation order |
| dashed, `constraint=false` | a value crossing a cycle boundary — real flow, no ordering claim, free to point against the layout |
| dotted | a dependency that is not dataflow, such as a parameter reaching an operation |
| `penwidth=2` | the product: what the pipeline exists to emit |
| curved, open head | an address: a stored offset or reference naming a place, never movement |

Drawing deferred reads dashed *and* unconstrained is what keeps a cyclic
program legible as a DAG: the cycle is visible as data without the layout
engine trying to satisfy it.

An address arrow is a claim about a field's value, not about bytes moving.
When a format figure shares a page or a key with flow charts — a chunk
anatomy beside the writer that produces it — give its offset arrows a
style the flow edges never use, so a reader cannot take a pointer for a
path. The curve is the reserved channel here; keep it out of the flow
charts.

Decoration layers onto a glyph without changing its kind, so give it the
same solid/dashed grammar the edges use: a solid frame for a property that
always holds, a dashed frame for one that holds conditionally, no frame for
absent — three states from one channel. Scale the frame with the
glyph — a padding that is a hairline at chart size is a slab on a key's
smaller copy.

When a chart must show *where* many things can go wrong — every point a
writer can fall behind, reallocate, drop, or block — label none of them in
place. Put a numbered tag on the node or edge and carry the what in a table
keyed by the same numbers, one row per tag. The chart answers where, the
table answers what and how badly, and the tags are bounds- and
collision-checked like any other label.

## Position carries meaning too

Lay the graph out along the direction of flow (`rankdir=LR` for a
pipeline) and pin each topological level to one rank. Columns then *are*
levels, and a column's height is how much of that work is independent —
which the reader gets for free from a layout they were going to read
anyway.

Where a program has a boundary the reader must respect — asynchronous
arrival on one side, deterministic evaluation on the other — pin the
inputs into a single column so the boundary can be drawn as one rule.

Number the compute nodes with their position in the evaluation order, as a
circled digit (U+2460 onward). That is a real fact about the program, and
numbering is only honest when order carries information.

Where one chart would carry two materially different implementations of the
same flow, give each its own panel on an obvious comparison axis — top and
bottom — and repeat the stages they share. Interleaving two backends'
operation names inside one set of nodes makes the shared lifecycle look like
shared mechanism, which is the opposite of what a comparison is for.
Repetition costs a reader less than one node carrying two vocabularies, and
what the variants genuinely share belongs in the prose beside the chart.

## The key is a graph, not a picture of one

Draw the key with the same node shapes the chart draws, from the same
generator. A key assembled out of table cells or text can only approximate
a shape, and an approximated shape teaches the reader a glyph the chart
never uses.

That usually means the key is its own graph, merged with the chart at
render time so the artifact stays one file. Order its entries by channel —
shapes, then line styles, then symbols — so a reader looking up a mark
searches one band rather than the whole panel.

Show the glyph, never its color's name. "Lilac means input" asks the
reader to translate, fails a colorblind reader entirely, and is
inconsistent with every other entry that shows the thing itself.

A key earns its place once the chart carries more than about two
orthogonal channels — the point at which a reader stops inferring the
encodings from context and starts guessing. Below that it is furniture: a
three-node chain with one edge style needs no legend, and adding one
implies distinctions the chart does not draw.

Keep it subordinate. Smaller type than the chart, tight rows, and seated
in the chart's own whitespace rather than stacked beneath it if the layout
has room — a key that reads as a second diagram competes with the first.

## Placement is computed, not eyeballed

An inset key must be checked against the laid-out graph, not positioned by
looking at it. Check against **node boxes and edge splines both**: an edge
routed through otherwise-empty space is invisible to a box-only check,
which will pass a layout it never looked at.

Make the check fail the build rather than warn. The free region moves
whenever an element is added, and a warning about a diagram nobody is
currently looking at is a warning nobody reads.

A layout engine is doing more placement work than the inset key makes
visible, and a generator that emits geometry itself inherits all of it —
anchoring, centering a multiline label as one line group, distributing the
connectors on one shape, routing to a resolved border, keeping children inside
their parent. `architecture-diagram` carries that set, because emitting
geometry directly is its default. Bounds are the weakest check of the group and
the only one most generators run: every one of those defects sits comfortably
inside the viewport.

Emit geometry directly for a dataflow chart when panels must column-align
across variants, or when tags and overlays sit on the chart and need
checking against it; explicit columns and routes are what make those
checks possible. Then run the whole set, and measure label fit against the
rendered text's width, not a character count times an average: a label
that overflows its box by the width of one wide glyph passes every estimate
and fails every reader.

## Verify the rendering, not the source

Layout engines accept attributes they ignore. A setting on the wrong graph,
an attribute the shape does not honor, a size that excludes the border it
draws — each fails by doing nothing, and a generator that emitted the
attribute looks correct.

After any change meant to alter layout, confirm the rendered geometry
actually moved: the bounding box, the node positions, the measured gap. If
the number is identical, the change did not land, however plausible the
source looks.

Keep the output a text format whose source is the artifact — a `.dot` or
`.d2` that renders to SVG, not a pasted raster. A text source diffs and
greps; an image is opaque to every tool.

## Diagnose and repair a specific failure

Report a failed check with a stable rule name, affected node/edge IDs, observed
geometry or claim, expected constraint, and the artifact checked. Reuse these
fields in existing test output; a new reporting framework is unnecessary.

Repair the implicated neighborhood in the generator. Preserve nodes, edge
direction, conditions, and labels' meaning; do not remove a retry or error path
to clear a collision. Check the rendered result, then rerun all affected claim
and layout checks. Fewer diagnostics alone is not progress if a different
relationship is now wrong.

Keep the last passing artifact while testing a candidate separately. If the
same issue survives two focused repairs, inspect its measurements and source
contract before one further attempt. If it persists, stop and report the
unresolved constraint. Retain the previous artifact as a labeled snapshot,
not evidence that the current source passes. These are repair limits, not
permission to skip the perceptual gate.

## The perceptual gate

Passing assertions and bounds checks does not establish that a chart is clear.
A human reads every new chart and visual change; approval of one revision does
not cover the next.

When the raster preview that review depends on is unavailable, say so and
name what stood in for it — markup validity, deterministic regeneration
compared by hash, bounds and collision checks, a reviewer reading the
committed artifact. A gate skipped without that record reads afterwards as a
gate passed.

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

Convert each reported visual defect into an assertion on the generator's output
so later layout changes cannot silently reintroduce it.

One thing readers catch that no check will: a label that spends one word on
two meanings. A stage named for `poll` in a chart comparing readiness polling
with task polling names neither, and the reader cannot tell which one the
node is about. Reach for the lifecycle word — `schedule` — and leave the
mechanism to the prose, unless the mechanism is the distinction the chart
exists to draw. This is `technical-prose`'s one-name-one-thing rule, and a
label, with no sentence around it to recover in, is where it hides best.

## Prose in the chart

Prose in and around a chart — node labels, the key, the caption, the charter —
follows `technical-prose`. Use American spelling unless the user or project
specifies another convention; apply the chosen convention consistently.

## Red flags

- A node list or edge list maintained by hand beside the code it describes.
- A generator that `continue`s past an element it cannot classify.
- The legend built by a different mechanism than the chart.
- A color named in words in the key.
- A glyph whose structure asserts more than the thing it marks possesses.
- An inset element positioned by adjusting a number until it looked right.
- A collision check that inspects nodes but not edges, or that silently
  matched fewer elements than the graph contains.
- Label fit estimated from a character count rather than measured text.
- A read-derived model with no provenance table the reader can see, or no
  statement of the revision it was read at.
- A chart of a program outside the repository presented as generated from
  it.
- An offset or reference arrow drawn in the style of a flow edge.
- A figure inlined in a themed page with its colors baked, or shipped as an
  image with colors left to the host.
- Hazards labeled in place on the chart instead of tagged and tabled.
- A multiline label centered by its first baseline rather than by its line
  group.
- Connectors attached to one shape at points chosen one at a time.
- A child shape crossing the bounds of the cluster that contains it.
- Two implementations interleaved in one flow rather than drawn as panels.
- A label carrying a word that means two things in the chart it sits in.
- A reported visual defect fixed in coordinates rather than in an assertion.
- A chart shipped without a human reading it, or a review gate skipped for an
  unavailable preview with no record of what stood in for it.
- A layout attribute added without confirming the rendered size changed.
- A diagram checked in as an image with no source beside it.
- Two visual channels carrying the same distinction, or one channel
  carrying two.
- Presenting the conventions here as settled when applying them, rather
  than as defaults the domain may override.
- Finishing a diagram without asking which defaults fought the domain.
- Compute and data sharing a silhouette, so kind can only be read from the
  label.
- A segmented glyph on something with no history to segment.
- A stateful component named inside another node's label rather than drawn.
- A box naming both a process and something that outlives it.
- A new shape introduced for what is really a property of an existing one.
- More shapes in the chart than kinds of thing in the program.
- Palette members borrowed from two different palettes.
- A legend on a chart with one edge style and one node kind.
- Numbered nodes whose numbers encode no order.
