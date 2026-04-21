//! State-transition checks — the "can I accept this?" invariants from
//! SCHEMA.md's resolution-propagation section.
//!
//! Each rule is deterministic and grounded in an edge query. The output
//! is a structured report with machine-stable rule ids, human-readable
//! messages, and evidence (the artifacts that block or warn) so the
//! reconciler can present it as a proposal rather than a verdict.

use anyhow::{Context, Result};
use rusqlite::params;
use serde::Serialize;

use crate::vault::index::Index;

/// Result of a proposed status transition.
#[derive(Debug, Clone, Serialize)]
pub struct TransitionCheck {
    pub id: String,
    pub r#type: String,
    pub current_status: Option<String>,
    pub new_status: String,
    pub allowed: bool,
    pub blockers: Vec<Finding>,
    pub warnings: Vec<Finding>,
}

/// A single rule outcome — either a blocker (prevents transition) or a
/// warning (propagates advisory state, doesn't block).
#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    /// Stable machine id of the rule (for tests, dashboards, deduping).
    pub rule: &'static str,
    /// Human-readable explanation.
    pub message: String,
    /// Artifacts that caused this finding — the reader shows these to
    /// the user so they can see *why*.
    pub offenders: Vec<ArtifactRef>,
}

/// Compact reference to an artifact, carried inside a finding's
/// `offenders` list. Full metadata can be looked up via vault_search
/// if the caller needs it.
#[derive(Debug, Clone, Serialize)]
pub struct ArtifactRef {
    pub id: String,
    pub r#type: String,
    pub status: Option<String>,
}

/// Evaluate a proposed status transition against the vault's invariants.
///
/// Returns Err only for internal failures (e.g. sqlite error or missing
/// artifact). Normal "transition not permitted" outcomes return Ok with
/// `allowed: false` and a populated `blockers` list.
pub fn check_transition(index: &Index, id: &str, new_status: &str) -> Result<TransitionCheck> {
    let (r#type, current_status) = load_pivot(index, id)?;

    let mut blockers = Vec::new();
    let mut warnings = Vec::new();

    match (r#type.as_str(), new_status) {
        ("decision", "accepted") => {
            rule_decision_derived_from_accepted(index, id, &mut blockers)?;
        }
        ("design-brief", "accepted") => {
            rule_design_brief_frames_not_draft(index, id, &mut blockers)?;
            rule_design_brief_depends_on_inquiry_resolved(index, id, &mut blockers)?;
        }
        ("arc", "closed") => {
            rule_arc_all_inquiries_closed(index, id, &mut blockers)?;
        }
        ("problem-brief", "obsolete") => {
            rule_problem_brief_obsolete_warns_framers(index, id, &mut warnings)?;
        }
        _ => {}
    }

    Ok(TransitionCheck {
        id: id.to_string(),
        r#type,
        current_status,
        new_status: new_status.to_string(),
        allowed: blockers.is_empty(),
        blockers,
        warnings,
    })
}

/// Look up the pivot artifact (the one being transitioned). Errors if
/// the id isn't present in the index.
fn load_pivot(index: &Index, id: &str) -> Result<(String, Option<String>)> {
    index
        .conn
        .query_row(
            "SELECT type, status FROM artifact WHERE id = ?1",
            params![id],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<String>>(1)?)),
        )
        .with_context(|| format!("no artifact with id {id}"))
}

/// Collect rows matching the given SQL (bound to ?1 = pivot id) into
/// a Vec<ArtifactRef>. The query must project (id, type, status) in
/// that order.
fn collect_offenders(index: &Index, sql: &str, id: &str) -> Result<Vec<ArtifactRef>> {
    let mut stmt = index.conn.prepare(sql)?;
    let mut rows = stmt.query(params![id])?;
    let mut out = Vec::new();
    while let Some(row) = rows.next()? {
        out.push(ArtifactRef {
            id: row.get(0)?,
            r#type: row.get(1)?,
            status: row.get(2)?,
        });
    }
    Ok(out)
}

