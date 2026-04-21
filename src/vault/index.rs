//! SQLite-backed index of the vault.
//!
//! The index is a derivative of the vault's markdown files. It is safe
//! to delete at any time — it will be rebuilt on next query. Schema drift
//! is handled by bumping the schema version and forcing a full rebuild;
//! there is no `ALTER TABLE` choreography.

use std::{
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use walkdir::WalkDir;

use crate::vault::{
    parse::{self, Artifact},
    schema,
};

/// Filename of the sqlite index inside the vault working tree.
pub const INDEX_FILENAME: &str = ".vault-index.sqlite";

/// Current internal index schema version. Bumped when the sqlite schema
/// shape changes (independent of the vault schema version).
const INDEX_SCHEMA_VERSION: u32 = 1;

/// A handle to a vault's sqlite index. Opening does not refresh — call
/// [`Index::refresh_if_stale`] or [`Index::rebuild`] explicitly.
pub struct Index {
    pub root: PathBuf,
    pub conn: Connection,
}

impl Index {
    /// Open (or create) the sqlite index file for the vault at `root`.
    ///
    /// Verifies the vault's `SCHEMA.md` version matches what this build
    /// supports; refuses to run on mismatch. Applies the index's own
    /// sqlite schema on first open.
    pub fn open(root: &Path) -> Result<Self> {
        let vault_version = parse::read_schema_version(root)?;
        if vault_version != schema::SUPPORTED_SCHEMA_VERSION {
            return Err(anyhow!(
                "vault SCHEMA.md is version {vault_version} but this build supports \
                 version {}. Upgrade iop-skills or roll the vault schema.",
                schema::SUPPORTED_SCHEMA_VERSION
            ));
        }

        let db_path = root.join(INDEX_FILENAME);
        let conn =
            Connection::open(&db_path).with_context(|| format!("open {}", db_path.display()))?;
        conn.pragma_update(None, "journal_mode", "WAL")
            .context("set journal_mode=WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")
            .context("set synchronous=NORMAL")?;

        let mut idx = Self {
            root: root.to_path_buf(),
            conn,
        };
        idx.apply_sqlite_schema()?;
        Ok(idx)
    }

    /// Ensure the index's sqlite schema is at the current version. If the
    /// stored version is older, the index is dropped and rebuilt from
    /// scratch — per SCHEMA.md, "there is no ALTER TABLE choreography."
    fn apply_sqlite_schema(&mut self) -> Result<()> {
        self.conn
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS meta (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );",
            )
            .context("create meta table")?;
        let stored: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM meta WHERE key = 'index_schema_version'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        let current = INDEX_SCHEMA_VERSION.to_string();
        if stored.as_deref() != Some(current.as_str()) {
            self.drop_all_tables()?;
            self.create_all_tables()?;
            self.conn.execute(
                "INSERT INTO meta (key, value) VALUES ('index_schema_version', ?1)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![current],
            )?;
        }
        Ok(())
    }

    fn drop_all_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "DROP TABLE IF EXISTS artifact_fts;
             DROP TABLE IF EXISTS edge;
             DROP TABLE IF EXISTS artifact;",
        )?;
        Ok(())
    }

    fn create_all_tables(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE artifact (
                id                TEXT PRIMARY KEY,
                type              TEXT NOT NULL,
                status            TEXT,
                author            TEXT,
                created           TEXT,
                path              TEXT NOT NULL,
                title             TEXT,
                frontmatter_json  TEXT NOT NULL
            );
            CREATE INDEX artifact_type_idx ON artifact (type);
            CREATE INDEX artifact_status_idx ON artifact (status);

            CREATE TABLE edge (
                from_id TEXT NOT NULL,
                to_id   TEXT NOT NULL,
                kind    TEXT NOT NULL,
                PRIMARY KEY (from_id, to_id, kind)
            );
            CREATE INDEX edge_to_idx ON edge (to_id, kind);
            CREATE INDEX edge_kind_idx ON edge (kind);

            CREATE VIRTUAL TABLE artifact_fts USING fts5(
                id UNINDEXED,
                title,
                body
            );",
        )?;
        Ok(())
    }

    /// Compare the vault's newest file mtime against the index's stored
    /// watermark. If the vault has changed, do a full rebuild.
    pub fn refresh_if_stale(&mut self) -> Result<RefreshOutcome> {
        let vault_mtime = newest_mtime(&self.root)?;
        let stored: Option<String> = self
            .conn
            .query_row(
                "SELECT value FROM meta WHERE key = 'vault_mtime_epoch'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        let stored_mtime: u64 = stored.as_deref().and_then(|s| s.parse().ok()).unwrap_or(0);

        if vault_mtime <= stored_mtime && stored.is_some() {
            return Ok(RefreshOutcome::UpToDate);
        }
        self.rebuild()?;
        self.conn.execute(
            "INSERT INTO meta (key, value) VALUES ('vault_mtime_epoch', ?1)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![vault_mtime.to_string()],
        )?;
        Ok(RefreshOutcome::Rebuilt)
    }

    /// Drop all rows and re-walk the vault from scratch.
    pub fn rebuild(&mut self) -> Result<()> {
        let artifacts = parse::walk_vault(&self.root)?;
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM edge", [])?;
        tx.execute("DELETE FROM artifact", [])?;
        tx.execute("DELETE FROM artifact_fts", [])?;
        {
            let mut insert_artifact = tx.prepare(
                "INSERT INTO artifact
                   (id, type, status, author, created, path, title, frontmatter_json)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
                 ON CONFLICT(id) DO UPDATE SET
                   type = excluded.type,
                   status = excluded.status,
                   author = excluded.author,
                   created = excluded.created,
                   path = excluded.path,
                   title = excluded.title,
                   frontmatter_json = excluded.frontmatter_json",
            )?;
            let mut insert_edge = tx
                .prepare("INSERT OR IGNORE INTO edge (from_id, to_id, kind) VALUES (?1, ?2, ?3)")?;
            let mut insert_fts =
                tx.prepare("INSERT INTO artifact_fts (id, title, body) VALUES (?1, ?2, ?3)")?;

            for a in &artifacts {
                insert_artifact_row(&mut insert_artifact, &self.root, a)?;
                for e in &a.edges {
                    insert_edge.execute(params![&e.from_id, &e.to_id, &e.kind])?;
                }
                insert_fts.execute(params![&a.id, a.title.as_deref().unwrap_or(""), &a.body])?;
            }
        }
        tx.commit()?;
        tracing::info!(count = artifacts.len(), "rebuilt vault index");
        Ok(())
    }
}

