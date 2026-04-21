//! Embedded SQLite index of a knowledge-iop vault.
//!
//! The vault is canonical markdown on disk. This module builds a
//! derivative index (`<vault>/.vault-index.sqlite`, gitignored) that
//! lets MCP tools answer structured questions without grepping every
//! query.

pub mod index;
pub mod parse;
pub mod query;
pub mod schema;
pub mod transitions;

#[allow(unused_imports)]
pub use index::{locate_vault, Index, RefreshOutcome, INDEX_FILENAME};
#[allow(unused_imports)]
pub use query::{edges_of, search, ArtifactRow, Direction, EdgeRow, SearchFilters};
#[allow(unused_imports)]
pub use transitions::{check_transition, ArtifactRef, Finding, TransitionCheck};
