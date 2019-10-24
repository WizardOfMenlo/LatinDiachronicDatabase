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
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

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

    fn intersect_sources(&self, sources: LitSubset, subset: LitSubset) -> Arc<HashSet<LemmaId>>;
}

fn count_lemma_occurrences_subset(db: &impl MainDatabase, id: LemmaId, subset: LitSubset) -> usize {
    db.lemma_occurrences_subset(id, subset).len()
}

fn count_form_occurrences_subset(db: &impl MainDatabase, id: FormId, subset: LitSubset) -> usize {
    db.form_occurrences_subset(id, subset).len()
}

fn intersect_sources(
    db: &impl MainDatabase,
    sources: LitSubset,
    subset: LitSubset,
) -> Arc<HashSet<LemmaId>> {
    // Get all the authors, and the selected sources
    let mut authors = HashMap::new();
    for (source, auth) in sources
        .sources()
        .iter()
        .map(|&s_id| (s_id, db.associated_author(s_id)))
    {
        authors
            .entry(auth)
            .or_insert_with(HashSet::new)
            .insert(source);
    }

    // Compute the rest of the literature
    let rest_of_literature =
        LitSubset::from_sources(subset.sources().difference(sources.sources()));

    // Get all the lemmas for each author
    let lemma_lists: Vec<_> = authors
        .values()
        .map(|s| db.lemmas_in_subset(LitSubset::from_sources(s)))
        .collect();

    // We need at least one of these
    if lemma_lists.is_empty() {
        return Arc::new(HashSet::new());
    }

    // Can optimize choosing the one with the least num
    // Compute the intersection
    let first = lemma_lists.get(0).unwrap();
    let intersection: HashSet<_> = first
        .iter()
        .filter(|l| lemma_lists.iter().all(|s| s.contains(l)))
        .cloned()
        .collect();

    // Compute diff with precedent
    Arc::new(
        intersection
            .difference(&*db.lemmas_in_subset(rest_of_literature))
            .cloned()
            .collect(),
    )
}
