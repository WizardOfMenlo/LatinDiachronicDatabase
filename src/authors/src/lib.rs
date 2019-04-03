use interner::{impl_arena_id, RawId};
use sources::SourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AuthorId(RawId);
impl_arena_id!(AuthorId);

#[derive(Debug, Clone)]
pub struct Author {
    author_name: String,
    sources: Vec<SourceId>,
}
