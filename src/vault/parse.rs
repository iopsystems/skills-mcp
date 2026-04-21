//! Parse vault markdown files into structured artifacts.
//!
//! A vault artifact is a markdown file with a YAML frontmatter block
//! delimited by `---` lines. The frontmatter's `type:` and `id:` fields
//! are required; everything else is type-specific and parsed lazily.

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};
use serde_yaml::Value;
use walkdir::WalkDir;

use crate::vault::schema;

/// A parsed artifact — frontmatter fields flattened into owned values,
/// plus the outgoing graph edges.
#[derive(Debug, Clone)]
pub struct Artifact {
    pub id: String,
    pub r#type: String,
    pub status: Option<String>,
    pub author: Option<String>,
    pub created: Option<String>,
    pub title: Option<String>,
    pub path: PathBuf,
    pub frontmatter_json: String,
    pub body: String,
    pub edges: Vec<Edge>,
}

/// A graph edge declared in an artifact's frontmatter.
#[derive(Debug, Clone)]
pub struct Edge {
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
}

/// Walk the vault root and return every artifact found. Errors on
/// individual files are logged and skipped — a malformed brief shouldn't
/// prevent indexing the rest of the vault.
pub fn walk_vault(root: &Path) -> Result<Vec<Artifact>> {
    let mut out = Vec::new();
    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_hidden(e.file_name()))
    {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                tracing::warn!(?err, "walkdir error; skipping");
                continue;
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        match parse_file(path) {
            Ok(Some(artifact)) => out.push(artifact),
            Ok(None) => {} // no frontmatter; not an artifact
            Err(err) => tracing::warn!(path = %path.display(), ?err, "failed to parse; skipping"),
        }
    }
    Ok(out)
}

fn is_hidden(name: &std::ffi::OsStr) -> bool {
    name.to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}

/// Parse a single markdown file. Returns Ok(None) if the file has no
/// YAML frontmatter (not every markdown file in the tree is an artifact —
/// e.g. README.md).
pub fn parse_file(path: &Path) -> Result<Option<Artifact>> {
    let raw = fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    let Some((fm_src, body)) = split_frontmatter(&raw) else {
        return Ok(None);
    };
    let fm: Value = serde_yaml::from_str(fm_src)
        .with_context(|| format!("parse frontmatter in {}", path.display()))?;

    let id = scalar_string(&fm, "id")
        .ok_or_else(|| anyhow!("missing id in {}", path.display()))?;
    let r#type = scalar_string(&fm, "type")
        .ok_or_else(|| anyhow!("missing type in {}", path.display()))?;

    let edges = extract_edges(&id, &fm);
    let frontmatter_json = serde_json::to_string(&fm).unwrap_or_default();
    let title = derive_title(body);

    Ok(Some(Artifact {
        id,
        r#type,
        status: scalar_string(&fm, "status"),
        author: scalar_string(&fm, "author"),
        created: scalar_string(&fm, "created"),
        title,
        path: path.to_path_buf(),
        frontmatter_json,
        body: body.to_string(),
        edges,
    }))
}

/// Split a markdown file into (yaml_source, body). Returns None if the
/// file does not begin with a `---` fence.
pub fn split_frontmatter(src: &str) -> Option<(&str, &str)> {
    let rest = src.strip_prefix("---")?;
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    let end = rest.find("\n---")?;
    let fm = &rest[..end];
    let after = &rest[end + 4..];
    let body = after.strip_prefix('\n').unwrap_or(after);
    Some((fm, body))
}

/// First `# heading` line in the body, if any.
fn derive_title(body: &str) -> Option<String> {
    for line in body.lines() {
        let line = line.trim_start();
        if let Some(rest) = line.strip_prefix("# ") {
            return Some(rest.trim().to_string());
        }
    }
    None
}

fn scalar_string(fm: &Value, key: &str) -> Option<String> {
    match fm.get(key)? {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn extract_edges(from_id: &str, fm: &Value) -> Vec<Edge> {
    let mut out = Vec::new();
    for (field, is_scalar) in schema::edge_fields() {
        let Some(val) = fm.get(field) else { continue };
        if is_scalar {
            if let Some(to) = edge_target(val) {
                out.push(Edge {
                    from_id: from_id.to_string(),
                    to_id: to,
                    kind: field.to_string(),
                });
            }
        } else if let Value::Sequence(seq) = val {
            for item in seq {
                if let Some(to) = edge_target(item) {
                    out.push(Edge {
                        from_id: from_id.to_string(),
                        to_id: to,
                        kind: field.to_string(),
                    });
                }
            }
        }
    }
    out
}

/// Accepts strings; skips nulls, lists, and maps. Rejects explicit
/// placeholders like `none` / `null` / empty strings — those are not
/// valid ids.
fn edge_target(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => {
            let t = s.trim();
            if t.is_empty() || t.eq_ignore_ascii_case("none") || t.eq_ignore_ascii_case("null") {
                None
            } else {
                Some(t.to_string())
            }
        }
        _ => None,
    }
}

/// Read the vault's `SCHEMA.md` and return its declared version. The
/// indexer refuses to run against a vault whose schema version doesn't
/// match what this build was compiled for.
pub fn read_schema_version(vault_root: &Path) -> Result<u32> {
    let path = vault_root.join("SCHEMA.md");
    let raw = fs::read_to_string(&path)
        .with_context(|| format!("read {}", path.display()))?;
    let (fm_src, _) = split_frontmatter(&raw)
        .ok_or_else(|| anyhow!("SCHEMA.md has no frontmatter"))?;
    let fm: Value = serde_yaml::from_str(fm_src)
        .with_context(|| format!("parse SCHEMA.md frontmatter"))?;
    match fm.get("version") {
        Some(Value::Number(n)) => n
            .as_u64()
            .map(|v| v as u32)
            .ok_or_else(|| anyhow!("SCHEMA.md version is not a non-negative integer")),
        Some(_) => Err(anyhow!("SCHEMA.md version is not a number")),
        None => Err(anyhow!("SCHEMA.md has no version field")),
    }
}
