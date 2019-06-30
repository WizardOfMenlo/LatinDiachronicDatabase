//! Mocking facilities for testing

use crate::middle::IntermediateQueries;
use crate::sources::SourcesQueryGroup;
use crate::types::InternersGroup;
use crate::traits::AuthorInternDatabase;
use crate::MainQueries;
use latin_lemmatizer::NaiveLemmatizer;
use std::collections::HashMap;
use authors_chrono::Author;
use crate::ids::AuthorId;

/// A simplified database, which we use for testing
#[salsa::database(SourcesQueryGroup, InternersGroup, MainQueries, IntermediateQueries)]
pub struct MockDatabase {
    runtime: salsa::Runtime<MockDatabase>,
    lemmatizer: NaiveLemmatizer,
    mock: Author
}

/// A mock database to be used for preliminary testing
/// Note, we have empty lemm, so this will fail complex queries
pub fn make_mock() -> MockDatabase {
    MockDatabase::new(NaiveLemmatizer::new(HashMap::new()))
}

impl MockDatabase {
    pub fn new(lemm: NaiveLemmatizer) -> Self {
        MockDatabase {
            runtime: salsa::Runtime::default(),
            lemmatizer: lemm,
            mock: Author::new("Mock")
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

impl AsRef<NaiveLemmatizer> for MockDatabase {
    fn as_ref(&self) -> &NaiveLemmatizer {
        &self.lemmatizer
    }
}

impl salsa::Database for MockDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }
}

impl salsa::ParallelDatabase for MockDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(MockDatabase {
            runtime: self.runtime.snapshot(self),
            lemmatizer: self.lemmatizer.clone(),
            mock: self.mock.clone()
        })
    }
}
