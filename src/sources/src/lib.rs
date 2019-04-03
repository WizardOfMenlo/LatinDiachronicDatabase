use interner::{impl_arena_id, RawId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(RawId);
impl_arena_id!(SourceId);

pub struct Source {}
