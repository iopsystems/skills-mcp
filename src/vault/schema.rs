//! Hardcoded knowledge of the knowledge-iop vault schema.
//!
//! The vault's `SCHEMA.md` carries a `version:` field in its own frontmatter
//! that is authoritative. This module declares which version this build
//! supports. If the vault's schema version differs, the reader refuses to
//! run rather than producing silently-wrong results.

/// Vault schema version this build of the indexer understands.
pub const SUPPORTED_SCHEMA_VERSION: u32 = 1;

/// All artifact types recognized by the schema.
///
/// The directory an artifact lives in is a human affordance; the `type:`
/// frontmatter field is authoritative. These strings are the canonical
/// values written in frontmatter.
#[allow(dead_code)]
pub const ARTIFACT_TYPES: &[&str] = &[
    "scope",
    "arc",
    "problem-brief",
    "design-brief",
    "decision",
    "discussion",
    "session-note",
    "inquiry",
    "exploration",
    "synthesis",
    "claim",
    "schema",
];

/// Frontmatter fields that reference other artifacts by `id`, holding a
/// single id as the value.
pub const SCALAR_EDGE_FIELDS: &[&str] =
    &["frames", "supersedes", "superseded_by", "arc", "inquiry"];

/// Frontmatter fields that reference other artifacts by `id`, holding a
/// list of ids as the value.
pub const LIST_EDGE_FIELDS: &[&str] = &[
    "relates_to",
    "conflicts_with",
    "depends_on",
    "derived_from",
    "scopes",
];

/// Iterate all edge field names (scalar + list), each paired with whether
/// it is scalar. Useful for generic frontmatter walkers.
pub fn edge_fields() -> impl Iterator<Item = (&'static str, bool)> {
    SCALAR_EDGE_FIELDS
        .iter()
        .copied()
        .map(|f| (f, true))
        .chain(LIST_EDGE_FIELDS.iter().copied().map(|f| (f, false)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn scalar_and_list_edge_fields_do_not_overlap() {
        let scalar: HashSet<_> = SCALAR_EDGE_FIELDS.iter().collect();
        let list: HashSet<_> = LIST_EDGE_FIELDS.iter().collect();
        let overlap: Vec<_> = scalar.intersection(&list).collect();
        assert!(
            overlap.is_empty(),
            "edge fields must not be both scalar and list: {overlap:?}"
        );
    }

    #[test]
    fn edge_fields_iter_covers_both_halves() {
        let pairs: Vec<_> = edge_fields().collect();
        let scalar_count = pairs.iter().filter(|(_, s)| *s).count();
        let list_count = pairs.iter().filter(|(_, s)| !*s).count();
        assert_eq!(scalar_count, SCALAR_EDGE_FIELDS.len());
        assert_eq!(list_count, LIST_EDGE_FIELDS.len());
    }

    #[test]
    fn required_edge_kinds_are_declared() {
        // These are the semantically load-bearing edge kinds from SCHEMA.md.
        // Dropping one would silently break graph queries the reconciler
        // depends on.
        let all: HashSet<&str> = SCALAR_EDGE_FIELDS
            .iter()
            .chain(LIST_EDGE_FIELDS.iter())
            .copied()
            .collect();
        for required in [
            "frames",
            "supersedes",
            "superseded_by",
            "relates_to",
            "conflicts_with",
            "depends_on",
            "derived_from",
            "arc",
            "scopes",
        ] {
            assert!(all.contains(required), "missing edge field: {required}");
        }
    }
}
