---
name: format-layout-diagram
description: |
  Draw the layout of a memory or wire format that already exists in code — a
  byte anatomy of one encoded instance, the per-variant conventions of a shared
  encoding, or the linkage between blocks. Use when asked to "diagram the
  format", "show the byte layout", "draw the header", "picture the on-disk
  structure", "explain the encoding", or when a format reference, design
  record, or pull request needs a figure of a structure a reader cannot hold in
  their head. Beta — the conventions come from one format in one repository, so
  record friction and confirmation through `engineering-journal`. Symptoms that
  this skill applies: a byte grid drawn by hand from a specification, a figure
  whose offsets no longer match the encoder, a byte grid routed through a graph
  layout engine, a figure that cannot say whether it shows what ships or what
  is planned, or a figure that looks right in a browser and breaks in the
  rasterizer that publishes it.
---

# Format layout diagrams

A layout figure answers a question prose answers badly: **what occupies which
bytes, and what addresses what.** Prose carries a sequence; a layout is a shape
with addresses, and re-deriving it from sentences is the work the figure exists
to remove.

This skill is the third in a family and inherits its principles:
derive-never-draw, fail-loud, verify-the-rendering. `architecture-diagram` owns
what exists and what contains what; `dataflow-diagram` owns what moves where.
When the question is "which byte is that field in", it is this one.

## What this is, and what it is not yet

Treat this skill as **beta**. Its conventions come from one binary collection
format in one repository, whose figures were built without a skill and whose
author proposed this one afterward. Three rules below are that author's, stated
in their own words; the rest is inference from the artifacts and carries less
weight. When one fights your format, the override and its reason are the most
valuable thing the effort produces: record it through `engineering-journal`.

## Three rules

**Position is the byte offset. No layout engine.** In a byte grid the x-axis
*is* the address, so nothing may be free to move a cell. Emit the drawing
directly, with integer geometry and fixed ordering. Routing a byte grid through
a graph layout engine was tried and abandoned: clusters and edges moved cells,
which broke the one claim the figure exists to make.

**Spans come from golden fixtures decoded by the shipping codec.** Not from the
specification, and not from a decoder written for the figure. A second decoder
is a second source of truth — one was written, then deleted for that reason.
The generator walks a frozen test fixture through the production decoder and
aborts when any drawn span disagrees with the bytes.

**Freshness is a deterministic regenerate-and-diff on the artifact readers
see.** Not a checksum of the source, and not a promise to remember. Because
emission is deterministic, regenerating and diffing covers exactly the file a
reader opens.

## The derivation contract

A derived figure is produced by a program that fails loudly rather than drawing
something plausible:

- **Assert every span against the bytes.** A length that does not match, a
  count that does not match the walk, a stored offset that does not land on the
  element it names — each aborts, naming what disagreed.
- **Re-encode and compare.** Decoding proves the reading; re-encoding each
  decoded value and comparing bytes proves the figure's labels describe a value
  the codec would produce. It catches a tier table that has drifted from the
  encoder, which decoding alone does not.
- **Check the frame, not only the content.** Cells past the drawing bounds are
  an abort, not a cosmetic problem. A figure that silently clips is worse than
  one that refuses to render.
- **Freeze the instance.** Draw a fixture the tests already pin, and name it.
- **State the regeneration command** in the artifact's own header and in the
  provenance table, so the next person changes the generator rather than the
  output.

Choose the instance for coverage, not size: the smallest encoding that
exercises the header, at least two entries, at least two width tiers, and every
cross-reference the format defines. A one-entry example teaches the header and
nothing else.

## Source format follows what the figure claims

A byte grid and a relationship schematic are different kinds of claim, so they
keep different sources. Record the choice in a provenance table beside the
figures — one row per figure, naming its source, its regeneration command, and
what it asserts.

- **Byte anatomy — generated, emitted directly, no render step.** The committed
  artifact is exactly what the generator produced. A render step between
  generator and artifact reintroduces a layout engine's freedom over a grid
  whose positions are addresses.
- **Relationship schematic — graph source, rendered.** Chaining, linkage,
  shipped-against-reserved. Here a layout engine is correct: the claim is what
  connects to what, not where anything sits. The graph source is the diffable
  artifact; its render depends on the renderer's version and is refreshed on
  edit.
- **Hand-authored snapshot — direct drawing, no generator.** Variant
  conventions and other design claims that no code produces. Date it and name
  the review it came from, because nothing will detect its drift.

