use interner::{RawId, impl_arena_id};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LemmaId(RawId);
impl_arena_id!(LemmaId);

