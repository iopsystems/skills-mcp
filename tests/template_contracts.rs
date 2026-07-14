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
struct EvalFile {
    evals: Vec<EvalCase>,
}

#[derive(Debug, Deserialize)]
struct EvalCase {
    name: String,
    case: String,
    prompt: String,
    expectations: Vec<String>,
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn templates_root() -> PathBuf {
    repository_root().join("templates")
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

fn frontmatter_keys(body: &str) -> BTreeSet<String> {
    let body = body
        .strip_prefix("---\n")
        .expect("skill should start with YAML frontmatter");
    let (yaml, _) = body
        .split_once("\n---\n")
        .expect("skill frontmatter should have a closing fence");
    let frontmatter: BTreeMap<String, serde_yaml::Value> =
        serde_yaml::from_str(yaml).expect("skill frontmatter should be valid YAML");
    frontmatter.into_keys().collect()
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
    let engineering = read(templates_root().join("engineering-journal-skill/SKILL.md"));
    let document = read(templates_root().join("document-feature-skill/SKILL.md"));

    let expected_frontmatter = BTreeSet::from(["description".into(), "name".into()]);
    assert_eq!(frontmatter_keys(&engineering), expected_frontmatter);
    assert_eq!(
        frontmatter_keys(&document),
        BTreeSet::from(["description".into(), "name".into()])
    );
    assert!(engineering.contains("(references/project-profile.md)"));
    assert!(document.contains("(references/audience-charter.md)"));
    assert!(!engineering.contains("](/"));
    assert!(!document.contains("](/"));

    let step_numbers: Vec<_> = document
        .lines()
        .filter_map(|line| line.split_once(". "))
        .filter_map(|(number, _)| number.parse::<u8>().ok())
        .collect();
    assert_eq!(step_numbers, (1..=10).collect::<Vec<_>>());
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
            "at most three rounds",
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
            "independent rank",
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

#[test]
fn trigger_evals_cover_positive_negative_and_boundary_cases() {
    for id in TEMPLATE_IDS {
        let path = templates_root().join(id).join("evals/trigger-evals.json");
        let evals: EvalFile = serde_json::from_str(&read(&path))
            .unwrap_or_else(|error| panic!("failed to parse {}: {error}", path.display()));
        assert!(!evals.evals.is_empty());

        let mut categories = BTreeSet::new();
        for eval in evals.evals {
            assert!(!eval.name.trim().is_empty());
            assert!(!eval.prompt.trim().is_empty());
            assert!(!eval.expectations.is_empty());
            assert!(eval
                .expectations
                .iter()
                .all(|expectation| !expectation.trim().is_empty()));
            categories.insert(eval.case);
        }
        assert_eq!(
            categories,
            BTreeSet::from(["boundary".into(), "negative".into(), "positive".into()])
        );
    }
}
