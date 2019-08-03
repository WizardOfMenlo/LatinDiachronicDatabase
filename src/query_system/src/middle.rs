//! Probably the most important mod, contains the traits needed to convert from
//! parsed files to highlevel representations

use crate::ids::{FormDataId, FormId, LemmaId, SourceId};
use crate::lit_subset::LitSubset;
use crate::sources::SourcesDatabase;
use crate::types::InternDatabase;

use latin_lemmatizer::NaiveLemmatizer;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::Arc;

/// This trait defines ways to aggregate lemmas and forms based on both authors and sources  
/// Usage: Load the source database, then run any query
#[salsa::query_group(IntermediateQueries)]
pub trait IntermediateDatabase: SourcesDatabase + InternDatabase {
    /// The lemmatizer that is used
    #[salsa::input]
    fn lemmatizer(&self) -> Arc<NaiveLemmatizer>;

    /// Parse multiple sources, and combine the result
    fn parse_subset(&self, subset: LitSubset) -> Arc<HashSet<FormDataId>>;

    // TODO, find a better name
    fn source_tree(&self, id: SourceId) -> Arc<HashMap<LemmaId, HashMap<FormId, Vec<FormDataId>>>>;
    fn subset_tree(
        &self,
        sub: LitSubset,
    ) -> Arc<HashMap<LemmaId, HashMap<FormId, Vec<FormDataId>>>>;

    // Index all sources for forms ------------------------------------------

    /// Get all the forms that appear in a source
    fn forms_in_source(&self, source: SourceId) -> Arc<HashSet<FormId>>;

    /// Get all the forms that appear in some sources
    fn forms_in_subset(&self, subset: LitSubset) -> Arc<HashSet<FormId>>;

    // -----------------------------------------------------------------------

    // Index all sources for lemmas ------------------------------------------

    /// Get all lemmas that appear in a source
    fn lemmas_in_source(&self, source: SourceId) -> Arc<HashSet<LemmaId>>;

    /// Get all lemmas that appear in some sources
    fn lemmas_in_subset(&self, subset: LitSubset) -> Arc<HashSet<LemmaId>>;

    // -----------------------------------------------------------------------

    /// For a form, get all the occurrences in the subset of the literature
    fn form_occurrences_subset(
        &self,
        form_id: FormId,
        subset: LitSubset,
    ) -> Arc<HashSet<FormDataId>>;

    /// For a lemma, get all the occurrences in the subset of the literature
    fn lemma_occurrences_subset(
        &self,
        lemma_id: LemmaId,
        subset: LitSubset,
    ) -> Arc<HashSet<FormDataId>>;
}

// Sum sets of sources together
fn combine<'a, T: Hash + Eq + Clone + 'a>(
    sets: impl IntoIterator<Item = Arc<HashSet<T>>>,
) -> Arc<HashSet<T>> {
    // TODO, most of these constr are then Arc, might be worthwhile to work with it here to remove allocations
    let mut res = HashSet::new();

    for s in sets.into_iter() {
        res.extend(s.iter().cloned())
    }

    Arc::new(res)
}

// Lemmatizes a form, in an interface that works well with above
fn lemmatize_form(db: &impl IntermediateDatabase, form_id: FormId) -> Arc<HashSet<LemmaId>> {
    let form = db.lookup_intern_form(form_id).0;
    let lemm = db.lemmatizer();

    Arc::new(
        lemm.get_possible_lemmas(&form)
            .cloned()
            .unwrap_or_else(HashSet::new)
            .into_iter()
            .map(|l| db.intern_lemma(crate::types::Lemma(l)))
            .collect(),
    )
}

fn parse_subset(db: &impl IntermediateDatabase, subset: LitSubset) -> Arc<HashSet<FormDataId>> {
    combine(subset.sources().iter().map(|s| db.parse_source(*s)))
}

fn source_tree(
    db: &impl IntermediateDatabase,
    id: SourceId,
) -> Arc<HashMap<LemmaId, HashMap<FormId, Vec<FormDataId>>>> {
    let data = db.parse_source(id);
    let mut res = HashMap::new();
    for fd_id in data.iter() {
        let form = db.lookup_intern_form_data(*fd_id).form();
        let lemmas = lemmatize_form(db, form);
        for lemma in lemmas.iter() {
            res.entry(*lemma)
                .or_insert_with(HashMap::new)
                .entry(form)
                .or_insert_with(Vec::new)
                .push(*fd_id);
        }
    }

    Arc::new(res)
}

fn subset_tree(
    db: &impl IntermediateDatabase,
    sub: LitSubset,
) -> Arc<HashMap<LemmaId, HashMap<FormId, Vec<FormDataId>>>> {
    let mut res = HashMap::new();
    for source in sub.sources() {
        let tree = db.source_tree(*source);
        for (lemma, lemma_tree) in tree.iter() {
            for (&form, formdata) in lemma_tree {
                res.entry(*lemma)
                    .or_insert_with(HashMap::new)
                    .entry(form)
                    .or_insert_with(Vec::new)
                    // Note, since the sources are distinct, no hashsetting is needed
                    .extend(formdata);
            }
        }
    }

    Arc::new(res)
}

fn forms_in_source(db: &impl IntermediateDatabase, source: SourceId) -> Arc<HashSet<FormId>> {
    let mut res = HashSet::new();
    for fd_id in db.parse_source(source).iter() {
        res.insert(db.lookup_intern_form_data(*fd_id).form());
    }
    Arc::new(res)
}

fn forms_in_subset(db: &impl IntermediateDatabase, subset: LitSubset) -> Arc<HashSet<FormId>> {
    combine(subset.sources().iter().map(|s| db.forms_in_source(*s)))
}

fn lemmas_in_source(db: &impl IntermediateDatabase, source: SourceId) -> Arc<HashSet<LemmaId>> {
    combine(
        db.forms_in_source(source)
            .iter()
            .map(|&f| lemmatize_form(db, f)),
    )
}

fn lemmas_in_subset(db: &impl IntermediateDatabase, subset: LitSubset) -> Arc<HashSet<LemmaId>> {
    combine(subset.sources().iter().map(|s| db.lemmas_in_source(*s)))
}

fn form_occurrences_subset(
    db: &impl IntermediateDatabase,
    id: FormId,
    subset: LitSubset,
) -> Arc<HashSet<FormDataId>> {
    info!(
        "Looking for form: {:?} in {} sources",
        id,
        subset.sources().len()
    );
    Arc::new(
        db.parse_subset(subset)
            .iter()
            .filter(|&fd| db.lookup_intern_form_data(*fd).form() == id)
            .cloned()
            .collect(),
    )
}

fn lemma_occurrences_subset(
    db: &impl IntermediateDatabase,
    id: LemmaId,
    subset: LitSubset,
) -> Arc<HashSet<FormDataId>> {
    info!(
        "Looking for lemma: {:?} in {} sources",
        id,
        subset.sources().len()
    );
    // TODO, making the lemmatizer bidirectional could save some time here?
    Arc::new(
        db.parse_subset(subset)
            .iter()
            .filter(|&fd| {
                let form = db.lookup_intern_form_data(*fd).form();
                lemmatize_form(db, form).contains(&id)
            })
            .cloned()
            .collect(),
    )
}