pub enum RefreshOutcome {
    UpToDate,
    Rebuilt,
}

fn insert_artifact_row(
    stmt: &mut rusqlite::Statement<'_>,
    root: &Path,
    a: &Artifact,
) -> Result<()> {
    let rel = a
        .path
        .strip_prefix(root)
        .unwrap_or(&a.path)
        .to_string_lossy()
        .into_owned();
    stmt.execute(params![
        &a.id,
        &a.r#type,
        a.status.as_deref(),
        a.author.as_deref(),
        a.created.as_deref(),
        rel,
        a.title.as_deref(),
        &a.frontmatter_json,
    ])?;
    Ok(())
}

/// Unix-epoch seconds of the newest modification in the vault tree,
/// ignoring hidden files (including the index itself).
fn newest_mtime(root: &Path) -> Result<u64> {
    let mut newest = 0u64;
    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| e.depth() == 0 || !is_hidden(e.file_name()))
    {
        let Ok(entry) = entry else { continue };
        if !entry.file_type().is_file() {
            continue;
        }
        if let Ok(meta) = entry.metadata() {
            if let Ok(modified) = meta.modified() {
                if let Ok(dur) = modified.duration_since(UNIX_EPOCH) {
                    let secs = dur.as_secs();
                    if secs > newest {
                        newest = secs;
                    }
                }
            }
        }
    }
    Ok(newest)
}

fn is_hidden(name: &std::ffi::OsStr) -> bool {
    name.to_str().map(|s| s.starts_with('.')).unwrap_or(false)
}

/// Resolve the vault root from an explicit env var or by walking up from
/// `start`, looking for a directory that contains either `SCHEMA.md` or
/// one of the canonical top-level folders.
pub fn locate_vault(start: &Path) -> Result<PathBuf> {
    if let Ok(env) = std::env::var("IOP_VAULT_PATH") {
        let p = PathBuf::from(env);
        if !p.exists() {
            return Err(anyhow!("IOP_VAULT_PATH={} does not exist", p.display()));
        }
        return Ok(p);
    }
    let mut cur = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    loop {
        if looks_like_vault(&cur) {
            return Ok(cur);
        }
        match cur.parent() {
            Some(parent) if parent != cur => cur = parent.to_path_buf(),
            _ => break,
        }
    }
    Err(anyhow!(
        "no vault found: set IOP_VAULT_PATH or run from inside a vault checkout"
    ))
}

