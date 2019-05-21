use crate::form_data::FormDataId;
use crate::interner::IdDatabase;
use crate::lemmas::{Lemma, LemmaId};

use interner::{impl_arena_id, RawId};
use latin_lemmatizer::NaiveLemmatizer;
use latin_utilities::NormalizedLatinString;
use std::iter::FromIterator;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormId(RawId);
impl_arena_id!(FormId);

// Strong typedefs for more intuitive api
#[derive(shrinkwraprs::Shrinkwrap, Clone, Eq, PartialEq, Hash)]
pub struct Form(pub NormalizedLatinString);

#[salsa::query_group(FormQueryStorage)]
trait FormQueryDatabase: salsa::Database + AsRef<NaiveLemmatizer> + IdDatabase {
    fn form_count(&self, id: FormId) -> usize;
    fn lemmas_for_form(&self, id: FormId) -> Arc<Vec<LemmaId>>;

    #[salsa::input]
    fn data_for_form(&self, id: FormId) -> Arc<Vec<FormDataId>>;
}

fn form_count(db: &impl FormQueryDatabase, id: FormId) -> usize {
    let forms = db.data_for_form(id);
    forms.len()
}

fn lemmas_for_form(db: &impl FormQueryDatabase, id: FormId) -> Arc<Vec<LemmaId>> {
    let int = db.get_interner();
    let form = int.form_interner.fetch(id);
    // TODO, we could save one allocation here
    let lemmas = db
        .as_ref()
        .get_possible_lemmas(form)
        .map(Vec::from_iter)
        .unwrap_or_else(Vec::new);
    Arc::new(
        lemmas
            .iter()
            .map(|&l| int.lemma_interner.to_id(&Lemma(l.clone())))
            .collect(),
    )
}
