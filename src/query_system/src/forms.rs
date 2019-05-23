use latin_lemmatizer::NaiveLemmatizer;
use latin_utilities::NormalizedLatinString;
use salsa::InternId;
use std::collections::HashSet;
use std::sync::Arc;

use crate::lemmas::{Lemma, LemmaId, LemmasDatabase};

#[salsa::query_group(FormsQueryGroup)]
pub trait FormsDatabase: AsRef<NaiveLemmatizer> + LemmasDatabase {
    #[salsa::interned]
    fn intern_form(&self, id: Form) -> FormId;

    fn lemmatize_form(&self, id: FormId) -> Arc<Vec<LemmaId>>;
}

fn lemmatize_form(db: &impl FormsDatabase, id: FormId) -> Arc<Vec<LemmaId>> {
    let lemmatizer = db.as_ref();
    let assoc_form = db.lookup_intern_form(id);
    let lemmas_o = lemmatizer.get_possible_lemmas(&assoc_form.0);

    // Here is where we would introduce not found
    if lemmas_o.is_none() {
        return Arc::new(Vec::new());
    }

    Arc::new(
        lemmas_o
            .unwrap()
            .iter()
            .cloned()
            .map(|e| db.intern_lemma(Lemma(e)))
            .collect(),
    )
}

#[derive(Debug, Hash, Eq, Copy, PartialEq, Clone)]
pub struct FormId(InternId);

impl salsa::InternKey for FormId {
    fn from_intern_id(v: InternId) -> Self {
        FormId(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Clone, Eq, PartialEq)]
pub struct Form(pub NormalizedLatinString);
