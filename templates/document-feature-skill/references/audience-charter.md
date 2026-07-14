# Feature Documentation Audience Charter

Fill this file from project evidence before using the installed skill. Replace
all placeholders, cite the repository sources for each choice, and recheck the
charter before every material documentation effort.

## Project Context

- Project name and project type: `<name and type>`
- Documentation scope: `<features and surfaces>`
- Established conventions and guidance: `<paths>`
- Shared terminology: `<terms and authoritative definitions>`

## Independent Audience Ranks

Rank each audience independently rather than forcing a single ordering. Record
priority, representative tasks, expected prior knowledge, and accessibility or
environment constraints.

| Audience | Independent rank | Representative tasks | Prior knowledge | Constraints |
| --- | --- | --- | --- | --- |
| Human users | `<rank>` | `<tasks>` | `<knowledge>` | `<constraints>` |
| Agent users | `<rank>` | `<tasks>` | `<knowledge>` | `<constraints>` |
| Human developers | `<rank>` | `<tasks>` | `<knowledge>` | `<constraints>` |
| Coding agents | `<rank>` | `<tasks>` | `<knowledge>` | `<constraints>` |

## Sources of Truth

| Claim type | Authoritative source | Verification method |
| --- | --- | --- |
| CLI syntax, defaults, and modes | `<parser/help source>` | `<rendered-help command>` |
| Configuration and schemas | `<paths>` | `<commands/tests>` |
| Runtime contracts and failures | `<code/tests>` | `<commands>` |
| Architecture and lifecycle | `<code/design records>` | `<checks>` |

## Synchronized Surfaces

- README and deeper guides: `<paths>`
- Code documentation: `<paths or boundaries>`
- CLI help and generated references: `<commands and paths>`
- Examples and configuration references: `<paths>`
- Diagrams and textual equivalents: `<paths>`

## Verification Commands

- Build or rendered output: `<commands>`
- Documentation, link, and example checks: `<commands>`
- CLI help and behavior checks: `<commands>`
- Code and schema tests: `<commands>`

## Diagram Tooling

- DOT executable and expected version: `<tool/version>`
- Parse/render command: `<command>`
- Freshness check: `<command>`
- Source/SVG placement: `<path convention>`
- Textual-equivalent convention: `<component table or workflow>`

## Review Gates

Record the project-specific risk thresholds and reviewers. Architecture/workflow
diagrams, visual hierarchy/navigation, major README restructures, onboarding
narratives, and other primarily perceptual or subjective features always require
human review.

| Change category | Required review | Reviewer or owner | Evidence location |
| --- | --- | --- | --- |
| Deterministic factual update | `<automated/peer/human>` | `<owner>` | `<location>` |
| Architecture or workflow diagram | `human` | `<owner>` | `<location>` |
| Visual hierarchy or navigation | `human` | `<owner>` | `<location>` |
| Major README restructure | `human` | `<owner>` | `<location>` |
| Onboarding narrative | `human` | `<owner>` | `<location>` |

## Charter Evidence

- Filled by and date: `<identity/date>`
- Evidence inspected: `<paths and commits>`
- Unknowns or conflicts: `<none or details>`
