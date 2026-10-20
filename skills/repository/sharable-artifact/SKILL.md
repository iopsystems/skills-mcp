---
name: sharable-artifact
description: |
  Build or revise a page a team reads on its own — a design preview, a status
  board, a reference on a format or a subsystem — as one self-contained HTML
  file published to a stable URL and republished in place. Use when asked to
  "write this up as a page", "make a design page", "publish a status board",
  "put the reference somewhere the team can read it", or when a write-up
  exists only in chat or in a file nobody will open. Beta — the conventions
  come from a handful of pages in one team; record friction through
  `engineering-journal`. Symptoms that this skill applies: a page whose
  sections renumber by hand, a figure cited as "below", a status row that says
  "fixed" when only the author says so, a number with no revision it was taken
  on, a figure whose labels overflow in the render, or a page that is white on
  white in dark mode.
---

# Sharable artifact

A page that ships alone is read without its author in the room. Every claim on
it must say where it came from, every element must be citable by a name that
survives editing, and the page must still be right the day after it is
published, when the work it describes has moved. Those three demands shape
everything below.

## What this is, and what it is not yet

The conventions come from a few design and reference pages written for one
team: a design preview with a progress board, and a reference page on a
recording format with nine generated figures. They held across those pages
and were revised each time a reader said what they could not find. They have
not been tried on a page for a different audience, and the reading-time and
word-count defaults are that team's. Treat this skill as **beta**: adopt the
shape wholesale, and when a convention fights the page, record the override
and its reason through `engineering-journal`.

Two neighbors own parts of the work. `technical-prose` owns the words: every
page gets its pass before publishing, with the reader named. The diagram
skills own the figures' claims: a chart of a running program follows
`dataflow-diagram`, a byte layout follows `format-layout-diagram`, and both
state the rule this page depends on, that an inlined figure takes the page's
color tokens.

## One file, one URL, one source

The page is one HTML file with inline CSS and inline SVG. Nothing depends on a
build step, so the file is the source, and the source is what gets republished.
Keep it in one place, republish to the same URL on every change with a short
label per version, and let the URL be the thing people bookmark. A second copy
in a repository, a doc tool, or a chat message is a second source of truth and
drifts the first time the page changes.

Name the page like a product: two to four words that identify it among many,
with the explainer in the publish description, never in the title.

## Shape: a short paper

The page reads like a short systems paper: a header, parts, numbered sections,
numbered figures and tables, and a footer of evidence and revision notes.

**Header.** An eyebrow line with the scope and the assumptions the page rests
on (the repository, the packages touched, the plan it serves). The name. The
problem in two sentences. A status sentence that links to the progress board
when the page has one.

**Reading guide.** A navigation panel after the intro, one column per part: the
part's title linked, a reading time ("~12 min"), one sentence on who reads it
and why, and its numbered sections as links. Then a quick path for the reader
with five minutes: the summary, the key figures and table, the decision. No
total time and no advisory sentence; the reader decides from the summary.

**Parts, one per audience.** Order them by how little time each reader has.
For a design page the default is three: *Design* for the reader assessing the
architecture (summary, options, the high-level design of each structure,
layering, the main consumer's use, the public contract and its guarantees,
signals and health, invariants); *Migration* for the reader planning the work
(where the existing solution falls short, what the change enables, the plan,
implementation considerations, the decision summary); *Implementation details*
for the reader writing or reviewing code. The first two parts carry figures,
tables, and short prose, and together stay under about 2,500 words; a section
there whose mechanism lives in the third part ends with a pointer line naming
the section by number and title. A reference page with no work to plan drops
the progress board and the migration part; it keeps the rest.

**Numbered sections, by CSS.** One section element per numbered section, with
the number drawn by a CSS counter rather than typed, so an inserted section
renumbers the rest. Part dividers are not sections and consume no number.

**Footer, in this order.** The evidence baseline: which commits the surveys
ran on, which branches, which external pull requests. Then one dated paragraph
per revision saying what changed on the page and why. Last, who wrote what.

