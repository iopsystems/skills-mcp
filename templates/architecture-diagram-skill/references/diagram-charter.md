# Architecture Diagram Charter

Fill this file from project evidence before using the installed skill. Replace
all placeholders, cite repository sources, and recheck the charter before
every material diagram effort.

## Chart Inventory

| Chart | Half | Output | Generator module | Claims |
| --- | --- | --- | --- | --- |
| `<chart name>` | `<build/runtime>` | `<committed output path>` | `<source path>` | `<what holds it to the code>` |

Record where charts are embedded and where each chart's textual equivalent
lives: `<paths>`.

## Generator

- Regeneration command: `<one command that regenerates every chart>`
- Toolchain: `<the project's native toolchain — a diagram generator must not
  add a contributor dependency>`
- Shared visual-language module: `<the one source file owning palette, type
  scale, and drawing primitives for all charts>`
- Source-claim helper: `<where positive and negative assertions live and how
  they fail>`

## Ground Truth Bindings

- Build half: `<build-manifest query interface (e.g. cargo metadata, go list,
  npm ls)>`; composition greps for wiring choices manifests cannot see:
  `<patterns and what they resolve>`
- Runtime half: `<source assertions: spawn sites and literal thread names,
  queue wiring, signal sets, ports, event-loop verbs — including negative
  claims for asserted absences>`
- Curated tables that need maintenance when the code changes (each validated
  at generation time so drift aborts the run): `<table names and locations>`

## Visual Language Bindings

- Palette: `<one palette, adopted or replaced wholesale — never mixed; role
  of each color>`
- Type scale: `<the few sizes in use and what each marks>`
- Style channels: `<what monospace, italic, dashed, underline each mean —
  one meaning per channel>`
- Edge weight: `<what heavy vs. thin asserts (e.g. process-boundary vs.
  internal)>`
- Layout: `<arrow discipline, label placement, panel arrangement, margin
  annotations>`

## Freshness

- CI: `<the job that regenerates all charts and fails on any diff against
  the committed output>`
- Locally: `<the same commands, and when contributors should run them>`

## Review Gate

- `<who reviews every new chart and every visual change; note that approval
  of an earlier revision does not cover a later one>`

## Charter Evidence

- Filled by and date: `<who, when>`
- Evidence: `<sources inspected to fill this charter>`
- Unknowns or conflicts: `<anything unresolved>`
