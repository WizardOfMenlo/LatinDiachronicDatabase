use super::{ids::FormId, ids::LemmaId, Form, Lemma};
use crate::form_data::{FormData, FormDataId};
use interner::Interner;

struct IdInterner {
    pub form_interner: Interner<FormId, Form>,
    pub lemma_interner: Interner<LemmaId, Lemma>,
    pub formdata_interner: Interner<FormDataId, FormData>,
}
