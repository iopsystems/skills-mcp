# Feature Documentation Audience Charter

This charter specializes the installed `document-feature` workflow for
`iop-skills`. Recheck it before every material documentation effort and keep
frozen tasks stable during evaluation.

## Project Context

- Project name and project type: `iop-skills`, a public Rust MCP server and
  repository of active agent skills plus inert, customizable skill templates.
- Documentation scope: the repository README, Rust boundary documentation,
  rendered MCP requests and responses, source-install guidance, template
  adoption, and DOT-authored architecture or workflow diagrams with adjacent SVG.
- Established conventions and guidance:
  `docs/journal/2026-07-13-skill-templates-and-project-documentation.md`,
  `docs/roadmap.md`, `docs/backlog.md`,
  `docs/assumptions-and-limitations.md`, and `.github/workflows/ci.yml`.
- Shared terminology: active skill means an invocable MCP instruction; skill
  template means inert catalog content; installed instance means a
  project-customized, provenance-tracked skill under a harness discovery path.
  The catalog and state schemas in `src/templates.rs`, `src/main.rs`, and
  `skills/seed-skill-template/SKILL.md` are authoritative.

## Audience Priorities and Success

Use the same priority scale for each audience:

- `P0` — primary audience; required tasks must pass before completion.
- `P1` — important audience; satisfy its criteria unless an explicit documented
  tradeoff is approved.
- `P2` — secondary audience; preserve correctness and meet criteria where they
  do not compromise P0 or P1.
- `out of scope` — not targeted by this effort; shared authoritative facts still
  must not be false for this audience.

| Audience | Priority | Frozen representative task | Measurable success criterion | Prior knowledge | Constraints |
| --- | --- | --- | --- | --- | --- |
| Human users | `P0` | Internal organizational engineers decide why the repository is useful, install and use the current source-built server, then ask: “which skills here should I install and use for my project XYZ? Give me some recommendations” | From the README alone, identify the value proposition, prerequisites, exact source-install command, MCP setup, recommendation question, and one recovery path | Comfortable with developer tooling, but not this repository’s architecture | Apple Silicon macOS is most common; Linux occurs sometimes; no prebuilt bundle or binary exists |
| Agent users | `P1` | Select applicable active skills or inert templates for a described project without confusing invocation with installation | Return ranked recommendations with type, evidence, caveats, and next action while preserving exact IDs and commands | Can consume explicit lists, tables, schemas, and MCP tool results | Needs authoritative text equivalents for every visual and cannot infer human usability |
| Human developers | `P1` | Turn lessons from their own development experience into a new skill or a reviewed improvement to an existing skill or template | Locate the correct contribution path, tests, journal evidence, validation commands, and human review gate | Can read Rust, Markdown, JSON, YAML, and repository history | Contributions must preserve active/inert boundaries, provenance, safety, and review evidence |
| Coding agents | `P1` | Modify a skill or template and verify registry, MCP, and documentation contracts without accidental invocation or overwrite | Identify authoritative files, use TDD and skill evaluation, run exact checks, and report unresolved human gates | Can inspect repository code and execute approved local commands | Must treat ordinary docs as evidence, preserve local customization, and stop for approval-gated writes or perceptual review |

Internal organizational engineers are the primary human users even though this
repository is public. Lead human documentation with why the skills are useful,
how to install and use the current source-built server, and the exact
recommendation question. Retain accurate paths for agent users, human developers,
and coding agents. Encourage developers to add skills or update existing ones
from concrete development experience.

Rank and success criteria drive content order, examples, visual and textual
emphasis, and verification. Resolve conflicts by satisfying P0 without making P1
contracts false. Document material tradeoffs and ask the human owner instead of
guessing.

## Sources of Truth

| Claim type | Authoritative source | Verification method |
| --- | --- | --- |
| MCP syntax, tool schemas, defaults, modes, and errors | `src/main.rs` plus actual server responses | `cargo build --locked` and `./scripts/mcp-smoke.sh`; inspect initialized, tools/list, and tools/call responses |
| Template catalog, provenance, digests, and merge strategy | `build.rs`, `src/templates.rs`, `templates/catalog.yaml`, and each `template.yaml` | `cargo test templates::tests --locked` and `cargo test --test template_contracts --locked` |
| Active skill and installed-instance contracts | `skills/*/SKILL.md`, this instance, and repository tests | `cargo test --test installed_instance --locked` and `cargo test --locked` |
| Architecture, lifecycle, roadmap, and limitations | `src/`, journal, roadmap, backlog, and assumptions/limitations | Trace claims to code and tests; reconcile durable documents before closure |
| Distribution | `Cargo.toml`, release workflow state, roadmap, and assumptions/limitations | Confirm source-only status and absence of published artifacts before documenting availability |

## Synchronized Surfaces

- README and deeper guides: `README.md`, `docs/roadmap.md`,
  `docs/backlog.md`, `docs/assumptions-and-limitations.md`, and
  `docs/journal/`.
- Code documentation: public and safety-sensitive boundaries in `src/main.rs`,
  `src/templates.rs`, and `build.rs`.
