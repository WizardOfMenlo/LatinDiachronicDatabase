//! Core idea, use salsa for queries ID to ID
//! The sources module has queries directly on source
//! The ids contains all the various id types that are used directly into salsa
//! Types are all the expanded types those ids refer to

#[macro_use]
extern crate log;

pub mod ids;
pub mod middle;
pub mod sources;
pub mod traits;
pub mod types;

use ids::*;

#[salsa::query_group(MainQueries)]
pub trait MainDatabase:
    sources::SourcesDatabase
    + types::InternDatabase
    + middle::IntermediateDatabase
    + salsa::Database
    + salsa::ParallelDatabase
{
    fn count_lemma_occurrences_sources(&self, id: LemmaId, sources: Vec<SourceId>) -> usize;
    fn count_lemma_occurrences_authors(&self, id: LemmaId, authors: Vec<AuthorId>) -> usize;

    fn count_form_occurrences_sources(&self, id: FormId, sources: Vec<SourceId>) -> usize;
    fn count_form_occurrences_authors(&self, id: FormId, authors: Vec<AuthorId>) -> usize;
}

fn count_lemma_occurrences_sources(
    db: &impl MainDatabase,
    id: LemmaId,
    sources: Vec<SourceId>,
) -> usize {
    db.lemma_occurrences_sources(id, sources).len()
}

fn count_lemma_occurrences_authors(
    db: &impl MainDatabase,
    id: LemmaId,
    authors: Vec<AuthorId>,
) -> usize {
    db.lemma_occurrences_authors(id, authors).len()
}

fn count_form_occurrences_sources(
    db: &impl MainDatabase,
    id: FormId,
    sources: Vec<SourceId>,
) -> usize {
    db.form_occurrences_sources(id, sources).len()
}

fn count_form_occurrences_authors(
    db: &impl MainDatabase,
    id: FormId,
    authors: Vec<AuthorId>,
) -> usize {
    db.form_occurrences_authors(id, authors).len()
}
