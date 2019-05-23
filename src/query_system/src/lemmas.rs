use latin_utilities::NormalizedLatinString;
use salsa::InternId;

#[salsa::query_group(LemmasQueryGroup)]
pub trait LemmasDatabase {
    #[salsa::interned]
    fn intern_lemma(&self, id: Lemma) -> LemmaId;
}

#[derive(Debug, Hash, Eq, Copy, PartialEq, Clone)]
pub struct LemmaId(InternId);

impl salsa::InternKey for LemmaId {
    fn from_intern_id(v: InternId) -> Self {
        LemmaId(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Clone, Eq, PartialEq)]
pub struct Lemma(pub NormalizedLatinString);
