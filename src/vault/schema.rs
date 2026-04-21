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
pub const SCALAR_EDGE_FIELDS: &[&str] = &[
    "frames",
    "supersedes",
    "superseded_by",
    "arc",
    "inquiry",
];

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