## Every element is citable

Every section, figure, table, and listing carries a stable id and shows its own
anchor beside its title, as `[#id]` in small monospace. Readers paste that into
chat and review comments, and the citation survives renumbering, which is the
point.

- Id prefixes carry the kind: `fig-`, `table-`, `listing-`, a part-specific
  prefix for deep-dive sections, plain words for the rest.
- Figures, tables, and listings are numbered by hand in their captions, in
  order of appearance across the whole page, never per section. The figure
  caption sits below; the table caption sits above.
- Prose refers to them by number, linked: "Figure 4", "Table 3", "Section 6".
  Never "above", "below", or "the following".
- The target of a shared link is highlighted when the page opens on it, and
  sits clear of the top edge.

## Every number names its baseline

A number with no revision is a number the reader cannot re-derive.

- A pull request is a link on its number. In a status row the link is followed
  by the commit the row's claims were checked at, or the merge date.
- A line of source is cited as `path:line` in code font, and the paragraph or
  table carrying it says which commit the line numbers belong to. A durable
  link uses the commit-pinned blob form with the full hash, never a branch
  link, which moves.
- Survey facts and measurements name the branch and commit they were taken on.
- Identifiers, flags, channel names, and paths are code font everywhere,
  including table cells and SVG text.

## Figures are inline SVG with the page's tokens

Each figure is hand- or generator-authored SVG with a viewBox, inside a
scrolling wrapper so a narrow screen scrolls the drawing rather than the page.
It carries an image role and a label that says in words what the picture
shows, arrows included; the caption says the same thing shorter, as a claim to
conclude rather than a list of contents. A small uppercase monospace label at
the top of each panel names what the reader is looking at.

Colors are the page's CSS tokens, so the figure follows the theme and a hue
means the same thing in the figure as in the prose beside it. That is the
inlined half of the rule `format-layout-diagram` states in full; a figure
shipped as an image bakes its ink instead. Arrowheads are marker definitions
per figure in the current color or a token. A legend appears under a figure
only when the same encoding repeats across figures, with each swatch drawn
from the token it names.

SVG text does not wrap. A label that outgrows its box is invisible in the
source and the first thing a reader sees, so **measure label fit in a real
renderer after any figure edit**: every text element against the shape that
contains it, free labels against boxes they do not belong to, text against
text, everything against the viewBox, with the page's fonts confirmed loaded
before measuring, since a fallback face measures differently and reports
phantom overflows. Hand-estimated character widths found none of the seven
overflows the first such check reported.

## Tables, cards, and listings

- A table's caption comes first. Column headers are small uppercase with
  letter-spacing. A cell colored by the layer it belongs to uses the same
  tokens as the figures; a yes/no matrix uses a dot, not a word. Wide tables
  scroll in their own wrapper.
- A table in the first part keeps the property columns: what the thing is,
  what a reader must see, which layer owns it. Columns that say how a property
  is derived, carried, built in, or verified move to a twin table in the
  implementation part that repeats the row labels, and the first table's
  caption points at it. Three or four columns is the target.
- Summary cards come in threes, each with an eyebrow naming its role
  (Problem, Proposal, Enforcement), a heading, and one paragraph. The card's
  top border color says its role in the page's palette.
- A listing is a figure with a preformatted block and three hand-applied
  spans: comments, the names being introduced, and what is hidden or elided.
  No syntax highlighter; the highlighting carries the argument, not the
  grammar.

## The progress board

A page that tracks work carries a board in its header, so the page doubles as
the status teammates read. One table: goal, what it delivers, the pull request
with its checked-at commit, and the state.

The state word comes first in its cell, bold, from a fixed vocabulary the
board's note spells out: *merged*, *approved*, *awaiting re-review*, *ready*,
*draft*, *not started*. Keep *awaiting re-review* distinct from *approved*: the
author saying the findings are addressed is not the reviewer confirming it,
and a board that collapses the two misleads the person deciding what to review
next. Color the state word by row class, and stamp the board with the date and
the main-branch commit it was checked against.