A generated figure may not draw what does not exist. A schematic may, if the
unbuilt half is visually distinct — dashed for reserved, solid for shipped —
and the caption says which is which.

## Verify the rendering, not the markup

A figure that is valid and unreadable has failed. Rasterize the committed
artifact before committing it, with the same tooling the publishing host uses.

Two defects found this way in one figure, neither visible in a browser: a
double hyphen inside an XML comment, which is illegal and rejected by strict
parsers; and a root style setting maximum width with automatic height, which
collapsed the canvas to zero height in the rasterizer while looking correct
locally.

Rasterizing proves the figure drew; it does not prove the figure is balanced.
A bracket label nearer a neighboring span than its own, a two-line label
centered by its first baseline, an offset arrow attached wherever the code
reached first — each is inside the frame and each is what a reader sees
first. `architecture-diagram` carries those placement rules as a set; a byte
grid inherits them the moment it emits its own geometry, which this skill
requires. And when the rasterizer a check depends on is unavailable, say so
and name what stood in for it — markup validity, a deterministic regenerate
compared by hash, bounds and collision checks, a reviewer reading the
committed artifact. A check skipped without that record reads afterwards as a
check passed.

Also: emit fixed ink over an opaque ground rather than colors inherited from
the page, because a host that embeds the figure as an image strips that
context. Give the figure a text alternative twice — an accessible label inside
it, and prose beside it stating the same claim.

## A default visual language

Adopt wholesale; override with a reason.

- **One cell per byte**, fixed width, in offset order, value in hex. Bytes are
  the unit even when fields are wider, because the reader's question is "which
  byte".
- **Fill by role, not by field** — header, tag, payload, trailer — reused
  across every entry. Four colors learned once beats one per field.
- **An offset ruler under the cells**, at reduced opacity: a reference the eye
  returns to, not content.
- **Brackets above for spans**, staggered when labels would collide, each
  carrying the field name and its decoded value. The value is what lets a
  reader check the figure against the bytes.
- **An arrow for every stored offset**, from the field to the element it
  addresses. This is the part readers remember, because it turns a number in a
  header into a place in the picture.
- **A caption that states the claim**, not the contents. "The tail offset
  points at the last entry's first byte, so tail access is constant time" earns
  its space; "diagram of a block" does not.

## Boundaries

`architecture-diagram` owns structure; `dataflow-diagram` owns movement. A
reader asking which byte holds a length is asking neither.

`technical-prose` owns wording and scale in captions and surrounding prose,
including
its default of American spelling. Field names, type names, and byte values are
untouchable — they are names, and a figure that renames them to read better has
stopped describing the format.

`engineering-journal` owns the durable record of why a format is shaped as it
is. The figure carries the shape; the entry carries the argument.

## Known dead ends

- **A graph layout engine for a byte grid.** Clusters and edges move cells,
  which breaks position-is-offset. Emit directly instead.
- **A second decoder written to drive the figure.** It is a second source of
  truth, and the two drift. Derive through the shipping codec.
- **Drawing the grid by hand from the specification.** Correct on the day it is
  drawn, wrong at the first encoder change, with nothing to detect it.
- **Freshness by convention.** "Regenerate when the format changes" is a
  promise, not a check. Until the regenerate-and-diff runs somewhere, assume
  the figure is stale.
- **One figure for the shipped and the planned shape, undifferentiated.**
  Readers act on all of it as though it exists.

## Red flags

- A byte grid whose cell positions came from a layout engine.
- A byte grid with no generator, or a generator with no abort path.
- A generator that decodes but never re-encodes to compare.
- A figure with no provenance row: no source, no regeneration command, no
  statement of what it asserts.
- A committed artifact that is a render of a generated intermediate rather than
  the generator's own output.
- Offsets that no longer match the encoder, with nothing that would have caught
  it.
- A fixture that is not the one the tests pin.
- Cells past the drawing bounds, silently clipped.
- A bracket or arrow label nearer a neighboring span than the one it names.
- A rasterization check skipped for unavailable tooling, with no record of
  what stood in for it.
- An artifact committed without being rasterized the way the host will.
- A figure that inherits its colors from the reader's theme.
- A schematic that omits which half is reserved.
- A caption that names the contents instead of stating the claim.
- A field name rewritten in a label so the figure reads better.
