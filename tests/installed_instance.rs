use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use serde::Deserialize;
use sha2::{Digest, Sha256};

const SOURCE_COMMIT: &str = "ade485632adf335661fdede554e9de548fed1648";
const TEMPLATE_ID: &str = "document-feature-skill";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Frontmatter {
    name: String,
    description: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TemplateState {
    schema_version: u32,
    instance_id: String,
    template: TemplateIdentity,
    source: Source,
    base: Base,
    installed_at: String,
    last_upgraded_at: Option<String>,
    customizations: Vec<Customization>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TemplateIdentity {
    id: String,
    version: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    repository: String,
    commit: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Base {
    aggregate_sha256: String,
    files: Vec<BaseFile>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BaseFile {
    path: String,
    sha256: String,
    merge_strategy: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Customization {
    path: String,
    rationale: String,
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
    files: Vec<BaseFile>,
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path.as_ref())
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.as_ref().display()))
}

fn sha256(body: &str) -> String {
    format!("{:x}", Sha256::digest(body.as_bytes()))
}

fn aggregate_sha256(files: &[BaseFile]) -> String {
    let mut files = files.iter().collect::<Vec<_>>();
    files.sort_by(|left, right| left.path.cmp(&right.path));

    let mut digest = Sha256::new();
    for file in files {
        digest.update(file.path.as_bytes());
        digest.update([0]);
        digest.update(file.sha256.as_bytes());
        digest.update(b"\n");
    }
    format!("{:x}", digest.finalize())
}

fn parse_frontmatter(body: &str) -> Frontmatter {
    let body = body
        .strip_prefix("---\n")
        .expect("skill should begin with YAML frontmatter");
    let (yaml, _) = body
        .split_once("\n---\n")
        .expect("skill frontmatter should have a closing fence");
    serde_yaml::from_str(yaml).expect("frontmatter should use only portable typed fields")
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn is_uuid_v4(value: &str) -> bool {
    let bytes = value.as_bytes();
    value.len() == 36
        && [8, 13, 18, 23].iter().all(|index| bytes[*index] == b'-')
        && value.chars().enumerate().all(|(index, character)| {
            [8, 13, 18, 23].contains(&index)
                || character.is_ascii_digit()
                || ('a'..='f').contains(&character)
        })
        && bytes[14] == b'4'
        && matches!(bytes[19], b'8' | b'9' | b'a' | b'b')
}

fn assert_contains_all(body: &str, phrases: &[&str]) {
    let normalized = body
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase();
    for phrase in phrases {
        assert!(
            normalized.contains(&phrase.to_ascii_lowercase()),
            "expected installed audience charter to contain {phrase:?}"
        );
    }
}

#[test]
fn installed_skill_uses_portable_frontmatter_and_preserves_the_base_algorithm() {
    let installed = read(root().join(".agents/skills/document-feature/SKILL.md"));
    let base = read(root().join("templates/document-feature-skill/SKILL.md"));
    let frontmatter = parse_frontmatter(&installed);

    assert_eq!(frontmatter.name, "document-feature");
    assert!(frontmatter.description.starts_with("Use when "));
    assert!((1..=1024).contains(&frontmatter.description.chars().count()));
    assert_eq!(
        installed, base,
        "the installed workflow must preserve the base algorithm"
    );
}

#[test]
fn installed_state_records_the_exact_clean_base_and_declared_customizations() {
    let state_path = root().join(".agents/skills/document-feature/template-state.yaml");
    let state: TemplateState =
        serde_yaml::from_str(&read(&state_path)).expect("installed state should be valid YAML");
    let manifest: Manifest = serde_yaml::from_str(&read(
        root().join("templates/document-feature-skill/template.yaml"),
    ))
    .expect("template manifest should be valid YAML");

    assert_eq!(state.schema_version, 1);
    assert!(is_uuid_v4(&state.instance_id));
    assert_eq!(state.template.id, TEMPLATE_ID);
    assert_eq!(state.template.version, manifest.version);
    assert_eq!(
        state.source.repository,
        "https://github.com/iopsystems/skills-mcp"
    );
    assert_eq!(state.source.commit, SOURCE_COMMIT);
    assert!(is_lower_hex(&state.source.commit, 40));
    assert_eq!(state.installed_at, "2026-07-14");
    assert_eq!(state.last_upgraded_at, None);
    assert!(is_lower_hex(&state.base.aggregate_sha256, 64));

    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.id, TEMPLATE_ID);
    assert_eq!(manifest.entrypoint, "SKILL.md");
    assert!(!manifest.purpose.is_empty());
    assert!(manifest.compatibility.iter().any(|value| value == "codex"));
    assert_eq!(
        state.base.aggregate_sha256,
        "cb18b1580d31580c63fa14d34c8a2438ebe55e4a67726e73c80be2a5369e423e"
    );
    assert_eq!(
        state.base.aggregate_sha256,
        aggregate_sha256(&state.base.files),
        "aggregate provenance must be derived from the recorded base-file rows"
    );

    let manifest_rows: Vec<_> = manifest
        .files
        .iter()
        .map(|file| (&file.path, &file.sha256, &file.merge_strategy))
        .collect();
    let state_rows: Vec<_> = state
        .base
        .files
        .iter()
        .map(|file| (&file.path, &file.sha256, &file.merge_strategy))
        .collect();
    assert_eq!(state_rows, manifest_rows);

    let declared: BTreeSet<_> = state
        .customizations
        .iter()
        .map(|customization| customization.path.as_str())
        .collect();
    assert_eq!(
        declared.len(),
        state.customizations.len(),
        "customization paths must be unique"
    );
    assert_eq!(declared, BTreeSet::from(["references/audience-charter.md"]));
    let base_paths: BTreeSet<_> = state
        .base
        .files
        .iter()
        .map(|file| file.path.as_str())
        .collect();
    assert!(declared.is_subset(&base_paths));
    assert!(state
        .customizations
        .iter()
        .all(|customization| !customization.rationale.trim().is_empty()));

    for file in &manifest.files {
        let base = read(
            root()
                .join("templates/document-feature-skill")
                .join(&file.path),
        );
        assert_eq!(sha256(&base), file.sha256);

        let installed = read(
            root()
                .join(".agents/skills/document-feature")
                .join(&file.path),
        );
        let differs = installed != base;
        assert_eq!(
            differs,
            declared.contains(file.path.as_str()),
            "{} must differ from base exactly when declared customized",
            file.path
        );
    }
}

#[test]
fn audience_charter_encodes_the_approved_project_profile() {
    let charter =
        read(root().join(".agents/skills/document-feature/references/audience-charter.md"));

    assert_contains_all(
        &charter,
        &[
            "internal organizational engineers",
            "human users",
            "agent users",
            "human developers",
            "coding agents",
            "which skills here should I install and use for my project XYZ? Give me some recommendations",
            "cargo install --git https://github.com/iopsystems/skills-mcp --locked",
            "development experience",
            "Homebrew",
            "install.sh",
            "DOT",
            "SVG",
            "human review",
            "test -d .claude/skills",
            "../../.agents/skills/document-feature",
            "missing platform binary",
            "Unknown command: /document-feature",
            "Not logged in",
        ],
    );

    assert!(charter.contains("| Human users | `P0`"));
    assert!(charter.contains("| Agent users | `P1`"));
    assert!(charter.contains("| Human developers | `P1`"));
    assert!(charter.contains("| Coding agents | `P1`"));
    assert!(!charter.contains("docs/superpowers/plans/2026-07-13-skill-template-system.md"));
    assert!(!charter.contains("`test -L .claude/skills`,"));
    assert!(!charter.contains("project-skill discovery must be tested"));
}

#[test]
fn claude_fallback_uses_a_real_skills_directory_and_relative_per_skill_link() {
    let skills = root().join(".claude/skills");
    let metadata = fs::symlink_metadata(&skills).expect(".claude/skills should exist");
    assert!(metadata.file_type().is_dir());

    let link = skills.join("document-feature");
    let metadata = fs::symlink_metadata(&link).expect("Claude skill link should exist");
    assert!(metadata.file_type().is_symlink());
    assert_eq!(
        fs::read_link(&link).unwrap(),
        Path::new("../../.agents/skills/document-feature")
    );

    let canonical = fs::canonicalize(root().join(".agents/skills/document-feature/SKILL.md"))
        .expect("canonical skill should resolve");
    let claude = fs::canonicalize(root().join(".claude/skills/document-feature/SKILL.md"))
        .expect("Claude skill should resolve");
    assert_eq!(claude, canonical);
}
