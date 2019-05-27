//! Core idea, use salsa for queries ID to ID
//! The sources module has queries directly on source
//! The ids contains all the various id types that are used directly into salsa
//! Types are all the expanded types those ids refer to

pub mod ids;
pub mod middle;
pub mod sources;
pub mod types;

use std::sync::Arc;

#[salsa::query_group(MainQueries)]
pub trait MainDatabase: sources::SourcesDatabase + types::InternDatabase {
    #[salsa::input]
    fn some(&self) -> u32;
}
