//! SQLite-backed index of the vault.
//!
//! The index is a derivative of the vault's markdown files. It is safe
//! to delete at any time — it will be rebuilt on next query. Schema drift
//! is handled by bumping the schema version and forcing a full rebuild;
//! there is no `ALTER TABLE` choreography.

use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant, UNIX_EPOCH},
};

use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, ErrorCode, OptionalExtension, TransactionBehavior};
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
const SQLITE_BUSY_TIMEOUT: Duration = Duration::from_secs(5);

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
                 version {}. Upgrade skills-mcp or roll the vault schema.",
                schema::SUPPORTED_SCHEMA_VERSION
            ));
        }

        let db_path = root.join(INDEX_FILENAME);
        let conn =
            Connection::open(&db_path).with_context(|| format!("open {}", db_path.display()))?;
        enable_wal(&conn, SQLITE_BUSY_TIMEOUT).context("set journal_mode=WAL")?;
        conn.busy_timeout(SQLITE_BUSY_TIMEOUT)?;
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
        // Acquire the writer lock before reading the version so concurrent
        // openers cannot both decide to replace the schema.
        let tx = self
            .conn
            .transaction_with_behavior(TransactionBehavior::Immediate)?;
        tx.execute_batch(
            "CREATE TABLE IF NOT EXISTS meta (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL
                );",
        )
        .context("create meta table")?;
        let stored: Option<String> = tx
            .query_row(
                "SELECT value FROM meta WHERE key = 'index_schema_version'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        let current = INDEX_SCHEMA_VERSION.to_string();
        if stored.as_deref() != Some(current.as_str()) {
            Self::drop_all_tables(&tx)?;
            Self::create_all_tables(&tx)?;
            tx.execute(
                "INSERT INTO meta (key, value) VALUES ('index_schema_version', ?1)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value",
                params![current],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    fn drop_all_tables(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "DROP TABLE IF EXISTS artifact_fts;
             DROP TABLE IF EXISTS edge;
             DROP TABLE IF EXISTS artifact;",
        )?;
        Ok(())
    }

    fn create_all_tables(conn: &Connection) -> Result<()> {
        conn.execute_batch(
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

/// WAL setup can return BUSY without invoking SQLite's busy handler. Retry
/// only that result, with a deadline, before restoring normal lock waiting.
fn enable_wal(conn: &Connection, timeout: Duration) -> rusqlite::Result<()> {
    conn.busy_timeout(Duration::ZERO)?;
    let deadline = Instant::now() + timeout;
    loop {
        match conn.pragma_update(None, "journal_mode", "WAL") {
            Err(error)
                if error.sqlite_error_code() == Some(ErrorCode::DatabaseBusy)
                    && Instant::now() < deadline =>
            {
                std::thread::sleep(
                    deadline
                        .saturating_duration_since(Instant::now())
                        .min(Duration::from_millis(10)),
                );
            }
            result => return result,
        }
    }
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
    fn concurrent_schema_initialization_and_upgrade() {
        use std::sync::{Arc, Barrier};

        for upgrade in [false, true] {
            for _ in 0..5 {
                let vault = make_vault(&[]);
                let db_path = vault.path().join(INDEX_FILENAME);
                let conn = Connection::open(&db_path).unwrap();
                conn.pragma_update(None, "journal_mode", "WAL").unwrap();
                if upgrade {
                    let idx = Index::open(vault.path()).unwrap();
                    idx.conn
                        .execute(
                            "UPDATE meta SET value = '0' WHERE key = 'index_schema_version'",
                            [],
                        )
                        .unwrap();
                }
                // Prepare connections before releasing the callers together.
                // This isolates schema initialization from journal-mode setup.
                let indexes: Vec<_> = (0..8)
                    .map(|_| Index {
                        root: vault.path().to_path_buf(),
                        conn: Connection::open(&db_path).unwrap(),
                    })
                    .collect();
                let barrier = Arc::new(Barrier::new(indexes.len()));
                let workers: Vec<_> = indexes
                    .into_iter()
                    .map(|mut idx| {
                        let barrier = Arc::clone(&barrier);
                        std::thread::spawn(move || {
                            barrier.wait();
                            idx.apply_sqlite_schema()
                        })
                    })
                    .collect();
                let errors: Vec<_> = workers
                    .into_iter()
                    .filter_map(|worker| worker.join().unwrap().err())
                    .map(|error| format!("{error:#}"))
                    .collect();
                assert!(errors.is_empty(), "upgrade={upgrade}: {errors:?}");
                let mut idx = Index::open(vault.path()).unwrap();
                idx.rebuild().unwrap();
                let found: String = idx
                    .conn
                    .query_row(
                        "SELECT id FROM artifact_fts WHERE artifact_fts MATCH 'schema'",
                        [],
                        |row| row.get(0),
                    )
                    .unwrap();
                assert_eq!(found, "schema");
            }
        }
    }

    #[test]
    fn failed_schema_upgrade_preserves_previous_index() {
        let vault = make_vault(&[(
            "briefs/p.md",
            "id: p\ntype: problem-brief\nstatus: accepted",
            "# Searchable\n\npreservedword",
        )]);
        let mut idx = Index::open(vault.path()).unwrap();
        idx.rebuild().unwrap();
        idx.conn
            .execute_batch(
                "UPDATE meta SET value = '0' WHERE key = 'index_schema_version';
                 CREATE TRIGGER reject_schema_version BEFORE UPDATE OF value ON meta
                 WHEN NEW.key = 'index_schema_version'
                 BEGIN SELECT RAISE(ABORT, 'injected version failure'); END;",
            )
            .unwrap();

        let error = idx.apply_sqlite_schema().unwrap_err();
        assert!(error.to_string().contains("injected version failure"));
        let version: String = idx
            .conn
            .query_row(
                "SELECT value FROM meta WHERE key = 'index_schema_version'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(version, "0");
        let found: String = idx
            .conn
            .query_row(
                "SELECT id FROM artifact_fts WHERE artifact_fts MATCH 'preservedword'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(found, "p");
        let count: i64 = idx
            .conn
            .query_row("SELECT COUNT(*) FROM artifact", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 2);

        idx.conn
            .execute_batch("DROP TRIGGER reject_schema_version")
            .unwrap();
        idx.apply_sqlite_schema().unwrap();
        idx.rebuild().unwrap();
    }

    #[test]
    fn wal_setup_retries_contention_and_stops_at_deadline() {
        let vault = make_vault(&[]);
        let db_path = vault.path().join(INDEX_FILENAME);
        let blocker = Connection::open(&db_path).unwrap();
        blocker
            .execute_batch("CREATE TABLE held (id INTEGER); BEGIN; SELECT * FROM held;")
            .unwrap();
        let conn = Connection::open(&db_path).unwrap();
        let error = enable_wal(&conn, Duration::from_millis(20)).unwrap_err();
        assert_eq!(error.sqlite_error_code(), Some(ErrorCode::DatabaseBusy));

        // Release the reader while the next WAL setup is waiting.
        let release = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(50));
            blocker.execute_batch("COMMIT").unwrap();
        });
        enable_wal(&conn, Duration::from_secs(2)).unwrap();
        release.join().unwrap();
        let mode: String = conn
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap();
        assert_eq!(mode, "wal");
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
