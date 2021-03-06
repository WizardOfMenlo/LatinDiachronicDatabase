//! This mod defines all the structs used for computation.
//! In most cases, the types are created by direct computation on
//! the sources, and are then interned in order to speed up computation

use super::ids::{AuthorId, FormDataId, SourceId};
use super::traits::MainDatabase;
use crate::word_db::{WordDatabase, WordId};

#[salsa::query_group(InternersGroup)]
pub trait InternDatabase: WordDatabase {
    #[salsa::interned]
    fn intern_form_data(&self, fd: FormData) -> FormDataId;
}

pub trait AuthorInternDatabase {
    fn intern_author(&mut self, author: Author) -> AuthorId;
    fn lookup_intern_author(&self, id: AuthorId) -> &Author;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FormData {
    source: SourceId,
    line_no: usize,
    form: Form,
}

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub struct Lemma(pub WordId);

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
pub struct Form(pub WordId);

pub use crate::authors_chrono::Author;

impl FormData {
    pub fn new(source: SourceId, line_no: usize, form: Form) -> Self {
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

    pub fn form(&self) -> Form {
        self.form
    }

    pub fn author(&self, db: &impl MainDatabase) -> AuthorId {
        db.associated_author(self.source())
    }
}