fn rule_decision_derived_from_accepted(
    index: &Index,
    id: &str,
    blockers: &mut Vec<Finding>,
) -> Result<()> {
    let offenders = collect_offenders(
        index,
        "SELECT a.id, a.type, a.status
         FROM edge e
         JOIN artifact a ON a.id = e.to_id
         WHERE e.from_id = ?1 AND e.kind = 'derived_from'
           AND (a.status IS NULL OR a.status != 'accepted')
         ORDER BY a.id",
        id,
    )?;
    if !offenders.is_empty() {
        blockers.push(Finding {
            rule: "decision_derived_from_accepted",
            message:
                "decision cannot be accepted while any derived_from design-brief is not accepted"
                    .to_string(),
            offenders,
        });
    }
    Ok(())
}

fn rule_design_brief_frames_not_draft(
    index: &Index,
    id: &str,
    blockers: &mut Vec<Finding>,
) -> Result<()> {
    let offenders = collect_offenders(
        index,
        "SELECT a.id, a.type, a.status
         FROM edge e
         JOIN artifact a ON a.id = e.to_id
         WHERE e.from_id = ?1 AND e.kind = 'frames'
           AND a.status = 'draft'
         ORDER BY a.id",
        id,
    )?;
    if !offenders.is_empty() {
        blockers.push(Finding {
            rule: "design_brief_frames_not_draft",
            message:
                "design-brief cannot be accepted while its framed problem-brief is still draft"
                    .to_string(),
            offenders,
        });
    }
    Ok(())
}

fn rule_design_brief_depends_on_inquiry_resolved(
    index: &Index,
    id: &str,
    blockers: &mut Vec<Finding>,
) -> Result<()> {
    let offenders = collect_offenders(
        index,
        "SELECT a.id, a.type, a.status
         FROM edge e
         JOIN artifact a ON a.id = e.to_id
         WHERE e.from_id = ?1 AND e.kind = 'depends_on'
           AND a.type = 'inquiry'
           AND a.status = 'open'
         ORDER BY a.id",
        id,
    )?;
    if !offenders.is_empty() {
        blockers.push(Finding {
            rule: "design_brief_depends_on_inquiry_resolved",
            message:
                "design-brief cannot be accepted while any inquiry in depends_on is still open"
                    .to_string(),
            offenders,
        });
    }
    Ok(())
}

fn rule_arc_all_inquiries_closed(
    index: &Index,
    id: &str,
    blockers: &mut Vec<Finding>,
) -> Result<()> {
    // Inquiries with arc: <this-arc-id> appear as outgoing 'arc' edges
    // from the inquiry, so we look at incoming edges on the arc side.
    let offenders = collect_offenders(
        index,
        "SELECT a.id, a.type, a.status
         FROM edge e
         JOIN artifact a ON a.id = e.from_id
         WHERE e.to_id = ?1 AND e.kind = 'arc'
           AND a.type = 'inquiry'
           AND a.status = 'open'
         ORDER BY a.id",
        id,
    )?;
    if !offenders.is_empty() {
        blockers.push(Finding {
            rule: "arc_all_inquiries_closed",
            message: "arc cannot be closed while any of its inquiries are still open".to_string(),
            offenders,
        });
    }
    Ok(())
}

