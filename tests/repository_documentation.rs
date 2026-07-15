use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn readme_leads_with_value_and_has_verified_source_installation() {
    let readme = read("README.md");
    let purpose = readme
        .find("## Why use this repository")
        .expect("plain-language value proposition heading");
    let install = readme
        .find("## Install from source")
        .expect("source installation heading");
    let internals = readme
        .find("## Architecture and repository layout")
        .expect("architecture heading");
    assert!(purpose < install && install < internals);
    assert!(readme.contains("Apple Silicon macOS"));
    assert!(readme.contains("Linux"));
    assert!(readme.contains("Git"));
    assert!(readme.contains("Rust toolchain"));
    assert!(readme.contains("MCP-capable"));
    assert!(
        readme.contains("cargo install --git https://github.com/iopsystems/skills-mcp --locked")
    );
}

#[test]
fn readme_exposes_recommendation_and_adoption_paths() {
    let readme = read("README.md");
    assert!(readme.contains(
        "which skills here should I install and use for my project XYZ? Give me some recommendations"
    ));
    assert!(readme.contains("| Active skill | Inert template | Installed instance |"));
    for term in [
        "use through MCP",
        "seed and customize locally",
        "provenance",
        "separate explicit approval",
        "skills/<name>/SKILL.md",
        "templates/catalog.yaml",
        "templates/<id>/",
    ] {
        assert!(readme.contains(term), "README missing {term:?}");
    }
}

#[test]
fn readme_documents_mcp_configuration_contribution_and_limits() {
    let readme = read("README.md");
    for term in [
        "\"mcpServers\"",
        "skill_catalog",
        "skill_template_get",
        "./scripts/mcp-smoke.sh",
        "cargo test --locked",
        "cargo clippy --all-targets --locked -- -D warnings",
        "source-only",
        "no prebuilt",
        "docs/roadmap.md",
        "docs/assumptions-and-limitations.md",
    ] {
        assert!(readme.contains(term), "README missing {term:?}");
    }
}

#[test]
fn raw_debug_example_invokes_the_binary_it_builds() {
    let readme = read("README.md");
    let raw = readme
        .split_once("## Raw MCP smoke and debugging")
        .expect("raw MCP section")
        .1;
    assert!(raw.contains("cargo build --locked"));
    assert!(raw.contains("./target/debug/iop-skills"));
    assert!(raw.contains("debug binary"));
    assert!(!raw.contains("release binary"));
}

#[test]
fn readme_features_diagram_and_exact_textual_equivalent() {
    let readme = read("README.md");
    assert!(readme.contains("![Skill feedback loop](docs/skill-feedback-loop.svg)"));
    let diagram = readme
        .find("![Skill feedback loop]")
        .expect("feedback-loop SVG");
    let install = readme
        .find("## Install from source")
        .expect("install section");
    assert!(
        diagram < install,
        "diagram should be prominent before installation"
    );
    for (number, phrase) in [
        (1, "Capture project experience"),
        (2, "Validate the candidate"),
        (3, "Pass the human review gate"),
        (4, "Share an active skill or inert template"),
        (5, "Use or seed it in projects"),
        (6, "Observe local customization"),
        (7, "Propose a reviewed base improvement"),
    ] {
        assert!(
            readme.contains(&format!("{number}. **{phrase}")),
            "missing exact textual workflow step {number}"
        );
    }
}

#[test]
fn readme_local_links_and_heading_anchors_resolve() {
    let readme = read("README.md");
    let headings: Vec<String> = readme
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .map(markdown_anchor)
        .collect();

    for target in markdown_targets(&readme) {
        if target.starts_with("http://") || target.starts_with("https://") {
            continue;
        }
        let (path, anchor) = target.split_once('#').unwrap_or((&target, ""));
        if path.is_empty() {
            assert!(
                headings.iter().any(|heading| heading == anchor),
                "missing heading #{anchor}"
            );
            continue;
        }
        let local = root().join(path);
        assert!(
            local.exists(),
            "README link target does not exist: {target}"
        );
        if !anchor.is_empty() {
            let linked = fs::read_to_string(&local).expect("linked Markdown should be UTF-8");
            let linked_headings: Vec<String> = linked
                .lines()
                .filter_map(|line| line.trim_start_matches('#').strip_prefix(' '))
                .map(markdown_anchor)
                .collect();
            assert!(
                linked_headings.iter().any(|heading| heading == anchor),
                "missing anchor {anchor:?} in {path}"
            );
        }
    }
}

#[test]
fn dot_and_svg_are_current_and_renderable() {
    let dot_path = root().join("docs/skill-feedback-loop.dot");
    let svg_path = root().join("docs/skill-feedback-loop.svg");
    let dot = fs::read(&dot_path).expect("authoritative DOT source");
    let svg = fs::read_to_string(&svg_path).expect("rendered SVG");
    let digest = format!("{:x}", Sha256::digest(&dot));
    assert!(svg.contains(&format!("<!-- source-sha256: {digest} -->")));

    if Command::new("dot").arg("-V").output().is_ok() {
        let output = tempfile::NamedTempFile::new().expect("temporary SVG");
        let status = Command::new("dot")
            .args(["-Tsvg"])
            .arg(&dot_path)
            .arg("-o")
            .arg(output.path())
            .status()
            .expect("run dot");
        assert!(status.success(), "Graphviz should parse and render DOT");
        assert!(fs::metadata(output.path()).unwrap().len() > 0);
    }
}

#[test]
fn render_script_and_ci_enforce_portable_freshness_checks() {
    let script = read("scripts/render-diagrams.sh");
    assert!(script.starts_with("#!/bin/sh\n"));
    assert!(script.contains("--check"));
    assert!(script.contains("shasum -a 256"));
    assert!(script.contains("sha256sum"));
    assert!(script.contains("mktemp"));
    assert!(script.contains("mv"));
    assert!(script.contains("chmod 0644"));
    assert!(
        !script.contains("cmp -s"),
        "freshness must not depend on Graphviz-version-specific SVG bytes"
    );

    let ci = read(".github/workflows/ci.yml");
    assert!(ci.contains("graphviz"));
    assert!(ci.contains("./scripts/render-diagrams.sh --check"));
}

#[test]
fn cargo_description_is_agent_portable() {
    let cargo = read("Cargo.toml");
    assert!(cargo.contains("agent skills and workflows"));
    assert!(!cargo.contains("bundle of Claude skills"));
}

fn markdown_targets(markdown: &str) -> Vec<String> {
    let bytes = markdown.as_bytes();
    let mut targets = Vec::new();
    let mut index = 0;
    while index + 1 < bytes.len() {
        if bytes[index] == b']' && bytes[index + 1] == b'(' {
            let start = index + 2;
            if let Some(relative_end) = markdown[start..].find(')') {
                targets.push(markdown[start..start + relative_end].to_owned());
                index = start + relative_end + 1;
                continue;
            }
        }
        index += 1;
    }
    targets
}

fn markdown_anchor(heading: &str) -> String {
    heading
        .trim()
        .to_ascii_lowercase()
        .chars()
        .filter_map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == ' ' {
                Some(if character == ' ' { '-' } else { character })
            } else {
                None
            }
        })
        .collect()
}
