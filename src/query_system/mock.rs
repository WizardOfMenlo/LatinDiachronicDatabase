//! Mocking facilities for testing

use super::gc::GCollectable;
use super::ids::AuthorId;
use super::middle::IntermediateQueries;
use super::sources::SourcesQueryGroup;
use super::traits::{AuthorInternDatabase, IntermediateDatabase};
use super::types::InternersGroup;
use super::MainQueries;
use crate::authors_chrono::Author;
use crate::latin_lemmatizer::NaiveLemmatizer;

use std::collections::HashMap;
use std::sync::Arc;

/// A simplified database, which we use for testing
#[salsa::database(SourcesQueryGroup, InternersGroup, MainQueries, IntermediateQueries)]
pub struct MockDatabase {
    runtime: salsa::Runtime<MockDatabase>,
    mock: Author,
}

/// A mock database to be used for preliminary testing
/// Note, we have empty lemm, so this will fail complex queries
pub fn make_mock() -> MockDatabase {
    let mut res = MockDatabase::new();
    res.set_lemmatizer(Arc::new(NaiveLemmatizer::new(HashMap::new())));
    res
}

impl MockDatabase {
    pub fn new() -> Self {
        MockDatabase {
            runtime: salsa::Runtime::default(),
            mock: Author::new("Mock"),
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

impl GCollectable for MockDatabase {
    fn garbage_sweep(&mut self) {}
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
        })
    }
}
