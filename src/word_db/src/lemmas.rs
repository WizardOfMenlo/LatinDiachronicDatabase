use interner::{impl_arena_id, RawId};
use latin_utilities::NormalizedLatinString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LemmaId(RawId);
impl_arena_id!(LemmaId);

// Strong typedefs for more intuitive api
pub type Lemma = NormalizedLatinString;
