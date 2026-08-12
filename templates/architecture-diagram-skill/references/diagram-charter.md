# Architecture Diagram Charter

This charter is a **delta on the skill's defaults**. Fill the required
bindings from project evidence; record a convention only where the project
deviates, with the reason. Absence means the default applies — which is
what keeps the skill usable with no charter at all.

## Required Bindings (no default exists)

### Chart Inventory

| Chart | Half | Output | Generator module | Claims |
| --- | --- | --- | --- | --- |
| `<chart name>` | `<build/runtime>` | `<committed output path>` | `<source path>` | `<what holds it to the code>` |

Record where charts are embedded and where each chart's textual equivalent
lives: `<paths>`.

### Generator

- Regeneration command: `<one command that regenerates every chart>`
- Toolchain: `<the project's native toolchain — a diagram generator must not
  add a contributor dependency>`
- Shared visual-language module: `<the one source file owning palette, type
  scale, and drawing primitives for all charts>`
- Source-claim helper: `<where positive and negative assertions live and how
  they fail>`

### Ground Truth

- Build half: `<build-manifest query interface (e.g. cargo metadata, go list,
  npm ls)>`; composition greps for wiring choices manifests cannot see:
  `<patterns and what they resolve>`
- Runtime half: `<source assertions: spawn sites and literal thread names,
  queue wiring, signal sets, ports, event-loop verbs — including negative
  claims for asserted absences>`
- Curated tables that need maintenance when the code changes (each validated
  at generation time so drift aborts the run): `<table names and locations>`

### Freshness

- CI: `<the job that regenerates all charts and fails on any diff against
  the committed output>`
- Locally: `<the same commands, and when contributors should run them>`

### Review Gate

- `<who reviews every new chart and every visual change; note that approval
  of an earlier revision does not cover a later one>`

## Overrides (defaults apply unless listed here)

Deviations from the skill's defaults — palette, type scale, style channels,
edge weights, panel layout, or any convention in the skill body — each with
its reason. This record is how the skill's defaults improve:

- `<override and reason, or "none — defaults adopted wholesale">`

## Charter Evidence

- Filled by and date: `<who, when>`
- Evidence: `<sources inspected to fill this charter>`
- Unknowns or conflicts: `<anything unresolved>`
