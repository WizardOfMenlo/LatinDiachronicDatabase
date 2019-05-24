use crate::ids::{FormId, LemmaId};
use latin_lemmatizer::NaiveLemmatizer;
use latin_utilities::NormalizedLatinString;
use std::sync::Arc;

use crate::lemmas::{Lemma, LemmasDatabase};

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

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Clone, Eq, PartialEq)]
pub struct Form(pub NormalizedLatinString);
