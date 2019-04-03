use crate::form_data::{FormData, FormDataId};
use crate::forms::{Form, FormId};
use crate::lemmas::{Lemma, LemmaId};
use interner::Interner;
use sources::{Source, SourceId};

pub struct IdInterner {
    pub form_interner: Interner<FormId, Form>,
    pub lemma_interner: Interner<LemmaId, Lemma>,
    pub formdata_interner: Interner<FormDataId, FormData>,
    pub source_interner: Interner<SourceId, Source>,
}

// We use this to allow to access the interner easily
pub trait IdDatabase: salsa::Database {
    fn get_interner(&self) -> &IdInterner;
}