fn rule_problem_brief_obsolete_warns_framers(
    index: &Index,
    id: &str,
    warnings: &mut Vec<Finding>,
) -> Result<()> {
    // Design-briefs that frame this problem-brief are incoming 'frames' edges.
    let offenders = collect_offenders(
        index,
        "SELECT a.id, a.type, a.status
         FROM edge e
         JOIN artifact a ON a.id = e.from_id
         WHERE e.to_id = ?1 AND e.kind = 'frames'
           AND a.type = 'design-brief'
         ORDER BY a.id",
        id,
    )?;
    if !offenders.is_empty() {
        warnings.push(Finding {
            rule: "problem_brief_obsolete_warns_framers",
            message:
                "marking a problem-brief obsolete propagates a warning to every design-brief that frames it"
                    .to_string(),
            offenders,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::schema;
    use std::fs;
    use tempfile::TempDir;

    /// Build a minimal vault tempdir with the given artifacts and open
    /// an index against it. Each artifact tuple is
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
    // decision(accepted) requires all derived_from design-briefs
    // to be accepted.
    // ------------------------------------------------------------

    #[test]
    fn decision_accepted_blocked_while_derived_from_is_draft() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/d.md",
                "id: d\ntype: design-brief\nstatus: draft\nframes: p",
                "# d",
            ),
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: accepted",
                "# p",
            ),
            (
                "decisions/dec.md",
                "id: dec\ntype: decision\nstatus: accepted\nderived_from: [d]",
                "# dec",
            ),
        ]);
        let check = check_transition(&idx, "dec", "accepted").unwrap();
        assert!(!check.allowed, "transition should be blocked");
        assert_eq!(check.blockers.len(), 1);
        assert_eq!(check.blockers[0].rule, "decision_derived_from_accepted");
        let offenders: Vec<_> = check.blockers[0]
            .offenders
            .iter()
            .map(|o| o.id.as_str())
            .collect();
        assert_eq!(offenders, vec!["d"]);
    }

    #[test]
    fn decision_accepted_allowed_when_all_derived_from_are_accepted() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: accepted",
                "# p",
            ),
            (
                "briefs/d.md",
                "id: d\ntype: design-brief\nstatus: accepted\nframes: p",
                "# d",
            ),
            (
                "decisions/dec.md",
                "id: dec\ntype: decision\nstatus: accepted\nderived_from: [d]",
                "# dec",
            ),
        ]);
        let check = check_transition(&idx, "dec", "accepted").unwrap();
        assert!(
            check.allowed,
            "expected allowed, got blockers: {:?}",
            check.blockers
        );
        assert!(check.blockers.is_empty());
    }

    // ------------------------------------------------------------
    // design-brief(accepted) requires framed problem-brief not draft.
    // ------------------------------------------------------------

    #[test]
    fn design_brief_accepted_blocked_while_frame_is_draft() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: draft",
                "# p",
            ),
            (
                "briefs/d.md",
                "id: d\ntype: design-brief\nstatus: proposed\nframes: p",
                "# d",
            ),
        ]);
        let check = check_transition(&idx, "d", "accepted").unwrap();
        assert!(!check.allowed);
        let rules: Vec<_> = check.blockers.iter().map(|b| b.rule).collect();
        assert!(rules.contains(&"design_brief_frames_not_draft"));
    }

    #[test]
    fn design_brief_accepted_allowed_when_frame_is_accepted() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: accepted",
                "# p",
            ),
            (
                "briefs/d.md",
                "id: d\ntype: design-brief\nstatus: proposed\nframes: p",
                "# d",
            ),
        ]);
        let check = check_transition(&idx, "d", "accepted").unwrap();
        assert!(check.allowed, "unexpected blockers: {:?}", check.blockers);
    }

    // ------------------------------------------------------------
    // design-brief(accepted) requires depends_on inquiries not open.
    // ------------------------------------------------------------

    #[test]
    fn design_brief_accepted_blocked_while_depends_on_inquiry_is_open() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: accepted",
                "# p",
            ),
            (
                "inquiries/q/inquiry.md",
                "id: q\ntype: inquiry\nstatus: open\nbarrier_condition: TBD",
                "# q",
            ),
            (
                "briefs/d.md",
                "id: d\ntype: design-brief\nstatus: proposed\nframes: p\ndepends_on: [q]",
                "# d",
            ),
        ]);
        let check = check_transition(&idx, "d", "accepted").unwrap();
        assert!(!check.allowed);
        let rules: Vec<_> = check.blockers.iter().map(|b| b.rule).collect();
        assert!(rules.contains(&"design_brief_depends_on_inquiry_resolved"));
    }

    #[test]
    fn design_brief_accepted_allowed_when_depends_on_inquiry_is_resolved() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: accepted",
                "# p",
            ),
            (
                "inquiries/q/inquiry.md",
                "id: q\ntype: inquiry\nstatus: resolved\nbarrier_condition: met",
                "# q",
            ),
            (
                "briefs/d.md",
                "id: d\ntype: design-brief\nstatus: proposed\nframes: p\ndepends_on: [q]",
                "# d",
            ),
        ]);
        let check = check_transition(&idx, "d", "accepted").unwrap();
        assert!(check.allowed, "unexpected blockers: {:?}", check.blockers);
    }

    // ------------------------------------------------------------
    // arc(closed) requires all of its inquiries to be resolved/abandoned.
    // ------------------------------------------------------------

    #[test]
    fn arc_close_blocked_while_any_inquiry_is_open() {
        let (_tmp, idx) = vault_with(&[
            ("arcs/a.md", "id: a\ntype: arc\nstatus: open", "# a"),
            (
                "inquiries/q1/inquiry.md",
                "id: q1\ntype: inquiry\nstatus: resolved\narc: a\nbarrier_condition: met",
                "# q1",
            ),
            (
                "inquiries/q2/inquiry.md",
                "id: q2\ntype: inquiry\nstatus: open\narc: a\nbarrier_condition: TBD",
                "# q2",
            ),
        ]);
        let check = check_transition(&idx, "a", "closed").unwrap();
        assert!(!check.allowed);
        assert_eq!(check.blockers.len(), 1);
        assert_eq!(check.blockers[0].rule, "arc_all_inquiries_closed");
        let offenders: Vec<_> = check.blockers[0]
            .offenders
            .iter()
            .map(|o| o.id.as_str())
            .collect();
        assert_eq!(offenders, vec!["q2"]);
    }

    #[test]
    fn arc_close_allowed_when_all_inquiries_are_resolved() {
        let (_tmp, idx) = vault_with(&[
            ("arcs/a.md", "id: a\ntype: arc\nstatus: open", "# a"),
            (
                "inquiries/q1/inquiry.md",
                "id: q1\ntype: inquiry\nstatus: resolved\narc: a\nbarrier_condition: met",
                "# q1",
            ),
            (
                "inquiries/q2/inquiry.md",
                "id: q2\ntype: inquiry\nstatus: abandoned\narc: a\nbarrier_condition: N/A",
                "# q2",
            ),
        ]);
        let check = check_transition(&idx, "a", "closed").unwrap();
        assert!(check.allowed, "unexpected blockers: {:?}", check.blockers);
    }

    // ------------------------------------------------------------
    // problem-brief(obsolete) emits warnings for every design-brief
    // that frames it — but does NOT block.
    // ------------------------------------------------------------

    #[test]
    fn problem_brief_obsolete_is_allowed_but_warns_framers() {
        let (_tmp, idx) = vault_with(&[
            (
                "briefs/p.md",
                "id: p\ntype: problem-brief\nstatus: accepted",
                "# p",
            ),
            (
                "briefs/d1.md",
                "id: d1\ntype: design-brief\nstatus: accepted\nframes: p",
                "# d1",
            ),
            (
                "briefs/d2.md",
                "id: d2\ntype: design-brief\nstatus: proposed\nframes: p",
                "# d2",
            ),
        ]);
        let check = check_transition(&idx, "p", "obsolete").unwrap();
        assert!(check.allowed, "obsolete should not be blocked");
        assert_eq!(check.warnings.len(), 1);
        assert_eq!(
            check.warnings[0].rule,
            "problem_brief_obsolete_warns_framers"
        );
        let mut offenders: Vec<_> = check.warnings[0]
            .offenders
            .iter()
            .map(|o| o.id.clone())
            .collect();
        offenders.sort();
        assert_eq!(offenders, vec!["d1", "d2"]);
    }

    // ------------------------------------------------------------
    // Edge cases: unknown id errors, irrelevant transitions are no-ops.
    // ------------------------------------------------------------

    #[test]
    fn unknown_id_returns_error() {
        let (_tmp, idx) = vault_with(&[]);
        assert!(check_transition(&idx, "does-not-exist", "accepted").is_err());
    }

    #[test]
    fn irrelevant_transition_has_no_blockers_or_warnings() {
        let (_tmp, idx) =
            vault_with(&[("scopes/s.md", "id: s\ntype: scope\nstatus: active", "# s")]);
        let check = check_transition(&idx, "s", "active").unwrap();
        assert!(check.allowed);
        assert!(check.blockers.is_empty());
        assert!(check.warnings.is_empty());
    }
}
