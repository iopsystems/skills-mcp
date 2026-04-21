//! Parse vault markdown files into structured artifacts.
//!
//! A vault artifact is a markdown file with a YAML frontmatter block
//! delimited by `---` lines. The frontmatter's `type:` and `id:` fields
//! are required; everything else is type-specific and parsed lazily.

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
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
        .filter_entry(|e| e.depth() == 0 || !is_hidden(e.file_name()))
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
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let Some((fm_src, body)) = split_frontmatter(&raw) else {
        return Ok(None);
    };
    let fm: Value = serde_yaml::from_str(fm_src)
        .with_context(|| format!("parse frontmatter in {}", path.display()))?;

    let id = scalar_string(&fm, "id").ok_or_else(|| anyhow!("missing id in {}", path.display()))?;
    let r#type =
        scalar_string(&fm, "type").ok_or_else(|| anyhow!("missing type in {}", path.display()))?;

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
    let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let (fm_src, _) =
        split_frontmatter(&raw).ok_or_else(|| anyhow!("SCHEMA.md has no frontmatter"))?;
    let fm: Value = serde_yaml::from_str(fm_src).context("parse SCHEMA.md frontmatter")?;
    match fm.get("version") {
        Some(Value::Number(n)) => n
            .as_u64()
            .map(|v| v as u32)
            .ok_or_else(|| anyhow!("SCHEMA.md version is not a non-negative integer")),
        Some(_) => Err(anyhow!("SCHEMA.md version is not a number")),
        None => Err(anyhow!("SCHEMA.md has no version field")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn split_frontmatter_happy_path() {
        let src = "---\nid: foo\ntype: scope\n---\n\nbody here\n";
        let (fm, body) = split_frontmatter(src).expect("should split");
        assert_eq!(fm, "id: foo\ntype: scope");
        assert_eq!(body, "\nbody here\n");
    }

    #[test]
    fn split_frontmatter_missing_opening_fence() {
        assert!(split_frontmatter("no fence here\nstill nothing").is_none());
    }

    #[test]
    fn split_frontmatter_missing_closing_fence() {
        assert!(split_frontmatter("---\nid: foo\nno close\n").is_none());
    }

    #[test]
    fn split_frontmatter_empty_body() {
        let (fm, body) = split_frontmatter("---\nid: x\n---").expect("should split");
        assert_eq!(fm, "id: x");
        assert_eq!(body, "");
    }

    fn write(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, contents).unwrap();
    }

    #[test]
    fn parse_file_extracts_core_fields_and_title() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("brief.md");
        write(
            &p,
            "---\nid: abc\ntype: design-brief\nstatus: proposed\nauthor: alex\ncreated: 2026-04-20\nframes: xyz\n---\n\n# Design: widget\n\nbody",
        );
        let a = parse_file(&p).unwrap().expect("should parse");
        assert_eq!(a.id, "abc");
        assert_eq!(a.r#type, "design-brief");
        assert_eq!(a.status.as_deref(), Some("proposed"));
        assert_eq!(a.author.as_deref(), Some("alex"));
        assert_eq!(a.created.as_deref(), Some("2026-04-20"));
        assert_eq!(a.title.as_deref(), Some("Design: widget"));
    }

    #[test]
    fn parse_file_returns_none_without_frontmatter() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("README.md");
        write(&p, "# Just a readme\n\nno frontmatter here");
        assert!(parse_file(&p).unwrap().is_none());
    }

    #[test]
    fn parse_file_errors_without_id() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("bad.md");
        write(&p, "---\ntype: scope\n---\nbody");
        assert!(parse_file(&p).is_err());
    }

    #[test]
    fn parse_file_extracts_scalar_edge() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("d.md");
        write(
            &p,
            "---\nid: design-1\ntype: design-brief\nframes: problem-1\n---\n",
        );
        let a = parse_file(&p).unwrap().unwrap();
        assert_eq!(a.edges.len(), 1);
        assert_eq!(a.edges[0].from_id, "design-1");
        assert_eq!(a.edges[0].to_id, "problem-1");
        assert_eq!(a.edges[0].kind, "frames");
    }

    #[test]
    fn parse_file_extracts_list_edges() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("d.md");
        write(
            &p,
            "---\nid: d\ntype: design-brief\nrelates_to: [a, b]\ndepends_on: [c]\nscopes: []\n---\n",
        );
        let a = parse_file(&p).unwrap().unwrap();
        let mut kinds: Vec<_> = a
            .edges
            .iter()
            .map(|e| (e.kind.as_str(), e.to_id.as_str()))
            .collect();
        kinds.sort();
        assert_eq!(
            kinds,
            vec![
                ("depends_on", "c"),
                ("relates_to", "a"),
                ("relates_to", "b"),
            ]
        );
    }

    #[test]
    fn parse_file_skips_none_and_null_placeholders() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("d.md");
        write(
            &p,
            "---\nid: d\ntype: design-brief\nframes: none\nsupersedes: null\nsuperseded_by: \"\"\n---\n",
        );
        let a = parse_file(&p).unwrap().unwrap();
        assert!(
            a.edges.is_empty(),
            "placeholders should not produce edges, got: {:?}",
            a.edges
        );
    }

    #[test]
    fn read_schema_version_happy_path() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("SCHEMA.md");
        write(&p, "---\nid: schema\ntype: schema\nversion: 7\n---\n");
        assert_eq!(read_schema_version(tmp.path()).unwrap(), 7);
    }

    #[test]
    fn read_schema_version_missing_field() {
        let tmp = TempDir::new().unwrap();
        let p = tmp.path().join("SCHEMA.md");
        write(&p, "---\nid: schema\ntype: schema\n---\n");
        assert!(read_schema_version(tmp.path()).is_err());
    }

    #[test]
    fn walk_vault_skips_hidden_and_non_markdown() {
        let tmp = TempDir::new().unwrap();
        write(
            &tmp.path().join("SCHEMA.md"),
            "---\nid: schema\ntype: schema\nversion: 1\n---\n",
        );
        write(
            &tmp.path().join("briefs/p1.md"),
            "---\nid: p1\ntype: problem-brief\n---\n",
        );
        write(&tmp.path().join("briefs/notes.txt"), "not markdown");
        write(
            &tmp.path().join(".hidden/skip.md"),
            "---\nid: hidden\ntype: scope\n---\n",
        );

        let artifacts = walk_vault(tmp.path()).unwrap();
        let ids: Vec<_> = artifacts.iter().map(|a| a.id.as_str()).collect();
        assert!(ids.contains(&"schema"));
        assert!(ids.contains(&"p1"));
        assert!(!ids.contains(&"hidden"));
    }
}
