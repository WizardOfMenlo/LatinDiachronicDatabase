use crate::sources::SourceId;
use interner::{impl_arena_id, RawId};
use std::sync::Arc;

#[derive(Debug, Hash, Eq, Copy, PartialEq, Clone)]
pub struct AuthorId(RawId);
impl_arena_id!(AuthorId);

// TODO, might refactor in its own crate, depending on the time involved (chrono)
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Author {
    name: String,
}

impl Author {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[salsa::query_group(AuthorsQueryGroup)]
pub trait AuthorsDatabase {
    #[salsa::input]
    fn associated_sources(&self, author_id: AuthorId) -> Arc<Vec<SourceId>>;
}
