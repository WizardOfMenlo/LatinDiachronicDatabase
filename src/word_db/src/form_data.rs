use authors::AuthorId;
use interner::{impl_arena_id, RawId};
use sources::SourceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormDataId(RawId);
impl_arena_id!(FormDataId);

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct FormData {
    author_id: AuthorId,
    source: SourceId,
    line_offset: usize,
}
