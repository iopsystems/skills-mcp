//! The `iop` Claude Code plugin is a tree of symlinks into `skills/repository/`.
//! The vault and catalog skills call tools only the MCP server provides, so they
//! stay out of it.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const PLUGIN_SKILLS: &str = "plugins/iop/skills";
const SOURCE_CLUSTER: &str = "skills/repository";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn entry_names(dir: &Path) -> BTreeSet<String> {
    fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("read {}: {error}", dir.display()))
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect()
}

#[test]
fn plugin_links_every_repository_skill_and_nothing_else() {
    let linked = entry_names(&root().join(PLUGIN_SKILLS));
    let sources = entry_names(&root().join(SOURCE_CLUSTER));
    assert!(!sources.is_empty());
    assert_eq!(linked, sources);
}

#[test]
fn plugin_skills_are_relative_symlinks_to_their_repository_skill() {
    for name in entry_names(&root().join(PLUGIN_SKILLS)) {
        let link = root().join(PLUGIN_SKILLS).join(&name);
        let meta = fs::symlink_metadata(&link).unwrap();
        assert!(
            meta.file_type().is_symlink(),
            "{name} must be a symlink, not a copy"
        );
        let target = fs::read_link(&link).unwrap();
        assert_eq!(
            target,
            Path::new("../../../skills/repository").join(&name),
            "{name} links to the wrong place"
        );
        assert!(
            link.join("SKILL.md").is_file(),
            "{name} does not resolve to a skill"
        );
    }
}

#[test]
fn marketplace_lists_the_plugin_without_a_pinned_version() {
    let marketplace: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root().join(".claude-plugin/marketplace.json")).unwrap(),
    )
    .unwrap();
    let plugins = marketplace["plugins"].as_array().unwrap();
    assert_eq!(plugins.len(), 1);
    assert_eq!(plugins[0]["name"], "iop");
    assert_eq!(plugins[0]["source"], "./plugins/iop");

    let plugin: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root().join("plugins/iop/.claude-plugin/plugin.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(plugin["name"], "iop");
    // Without a version, an install's version is the commit SHA, so every merge
    // is an update.
    assert!(plugin.get("version").is_none());
}