- CLI help and generated references: this project is an MCP stdio server rather
  than a flag-oriented CLI. Treat actual `initialize`, `tools/list`, and
  `tools/call` output as the rendered interface and verify it with
  `./scripts/mcp-smoke.sh`.
- Examples and configuration references: README MCP configuration, source install,
  raw JSON-RPC examples, template manifests, and eval fixtures.
- Diagrams and textual equivalents: `docs/*.dot` beside `docs/*.svg`, with an
  adjacent numbered workflow or table in the relevant guide.

## Installation and Contribution Contract

The current distribution is source-only: there is no prebuilt bundle or binary,
Homebrew package, or coding-agent plugin. For current installation documentation,
use exactly:

```sh
cargo install --git https://github.com/iopsystems/skills-mcp --locked
```

This command performs network access and installs a binary; document it for human
users but do not execute it during documentation work without explicit
authorization. Do not imply that source-only distribution is packaged
distribution. Link packaging expectations to `docs/roadmap.md` and
`docs/assumptions-and-limitations.md`.

Prominently invite users to ask their connected agent exactly:

> which skills here should I install and use for my project XYZ? Give me some recommendations

Prominently invite human developers to contribute lessons from their own
development experience by adding an active skill under `skills/`, adding an
inert reusable base under `templates/`, or improving an existing artifact with
baseline, forward, critic, and deterministic evidence. Preserve the distinction
between using an active skill and seeding a customized template instance.

## Verification Commands

- Build and rendered MCP output:
  `cargo build --locked`, `cargo build --release --locked`, and
  `./scripts/mcp-smoke.sh`.
- Documentation, link, and example checks:
  `cargo test --test template_contracts --locked`,
  `cargo test --test installed_instance --locked`, and the Task 8 repository
  documentation tests once present.
- Code quality and schema tests: `cargo fmt --all -- --check`,
  `cargo clippy --all-targets --locked -- -D warnings`, and
  `cargo test --locked`.
- Harness links:
  `test -d .claude/skills`,
  `test -L .claude/skills/document-feature`,
  `test "$(readlink .claude/skills/document-feature)" =
  "../../.agents/skills/document-feature"`,
  `test -f .agents/skills/document-feature/SKILL.md`, and
  `test -f .claude/skills/document-feature/SKILL.md`.
- Harness discovery evidence on 2026-07-14: the installed Codex launcher was
  unavailable because of a missing platform binary (`ENOENT`). Claude Code
  2.1.202 bare mode returned `Unknown command: /document-feature` after both the
  directory-link attempt and the approved per-skill fallback; normal mode
  returned `Not logged in`. These are limitations, not discovery passes.

## Diagram Tooling

- DOT executable and observed version during installation:
  Graphviz `dot - graphviz version 15.1.0 (20260618.0150)`.
- Parse/render command:
  `dot -Tsvg docs/skill-feedback-loop.dot -o /tmp/skill-feedback-loop.svg`.
- Freshness check: `./scripts/render-diagrams.sh --check` once Task 8 adds it;
  the committed SVG records the SHA-256 of the authoritative DOT bytes.
- Source/SVG placement: adjacent `docs/<name>.dot` and
  `docs/<name>.svg`.
- Textual-equivalent convention: a numbered workflow or comparison table beside
  every diagram.

For complex architecture, ownership, or workflows, feature the SVG prominently
for fast human comprehension. Use DOT as the authoritative source because
clusters express nesting well. Preserve agent and accessibility comprehension
with equivalent lists, tables, and explicit contracts.

## Review Gates

Human-oriented and primarily perceptual features are gated by human review.
Agent simulation and deterministic checks can establish correctness or reveal
ambiguity, but cannot establish human usability.

| Change category | Required review | Reviewer or owner | Evidence location |
| --- | --- | --- | --- |
| Deterministic factual update | automated checks plus peer review when risk warrants | repository maintainer | engineering journal and test output |
| Architecture or workflow diagram | `human` | repository maintainer | final rendered SVG review recorded in the engineering journal |
| Visual hierarchy or navigation | `human` | repository maintainer | final README diff review recorded in the engineering journal |
| Major README restructure | `human` | repository maintainer | final README diff and approval recorded in the engineering journal |
| Onboarding narrative | `human` | repository maintainer | final revision and approval recorded in the engineering journal |

Any later change to a gated surface invalidates its earlier approval. Obtain human
review of the final revision. The Task 8 README and DOT/SVG work must stop for
that review before closure.

## Charter Evidence

- Filled by and date: Codex implementation agent, 2026-07-14.
- Evidence inspected: live MCP template retrieval at commit
  `ade485632adf335661fdede554e9de548fed1648`; `README.md`; `Cargo.toml`;
  `src/main.rs`; `src/templates.rs`; `build.rs`; `scripts/mcp-smoke.sh`;
  `.github/workflows/ci.yml`; roadmap, backlog, assumptions/limitations, and
  the open engineering journal.
- Unknowns or conflicts: current Codex and Claude Code discovery is not verified
  for the reasons recorded above. The current README predates the template system
  and will be revised under this workflow with final human review pending.
