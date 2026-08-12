# Dataflow Diagram Charter

This charter is a **delta on the skill's defaults**. Fill the required
bindings from project evidence; record a convention only where the project
deviates, with the reason. Absence means the default applies — which is
what keeps the skill usable with no charter at all.

## Required Bindings (no default exists)

### Chart Inventory

| Chart | Program | Output | Generator module | Derived from |
| --- | --- | --- | --- | --- |
| `<chart name>` | `<binary/service/pipeline it depicts>` | `<committed source + rendered output paths>` | `<source path>` | `<the program structures nodes and edges come from>` |

Record where charts are embedded and where each chart's textual equivalent
lives: `<paths>`.

### Generator

- Regeneration command: `<one command that regenerates every chart>`
- Toolchain: `<the project's native toolchain — a diagram generator must not
  add a contributor dependency>`
- Output format: `<the text format that is the artifact (.dot, .d2, direct
  SVG) and the render command>`
- Layout engine: `<engine and version, or "geometry emitted directly">`

### Ground Truth

- Node and edge sources: `<the registries, step declarations, or wiring
  functions read at generation time — never a hand-kept list>`
- Classification tables: `<where element-to-role tables live; each must abort
  the run on an unclassified or stale entry>`
- Properties that must not be guessed: `<values rendered only when known,
  left unencoded otherwise>`

### Freshness

- CI: `<the job that regenerates all charts and fails on any diff against
  the committed output>`
- Locally: `<the same commands, and when contributors should run them>`

### Review Gate

- `<who reviews every new chart and every visual change; note that approval
  of an earlier revision does not cover a later one>`

## Overrides (defaults apply unless listed here)

Deviations from the skill's defaults — palette, shape grammar, edge
vocabulary, key conventions, or any convention in the skill body — each with
its reason. This record is how the skill's defaults improve:

- `<override and reason, or "none — defaults adopted wholesale">`

## Charter Evidence

- Filled by and date: `<who, when>`
- Evidence: `<sources inspected to fill this charter>`
- Unknowns or conflicts: `<anything unresolved>`
