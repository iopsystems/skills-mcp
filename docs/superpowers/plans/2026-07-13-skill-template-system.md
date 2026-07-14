# Skill Template System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use
> superpowers:subagent-driven-development (recommended) or
> superpowers:executing-plans to implement this plan.

**Goal:** Ship a validated catalog and seeding system for inert skill templates,
dogfood a project-specific feature-documentation instance, and replace the
repository README with a human-first, agent-usable guide.

**Architecture:** Keep `skills/` as the only invocable MCP instruction surface and
embed `templates/` through a separate validated registry. The two Rust-backed
read-only tools expose catalog metadata and manifest-declared template files;
`recommend-skills` and `seed-skill-template` retain project judgment and mutation
as agent workflows. Installed instances live in project harness directories and
carry content-addressed provenance so future upgrades can compare old base,
customized instance, and new base.

**Tech Stack:** Rust 2021, `rmcp`, `include_dir`, Serde/YAML/JSON, SHA-256,
Graphviz DOT/SVG, Markdown, GitHub Actions.

---

## Execution boundaries

- Work only in the `codex/skill-template-system` worktree.
- Use test-driven development for every Rust or skill-contract change: observe the
  new test fail, implement the smallest behavior, then observe it pass.
- Use the `superpowers:writing-skills` evaluation loop for each new active skill
  and template: baseline without the artifact, author, forward test with the
  artifact, then run a separate critic.
- Commit the catalog/templates/tooling before installing the dogfood instance.
  Rebuild from that clean commit so `template-state.yaml` can name a commit that
  actually contains the exact embedded bytes.
- Stop after rendering the new README and SVG for explicit human review. Do not
  mark the journal shipped or open a ready PR until the user approves the visual
  hierarchy, onboarding narrative, and diagram.
- The plan is temporary. Before the final commit, absorb durable implementation
  and verification details into the engineering journal and remove this file.

## Task 1: Define the embedded template registry and provenance boundary

**Files:**

