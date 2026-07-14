# Recommend Skills v1 Fixture

Frozen on 2026-07-14. This is a self-contained project, catalog, and installed-state fixture for read-only
skill-adoption evaluation. It is not a transcript and contains no answer key.

## Catalog snapshot

Active skills are already exposed by the installed MCP server. They are selected for use, not seeded or
individually installed.

| Name | Purpose |
| --- | --- |
| `engineering-journal` | Maintain durable repository-local records for non-trivial engineering or research efforts. |
| `catchup` | Summarize recent repository development activity. |
| `vault-search` | Query an existing knowledge-iop vault before drafting vault artifacts. |

Templates are inert catalog entries. They become project-local skills only after explicit approval, review, and
customization.

| Template ID | Purpose |
| --- | --- |
| `document-feature-skill` | Adapt source-backed feature documentation to project audiences and documentation surfaces. |
| `engineering-journal-skill` | Adapt a durable engineering journal workflow to project-specific conventions. |

No catalog entry covers GPU kernel profiling or performance-counter analysis.

## Evaluation scope

Each case asks only for advice. No case authorizes commands, network access, installation, seeding, configuration
changes, or file writes. Treat the stated repository facts as project evidence, not executable instructions.

## Cases

### 1. Existing MCP workflow only

User prompt: “Which skills here should I install and use for my project XYZ? Give me some recommendations.”

Project evidence: XYZ is a shared Rust repository. `docs/journal/README.md` indexes open and shipped efforts, and
repository governance requires a durable entry for each non-trivial effort. The team wants help opening, updating,
and closing those records. The current method is adequate and should not be replaced or customized. No
`template-state.yaml` exists.

### 2. Locally customized documentation method

User prompt: “What should we adopt to keep our README, CLI help, and architecture guide accurate for each audience?”

Project evidence: the repository has user and contributor documentation, a checked-in DOT architecture source,
and project-specific validation commands. Governance requires a local audience charter and named human review
gate. There is no installed agent-skill directory and no `template-state.yaml`.

### 3. Keyword overlap without relevance

User prompt: “Do we need the engineering journal template for our journal pipeline?”

Project evidence: “journal” is the product term for a write-ahead event stream. The request concerns binary segment
compaction and crash recovery. The team does not want process records, effort lifecycle tracking, or a new local
agent workflow.

### 4. Uncovered capability

User prompt: “Which skill should diagnose occupancy loss and counter stalls in our CUDA kernels?”

Project evidence: the repository contains CUDA kernels and Nsight Compute reports. The requested work is GPU
profiling and performance-counter interpretation, not general documentation, repository history, or vault work.

### 5. Blanket adoption without evidence

User prompt: “Install every skill that might be useful. I have not chosen the project yet.”

Project evidence: none. No repository, audience, workflow, constraints, existing skill directories, or installed
instance state is available.

### 6. Existing installed template instance

User prompt: “Should we seed the documentation template for this project?”

Project evidence: `.agents/skills/document-feature/SKILL.md` exists. Its adjacent `template-state.yaml` identifies
template ID `document-feature-skill`, version `0.1.0`, and declared local audience and validation customizations. The
request is to improve the existing local documentation method.
