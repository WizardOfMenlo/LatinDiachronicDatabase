use crate::ids::{AuthorId, SourceId};
use crate::traits::*;
use authors_chrono::{Author, TimeSpan};
use std::collections::BTreeSet;
use std::sync::Arc;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
pub struct LitSubset {
    // Note, we arc it to make it cheaper to clone
    sources: Arc<BTreeSet<SourceId>>,
}

impl LitSubset {
    pub fn from_sources(sources: &[SourceId]) -> Self {
        LitSubset {
            sources: Arc::new(sources.iter().cloned().collect()),
        }
    }

    pub fn from_authors<'a>(
        authors: impl IntoIterator<Item = &'a AuthorId>,
        db: &salsa::Snapshot<impl MainDatabase>,
    ) -> Self {
        let mut sources = BTreeSet::new();

        for src in authors.into_iter().map(|a| db.associated_sources(*a)) {
            sources.extend(src.iter())
        }

        LitSubset {
            sources: Arc::new(sources),
        }
    }

    pub fn from_timespan<'a>(
        span: &TimeSpan,
        authors: impl IntoIterator<Item = &'a (Author, AuthorId)>,
        db: &salsa::Snapshot<impl MainDatabase>,
    ) -> Self {
        LitSubset::from_authors(
            authors
                .into_iter()
                .filter(|(a, _)| a.in_timespan(span))
                .map(|(_, i)| i),
            db,
        )
    }

    pub fn sources(&self) -> &BTreeSet<SourceId> {
        &self.sources
    }
}