- Create: `build.rs`
- Create: `src/templates.rs`
- Modify: `src/main.rs`
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`

- [ ] **Step 1: Add failing registry unit tests**

In a new `src/templates.rs`, write fixture-driven tests before implementation for:

- valid catalog and manifest parsing;
- a duplicate catalog ID;
- catalog ID not matching manifest ID;
- invalid semantic version;
- absolute, parent-traversing, empty, and backslash-containing declared paths;
- duplicate declared paths;
- missing entry point;
- undeclared retrieval path;
- file SHA-256 mismatch; and
- aggregate digest stability independent of manifest file order.

Use these public model shapes in the tests so the API is fixed before the loader:

```rust
#[derive(Clone, Debug, Deserialize)]
pub struct CatalogIndex {
    pub schema_version: u32,
    pub templates: Vec<CatalogEntry>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CatalogEntry {
    pub id: String,
    pub manifest: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateManifest {
    pub schema_version: u32,
    pub id: String,
    pub version: String,
    pub purpose: String,
    pub entrypoint: String,
    pub compatibility: Vec<String>,
    pub files: Vec<TemplateFile>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TemplateFile {
    pub path: String,
    pub sha256: String,
    pub merge_strategy: MergeStrategy,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MergeStrategy {
    ThreeWay,
    PreserveLocal,
}
```

Run:

```bash
cargo test templates::tests --locked
```

Expected: FAIL because parsing, validation, retrieval, and digest code does not
exist.

- [ ] **Step 2: Implement parsing, validation, and content retrieval**

Add `sha2 = "0.10"` and `semver = "1"` to dependencies. Implement:

```rust
pub struct TemplateRegistry {
    templates: BTreeMap<String, LoadedTemplate>,
}

impl TemplateRegistry {
    pub fn from_dir(root: &'static Dir<'static>) -> Result<Self>;
    pub fn summaries(&self) -> Vec<TemplateSummary>;
    pub fn get(&self, id: &str, path: Option<&str>) -> Result<TemplateBundle>;
}
```

Rules:

1. `catalog.yaml` must have `schema_version: 1` and unique IDs.
2. Each manifest path is relative, normalized, and contained by the embedded
   template root.
3. Each manifest must have `schema_version: 1`, a SemVer version, the same ID as
   its catalog row, a declared entry point, and unique declared file paths.
4. Declared file paths contain only normal UTF-8 components, use `/`, and reject
   empty components, `.`, `..`, absolute roots, and `\`.
5. Each file must exist, be UTF-8, and match its lowercase 64-character SHA-256.
6. Retrieval with `path: null` returns every declared file sorted by path;
   retrieval with a path returns exactly that declared file; undeclared paths are
   errors even when a matching embedded file exists.
7. Compute the aggregate digest by hashing sorted records in the exact form
   `path + "\0" + file_sha256 + "\n"`.

- [ ] **Step 3: Embed source provenance at build time**

Create `build.rs` using only `std::process::Command`. Emit:

```text
IOP_SKILLS_SOURCE_REPOSITORY=https://github.com/iopsystems/skills-mcp
IOP_SKILLS_SOURCE_COMMIT=<40 hex Git HEAD, GITHUB_SHA, or "unknown">
IOP_SKILLS_SOURCE_DIRTY=<true when tracked/untracked templates differ from HEAD>
```

Check dirty state with `git status --porcelain --untracked-files=all -- templates`.
Add `cargo:rerun-if-changed=templates`, a rerun marker for `GITHUB_SHA`, and rerun
markers for the paths returned by `git rev-parse --git-path HEAD` and
`git rev-parse --git-path index`. The Git-path lookup is required because this is
a linked worktree whose `.git` is a file. The registry response must expose this
metadata; `seed-skill-template` will refuse a provenance-complete install when
commit is unknown or dirty.

Declare `mod templates;` from `src/main.rs`, but do not wire MCP tools yet.

- [ ] **Step 4: Run focused and full tests**

```bash
cargo test templates::tests --locked
cargo test --locked
cargo fmt --all -- --check
```

Expected: all pass; existing 56 tests remain green.

- [ ] **Step 5: Commit the registry foundation**

```bash
git add Cargo.toml Cargo.lock build.rs src/main.rs src/templates.rs
git commit -m "Add validated skill template registry"
```

## Task 2: Add the two inert, versioned template bases

**Files:**

- Create: `templates/catalog.yaml`
- Create: `templates/engineering-journal-skill/template.yaml`
- Create: `templates/engineering-journal-skill/SKILL.md`
- Create: `templates/engineering-journal-skill/references/project-profile.md`
- Create: `templates/engineering-journal-skill/evals/trigger-evals.json`
- Create: `templates/document-feature-skill/template.yaml`
- Create: `templates/document-feature-skill/SKILL.md`
- Create: `templates/document-feature-skill/references/audience-charter.md`
- Create: `templates/document-feature-skill/evals/trigger-evals.json`
- Create: `tests/template_contracts.rs`

- [ ] **Step 1: Freeze baseline behavior before authoring**

Use isolated subagents that cannot read the proposed template. Give each the same
small fixture repository description and ask it to handle:

1. opening and later closing a journaled feature with a project-specific index;
2. documenting a CLI with defaults, modes, failure behavior, code contracts, and a
   nested architecture;
3. deciding whether a diagram requires human review; and
4. responding after three repeated comprehension failures.

Record only observed gaps and stable expectations in the open engineering journal;
do not paste transient transcripts into the repository.

- [ ] **Step 2: Add failing repository-level template contract tests**

In `tests/template_contracts.rs`, load repository files through
`env!("CARGO_MANIFEST_DIR")` and assert:

- exactly the two approved template IDs exist;
- both manifests use version `0.1.0` and list every distributed file;
- every listed digest matches;
- the `skills/` and `templates/` roots are distinct, no template ID exists as a
  directory under `skills/`, and the later MCP contract test proves the same
  runtime boundary;
- the engineering-journal template names the four lifecycle states, both operating
  modes, reconciliation, derived-document policy, and project profile;
- the document-feature template independently ranks human users, agent users,
  human developers, and coding agents;
- document-feature covers README, code documentation, and rendered CLI help;
- document-feature requires frozen tasks, authoritative-source verification,
  blind simulations, a separate critic, the three-round cap, DOT source, SVG
  output, textual equivalents, and risk-based human gates; and
- each eval JSON has named prompts and non-empty expectations covering positive,
  negative, and boundary cases.

Run:

```bash
cargo test --test template_contracts --locked
```

Expected: FAIL because the catalog and template files do not exist.

- [ ] **Step 3: Author the engineering-journal template**

Adapt the active `skills/engineering-journal/SKILL.md` without making the template
an MCP tool. Keep lifecycle and evidence invariants. Add a mandatory discovery
step that fills `references/project-profile.md` with:

- journal and index paths;
- frontmatter and lifecycle extensions;
- durable derived documents such as backlog, roadmap, and assumptions;
- project validation commands;
- whether intent-first or single-PR is preferred; and
- reconciliation boundaries.

Use the common Agent Skills frontmatter subset (`name`, `description`) and relative
supporting-file links.

- [ ] **Step 4: Author the feature-documentation template**

Adapt the Rezolus and SystemsLab workflow into a project-neutral template. Its
algorithm must be the ten-step loop approved in the journal: discovery, source
map, mode enumeration, frozen tasks, shared terminology, deterministic checks,
blind simulations and critic, risk-based human review, at most three targeted
revisions, and evidence/reporting.

`references/audience-charter.md` must require ranked emphasis and success criteria
for the four audiences, plus project type, prior knowledge, authoritative sources,
synchronized surfaces, verification commands, diagram tooling, and human review
gates. Include distinct checklists for README, code documentation, and actual
rendered CLI help. Make DOT authoritative, SVG committed, nesting/clusters
preferred where meaningful, and a text/table equivalent mandatory.

- [ ] **Step 5: Generate file digests and complete manifests**

For every declared file except `template.yaml`, compute:

```bash
shasum -a 256 templates/<template-id>/<declared-path>
```

Set `merge_strategy: three-way` for `SKILL.md` and project/audience profiles. Set
`merge_strategy: preserve-local` for eval fixtures so upgrades never delete local
cases silently. `templates/catalog.yaml` contains only schema version, template ID,
and manifest path; manifests remain authoritative for metadata.

- [ ] **Step 6: Run contract and registry tests**

```bash
cargo test --test template_contracts --locked
cargo test templates::tests --locked
cargo test --locked
```

Expected: all pass.

- [ ] **Step 7: Run forward and critic evaluations**

Give fresh isolated subagents only the template under test plus the same frozen
prompts from Step 1. A separate critic receives the output, frozen expectations,
and template. Revise only concrete misses; cap at three rounds. Update hashes after
each content edit. Record final findings and any remaining ambiguity in the
journal.

- [ ] **Step 8: Commit inert templates**

```bash
git add templates tests/template_contracts.rs docs/journal/2026-07-13-skill-templates-and-project-documentation.md
git commit -m "Add engineering and feature documentation templates"
```

## Task 3: Expose catalog and retrieval as read-only MCP tools

**Files:**

- Modify: `src/main.rs`
- Modify: `src/templates.rs`

- [ ] **Step 1: Add failing MCP contract tests**

Add tests in `src/main.rs` for:

- `skill_catalog` and `skill_template_get` tool names and JSON schemas;
- combined catalog ordering and `kind` values `active-skill` and `template`;
- catalog descriptions without skill/template bodies;
- a full template retrieval returning source provenance, manifest, aggregate
  digest, and all declared files;
- one-file retrieval;
- unknown template, undeclared path, and malformed arguments returning MCP tool
  errors rather than panics; and
- templates remaining absent as invocable skill tools.

Lock the retrieval schema to:

```json
{
  "type": "object",
  "properties": {
    "template_id": {"type": "string"},
    "path": {"type": "string"}
  },
  "required": ["template_id"],
  "additionalProperties": false
}
```

Run:

```bash
cargo test skill_template --locked
cargo test skill_catalog --locked
```

Expected: FAIL because the server does not own the registry or expose handlers.

- [ ] **Step 2: Wire the registry into `SkillsServer`**

Change construction to:

```rust
struct SkillsServer {
    skills: Arc<Vec<LoadedSkill>>,
    templates: Arc<TemplateRegistry>,
    vault: Arc<Mutex<VaultState>>,
}
```

Load the registry once in `main()` and fail startup with context on invalid
embedded templates. Add both tool declarations to `programmatic_tools()` and route
them before the instructional skill fallback.

`skill_catalog` returns:

```json
{
  "schema_version": 1,
  "items": [
    {"kind": "active-skill", "id": "...", "description": "..."},
    {"kind": "template", "id": "...", "version": "0.1.0", "purpose": "...", "compatibility": ["..."]}
  ]
}
```

`skill_template_get` returns the registry bundle plus:

```json
{
  "source": {
    "repository": "https://github.com/iopsystems/skills-mcp",
    "commit": "<build commit or unknown>",
    "dirty": false
  }
}
```

Never write a file, fetch a network resource, or accept a caller-supplied path
outside the manifest declaration.

- [ ] **Step 3: Run MCP and full Rust verification**

```bash
cargo test skill_template --locked
cargo test skill_catalog --locked
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
```

Expected: all pass.

- [ ] **Step 4: Commit read-only tools**

```bash
git add src/main.rs src/templates.rs
git commit -m "Expose read-only skill template tools"
```

## Task 4: Add the read-only `recommend-skills` adoption adviser

**Files:**

- Create: `skills/recommend-skills/SKILL.md`
- Create: `skills/recommend-skills/evals/trigger-evals.json`
- Modify: `tests/template_contracts.rs`

- [ ] **Step 1: Freeze and run baseline recommendation cases**

Use a fresh subagent without the new skill for these cases:

- a project needing an existing MCP workflow only;
- a project needing a locally customized documentation method;
- an irrelevant template despite keyword overlap;
- an uncovered capability;
- a request asking for every skill without project evidence; and
- a project already containing an installed template instance.

Capture whether the baseline over-recommends, mutates, confuses active skills with
templates, or misses evidence.

- [ ] **Step 2: Add failing skill/eval contract assertions**

Require the skill body to:

- call/read `skill_catalog` before deciding;
- inspect repository evidence and ask at most one narrow question only when the
  missing answer changes the recommendation;
- classify each considered item as `use through MCP`, `seed and customize`, `do
  not adopt`, or `missing capability`;
- recommend a minimal set with evidence and tradeoffs;
- recognize an installed instance and avoid duplicate seeding; and
- stop before mutation, installation, or invoking `seed-skill-template` without
  explicit user approval.

Run `cargo test --test template_contracts recommend --locked`; expect FAIL.

- [ ] **Step 3: Author the skill and evals**

The output contract is a compact table with columns `Recommendation`, `Action`,
`Project evidence`, and `Why/why not`, followed by the single next action. Include
the motivating prompt verbatim in an eval:

```text
Which skills here should I install and use for my project XYZ? Give me some recommendations.
```

Cover the six frozen cases plus a negative trigger unrelated to skill adoption.

- [ ] **Step 4: Forward-test and run a separate critic**

Use fresh isolated subagents with only the skill and catalog summaries. Verify no
mutation occurs and classifications stay role-correct. Revise concrete failures,
at most three rounds, and record findings in the journal.

- [ ] **Step 5: Run tests and commit**

```bash
cargo test --test template_contracts recommend --locked
cargo test --locked
git add skills/recommend-skills tests/template_contracts.rs docs/journal/2026-07-13-skill-templates-and-project-documentation.md
git commit -m "Add skill adoption recommendations"
```

## Task 5: Add provenance-aware `seed-skill-template`

**Files:**

- Create: `skills/seed-skill-template/SKILL.md`
- Create: `skills/seed-skill-template/evals/trigger-evals.json`
- Modify: `tests/template_contracts.rs`

- [ ] **Step 1: Freeze and run baseline seeding cases**

Use a disposable fixture project and a fresh subagent without the skill for:

- no existing harness convention;
- existing `.agents/skills`;
- existing `.claude/skills` with Claude-specific content;
- a safe empty dual-harness layout;
- an existing destination skill;
- dirty or unknown source provenance;
- a locally customized instance upgrade; and
- a missing historical base or digest mismatch.

No baseline subagent may write outside its disposable fixture.

- [ ] **Step 2: Add failing seeder contract assertions**

Require the body and evals to encode this state shape:

```yaml
schema_version: 1
instance_id: <stable UUID>
template:
  id: document-feature-skill
  version: 0.1.0
source:
  repository: https://github.com/iopsystems/skills-mcp
  commit: <40-hex immutable commit>
base:
  aggregate_sha256: <64 hex>
  files:
    - path: SKILL.md
      sha256: <64 hex>
      merge_strategy: three-way
installed_at: YYYY-MM-DD
last_upgraded_at: null
customizations:
  - path: references/audience-charter.md
    rationale: <project-specific reason>
```

The contract must discover conventions, retrieve only through
`skill_template_get`, require user approval before writes, preserve local files,
never overwrite a real directory/symlink/file silently, stop on dirty/unknown
source provenance, and perform old-base/current/new-base comparison for upgrades.

Run `cargo test --test template_contracts seed --locked`; expect FAIL.

- [ ] **Step 3: Author the workflow and evaluation matrix**

Use `.agents/skills` only as the default when no convention exists. For a dual
Codex/Claude project, prefer `.claude/skills -> ../.agents/skills` only when
`.claude/skills` is absent and every installed skill uses the common Agent Skills
subset. Otherwise preserve the existing directory and propose per-skill relative
links. Any link creation requires discovery verification after the write.

For upgrades, retrieve the recorded old base from the public repository at the
stored commit, verify the stored hashes, retrieve the new base from the tool, and
show a three-way merge proposal. Stop when history or hashes are unavailable.

- [ ] **Step 4: Forward-test and criticize in disposable fixtures**

Run all frozen cases with fresh subagents, inspect the filesystem diff after every
case, and use a separate critic to identify unsafe writes or provenance gaps.
Revise concrete misses for at most three rounds.

- [ ] **Step 5: Run tests and commit**

```bash
cargo test --test template_contracts seed --locked
cargo test --locked
git add skills/seed-skill-template tests/template_contracts.rs docs/journal/2026-07-13-skill-templates-and-project-documentation.md
git commit -m "Add provenance-aware template seeding"
```

## Task 6: Verify the embedded MCP surface end to end

**Files:**

- Create: `scripts/mcp-smoke.sh`
- Modify: `.github/workflows/ci.yml`
- Modify: `tests/template_contracts.rs`

- [ ] **Step 1: Add a failing smoke assertion**

Create a portable shell smoke test that starts `target/debug/iop-skills`, sends
initialize, initialized notification, tools/list, `skill_catalog`, and a
`skill_template_get` call for `document-feature-skill`, then checks JSON with `jq`.
Assert:

- active tools include `recommend-skills` and `seed-skill-template`;
- programmatic tools include `skill_catalog` and `skill_template_get`;
- no tool is named `document-feature-skill` or `engineering-journal-skill`; and
- retrieved files include `SKILL.md` and match the advertised aggregate digest.

Run `scripts/mcp-smoke.sh`; expect FAIL until the script and any response parsing
gaps are complete.

- [ ] **Step 2: Implement the portable smoke script and CI call**

Use `set -euo pipefail`, `mktemp`, and a cleanup trap. Do not install packages or
touch user configuration. Add a CI step after `cargo test`:

```yaml
- name: MCP smoke test
  run: ./scripts/mcp-smoke.sh
```

- [ ] **Step 3: Run the release-quality code checks**

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --release --locked
./scripts/mcp-smoke.sh
```

- [ ] **Step 4: Commit the tested distribution surface**

```bash
git add .github/workflows/ci.yml scripts/mcp-smoke.sh tests/template_contracts.rs
git commit -m "Test the embedded skill and template surface"
```

At this point run `git status --short`; it must be clean. Rebuild once more so the
binary reports this clean commit rather than an earlier dirty working tree.

## Task 7: Dogfood a customized feature-documentation instance

**Files:**

- Create: `.agents/skills/document-feature/SKILL.md`
- Create: `.agents/skills/document-feature/references/audience-charter.md`
- Create: `.agents/skills/document-feature/evals/trigger-evals.json`
- Create: `.agents/skills/document-feature/template-state.yaml`
- Create: `.claude/skills` (relative symlink to `../.agents/skills` if verified)
- Create: `tests/installed_instance.rs`

- [ ] **Step 1: Retrieve the clean committed base through MCP**

Build and call the tool rather than copying directly from `templates/`:

```bash
cargo build --locked
./scripts/mcp-smoke.sh
```

Capture the `skill_template_get` response for
`template_id=document-feature-skill`. Verify `source.dirty` is `false`, commit is
40 lowercase hex characters, and every file digest matches. Stop if not.

- [ ] **Step 2: Add failing installed-instance tests**

In `tests/installed_instance.rs`, require:

- canonical `.agents/skills/document-feature/SKILL.md` exists and uses only
  portable `name` and `description` frontmatter;
- state has a stable UUID, matching ID/version/source commit/base digests, install
  date, merge strategies, and declared customization rationale;
- current files can differ from base only when declared in `customizations`;
- the audience charter makes internal human users primary, while retaining agent
  users, human developers, and coding agents;
- the charter emphasizes install/use, the recommendations question, contribution
  from development experience, source-only distribution, and human review; and
- `.claude/skills/document-feature/SKILL.md` resolves to the canonical file.

Run `cargo test --test installed_instance --locked`; expect FAIL.

- [ ] **Step 3: Seed and customize the instance**

Follow `seed-skill-template` with the user-approved project profile. Preserve the
base algorithm and customize the audience charter and repository commands. Record
each changed base file in `template-state.yaml`; do not claim an unchanged install.

- [ ] **Step 4: Verify the dual-harness layout**

Because this repository has no prior `.claude/skills`, create the relative
directory symlink only if filesystem discovery succeeds:

```bash
test -L .claude/skills
test "$(readlink .claude/skills)" = "../.agents/skills"
test -f .agents/skills/document-feature/SKILL.md
test -f .claude/skills/document-feature/SKILL.md
```

Also run current Codex and Claude Code discovery when those CLIs are installed.
Record unavailable harness validation as a limitation, not a pass. If directory
discovery fails, replace only the directory link with a real `.claude/skills`
directory and a relative per-skill link.

- [ ] **Step 5: Run tests and commit the installed instance**

```bash
cargo test --test installed_instance --locked
cargo test --locked
git add .agents .claude tests/installed_instance.rs
git commit -m "Dogfood the feature documentation template"
```

## Task 8: Redesign the README and visual feedback loop

**Files:**

- Modify: `README.md`
- Modify: `Cargo.toml`
- Create: `docs/skill-feedback-loop.dot`
- Create: `docs/skill-feedback-loop.svg`
- Create: `scripts/render-diagrams.sh`
- Create: `tests/repository_documentation.rs`
- Modify: `.github/workflows/ci.yml`

- [ ] **Step 1: Freeze README user and developer tasks**

Before editing, record expected answers for these tasks in the journal:

1. Why should an internal engineer care about this repository?
2. How do they install the current source-built server on Apple Silicon macOS or
   Linux?
3. What exact question asks the agent for project-specific recommendations?
4. What is the difference between using an active skill and seeding a template?
5. How does a developer add or improve a skill/template from project experience?
6. Where are raw MCP, architecture, validation, and packaging limitations?
7. What happens from local learning to reviewed base-template improvement?

- [ ] **Step 2: Add failing documentation contract tests**

In `tests/repository_documentation.rs`, assert the README contains and links:

- a plain-language value proposition before build internals;
- source installation with explicit prerequisites and
  `cargo install --git https://github.com/iopsystems/skills-mcp --locked`;
- the exact recommendations question;
- an active-skill/template/installed-instance comparison table;
- MCP client configuration and a verified call path;
- contribution paths for active skills and inert templates;
- source-only/no-prebuilt status and packaging roadmap link;
- the SVG diagram and an equivalent numbered workflow or table; and
- valid local relative links and headings.

Add a DOT/SVG freshness test that hashes the DOT bytes with SHA-256 and requires
the SVG to contain `<!-- source-sha256: <digest> -->`. If `dot` is available, also
run `dot -Tsvg` into a temporary file and require successful parsing/rendering.

Run:

```bash
cargo test --test repository_documentation --locked
```

Expected: FAIL against the old README and missing diagram.

- [ ] **Step 3: Author and render the feedback-loop diagram**

Create DOT with nested clusters for project experience, local validation, shared
catalog/template reuse, observed customization, and reviewed base improvement.
Make the human review gate visually explicit. Keep node labels short and encode
the same steps in README prose/table.

`scripts/render-diagrams.sh` must work on macOS and Linux, require `dot`, compute
SHA-256 with `shasum -a 256` or `sha256sum`, render to a temporary SVG, inject the
source-digest comment after the XML declaration, and atomically replace the target.

Add Graphviz installation and `./scripts/render-diagrams.sh --check` to CI. The
check mode renders to a temporary file and verifies the committed digest marker
without replacing repository files.

- [ ] **Step 4: Rewrite README through the installed workflow**

Use this order:

1. purpose and who benefits;
2. prominent feedback-loop SVG plus textual equivalent;
3. install from source;
4. ask for recommendations and use an active skill;
5. active skill vs template vs installed instance;
6. seed/customize a template with approval;
7. contribute lessons and validate changes;
8. architecture/layout and maintainer reference;
9. raw MCP smoke/debug instructions; and
10. present limitations and roadmap.

Update the Cargo package description from “Claude skills” to portable agent skills
and workflows. Verify every command against the built binary or CLI help; do not
promise prebuilt artifacts, Homebrew, or plugin installation.

- [ ] **Step 5: Run deterministic documentation checks**

```bash
./scripts/render-diagrams.sh
dot -Tsvg docs/skill-feedback-loop.dot -o /tmp/skill-feedback-loop.svg
cargo test --test repository_documentation --locked
cargo test --locked
cargo build --release --locked
./scripts/mcp-smoke.sh
```

Expected: all pass and the committed SVG digest matches DOT.

- [ ] **Step 6: Run blind comprehension and critic checks**

Give fresh isolated subagents only the README (not the frozen answers) and assign
the seven frozen tasks separately across human-user, agent-user, human-developer,
and coding-agent perspectives. Then give a separate critic their answers, README,
and frozen expected outcomes. Revise concrete findings only, for at most three
rounds. Re-run deterministic checks after every round.

- [ ] **Step 7: Pause for the mandatory human review**

Show the user the rendered SVG and the complete README diff. Ask specifically for
approval of:

- information hierarchy and first-use path;
- clarity and prominence of the diagram;
- accuracy of the textual equivalent;
- internal-user motivation and contribution narrative; and
- whether the document feels appropriately human without obscuring agent-readable
  contracts.

Do not proceed to Task 9 until the user approves or requested changes are applied
and reverified.

- [ ] **Step 8: Commit the human-approved documentation**

```bash
git add README.md Cargo.toml Cargo.lock docs/skill-feedback-loop.dot docs/skill-feedback-loop.svg scripts/render-diagrams.sh tests/repository_documentation.rs .github/workflows/ci.yml
git commit -m "Redesign the skill repository guide"
```

## Task 9: Reconcile durable records and publish the implementation PR

**Files:**

- Modify: `docs/journal/2026-07-13-skill-templates-and-project-documentation.md`
- Modify: `docs/journal/README.md`
- Modify: `docs/backlog.md`
- Modify: `docs/roadmap.md` only if stage wording changed materially
- Modify: `docs/assumptions-and-limitations.md` only for verified new limits
- Delete: `docs/superpowers/plans/2026-07-13-skill-template-system.md`

- [ ] **Step 1: Add failing closure checks where mechanical**

Extend repository documentation tests to require that a `shipped` journal entry
has implementation paths, commit/PR evidence when known, deterministic verification
commands/results, behavioral evaluation findings, and human approval. Require the
backlog Now items completed by this change to be checked.

Run `cargo test --test repository_documentation journal --locked`; expect FAIL
while the entry remains open.

- [ ] **Step 2: Close and reconcile the journal**

Set the entry to `status: shipped`, update dates, and record:

- catalog/manifest/tool implementation paths;
- active skills and inert template paths;
- installed-instance provenance and actual harness validation;
- README/DOT/SVG implementation;
- baseline, forward, critic, deterministic, and human-review evidence;
- unresolved constraints and precise reopen conditions.

Update the journal index. Check completed initial backlog boxes; leave distribution,
survey, and template-evolution work open. Reconcile roadmap/assumptions only when
the implementation produced evidence that changes them.

- [ ] **Step 3: Remove the temporary plan**

Ensure all durable rationale, dead ends, interfaces, and verification evidence are
present in the journal, then delete this plan as required by the repository's
journal-first convention.

- [ ] **Step 4: Run the complete verification matrix from a clean tree**

```bash
./scripts/render-diagrams.sh --check
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --release --locked
./scripts/mcp-smoke.sh
git diff --check
```

Expected: all commands pass; no generated drift or untracked required file remains.

- [ ] **Step 5: Commit closure**

```bash
git add -A
git commit -m "Close skill template system journal"
```

- [ ] **Step 6: Request final code review**

Use `superpowers:requesting-code-review` against the merge base. Address only
verified actionable findings, rerun the full matrix, and keep the working tree
clean.

- [ ] **Step 7: Publish the implementation branch**

Use `superpowers:finishing-a-development-branch`. Push
`codex/skill-template-system` and open a ready PR summarizing:

- the active-skill/template/instance separation;
- catalog/retrieval security and provenance;
- the two workflows and two templates;
- dogfood harness layout and README visual; and
- deterministic, behavioral, and human-review evidence.
