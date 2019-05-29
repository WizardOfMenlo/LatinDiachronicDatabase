use crate::ids::{AuthorId, SourceId};
use crate::traits::*;
use std::collections::BTreeSet;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct LitSubset {
    sources: BTreeSet<SourceId>,
}

impl LitSubset {
    pub fn from_sources(sources: &[SourceId]) -> Self {
        LitSubset {
            sources: sources.iter().cloned().collect(),
        }
    }

    pub fn from_authors(authors: &[AuthorId], db: &salsa::Snapshot<impl MainDatabase>) -> Self {
        let mut sources = BTreeSet::new();

        for src in authors.iter().map(|a| db.associated_sources(*a)) {
            sources.extend(src.iter())
        }

        LitSubset { sources }
    }

    pub fn sources(&self) -> &BTreeSet<SourceId> {
        &self.sources
    }
}
