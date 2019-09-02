//! Core idea, use salsa for queries ID to ID
//! The sources module has queries directly on source
//! The ids contains all the various id types that are used directly into salsa
//! Types are all the expanded types those ids refer to

#[macro_use]
extern crate log;

pub mod ids;
pub mod lit_subset;
pub mod middle;
pub mod sources;
pub mod traits;
pub mod types;

pub mod mock;

use ids::*;
use lit_subset::LitSubset;

/// The main trait, which any database should implement
#[salsa::query_group(MainQueries)]
pub trait MainDatabase:
    sources::SourcesDatabase
    + types::InternDatabase
    + types::AuthorInternDatabase
    + middle::IntermediateDatabase
    + salsa::Database
    + salsa::ParallelDatabase
{
    // TODO, this is really slow (I think? It is)

    /// Count the number of occurrences of lemma in a subset of the literature
    fn count_lemma_occurrences_subset(&self, id: LemmaId, subset: LitSubset) -> usize;

    /// Count the number of occurrences of a form in a subset of the literature
    fn count_form_occurrences_subset(&self, id: FormId, subset: LitSubset) -> usize;
}

fn count_lemma_occurrences_subset(db: &impl MainDatabase, id: LemmaId, subset: LitSubset) -> usize {
    db.lemma_occurrences_subset(id, subset).len()
}

fn count_form_occurrences_subset(db: &impl MainDatabase, id: FormId, subset: LitSubset) -> usize {
    db.form_occurrences_subset(id, subset).len()
}
