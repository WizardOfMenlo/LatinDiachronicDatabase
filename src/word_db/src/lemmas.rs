use crate::forms::FormId;

use interner::{impl_arena_id, RawId};
use latin_utilities::NormalizedLatinString;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LemmaId(RawId);
impl_arena_id!(LemmaId);

// Strong typedefs for more intuitive api
pub type Lemma = NormalizedLatinString;

#[salsa::query_group(LemmaQueryStorage)]
trait LemmaQueryDatabase: salsa::Database {
    #[salsa::input]
    fn forms_for_lemma(&self, id: LemmaId) -> Arc<Vec<FormId>>;
}