A status edit is four touches made together: replace the row's pull-request
cell and state cell, set the row class, bump the stamp, and append a dated
footer paragraph. The board shows the present; the footer shows how it got
there; the four touches are what keep them consistent. Re-read every row
against the live pull-request state before publishing, not from memory.

## Theme and type

- Color tokens live on the root element, with the complete light palette
  there, redefined for dark under both the system-preference query (guarded so
  an explicit light choice wins) and the explicit dark attribute, so the
  viewer's choice wins in both directions and the unstamped state still gets
  dark. The body sets its own background from a token. Nothing is colored
  only inside a theme block.
- The palette is semantic, not decorative: one hue per kind of thing the page
  argues about (the public contract, hidden implementation, a leak or a
  problem, signals and health, not built yet), each with a fill twin for
  boxes, and the same hue means the same thing in every figure, cell, and
  card. Neutrals carry a slight bias toward the accent rather than pure gray.
- Fonts come from a host the publishing platform allows, each with a system
  fallback; a condensed display face, a text face around 17px at 1.5 line
  height, and a monospace for code, anchors, and eyebrows.
- One column at a readable maximum width; sections as grids with a fixed gap;
  a side gutter on phones; cards and guide columns collapse to one column on
  narrow screens; no horizontal page scroll.

## Before publishing

1. Run `technical-prose` over the page text with the reader named: an
   engineer on the team who has not read the plan, since the page ships alone.
   The pass has caught factual errors, not only wording.
2. Run the label-fit check after any figure edit and fix every overflow,
   crossing, and out-of-bounds finding.
3. Re-read progress rows against live pull-request state.
4. Confirm both theme states render: the light palette on the root, both dark
   blocks, an explicit body background.
5. Re-read the live published version and merge onto it, so an edit a
   teammate made since the last publish is not overwritten.
6. Publish to the same URL with a version label.

A review of the page by another agent goes through `review-exchange` with a
local Markdown render, since reviewers comment on text, not on markup. When
another agent returns a prose pass, compare before adopting: preformatted
blocks and table rows byte-identical, decision-critical terms still present,
per-section word counts to see where cuts fell, the fit check on the result,
and the attribution line still in place.

## Checklist for a new page

1. The title is a name; the description carries the explainer.
2. Header: eyebrow with scope and assumptions, name, problem in two sentences,
   status sentence linking the board.
3. Reading guide: parts with reading times, numbered section links, a quick
   path.
4. Progress board with the state vocabulary written out and an as-of stamp,
   if the page tracks work.
5. Sections with ids and shown anchors; figures, tables, and listings
   numbered in order, each with an id and a caption anchor.
6. Every figure: viewBox, image role, label, token colors, scroll wrapper, a
   legend only when an encoding repeats.
7. Citations pinned to a commit; pull-request links on numbers; checked-at
   commits and dates beside them.
8. Footer: evidence baseline, dated revision notes, authorship.
9. Tokens on the root, both dark blocks, explicit body background, phone
   gutter.
10. Prose pass, fit check, then publish to the same URL with a label.

## Red flags

- A section number typed by hand.
- A figure cited as "below" or "the following figure".
- An element with no id, or an anchor hidden at the end of a caption where a
  reader cannot find it.
- A number, a line citation, or a survey fact with no revision it was taken on.
- A branch link where a commit-pinned link was wanted.
- A status row that says fixed because the author said so.
- A board updated without its footer paragraph, or a footer paragraph with no
  board change.
- A figure with its colors baked while the page around it switches theme.
- A figure published after a figure edit with no fit check, or a fit check
  that measured the fallback face.
- A color defined only inside a theme block.
- A reading guide with a total time or an advisory sentence.
- A first-part table carrying derivation columns.
- A second copy of the page kept anywhere.
- A page published without the prose pass.
