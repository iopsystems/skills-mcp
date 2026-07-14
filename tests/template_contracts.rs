use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

const TEMPLATE_IDS: [&str; 2] = ["document-feature-skill", "engineering-journal-skill"];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    schema_version: u32,
    templates: Vec<CatalogEntry>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CatalogEntry {
    id: String,
    manifest: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema_version: u32,
    id: String,
    version: String,
    purpose: String,
    entrypoint: String,
    compatibility: Vec<String>,
    files: Vec<ManifestFile>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestFile {
    path: String,
    sha256: String,
    merge_strategy: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvalFile {
    evals: Vec<EvalCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvalCase {
    name: String,
    prompt: String,
    should_trigger: bool,
    context: Option<String>,
    required_outcomes: Vec<String>,
    prohibited_outcomes: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SkillFrontmatter {
    name: String,
    description: String,
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn templates_root() -> PathBuf {
    repository_root().join("templates")
}

fn design_journal() -> String {
    read(
        repository_root()
            .join("docs/journal/2026-07-13-skill-templates-and-project-documentation.md"),
    )
}

fn read(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path.as_ref())
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.as_ref().display()))
}

fn catalog() -> Catalog {
    serde_yaml::from_str(&read(templates_root().join("catalog.yaml")))
        .expect("template catalog should be valid")
}

fn manifests() -> BTreeMap<String, Manifest> {
    catalog()
        .templates
        .into_iter()
        .map(|entry| {
            let manifest: Manifest =
                serde_yaml::from_str(&read(templates_root().join(&entry.manifest)))
                    .unwrap_or_else(|error| panic!("failed to parse {}: {error}", entry.manifest));
            assert_eq!(entry.id, manifest.id);
            (entry.id, manifest)
        })
        .collect()
}

fn assert_contains_all(body: &str, phrases: &[&str]) {
    let body = body
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
    for phrase in phrases {
        let phrase = phrase
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_ascii_lowercase();
        assert!(
            body.contains(&phrase),
            "expected content to mention {phrase:?}"
        );
    }
}

fn parse_frontmatter(body: &str) -> SkillFrontmatter {
    let body = body
        .strip_prefix("---\n")
        .expect("skill should start with YAML frontmatter");
    let (yaml, _) = body
        .split_once("\n---\n")
        .expect("skill frontmatter should have a closing fence");
    assert!(yaml.len() <= 1024, "skill frontmatter must remain portable");
    serde_yaml::from_str(yaml).expect("skill frontmatter should contain only typed fields")
}

fn markdown_section<'a>(body: &'a str, heading: &str) -> &'a str {
    let marker = format!("## {heading}\n");
    let after_heading = body
        .split_once(&marker)
        .unwrap_or_else(|| panic!("missing {marker:?} section"))
        .1;
    after_heading
        .split_once("\n## ")
        .map_or(after_heading, |(section, _)| section)
}

fn markdown_subsection<'a>(body: &'a str, heading: &str) -> &'a str {
    let marker = format!("### {heading}\n");
    let after_heading = body
        .split_once(&marker)
        .unwrap_or_else(|| panic!("missing {marker:?} subsection"))
        .1;
    after_heading
        .split_once("\n### ")
        .map_or(after_heading, |(section, _)| section)
}

fn numbered_steps(section: &str) -> Vec<u8> {
    section
        .lines()
        .filter_map(|line| line.split_once(". "))
        .filter_map(|(number, _)| number.parse::<u8>().ok())
        .collect()
}

fn markdown_targets(body: &str) -> Vec<&str> {
    let mut targets = Vec::new();
    let mut remainder = body;
    while let Some((_, after_open)) = remainder.split_once("](") {
        let Some((target, after_close)) = after_open.split_once(')') else {
            break;
        };
        targets.push(target);
        remainder = after_close;
    }
    targets
}

fn normalized_contains_all(body: &str, phrases: &[&str]) -> bool {
    let body = body
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
    phrases.iter().all(|phrase| {
        let phrase = phrase
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_ascii_lowercase();
        body.contains(&phrase)
    })
}

