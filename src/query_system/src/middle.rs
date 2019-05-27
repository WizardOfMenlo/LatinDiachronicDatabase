use crate::ids::{AuthorId, FormDataId, FormId, LemmaId, SourceId};
use crate::sources::SourcesDatabase;
use crate::types::InternDatabase;

use latin_lemmatizer::NaiveLemmatizer;

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator as _;
use std::sync::Arc;

/// This trait defines ways to aggregate lemmas and forms based on both authors and sources
#[salsa::query_group(IntermediateQueries)]
pub trait IntermediateDatabase: SourcesDatabase + InternDatabase + AsRef<NaiveLemmatizer> {
    fn parse_sources(&self, sources: Vec<SourceId>) -> Arc<HashSet<FormDataId>>;
    fn parse_authors(&self, authors: Vec<AuthorId>) -> Arc<HashSet<FormDataId>>;

    fn authors_sources(&self, authors: Vec<AuthorId>) -> Arc<HashSet<SourceId>>;

    // Index all sources for forms ------------------------------------------
    fn forms_in_source(&self, source: SourceId) -> Arc<HashSet<FormId>>;
    fn forms_in_sources(&self, sources: Vec<SourceId>) -> Arc<HashSet<FormId>>;
    fn forms_in_authors(&self, authors: Vec<AuthorId>) -> Arc<HashSet<FormId>>;
    // -----------------------------------------------------------------------

    // Index all sources for lemmas ------------------------------------------
    fn lemmas_in_source(&self, source: SourceId) -> Arc<HashSet<LemmaId>>;
    fn lemmas_in_sources(&self, sources: Vec<SourceId>) -> Arc<HashSet<LemmaId>>;
    fn lemmas_in_authors(&self, authors: Vec<AuthorId>) -> Arc<HashSet<LemmaId>>;
    // -----------------------------------------------------------------------

    fn form_occurrences_sources(
        &self,
        form_id: FormId,
        sources: Vec<SourceId>,
    ) -> Arc<HashSet<FormDataId>>;

    fn form_occurrences_authors(
        &self,
        form_id: FormId,
        sources: Vec<AuthorId>,
    ) -> Arc<HashSet<FormDataId>>;

    fn lemma_occurrences_sources(
        &self,
        lemma_id: LemmaId,
        sources: Vec<SourceId>,
    ) -> Arc<HashSet<FormDataId>>;

    fn lemma_occurrences_authors(
        &self,
        lemma_id: LemmaId,
        sources: Vec<AuthorId>,
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
    let lemm = db.as_ref();

    Arc::new(
        lemm.get_possible_lemmas(&form)
            .cloned()
            .unwrap_or_else(HashSet::new)
            .into_iter()
            .map(|l| db.intern_lemma(crate::types::Lemma(l)))
            .collect(),
    )
}

fn parse_sources(
    db: &impl IntermediateDatabase,
    sources: Vec<SourceId>,
) -> Arc<HashSet<FormDataId>> {
    // TODO, I think this allocates twice?
    combine(sources.into_iter().map(|s| db.parse_source(s)))
}

fn authors_sources(
    db: &impl IntermediateDatabase,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<SourceId>> {
    combine(authors.into_iter().map(|a| db.associated_sources(a)))
}

fn parse_authors(
    db: &impl IntermediateDatabase,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<FormDataId>> {
    db.parse_sources(Vec::from_iter(db.authors_sources(authors).iter().cloned()))
}

fn forms_in_source(db: &impl IntermediateDatabase, source: SourceId) -> Arc<HashSet<FormId>> {
    let mut res = HashSet::new();
    for fd_id in db.parse_source(source).iter() {
        res.insert(db.lookup_intern_form_data(*fd_id).form());
    }
    Arc::new(res)
}

fn forms_in_sources(
    db: &impl IntermediateDatabase,
    sources: Vec<SourceId>,
) -> Arc<HashSet<FormId>> {
    combine(sources.into_iter().map(|s| db.forms_in_source(s)))
}

fn forms_in_authors(
    db: &impl IntermediateDatabase,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<FormId>> {
    db.forms_in_sources(Vec::from_iter(db.authors_sources(authors).iter().cloned()))
}

fn lemmas_in_source(db: &impl IntermediateDatabase, source: SourceId) -> Arc<HashSet<LemmaId>> {
    combine(
        db.forms_in_source(source)
            .iter()
            .map(|&f| lemmatize_form(db, f)),
    )
}

fn lemmas_in_sources(
    db: &impl IntermediateDatabase,
    sources: Vec<SourceId>,
) -> Arc<HashSet<LemmaId>> {
    combine(sources.into_iter().map(|s| db.lemmas_in_source(s)))
}

fn lemmas_in_authors(
    db: &impl IntermediateDatabase,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<LemmaId>> {
    db.lemmas_in_sources(Vec::from_iter(db.authors_sources(authors).iter().cloned()))
}

fn form_occurrences_sources(
    db: &impl IntermediateDatabase,
    id: FormId,
    sources: Vec<SourceId>,
) -> Arc<HashSet<FormDataId>> {
    info!("Looking for form: {:?} in {} sources", id, sources.len());
    Arc::new(
        db.parse_sources(sources)
            .iter()
            .filter(|&fd| db.lookup_intern_form_data(*fd).form() == id)
            .cloned()
            .collect(),
    )
}

fn form_occurrences_authors(
    db: &impl IntermediateDatabase,
    id: FormId,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<FormDataId>> {
    info!("Looking for form: {:?} in {} authors", id, authors.len());
    Arc::new(
        db.parse_authors(authors)
            .iter()
            .filter(|&fd| db.lookup_intern_form_data(*fd).form() == id)
            .cloned()
            .collect(),
    )
}

fn lemma_occurrences_sources(
    db: &impl IntermediateDatabase,
    id: LemmaId,
    sources: Vec<SourceId>,
) -> Arc<HashSet<FormDataId>> {
    info!("Looking for lemma: {:?} in {} sources", id, sources.len());
    // TODO, making the lemmatizer bidirectional could save some time here?
    Arc::new(
        db.parse_sources(sources)
            .iter()
            .filter(|&fd| {
                let form = db.lookup_intern_form_data(*fd).form();
                lemmatize_form(db, form).contains(&id)
            })
            .cloned()
            .collect(),
    )
}

fn lemma_occurrences_authors(
    db: &impl IntermediateDatabase,
    id: LemmaId,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<FormDataId>> {
    info!("Looking for lemma: {:?} in {} authors", id, authors.len());
    Arc::new(
        db.parse_authors(authors)
            .iter()
            .filter(|&fd| {
                let form = db.lookup_intern_form_data(*fd).form();
                lemmatize_form(db, form).contains(&id)
            })
            .cloned()
            .collect(),
    )
}