fn looks_like_vault(p: &Path) -> bool {
    if p.join("SCHEMA.md").is_file() {
        return true;
    }
    ["scopes", "briefs", "arcs"]
        .iter()
        .any(|d| p.join(d).is_dir())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Build a minimal vault tempdir containing a SCHEMA.md (matching
    /// this build's supported version) plus the provided artifacts.
    /// Each tuple is (relative_path, frontmatter_body, markdown_body).
    fn make_vault(artifacts: &[(&str, &str, &str)]) -> TempDir {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("SCHEMA.md"),
            format!(
                "---\nid: schema\ntype: schema\nversion: {}\n---\n# schema\n",
                schema::SUPPORTED_SCHEMA_VERSION
            ),
        )
        .unwrap();
        for (rel, fm, body) in artifacts {
            let p = tmp.path().join(rel);
            if let Some(parent) = p.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&p, format!("---\n{fm}\n---\n{body}\n")).unwrap();
        }
        tmp
    }

    #[test]
    fn open_creates_index_file_and_tables() {
        let vault = make_vault(&[]);
        let idx = Index::open(vault.path()).unwrap();
        assert!(vault.path().join(INDEX_FILENAME).is_file());
        // meta table exists and has the index_schema_version row
        let v: String = idx
            .conn
            .query_row(
                "SELECT value FROM meta WHERE key = 'index_schema_version'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(v, INDEX_SCHEMA_VERSION.to_string());
    }

    #[test]
    fn open_refuses_on_schema_version_mismatch() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("SCHEMA.md"),
            format!(
                "---\nid: schema\ntype: schema\nversion: {}\n---\n",
                schema::SUPPORTED_SCHEMA_VERSION + 99
            ),
        )
        .unwrap();
        let err = Index::open(tmp.path()).map(|_| ()).unwrap_err();
        assert!(
            err.to_string().contains("SCHEMA.md is version"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn rebuild_populates_artifacts_and_edges() {
        let vault = make_vault(&[
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: accepted",
                "# problem\n\nsome problem text",
            ),
            (
                "briefs/d.md",
                "id: d\ntype: design-brief\nstatus: proposed\nframes: p\nrelates_to: [p]",
                "# design\n\nsome design text",
            ),
        ]);
        let mut idx = Index::open(vault.path()).unwrap();
        idx.rebuild().unwrap();

        let n_artifact: i64 = idx
            .conn
            .query_row("SELECT COUNT(*) FROM artifact", [], |r| r.get(0))
            .unwrap();
        // 2 briefs + schema
        assert_eq!(n_artifact, 3);

        let n_edges: i64 = idx
            .conn
            .query_row("SELECT COUNT(*) FROM edge WHERE from_id = 'd'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(n_edges, 2); // frames + relates_to

        let frames_target: String = idx
            .conn
            .query_row(
                "SELECT to_id FROM edge WHERE from_id = 'd' AND kind = 'frames'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(frames_target, "p");
    }

    #[test]
    fn refresh_is_idempotent_when_nothing_changed() {
        let vault = make_vault(&[(
            "briefs/p.md",
            "id: p\ntype: problem-brief\nstatus: accepted",
            "# p",
        )]);
        let mut idx = Index::open(vault.path()).unwrap();
        assert!(matches!(
            idx.refresh_if_stale().unwrap(),
            RefreshOutcome::Rebuilt
        ));
        assert!(matches!(
            idx.refresh_if_stale().unwrap(),
            RefreshOutcome::UpToDate
        ));
    }

    #[test]
    fn refresh_rebuilds_when_a_file_changes() {
        use std::thread::sleep;
        use std::time::Duration;

        let vault = make_vault(&[(
            "briefs/p.md",
            "id: p\ntype: problem-brief\nstatus: accepted",
            "# p",
        )]);
        let mut idx = Index::open(vault.path()).unwrap();
        idx.refresh_if_stale().unwrap();

        // mtime resolution is 1s on some filesystems; make sure our edit
        // lands in a later second.
        sleep(Duration::from_millis(1100));
        fs::write(
            vault.path().join("briefs/p.md"),
            "---\nid: p\ntype: problem-brief\nstatus: obsolete\n---\n# p\n",
        )
        .unwrap();

        assert!(matches!(
            idx.refresh_if_stale().unwrap(),
            RefreshOutcome::Rebuilt
        ));
        let status: String = idx
            .conn
            .query_row("SELECT status FROM artifact WHERE id = 'p'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(status, "obsolete");
    }

    #[test]
    fn locate_vault_finds_schema_md() {
        let vault = make_vault(&[]);
        let nested = vault.path().join("nested/deeply/inside");
        fs::create_dir_all(&nested).unwrap();
        let found = locate_vault(&nested).unwrap();
        // Both paths should resolve to the same canonical vault root.
        assert_eq!(
            found.canonicalize().unwrap(),
            vault.path().canonicalize().unwrap()
        );
    }
}
