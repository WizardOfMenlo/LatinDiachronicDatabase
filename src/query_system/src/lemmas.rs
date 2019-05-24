use crate::ids::LemmaId;
use latin_utilities::NormalizedLatinString;

#[salsa::query_group(LemmasQueryGroup)]
pub trait LemmasDatabase {
    #[salsa::interned]
    fn intern_lemma(&self, id: Lemma) -> LemmaId;
}

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Clone, Eq, PartialEq)]
pub struct Lemma(pub NormalizedLatinString);
