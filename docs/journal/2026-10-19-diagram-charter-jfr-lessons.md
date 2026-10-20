---
status: open
opened: 2026-10-19
updated: 2026-10-19
---

# A third contact: the JFR reference page's charter

## Goal and Scope

Fold the charter of a real artifact back into the diagram skills. The artifact
is a reference page on JFR read and write patterns: nine generated SVG figures
(chunk anatomy, five writer charts, two reader charts, a key), a 26-row hazard
table, and a 57-row provenance table, built with a Node generator in a
documentation repository that contains none of the source it describes. Its
charter section, "Where the diagram conventions fought this subject", names
four places the defaults fought the page. The skills ask for exactly that
record; this entry is the return trip.

Scope is the three diagram skills and the two diagram templates. No generator,
renderer, or tool surface changes.

## Evidence

The four charter findings, in the page's own terms:

1. **Derived by reading, not generated from a build.** The writers and readers
   live in OpenJDK and async-profiler. The model was written by hand from
   named sources, the generator still classifies through a fail-loud table
   and checks geometry, freshness is not enforced, and the page calls itself
   a dated snapshot. Neither skill had a name for this case: `dataflow-diagram`
   offered derive-or-nothing, and `architecture-diagram`'s single-use section
   assumed the program was readable at generation time.
2. **Fill encodes the page's semantic palette, not pipeline role.** The default
   operation fills (sensing, estimation, decision, control) do not partition a
   writer. The page used five families: implementation, read-then-write tools,
   buffers where pressure becomes loss, files as the shared contract, fixed
   values, carried into the SVGs as the page's CSS tokens.
3. **Offsets drawn as arcs.** The chunk anatomy's header-offset arrows are
   claims about field values, so the page curved them and kept them outside
   the flow charts' edge vocabulary.
4. **Geometry emitted directly.** Explicit columns and routes, with checks for
   bounds, node and label collisions, label overflow against measured text
   width, hazard-tag bounds, and edge endpoints on borders. All nine figures
   pass.

The page's revision notes add two smaller lessons already covered by the
skills' one-name-one-thing rule and the panel-per-variant rule: "block" had
carried bounded self-cost and open-ended waits and was split; the writer chart
was split into one panel per entrance plus the shared tail.

### Baseline

Before editing, two fresh agents answered two scenarios with the unedited
skills. The first scenario asked for a writer chart of a program whose source
is in another repository. The agent transcribed a model with upstream file and
symbol per node, pinned a commit, captioned the figure as transcribed, replaced
the role palette wholesale, and drew the two writers as separate panels. The
second scenario asked how to fill nodes given a page-owned palette and how to
style offset arrows beside flow charts. The agent replaced the fills with the
page's tokens, styled offset arrows with an open head and a route above the
grid, and added a key entry.

So the controls largely improvised what the charter did. Two gaps were real:
neither agent produced a reader-facing provenance table, and the second agent
baked every token to a literal hex for a figure inlined in a themed page,
which is what `format-layout-diagram`'s unconditional "emit fixed ink" rule
told it to do.

## Design

`dataflow-diagram`:

- A new section, "When the program is not in the repository", stating the
  parts of a read-derived model as a recipe: a model file that says it is a
  transcription with source path and symbol per element, a provenance table
  rendered for the reader, and an evidence baseline naming the revision read
  and saying freshness is not enforced.
- The palette section now says the default operation roles are a
  sensing-and-control pipeline's and names the families one writer chart used
  instead, then states the embedding predicate: an inlined figure uses the
  page's tokens, an image bakes its ink.
- The edge vocabulary gains a curved, open-headed address arrow reserved for
  "names a place", with the rule that flow charts never use the curve.
- The decoration section gains the tag-and-table pattern for charts that must
  show where many things can go wrong.
- The placement section says when to emit geometry directly for a dataflow
  chart and requires label fit measured against rendered text width.
- The contact count in "What this is" is three, with the third named.

`architecture-diagram`: a "Label fit is measured, not estimated" rule in
"Bounds are not balance", and a paragraph in "Single-use charts" pointing at
the read-derived model for a program outside any readable repository.

`format-layout-diagram`: the fixed-ink rule becomes a conditional on how the
figure is embedded, and the stored-offset bullet says how offset arrows must
differ from flow edges when the anatomy shares a page or a key with flow
charts. The existing eval "theme-inherited colors fail on the host" now states
that the host embeds the figure as an image.

Templates mirror the active bodies at versions `0.1.2` (dataflow) and `0.2.2`
(architecture) with refreshed digests. Five dataflow scenarios, one
architecture scenario, and two format-layout scenarios join the corpora; the
dataflow and architecture additions also join the template corpora.

## Decision Criteria

An agent asked to chart a program it cannot read must name the model as
derived by reading, show the reader where each element came from, and state
the revision and the absence of a freshness check. An agent drawing into a
themed page must keep the page's tokens, and one shipping an image must bake
ink. An offset arrow beside flow charts must not be mistakable for flow.
Label fit must be measured, not estimated.

## Validation and Outcome

Local CI on the branch: `cargo fmt`, `cargo clippy -D warnings`, `cargo test
--locked` (including active/template body equivalence and manifest digests),
the diagram freshness check, the MCP smoke test, the bridge test, and the
release build. The two scenarios were re-run with the edited skills.

Re-run, scenario one: the agent now names the model as derived by reading,
renders a provenance table beside the charts, states an evidence baseline as
pinned or as an unpinned version range, says freshness is not enforced, tags
drop points and tables them, measures label fit against text width, and uses
the page's tokens because the figure is inlined. The baseline had none of the
provenance table, the baseline statement, or the tag-and-table pattern.

Re-run, scenario two: the agent keeps the page's tokens for the inlined figure
and reserves baked ink for an image embed, where the baseline had resolved
every token to hex. Offset arrows are curved with an open head, never used in
the flow charts, with their own key entry.

The scenario additions specify expected behavior. Two fresh-agent runs per
scenario, one before and one after, are a wording check, not a behavioral
evaluation; variance across repetitions was not measured.

## Deferred Work

- Apply the read-derived recipe to a second artifact before treating the
  three-part shape as settled.
- The hazard tag-and-table pattern came from one page with 26 hazards; the
  threshold at which in-place labels stop working was not measured.
- The architecture charter's "Ground Truth" bindings still assume a readable
  program; a read-derived binding may belong there.

## Appendix: Skills Invoked

- `superpowers:writing-skills` — baseline before editing, recipe form for
  shaping failures, conditionals keyed to an observable predicate.
- `technical-prose` — one claim per sentence in the new sections.
- `engineering-journal` — this record.
