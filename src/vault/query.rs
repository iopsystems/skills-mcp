//! Structured queries against the vault index.
//!
//! These functions return JSON-serializable rows that callers (the MCP
//! tool handlers, the reconciler skill, tests) can consume directly. The
//! caller is responsible for running [`Index::refresh_if_stale`] first —
//! queries here assume the index is current.

use anyhow::Result;
use rusqlite::params;
use serde::Serialize;

use crate::vault::index::Index;

/// A single artifact row returned by search / edges queries.
#[derive(Debug, Clone, Serialize)]
pub struct ArtifactRow {
    pub id: String,
    pub r#type: String,
    pub status: Option<String>,
    pub author: Option<String>,
    pub created: Option<String>,
    pub path: String,
    pub title: Option<String>,
}

/// A single edge row returned by [`edges_of`].
#[derive(Debug, Clone, Serialize)]
pub struct EdgeRow {
    pub from_id: String,
    pub to_id: String,
    pub kind: String,
    /// The artifact on the *other* end of the edge relative to the query
    /// pivot. Populated when the caller asks for edges of a specific id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbor: Option<ArtifactRow>,
}

/// Filters accepted by [`search`].
#[derive(Debug, Default, Clone)]
pub struct SearchFilters<'a> {
    pub r#type: Option<&'a str>,
    pub status: Option<&'a str>,
    pub author: Option<&'a str>,
    /// Full-text search term. When provided, artifacts are ranked by
    /// FTS score over title + body.
    pub topic: Option<&'a str>,
    pub limit: Option<u32>,
}

/// Search the vault for artifacts matching the given filters. At least
/// one filter should be set; an empty filter returns an empty result
/// rather than the entire vault (protect the caller from hauling the
/// whole graph into a context window).
pub fn search(index: &Index, f: &SearchFilters<'_>) -> Result<Vec<ArtifactRow>> {
    let any_filter = f.r#type.is_some()
        || f.status.is_some()
        || f.author.is_some()
        || f.topic.is_some();
    if !any_filter {
        return Ok(Vec::new());
    }

    // We build the SQL dynamically based on which filters are set.
    // Bind params are collected in order so the `?N` indexes line up.
    let mut sql = String::from(
        "SELECT a.id, a.type, a.status, a.author, a.created, a.path, a.title
         FROM artifact a",
    );
    let mut conds: Vec<String> = Vec::new();
    let mut binds: Vec<String> = Vec::new();

    if let Some(topic) = f.topic {
        sql.push_str(" JOIN artifact_fts ON artifact_fts.id = a.id");
        conds.push(format!("artifact_fts MATCH ?{}", binds.len() + 1));
        binds.push(topic.to_string());
    }
    if let Some(t) = f.r#type {
        conds.push(format!("a.type = ?{}", binds.len() + 1));
        binds.push(t.to_string());
    }
    if let Some(s) = f.status {
        conds.push(format!("a.status = ?{}", binds.len() + 1));
        binds.push(s.to_string());
    }
    if let Some(author) = f.author {
        conds.push(format!("a.author = ?{}", binds.len() + 1));
        binds.push(author.to_string());
    }
    if !conds.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&conds.join(" AND "));
    }
    if f.topic.is_some() {
        sql.push_str(" ORDER BY artifact_fts.rank");
    } else {
        sql.push_str(" ORDER BY a.created DESC NULLS LAST, a.id");
    }
    let limit = f.limit.unwrap_or(50).min(500);
    sql.push_str(&format!(" LIMIT {limit}"));

    let mut stmt = index.conn.prepare(&sql)?;
    let rows = stmt.query_map(
        rusqlite::params_from_iter(binds.iter()),
        row_to_artifact,
    )?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

/// Return edges touching `id`. If `kind` is set, only edges of that type
/// are returned. If `direction` is `Outgoing`, returns edges where `id`
/// appears as `from_id`; `Incoming`, as `to_id`; `Both` returns both.
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Outgoing,
    Incoming,
    Both,
}

pub fn edges_of(
    index: &Index,
    id: &str,
    kind: Option<&str>,
    direction: Direction,
) -> Result<Vec<EdgeRow>> {
    let mut out = Vec::new();
    if matches!(direction, Direction::Outgoing | Direction::Both) {
        collect_edges(
            index,
            "SELECT e.from_id, e.to_id, e.kind,
                    a.id, a.type, a.status, a.author, a.created, a.path, a.title
             FROM edge e
             LEFT JOIN artifact a ON a.id = e.to_id
             WHERE e.from_id = ?1",
            id,
            kind,
            &mut out,
        )?;
    }
    if matches!(direction, Direction::Incoming | Direction::Both) {
        collect_edges(
            index,
            "SELECT e.from_id, e.to_id, e.kind,
                    a.id, a.type, a.status, a.author, a.created, a.path, a.title
             FROM edge e
             LEFT JOIN artifact a ON a.id = e.from_id
             WHERE e.to_id = ?1",
            id,
            kind,
            &mut out,
        )?;
    }
    Ok(out)
}

fn collect_edges(
    index: &Index,
    base_sql: &str,
    id: &str,
    kind: Option<&str>,
    out: &mut Vec<EdgeRow>,
) -> Result<()> {
    let sql = match kind {
        Some(_) => format!("{base_sql} AND e.kind = ?2"),
        None => base_sql.to_string(),
    };
    let mut stmt = index.conn.prepare(&sql)?;
    let mut rows = match kind {
        Some(k) => stmt.query(params![id, k])?,
        None => stmt.query(params![id])?,
    };
    while let Some(row) = rows.next()? {
        let neighbor_id: Option<String> = row.get(3)?;
        let neighbor = if let Some(nid) = neighbor_id {
            Some(ArtifactRow {
                id: nid,
                r#type: row.get(4)?,
                status: row.get(5)?,
                author: row.get(6)?,
                created: row.get(7)?,
                path: row.get::<_, Option<String>>(8)?.unwrap_or_default(),
                title: row.get(9)?,
            })
        } else {
            None
        };
        out.push(EdgeRow {
            from_id: row.get(0)?,
            to_id: row.get(1)?,
            kind: row.get(2)?,
            neighbor,
        });
    }
    Ok(())
}

fn row_to_artifact(row: &rusqlite::Row<'_>) -> rusqlite::Result<ArtifactRow> {
    Ok(ArtifactRow {
        id: row.get(0)?,
        r#type: row.get(1)?,
        status: row.get(2)?,
        author: row.get(3)?,
        created: row.get(4)?,
        path: row.get(5)?,
        title: row.get(6)?,
    })
}
