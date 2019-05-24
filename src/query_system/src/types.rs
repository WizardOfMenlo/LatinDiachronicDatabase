//! This mod defines all the structs used for computation.
//! In most cases, the types are created by direct computation on
//! the sources, and are then interned in order to speed up computation

use crate::ids::{FormDataId, FormId, LemmaId, SourceId};
use latin_utilities::NormalizedLatinString;

#[salsa::query_group(InternersGroup)]
pub trait InternDatabase {
    #[salsa::interned]
    fn intern_form_data(&self, fd: FormData) -> FormDataId;

    #[salsa::interned]
    fn intern_form(&self, fd: Form) -> FormId;

    #[salsa::interned]
    fn intern_lemma(&self, fd: Lemma) -> LemmaId;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Author {
    name: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FormData {
    source: SourceId,
    line_no: usize,
    form: FormId,
}

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Eq, PartialEq, Clone)]
pub struct Lemma(pub NormalizedLatinString);

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Eq, PartialEq, Clone)]
pub struct Form(pub NormalizedLatinString);

impl Author {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FormData {
    pub fn new(source: SourceId, line_no: usize, form: FormId) -> Self {
        Self {
            source,
            line_no,
            form,
        }
    }

    pub fn source(&self) -> SourceId {
        self.source
    }

    pub fn line_no(&self) -> usize {
        self.line_no
    }

    pub fn form(&self) -> FormId {
        self.form
    }
}