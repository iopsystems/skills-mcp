---
status: shipped
opened: 2026-08-24
updated: 2026-08-24
beta_skills: [architecture-diagram, dataflow-diagram, format-layout-diagram]
---

# Diagram layout rules from the ringline journal

## Goal

Ingest the skill feedback recorded in another repository's engineering journal.
The ringline runtime library used `architecture-diagram` and `dataflow-diagram`
to build a source-asserted runtime topology and request-lifecycle chart, and its
journal entry for that effort carries eleven friction items and two
confirmations. None of them had reached the skills.

## Scope

`skills/architecture-diagram`, `skills/dataflow-diagram`,
`skills/format-layout-diagram`, and the two diagram templates that mirror the
active bodies. One lesson outside the diagram family reached
`templates/document-feature-skill` and its installed instance. No change to the
derivation contract, the fail-loud rules, the visual languages, or any trigger
corpus.

## Evidence

The reporting repository is an io_uring runtime library with protocol client
crates. It built a `doc-diagrams` workspace tool that emits SVG directly,
asserts exact source markers before rendering, and runs in `--check` mode in CI.
That is the shape both skills ask for, and the derivation half worked: requiring
exact source claims before rendering caught three stale markers on the first
run.

The layout half did not. Seven of the reported defects were found by a human
reading the committed artifact, and every one of them was inside the viewport:

- a two-line label centered by its first baseline, so the visible block sat high
  while the arithmetic reported it centered;
- connectors attached to one shape at points chosen one at a time, leaving the
  first arrow on the midpoint and the rest pushed aside;
- a connector routed to a nominal column, which penetrated a sibling box that
  happened to be wider;
- both labels of a parallel pair placed above their lines, putting the inner
  label nearer the arrow it did not describe;
- child shapes crossing their parent's border, and parent labels sharing the
  text rhythm of their children;
- a stage labeled `wake_recv + poll owner task` in a chart comparing readiness
  polling with task polling, so `poll` named neither; and
- two backends interleaved in one flow, which made the stages they genuinely
  share read as shared mechanism.

Both skills already said to bounds-check every element and to verify the
rendering rather than the source. Both checks passed on every one of these.

Two further reports were about the gate rather than the geometry. The raster
preview the review step depends on could not start, and the effort proceeded on
markup validation, deterministic regeneration, bounds checks, and an independent
reader — a substitution neither skill described. And the fixes were durable only
because the reader's feedback was converted into generator assertions; as
coordinates they would have returned at the next layout change.

## Design and Implementation

The seven geometry defects become one section, `Bounds are not balance`, in
`architecture-diagram`, which is the skill that mandates emitting geometry
directly and therefore owns every placement decision a layout engine would
otherwise have made. It opens by naming bounds as the weakest layout claim
available and the only one most generators check, then states each rule with the
defect that produced it: resolved-shape anchoring with padding and text/shape
collision checks, the multiline label as one layout object, per-edge connector
groups, routing against resolved borders, outward labels on a connector pair,
and containment validated at every semantic layer. It closes on the rule that
makes the rest hold: a visual fix that is not a test recurs.

`dataflow-diagram` hands placement to a layout engine, so it gets the same
material at a different altitude. Its `Placement is computed, not eyeballed`
section now names what a generator inherits the moment it emits geometry itself
and points at `architecture-diagram` for the set. A new `The perceptual gate`
section carries what applies whatever draws the chart: assertions catch drift
and not confusion, a human reads every chart, the substitutes to name when the
preview is unavailable, feedback converted into assertions, and the label that
spends one word on two meanings.

The two labeling lessons land where they contradict something. `poll` is the
code's own verb, so the mechanism-neutral rule is written as the exception to
`Stage verbs are the code's verbs` rather than as a separate rule that would
quietly disagree with it. The variant-panel rule joins the request-flow bullets,
which already assumed panels without saying when to split.

`format-layout-diagram` shares the direct-emission constraint and gets the short
form: rasterizing proves the figure drew, not that it is balanced, plus the
unavailable-tooling substitution.

Both beta notes are now factually different. `architecture-diagram` claimed its
conventions were tested against exactly one system; it is two. `dataflow-diagram`
recorded one contact with a different domain; it records two, and names what the
second one found.

One lesson from the same entry is not about diagrams: a platform primer and a
library architecture guide answer different review questions, and merging them
makes the technology argument read as a property of the implementation. That
became `One Document, One Question` in `document-feature-skill`, which required
upgrading this repository's installed instance and the digests that pin it.

## Outcome

Shipped. `cargo test --locked` passes, the two diagram templates match their
active bodies byte for byte outside the template-only sections, all four
manifest digests verify, and the citation guard is green after relocating five
line citations that the edits shifted.

Whether the rules produce better-laid-out charts is unmeasured. They were
derived from one reader's report on two charts in one repository, and nothing in
this repository renders a chart that would exercise them.

## Derived Documents

None.

## Deferred or Reopen Items

- The reporting repository's journal entry is still open and still lists this
  feedback as unaddressed. Closing that loop is its author's, not this
  repository's, but nothing connects the two records.
- `.agents/skills/document-feature/template-state.yaml` records the upgrade
  through `last_upgraded_at` and the new base digests, but `source.commit` still
  names the commit the instance was installed from. A base upgraded in-tree
  cannot record the SHA of the commit that carries it.
- No check enforces any layout rule added here. They are stated for an agent
  reading them, exactly like the prose bar and the concept-before-identifier
  rule, and the durability rule they end on is itself only a rule.

## Skill Feedback

### architecture-diagram, dataflow-diagram (beta)

- **Friction** — both skills treated a bounds check as the layout verification
  step. Seven reported defects passed it. Bounds prove containment; nothing in
  either skill asked whether an element was placed *well* within its bounds.
  Fixed by naming the placement decisions a direct-emission generator inherits
  and making each one a check.
- **Friction** — `Human review gates every chart` assumed the review could
  happen. When the preview tooling failed, the skill offered no substitute, and
  the effort had to invent one. Fixed by naming the substitutes and requiring
  that the skipped step be recorded, since a gate skipped silently is
  indistinguishable afterwards from a gate passed.
- **Friction** — `Stage verbs are the code's verbs` produced an ambiguous label
  when the code spent one verb on two mechanisms. The rule was right and
  incomplete; it needed its exception rather than a competing rule.
- **Confirmation** — the derivation half held under a second system. Exact
  source markers asserted before rendering caught three stale claims on the
  first run, which is the failure mode the fail-loud rules exist for.
- **Confirmation** — this is the second time feedback has arrived as a report
  from a reader of the output rather than from the author following the skill,
  and the second time the defect was invisible to every automated check the
  skill already required.

## Appendix: Skills Invoked

- `engineering-journal` — this entry.
