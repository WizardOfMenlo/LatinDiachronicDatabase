use crate::authors::{Author, AuthorId};
use crate::sources::SourceId;
use interner::Interner;
use std::path::PathBuf;

#[derive(Debug)]
pub struct MainInterner {
    pub source_interner: Interner<SourceId, PathBuf>,
    pub author_interner: Interner<AuthorId, Author>,
}

impl MainInterner {
    pub fn new(
        source_interner: Interner<SourceId, PathBuf>,
        author_interner: Interner<AuthorId, Author>,
    ) -> Self {
        Self {
            source_interner,
            author_interner,
        }
    }
}
