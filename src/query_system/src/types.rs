//! This mod defines all the structs used for computation.
//! In most cases, the types are created by direct computation on
//! the sources, and are then interned in order to speed up computation

use crate::ids::{AuthorId, FormDataId, FormId, LemmaId, SourceId};

use crate::traits::MainDatabase;
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

pub trait AuthorInternDatabase {
    fn intern_author(&mut self, author: Author) -> AuthorId;
    fn lookup_intern_author(&self, id: AuthorId) -> &Author;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FormData {
    source: SourceId,
    line_no: usize,
    form: FormId,
}

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct Lemma(pub NormalizedLatinString);

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct Form(pub NormalizedLatinString);

pub use authors_chrono::Author;

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

    pub fn author(&self, db: &impl MainDatabase) -> AuthorId {
        db.associated_author(self.source())
    }
}
