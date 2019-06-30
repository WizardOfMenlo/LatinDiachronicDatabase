//! Module that re-exports all the traits used in the query_system, for convenience

pub use crate::middle::IntermediateDatabase;
pub use crate::sources::SourcesDatabase;
pub use crate::types::AuthorInternDatabase;
pub use crate::types::InternDatabase;
pub use crate::MainDatabase;
pub use salsa::Database;
pub use salsa::ParallelDatabase;
