//! Mocking facilities for testing

use super::gc::GCollectable;
use super::ids::AuthorId;
use super::middle::IntermediateQueries;
use super::sources::SourcesQueryGroup;
use super::traits::{AuthorInternDatabase, IntermediateDatabase};
use super::types::InternersGroup;
use super::MainQueries;
use crate::authors_chrono::Author;
use crate::filesystem::{GetFileSystem, MockFileSystem};
use crate::latin_lemmatizer::compressed::CompressedLemmatizer;
use crate::latin_utilities::NormalizedLatinString;
use crate::word_db::{WordDatabase, WordDb, WordId};

use std::sync::Arc;

/// A simplified database, which we use for testing
#[salsa::database(SourcesQueryGroup, InternersGroup, MainQueries, IntermediateQueries)]
pub struct MockDatabase {
    runtime: salsa::Runtime<MockDatabase>,
    mock: Author,
    fs: MockFileSystem,
    word_db: WordDb,
}

/// A mock database to be used for preliminary testing
/// Note, we have empty lemm, so this will fail complex queries
pub fn make_mock() -> MockDatabase {
    let mut res = MockDatabase::new();
    res.set_lemmatizer(Arc::new(CompressedLemmatizer::default()));
    res
}

impl MockDatabase {
    pub fn new() -> Self {
        MockDatabase {
            runtime: salsa::Runtime::default(),
            mock: Author::new("Mock"),
            fs: MockFileSystem::default(),
            word_db: WordDb::default(),
        }
    }
}

impl AuthorInternDatabase for MockDatabase {
    fn intern_author(&mut self, _: Author) -> AuthorId {
        AuthorId::from_integer(0)
    }

    fn lookup_intern_author(&self, _: AuthorId) -> &Author {
        &self.mock
    }
}

impl Default for MockDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl GetFileSystem for MockDatabase {
    type Fs = MockFileSystem;

    fn filesystem(&self) -> &Self::Fs {
        &self.fs
    }

    fn filesystem_mut(&mut self) -> &mut Self::Fs {
        &mut self.fs
    }
}

impl GCollectable for MockDatabase {
    fn garbage_sweep(&mut self) {}
}

impl WordDatabase for MockDatabase {
    fn intern_word(&self, s: NormalizedLatinString) -> WordId {
        self.word_db.intern_word(s)
    }

    fn lookup_word(&self, id: WordId) -> NormalizedLatinString {
        self.word_db.lookup_word(id)
    }

    fn lookup_interned_word(&self, s: NormalizedLatinString) -> Option<WordId> {
        Some(self.word_db.intern_word(s))
    }
}

impl salsa::Database for MockDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }

    fn salsa_runtime_mut(&mut self) -> &mut salsa::Runtime<Self> {
        &mut self.runtime
    }
}

impl salsa::ParallelDatabase for MockDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(MockDatabase {
            runtime: self.runtime.snapshot(self),
            mock: self.mock.clone(),
            fs: self.fs.clone(),
            word_db: self.word_db.clone(),
        })
    }
}
