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

#[allow(unused_imports)]
pub use index::{INDEX_FILENAME, Index, RefreshOutcome, locate_vault};
#[allow(unused_imports)]
pub use query::{ArtifactRow, Direction, EdgeRow, SearchFilters, edges_of, search};
