//! Aggregate reflection queries powering the reconciler.
//!
//! Part A of the dream-mode output (graph hygiene) leans on these —
//! they are deterministic counts and joins over the existing index.
//! Part B (strategic reflection, judgment) is the skill's job; no
//! SQL earns that part of the report.

use anyhow::Result;
use rusqlite::params;
use serde::Serialize;

use crate::vault::index::Index;

// ----------------------------------------------------------------------
// Activity heatmap — scopes and arcs, by recent artifact volume.
// ----------------------------------------------------------------------

/// Activity for a single scope: total artifacts ever tagged with this
/// scope, artifacts in the last `window_days`, and date of the most
/// recent.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ScopeActivity {
    pub scope_id: String,
    pub n_total: u32,
    pub n_recent: u32,
    pub last_activity: Option<String>,
}

/// Momentum of a single arc: total artifacts in this arc, recent
/// artifacts, and date of the most recent.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ArcMomentum {
    pub arc_id: String,
    pub status: Option<String>,
    pub n_total: u32,
    pub n_recent: u32,
    pub last_activity: Option<String>,
}

/// Return one row per scope, sorted by n_recent (desc) then
/// last_activity (desc).
pub fn scope_activity(index: &Index, window_days: u32) -> Result<Vec<ScopeActivity>> {
    let sql = "
        SELECT s.id,
               COUNT(DISTINCT e.from_id) AS n_total,
               COUNT(DISTINCT CASE WHEN a.created >= date('now', ?1) THEN a.id END) AS n_recent,
               MAX(a.created) AS last_activity
        FROM artifact s
        LEFT JOIN edge e ON e.to_id = s.id AND e.kind = 'scopes'
        LEFT JOIN artifact a ON a.id = e.from_id
        WHERE s.type = 'scope'
        GROUP BY s.id
        ORDER BY n_recent DESC, last_activity DESC NULLS LAST, s.id
    ";
    let mut stmt = index.conn.prepare(sql)?;
    let window = format!("-{} days", window_days);
    let rows = stmt.query_map(params![window], |r| {
        Ok(ScopeActivity {
            scope_id: r.get(0)?,
            n_total: r.get::<_, i64>(1)? as u32,
            n_recent: r.get::<_, i64>(2)? as u32,
            last_activity: r.get(3)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// Return one row per arc, sorted by n_recent (desc) then
/// last_activity (desc).
pub fn arc_momentum(index: &Index, window_days: u32) -> Result<Vec<ArcMomentum>> {
    let sql = "
        SELECT ar.id, ar.status,
               COUNT(DISTINCT e.from_id) AS n_total,
               COUNT(DISTINCT CASE WHEN a.created >= date('now', ?1) THEN a.id END) AS n_recent,
               MAX(a.created) AS last_activity
        FROM artifact ar
        LEFT JOIN edge e ON e.to_id = ar.id AND e.kind = 'arc'
        LEFT JOIN artifact a ON a.id = e.from_id
        WHERE ar.type = 'arc'
        GROUP BY ar.id
        ORDER BY n_recent DESC, last_activity DESC NULLS LAST, ar.id
    ";
    let mut stmt = index.conn.prepare(sql)?;
    let window = format!("-{} days", window_days);
    let rows = stmt.query_map(params![window], |r| {
        Ok(ArcMomentum {
            arc_id: r.get(0)?,
            status: r.get(1)?,
            n_total: r.get::<_, i64>(2)? as u32,
            n_recent: r.get::<_, i64>(3)? as u32,
            last_activity: r.get(4)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

// ----------------------------------------------------------------------
// Gap detectors — problem-briefs without designs, stuck designs,
// pending syntheses, stale arcs.
// ----------------------------------------------------------------------

/// A problem-brief in `draft` or `accepted` status that has no
/// corresponding design-brief framing it (ignoring rejected/superseded
/// designs).
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrphanProblemBrief {
    pub id: String,
    pub status: Option<String>,
    pub created: Option<String>,
    pub title: Option<String>,
    pub path: String,
}

pub fn orphan_problem_briefs(index: &Index) -> Result<Vec<OrphanProblemBrief>> {
    let sql = "
        SELECT p.id, p.status, p.created, p.title, p.path
        FROM artifact p
        WHERE p.type = 'problem-brief'
          AND (p.status IS NULL OR p.status IN ('draft', 'accepted'))
          AND NOT EXISTS (
            SELECT 1 FROM edge e
            JOIN artifact d ON d.id = e.from_id
            WHERE e.to_id = p.id AND e.kind = 'frames'
              AND d.type = 'design-brief'
              AND (d.status IS NULL OR d.status NOT IN ('rejected', 'superseded'))
          )
        ORDER BY p.created DESC NULLS LAST, p.id
    ";
    let mut stmt = index.conn.prepare(sql)?;
    let rows = stmt.query_map([], |r| {
        Ok(OrphanProblemBrief {
            id: r.get(0)?,
            status: r.get(1)?,
            created: r.get(2)?,
            title: r.get(3)?,
            path: r.get(4)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// A design-brief stuck in `draft` or `proposed` for longer than
/// `min_days_old` days.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct StaleDesignBrief {
    pub id: String,
    pub status: Option<String>,
    pub created: Option<String>,
    pub days_old: u32,
    pub title: Option<String>,
    pub path: String,
}

pub fn stale_design_briefs(index: &Index, min_days_old: u32) -> Result<Vec<StaleDesignBrief>> {
    let sql = "
        SELECT d.id, d.status, d.created, d.title, d.path,
               CAST(julianday('now') - julianday(d.created) AS INTEGER) AS days_old
        FROM artifact d
        WHERE d.type = 'design-brief'
          AND d.status IN ('draft', 'proposed')
          AND d.created IS NOT NULL
          AND julianday('now') - julianday(d.created) > ?1
        ORDER BY days_old DESC, d.id
    ";
    let mut stmt = index.conn.prepare(sql)?;
    let rows = stmt.query_map(params![min_days_old as i64], |r| {
        Ok(StaleDesignBrief {
            id: r.get(0)?,
            status: r.get(1)?,
            created: r.get(2)?,
            title: r.get(3)?,
            path: r.get(4)?,
            days_old: r.get::<_, i64>(5)? as u32,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// An inquiry in `ready_for_synthesis` without a synthesis artifact
/// pointing back at it.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PendingSynthesis {
    pub id: String,
    pub status: Option<String>,
    pub created: Option<String>,
    pub title: Option<String>,
    pub path: String,
}

pub fn pending_syntheses(index: &Index) -> Result<Vec<PendingSynthesis>> {
    let sql = "
        SELECT i.id, i.status, i.created, i.title, i.path
        FROM artifact i
        WHERE i.type = 'inquiry'
          AND i.status = 'ready_for_synthesis'
          AND NOT EXISTS (
            SELECT 1 FROM edge e
            JOIN artifact s ON s.id = e.from_id
            WHERE e.to_id = i.id AND e.kind = 'inquiry' AND s.type = 'synthesis'
          )
        ORDER BY i.created DESC NULLS LAST, i.id
    ";
    let mut stmt = index.conn.prepare(sql)?;
    let rows = stmt.query_map([], |r| {
        Ok(PendingSynthesis {
            id: r.get(0)?,
            status: r.get(1)?,
            created: r.get(2)?,
            title: r.get(3)?,
            path: r.get(4)?,
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

/// An `open` arc whose most recent artifact (by `created`) is older
/// than `min_days_stale` days, or has no artifacts at all.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct StaleArc {
    pub id: String,
    pub status: Option<String>,
    pub created: Option<String>,
    pub last_activity: Option<String>,
    pub days_since_activity: Option<u32>,
    pub title: Option<String>,
    pub path: String,
}

pub fn stale_arcs(index: &Index, min_days_stale: u32) -> Result<Vec<StaleArc>> {
    let sql = "
        SELECT ar.id, ar.status, ar.created, ar.title, ar.path,
               MAX(a.created) AS last_activity,
               CASE WHEN MAX(a.created) IS NULL THEN NULL
                    ELSE CAST(julianday('now') - julianday(MAX(a.created)) AS INTEGER)
               END AS days_since_activity
        FROM artifact ar
        LEFT JOIN edge e ON e.to_id = ar.id AND e.kind = 'arc'
        LEFT JOIN artifact a ON a.id = e.from_id
        WHERE ar.type = 'arc' AND ar.status = 'open'
        GROUP BY ar.id
        HAVING last_activity IS NULL OR days_since_activity > ?1
        ORDER BY days_since_activity DESC NULLS LAST, ar.id
    ";
    let mut stmt = index.conn.prepare(sql)?;
    let rows = stmt.query_map(params![min_days_stale as i64], |r| {
        Ok(StaleArc {
            id: r.get(0)?,
            status: r.get(1)?,
            created: r.get(2)?,
            title: r.get(3)?,
            path: r.get(4)?,
            last_activity: r.get(5)?,
            days_since_activity: r.get::<_, Option<i64>>(6)?.map(|v| v as u32),
        })
    })?;
    let mut out = Vec::new();
    for row in rows {
        out.push(row?);
    }
    Ok(out)
}

// ----------------------------------------------------------------------
// Composite report — one struct that the reconciler's MCP tool returns
// so the skill can produce a coherent Part A in a single call.
// ----------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
pub struct ReflectionReport {
    pub window_days: u32,
    pub min_days_stale_design: u32,
    pub min_days_stale_arc: u32,
    pub scope_activity: Vec<ScopeActivity>,
    pub arc_momentum: Vec<ArcMomentum>,
    pub orphan_problem_briefs: Vec<OrphanProblemBrief>,
    pub stale_design_briefs: Vec<StaleDesignBrief>,
    pub pending_syntheses: Vec<PendingSynthesis>,
    pub stale_arcs: Vec<StaleArc>,
}

/// Produce the full Part A reflection report by running every query.
/// The skill body then layers judgment (Part B) on top.
pub fn reflect(
    index: &Index,
    window_days: u32,
    min_days_stale_design: u32,
    min_days_stale_arc: u32,
) -> Result<ReflectionReport> {
    Ok(ReflectionReport {
        window_days,
        min_days_stale_design,
        min_days_stale_arc,
        scope_activity: scope_activity(index, window_days)?,
        arc_momentum: arc_momentum(index, window_days)?,
        orphan_problem_briefs: orphan_problem_briefs(index)?,
        stale_design_briefs: stale_design_briefs(index, min_days_stale_design)?,
        pending_syntheses: pending_syntheses(index)?,
        stale_arcs: stale_arcs(index, min_days_stale_arc)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::schema;
    use std::fs;
    use tempfile::TempDir;

    /// Build a vault tempdir with the given artifacts. Each tuple is
    /// (relative_path, frontmatter, body).
    fn vault_with(artifacts: &[(&str, &str, &str)]) -> (TempDir, Index) {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("SCHEMA.md"),
            format!(
                "---\nid: schema\ntype: schema\nversion: {}\n---\n",
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
        let mut idx = Index::open(tmp.path()).unwrap();
        idx.refresh_if_stale().unwrap();
        (tmp, idx)
    }

    // ------------------------------------------------------------
    // scope_activity
    // ------------------------------------------------------------

    #[test]
    fn scope_activity_counts_total_and_recent() {
        let today = chrono_date_stub(0);
        let (_tmp, idx) = vault_with(&[
            (
                "scopes/billing.md",
                "id: billing\ntype: scope\nstatus: active",
                "# billing",
            ),
            (
                &format!("briefs/{today}-recent.md"),
                &format!("id: {today}-recent\ntype: problem-brief\nstatus: accepted\ncreated: {today}\nscopes: [billing]"),
                "# recent",
            ),
            (
                "briefs/2020-01-01-old.md",
                "id: 2020-01-01-old\ntype: problem-brief\nstatus: accepted\ncreated: 2020-01-01\nscopes: [billing]",
                "# old",
            ),
        ]);
        let rows = scope_activity(&idx, 30).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].scope_id, "billing");
        assert_eq!(rows[0].n_total, 2);
        assert_eq!(rows[0].n_recent, 1);
    }

    #[test]
    fn scope_activity_zero_when_no_artifacts_touch_scope() {
        let (_tmp, idx) = vault_with(&[(
            "scopes/unused.md",
            "id: unused\ntype: scope\nstatus: active",
            "# unused",
        )]);
        let rows = scope_activity(&idx, 30).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].n_total, 0);
        assert_eq!(rows[0].n_recent, 0);
        assert_eq!(rows[0].last_activity, None);
    }

    // ------------------------------------------------------------
    // arc_momentum
    // ------------------------------------------------------------

    #[test]
    fn arc_momentum_orders_by_recent_volume() {
        let today = chrono_date_stub(0);
        let (_tmp, idx) = vault_with(&[
            (
                "arcs/hot.md",
                "id: hot\ntype: arc\nstatus: open",
                "# hot",
            ),
            (
                "arcs/cold.md",
                "id: cold\ntype: arc\nstatus: open",
                "# cold",
            ),
            (
                &format!("briefs/{today}-a.md"),
                &format!("id: {today}-a\ntype: problem-brief\nstatus: accepted\ncreated: {today}\narc: hot"),
                "# a",
            ),
            (
                &format!("briefs/{today}-b.md"),
                &format!("id: {today}-b\ntype: problem-brief\nstatus: accepted\ncreated: {today}\narc: hot"),
                "# b",
            ),
        ]);
        let rows = arc_momentum(&idx, 30).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].arc_id, "hot");
        assert_eq!(rows[0].n_recent, 2);
        assert_eq!(rows[1].arc_id, "cold");
        assert_eq!(rows[1].n_recent, 0);
    }

    // ------------------------------------------------------------
    // orphan_problem_briefs
    // ------------------------------------------------------------

    #[test]
    fn orphan_problem_briefs_finds_problems_without_designs() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/orphan.md",
                "id: orphan\ntype: problem-brief\nstatus: accepted",
                "# orphan",
            ),
            (
                "briefs/paired-p.md",
                "id: paired-p\ntype: problem-brief\nstatus: accepted",
                "# paired-p",
            ),
            (
                "briefs/paired-d.md",
                "id: paired-d\ntype: design-brief\nstatus: draft\nframes: paired-p",
                "# paired-d",
            ),
        ]);
        let rows = orphan_problem_briefs(&idx).unwrap();
        let ids: Vec<_> = rows.iter().map(|o| o.id.clone()).collect();
        assert_eq!(ids, vec!["orphan"]);
    }

    #[test]
    fn orphan_problem_briefs_ignores_rejected_and_superseded_designs() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: accepted",
                "# p",
            ),
            (
                "briefs/d1.md",
                "id: d1\ntype: design-brief\nstatus: rejected\nframes: p",
                "# d1",
            ),
            (
                "briefs/d2.md",
                "id: d2\ntype: design-brief\nstatus: superseded\nframes: p",
                "# d2",
            ),
        ]);
        let rows = orphan_problem_briefs(&idx).unwrap();
        assert_eq!(rows.iter().filter(|o| o.id == "p").count(), 1);
    }

    #[test]
    fn orphan_problem_briefs_excludes_obsolete_problems() {
        let (_tmp, idx) = vault_with(&[(
            "briefs/p.md",
            "id: p\ntype: problem-brief\nstatus: obsolete",
            "# p",
        )]);
        let rows = orphan_problem_briefs(&idx).unwrap();
        assert!(rows.is_empty());
    }

    // ------------------------------------------------------------
    // stale_design_briefs
    // ------------------------------------------------------------

    #[test]
    fn stale_design_briefs_flags_old_drafts() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: accepted",
                "# p",
            ),
            (
                "briefs/old.md",
                "id: old\ntype: design-brief\nstatus: draft\ncreated: 2020-01-01\nframes: p",
                "# old",
            ),
            (
                "briefs/accepted.md",
                "id: accepted\ntype: design-brief\nstatus: accepted\ncreated: 2020-01-01\nframes: p",
                "# accepted",
            ),
        ]);
        let rows = stale_design_briefs(&idx, 30).unwrap();
        let ids: Vec<_> = rows.iter().map(|d| d.id.clone()).collect();
        assert_eq!(ids, vec!["old"]);
        assert!(rows[0].days_old > 30);
    }

    // ------------------------------------------------------------
    // pending_syntheses
    // ------------------------------------------------------------

    #[test]
    fn pending_syntheses_flags_ready_without_synthesis() {
        let (_tmp, idx) = vault_with(&[
            (
                "arcs/a.md",
                "id: a\ntype: arc\nstatus: open",
                "# a",
            ),
            (
                "inquiries/q1/inquiry.md",
                "id: q1\ntype: inquiry\nstatus: ready_for_synthesis\narc: a\nbarrier_condition: met",
                "# q1",
            ),
            (
                "inquiries/q2/inquiry.md",
                "id: q2\ntype: inquiry\nstatus: ready_for_synthesis\narc: a\nbarrier_condition: met",
                "# q2",
            ),
            (
                "inquiries/q2/synthesis.md",
                "id: q2-synthesis\ntype: synthesis\nstatus: draft\ninquiry: q2\ncites: []",
                "# q2 synthesis",
            ),
        ]);
        let rows = pending_syntheses(&idx).unwrap();
        let ids: Vec<_> = rows.iter().map(|p| p.id.clone()).collect();
        assert_eq!(ids, vec!["q1"]);
    }

    // ------------------------------------------------------------
    // stale_arcs
    // ------------------------------------------------------------

    #[test]
    fn stale_arcs_flags_open_arcs_without_recent_activity() {
        let today = chrono_date_stub(0);
        let (_tmp, idx) = vault_with(&[
            (
                "arcs/active.md",
                "id: active\ntype: arc\nstatus: open",
                "# active",
            ),
            (
                "arcs/stale.md",
                "id: stale\ntype: arc\nstatus: open",
                "# stale",
            ),
            (
                "arcs/closed.md",
                "id: closed\ntype: arc\nstatus: closed",
                "# closed",
            ),
            (
                &format!("briefs/{today}-fresh.md"),
                &format!(
                    "id: {today}-fresh\ntype: problem-brief\nstatus: accepted\ncreated: {today}\narc: active"
                ),
                "# fresh",
            ),
            (
                "briefs/2020-01-01-ancient.md",
                "id: 2020-01-01-ancient\ntype: problem-brief\nstatus: accepted\ncreated: 2020-01-01\narc: stale",
                "# ancient",
            ),
        ]);
        let rows = stale_arcs(&idx, 60).unwrap();
        let ids: Vec<_> = rows.iter().map(|a| a.id.clone()).collect();
        assert!(ids.contains(&"stale".to_string()));
        assert!(!ids.contains(&"active".to_string()));
        // closed arcs are never flagged as stale
        assert!(!ids.contains(&"closed".to_string()));
    }

    #[test]
    fn stale_arcs_flags_empty_open_arcs() {
        let (_tmp, idx) = vault_with(&[(
            "arcs/empty.md",
            "id: empty\ntype: arc\nstatus: open",
            "# empty",
        )]);
        let rows = stale_arcs(&idx, 0).unwrap();
        let ids: Vec<_> = rows.iter().map(|a| a.id.clone()).collect();
        assert_eq!(ids, vec!["empty"]);
        assert_eq!(rows[0].last_activity, None);
    }

    // ------------------------------------------------------------
    // reflect() composite
    // ------------------------------------------------------------

    #[test]
    fn reflect_runs_every_query_and_populates_report() {
        let (_tmp, idx) =
            vault_with(&[("scopes/s.md", "id: s\ntype: scope\nstatus: active", "# s")]);
        let report = reflect(&idx, 30, 14, 60).unwrap();
        assert_eq!(report.window_days, 30);
        assert_eq!(report.min_days_stale_design, 14);
        assert_eq!(report.min_days_stale_arc, 60);
        // scope_activity finds the scope (with zero activity).
        assert_eq!(report.scope_activity.len(), 1);
        // everything else is empty for this minimal vault.
        assert!(report.arc_momentum.is_empty());
        assert!(report.orphan_problem_briefs.is_empty());
        assert!(report.stale_design_briefs.is_empty());
        assert!(report.pending_syntheses.is_empty());
        assert!(report.stale_arcs.is_empty());
    }

    /// Today's date as YYYY-MM-DD, synthesized without pulling in chrono
    /// (the crate doesn't take chrono as a runtime dep, only tests).
    /// Uses the system clock.
    fn chrono_date_stub(days_offset: i64) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            + days_offset * 86400;
        // Simple YYYY-MM-DD from epoch seconds. Good enough for tests —
        // we just need "today or near-today" vs "decades ago".
        let days = secs / 86400;
        let (y, m, d) = days_to_ymd(days);
        format!("{y:04}-{m:02}-{d:02}")
    }

    /// Gregorian conversion from days-since-1970-01-01.
    fn days_to_ymd(days: i64) -> (i32, u32, u32) {
        let mut days = days;
        let mut y = 1970i32;
        loop {
            let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
            let in_year = if leap { 366 } else { 365 };
            if days < in_year {
                break;
            }
            days -= in_year;
            y += 1;
        }
        let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
        let months = if leap {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
        let mut m = 0u32;
        for (i, &len) in months.iter().enumerate() {
            if days < len {
                m = i as u32 + 1;
                break;
            }
            days -= len;
        }
        (y, m, days as u32 + 1)
    }
}
