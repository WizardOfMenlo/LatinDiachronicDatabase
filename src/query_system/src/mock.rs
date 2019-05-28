use crate::middle::IntermediateQueries;
use crate::sources::SourcesQueryGroup;
use crate::types::InternersGroup;
use crate::MainQueries;
use latin_lemmatizer::NaiveLemmatizer;

#[salsa::database(SourcesQueryGroup, InternersGroup, MainQueries, IntermediateQueries)]
pub(crate) struct MockDatabase {
    runtime: salsa::Runtime<MockDatabase>,
    lemmatizer: NaiveLemmatizer,
}

impl MockDatabase {
    pub(crate) fn new(lemm: NaiveLemmatizer) -> Self {
        MockDatabase {
            runtime: salsa::Runtime::default(),
            lemmatizer: lemm,
        }
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
        })
    }
}
