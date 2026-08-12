# Dataflow Diagram Charter

Fill this file from project evidence before using the installed skill. Replace
all placeholders, cite repository sources, and recheck the charter before
every material diagram effort.

## Chart Inventory

| Chart | Program | Output | Generator module | Derived from |
| --- | --- | --- | --- | --- |
| `<chart name>` | `<binary/service/pipeline it depicts>` | `<committed source + rendered output paths>` | `<source path>` | `<the program structures nodes and edges come from>` |

Record where charts are embedded and where each chart's textual equivalent
lives: `<paths>`.

## Generator

- Regeneration command: `<one command that regenerates every chart>`
- Toolchain: `<the project's native toolchain — a diagram generator must not
  add a contributor dependency>`
- Output format: `<the text format that is the artifact (.dot, .d2, direct
  SVG) and the render command>`
- Layout engine: `<engine and version, or "geometry emitted directly">`

## Ground Truth Bindings

- Node and edge sources: `<the registries, step declarations, or wiring
  functions read at generation time — never a hand-kept list>`
- Classification tables: `<where element-to-role tables live; each must abort
  the run on an unclassified or stale entry>`
- Properties that must not be guessed: `<values rendered only when known,
  left unencoded otherwise>`

## Visual Language Bindings

- Palette: `<adopted default palette or wholesale replacement — never mixed;
  role of each color>`
- Shape grammar: `<what rounded vs. square (or the project's kinds) assert;
  any third kind the compute/data partition did not cover>`
- Edge vocabulary: `<solid/dashed/dotted/penwidth meanings>`
- Key: `<how the key is generated from the same shapes, and when a chart is
  simple enough to omit it>`
- Placement check: `<the computed collision check (nodes and edge splines
  both) and how it fails the build>`

## Freshness

- CI: `<the job that regenerates all charts and fails on any diff against
  the committed output>`
- Locally: `<the same commands, and when contributors should run them>`

## Review Gate

- `<who reviews every new chart and every visual change; note that approval
  of an earlier revision does not cover a later one>`

## Recorded Overrides

Every default from the skill that this project rejected or extended, with the
reason — the skill's conventions improve only through this record:

- `<override and reason, or "none yet">`

## Charter Evidence

- Filled by and date: `<who, when>`
- Evidence: `<sources inspected to fill this charter>`
- Unknowns or conflicts: `<anything unresolved>`