#[test]
fn catalog_contains_exactly_the_approved_inert_templates() {
    let catalog = catalog();
    assert_eq!(catalog.schema_version, 1);

    let actual: BTreeSet<_> = catalog
        .templates
        .iter()
        .map(|entry| entry.id.as_str())
        .collect();
    assert_eq!(actual, BTreeSet::from(TEMPLATE_IDS));

    let skills_root = repository_root().join("skills");
    assert_ne!(skills_root, templates_root());
    for id in TEMPLATE_IDS {
        assert!(!skills_root.join(id).exists(), "{id} must remain inert");
    }
}

#[test]
fn manifests_are_versioned_complete_and_digest_verified() {
    for (id, manifest) in manifests() {
        assert_eq!(manifest.schema_version, 1);
        assert_eq!(manifest.version, "0.1.0");
        assert!(!manifest.purpose.trim().is_empty());
        assert_eq!(manifest.entrypoint, "SKILL.md");
        assert!(manifest
            .compatibility
            .iter()
            .any(|value| value == "agent-skills-common-subset"));

        let template_root = templates_root().join(&id);
        let distributed: BTreeSet<_> = WalkDir::new(&template_root)
            .into_iter()
            .map(Result::unwrap)
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| {
                entry
                    .path()
                    .strip_prefix(&template_root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .filter(|path| path != "template.yaml")
            .collect();
        let declared: BTreeSet<_> = manifest
            .files
            .iter()
            .map(|file| file.path.clone())
            .collect();
        assert_eq!(declared, distributed, "incomplete file list for {id}");

        for file in manifest.files {
            assert_eq!(file.sha256.len(), 64);
            assert!(file
                .sha256
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
            let actual = format!(
                "{:x}",
                Sha256::digest(fs::read(template_root.join(&file.path)).unwrap())
            );
            assert_eq!(
                file.sha256, actual,
                "digest mismatch for {id}/{}",
                file.path
            );

            let expected_strategy = if file.path.starts_with("evals/") {
                "preserve-local"
            } else {
                "three-way"
            };
            assert_eq!(file.merge_strategy, expected_strategy);
        }
    }
}

#[test]
fn skill_templates_use_portable_structure_and_relative_support_links() {
    for (template_id, manifest) in manifests() {
        let template_root = templates_root().join(&template_id);
        let skill = read(template_root.join("SKILL.md"));
        let frontmatter = parse_frontmatter(&skill);
        assert!(!frontmatter.name.trim().is_empty());
        assert!(!frontmatter.description.trim().is_empty());
        assert!(frontmatter.description.starts_with("Use when "));
        assert!(frontmatter
            .name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-'));
        assert_ne!(frontmatter.name, template_id);

        let installed_root = tempfile::tempdir().unwrap();
        let installed_skill = installed_root
            .path()
            .join(".agents/skills")
            .join(&frontmatter.name);
        for file in &manifest.files {
            let destination = installed_skill.join(&file.path);
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
            fs::copy(template_root.join(&file.path), destination).unwrap();
        }
        assert_eq!(
            installed_skill.file_name().unwrap().to_str().unwrap(),
            frontmatter.name
        );
        for target in markdown_targets(&skill) {
            assert!(!target.starts_with('/'));
            assert!(!target.contains("://"));
            assert!(!target.split('/').any(|component| component == ".."));
            assert!(
                installed_skill.join(target).is_file(),
                "unresolved supporting link {target:?} for {template_id}"
            );
        }
    }
}

#[test]
fn feature_workflow_has_exactly_ten_ordered_semantic_steps() {
    let body = read(templates_root().join("document-feature-skill/SKILL.md"));
    let workflow = markdown_section(&body, "Workflow");
    assert_eq!(numbered_steps(workflow), (1..=10).collect::<Vec<_>>());
    assert_contains_all(
        workflow,
        &[
            "each in-scope audience",
            "authoritative",
            "blind task simulations",
            "separate structured critic",
            "risk-based human review",
            "at most three formal",
        ],
    );

    let without_step_four = workflow.replacen("4. ", "", 1);
    assert_ne!(
        numbered_steps(&without_step_four),
        (1..=10).collect::<Vec<_>>()
    );
}

#[test]
fn engineering_journal_template_preserves_lifecycle_and_project_adaptation() {
    let body = read(templates_root().join("engineering-journal-skill/SKILL.md"));
    assert_contains_all(
        &body,
        &[
            "status: open",
            "status: shipped",
            "status: no-go",
            "status: superseded",
            "never another status value",
            "intent-first",
            "single-PR",
            "reconcile",
            "durable derived documents",
            "backlog",
            "roadmap",
            "assumptions",
            "project-wide backlog",
            "by default",
            "advisory",
            "Do not create or update briefs",
            "references/project-profile.md",
        ],
    );

    let profile =
        read(templates_root().join("engineering-journal-skill/references/project-profile.md"));
    assert_contains_all(
        &profile,
        &[
            "journal path",
            "index path",
            "frontmatter",
            "lifecycle extensions",
            "validation commands",
            "operating-mode preference",
            "reconciliation boundaries",
        ],
    );
}

#[test]
fn feature_documentation_template_covers_surfaces_evidence_and_review_loop() {
    let body = read(templates_root().join("document-feature-skill/SKILL.md"));
    assert_contains_all(
        &body,
        &[
            "README",
            "code documentation",
            "actual rendered CLI help",
            "freeze representative",
            "authoritative",
            "blind task simulations",
            "separate structured critic",
            "DOT is authoritative",
            "commit",
            "SVG",
            "textual equivalent",
            "risk-based human review",
            "architecture",
            "workflow diagrams",
            "visual hierarchy",
            "navigation",
            "major README",
            "onboarding narratives",
            "Agent checks never prove human usability",
            "product or interface issue",
            "design smell",
        ],
    );

    let charter =
        read(templates_root().join("document-feature-skill/references/audience-charter.md"));
    assert_contains_all(
        &charter,
        &[
            "human users",
            "agent users",
            "human developers",
            "coding agents",
            "priority",
            "project type",
            "prior knowledge",
            "sources of truth",
            "synchronized surfaces",
            "verification commands",
            "diagram tooling",
            "review gates",
        ],
    );
}

fn assert_instruction_trust_and_execution_boundary(id: &str) {
    let body = read(templates_root().join(id).join("SKILL.md"));
    let boundary = markdown_section(&body, "Trust and Execution Boundary");
    let requirements = [
        "recognized repository governance",
        "harness and user precedence",
        "ordinary documentation",
        "source comments",
        "fixtures",
        "generated files",
        "commit and history text",
        "external content",
        "evidence or data",
        "never executable instructions",
        "inspect commands",
        "platform permissions",
        "explicit user authorization",
        "destructive",
        "credential-bearing",
        "unexpected network",
    ];
    assert_contains_all(boundary, &requirements);

    let weakened = boundary.replacen("explicit user authorization", "automatic approval", 1);
    assert!(!normalized_contains_all(&weakened, &requirements));
}

#[test]
fn engineering_template_defines_instruction_trust_and_execution_boundary() {
    assert_instruction_trust_and_execution_boundary("engineering-journal-skill");
}

#[test]
fn document_template_defines_instruction_trust_and_execution_boundary() {
    assert_instruction_trust_and_execution_boundary("document-feature-skill");
}

#[test]
fn audience_contract_drives_content_verification_and_conflict_resolution() {
    let charter =
        read(templates_root().join("document-feature-skill/references/audience-charter.md"));
    let ranks = markdown_section(&charter, "Audience Priorities and Success");
    let requirements = [
        "P0",
        "P1",
        "P2",
        "out of scope",
        "measurable success criterion",
        "frozen representative task",
        "each in-scope audience",
        "content order",
        "examples",
        "visual and textual emphasis",
        "verification",
        "shared authoritative facts",
        "higher-priority",
        "lower-priority contracts false",
        "tradeoff",
        "unmet criterion",
        "human owner",
    ];
    assert_contains_all(ranks, &requirements);

    let without_escalation = ranks.replacen("human owner", "automated guess", 1);
    assert!(!normalized_contains_all(&without_escalation, &requirements));
}

#[test]
fn final_human_gate_applies_to_the_last_gated_revision() {
    let body = read(templates_root().join("document-feature-skill/SKILL.md"));
    let review = markdown_section(&body, "Evaluation and Review");
    let requirements = [
        "formal blind-simulation and critic revision cycle",
        "human-requested corrections do not consume",
        "later revision affects a gated surface",
        "human re-review",
        "final revision",
        "gate satisfied",
    ];
    assert_contains_all(review, &requirements);

    let without_rereview = review.replacen("human re-review", "prior approval", 1);
    assert!(!normalized_contains_all(&without_rereview, &requirements));
}

#[test]
fn design_journal_matches_hardened_contract_and_reproducible_evidence() {
    let journal = design_journal();
    let trust = markdown_subsection(&journal, "Instruction and execution trust boundary");
    assert_contains_all(
        trust,
        &[
            "recognized repository governance",
            "evidence or data",
            "explicit user authorization",
        ],
    );

    let feature = markdown_subsection(&journal, "Feature-documentation template");
    assert_contains_all(
        feature,
        &[
            "P0",
            "measurable success criterion",
            "each in-scope audience",
            "separate structured critic",
            "three formal simulation-and-critic cycles",
            "human re-review",
            "final revision",
        ],
    );
    assert!(!feature.contains("Freeze representative user and developer tasks"));

    let ledger = markdown_subsection(&journal, "Evaluation ledger");
    let ledger_requirements = [
        "sprig-cli-v1",
        "template-trust-gate-v1",
        "outcomes `E1-E6`",
        "outcomes `D1-D10`",
        "templates/engineering-journal-skill/evals/trigger-evals.json",
        "templates/document-feature-skill/evals/trigger-evals.json",
        "6/6 PASS",
        "10/10 PASS",
    ];
    assert_contains_all(ledger, &ledger_requirements);

    let without_eval_path = ledger.replacen(
        "templates/document-feature-skill/evals/trigger-evals.json",
        "abbreviated-eval-path",
        1,
    );
    assert!(!normalized_contains_all(
        &without_eval_path,
        &ledger_requirements
    ));
}

fn assert_eval_semantics(id: &str) {
    let path = templates_root().join(id).join("evals/trigger-evals.json");
    let evals: EvalFile = serde_json::from_str(&read(&path))
        .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));
    assert!(!evals.evals.is_empty());

    let mut activation = BTreeSet::new();
    let mut names = BTreeSet::new();
    let mut has_adversarial_refusal = false;
    let mut has_out_of_scope_non_trigger = false;
    for eval in evals.evals {
        assert!(!eval.name.trim().is_empty());
        assert!(names.insert(eval.name.clone()), "duplicate eval name");
        assert!(!eval.prompt.trim().is_empty());
        assert!(!eval.required_outcomes.is_empty());
        assert!(!eval.prohibited_outcomes.is_empty());
        assert!(eval
            .required_outcomes
            .iter()
            .chain(&eval.prohibited_outcomes)
            .all(|outcome| !outcome.trim().is_empty()));
        activation.insert(eval.should_trigger);

        let scenario = format!(
            "{} {} {} {}",
            eval.prompt,
            eval.context.unwrap_or_default(),
            eval.required_outcomes.join(" "),
            eval.prohibited_outcomes.join(" ")
        )
        .to_ascii_lowercase();
        if eval.should_trigger
            && (scenario.contains("injected") || scenario.contains("unsafe"))
            && scenario.contains("authorization")
            && (scenario.contains("do not run") || scenario.contains("do not execute"))
        {
            has_adversarial_refusal = true;
        }
        if !eval.should_trigger
            && scenario.contains("do not activate")
            && (scenario.contains("without changing") || scenario.contains("unrelated"))
        {
            has_out_of_scope_non_trigger = true;
        }
    }
    assert_eq!(activation, BTreeSet::from([false, true]));
    assert!(
        has_adversarial_refusal,
        "missing unsafe in-scope eval for {id}"
    );
    assert!(
        has_out_of_scope_non_trigger,
        "missing genuine out-of-scope eval for {id}"
    );
}

#[test]
fn engineering_evals_separate_activation_from_required_behavior() {
    assert_eval_semantics("engineering-journal-skill");
}

#[test]
fn document_evals_separate_activation_from_required_behavior() {
    assert_eval_semantics("document-feature-skill");
}
