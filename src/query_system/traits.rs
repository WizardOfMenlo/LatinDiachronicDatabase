//! Module that re-exports all the traits used in the query_system, for convenience

pub use super::middle::IntermediateDatabase;
pub use super::sources::SourcesDatabase;
pub use super::types::AuthorInternDatabase;
pub use super::types::InternDatabase;
pub use super::MainDatabase;
pub use salsa::Database;
pub use salsa::ParallelDatabase;
