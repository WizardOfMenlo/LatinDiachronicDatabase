use crate::ids::{AuthorId, FormDataId, FormId, LemmaId, SourceId};
use crate::sources::SourcesDatabase;
use crate::types::InternDatabase;

use latin_lemmatizer::NaiveLemmatizer;

use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator as _;
use std::sync::Arc;

#[salsa::query_group(IntermediateQueries)]
pub trait IntermediateDatabase: SourcesDatabase + InternDatabase + AsRef<NaiveLemmatizer> {
    fn parse_sources(&self, sources: Vec<SourceId>) -> Arc<HashSet<FormDataId>>;
    fn parse_authors(&self, authors: Vec<AuthorId>) -> Arc<HashSet<FormDataId>>;

    fn authors_sources(&self, authors: Vec<AuthorId>) -> Arc<HashSet<SourceId>>;

    fn forms_in_source(&self, source: SourceId) -> Arc<HashSet<FormId>>;
    fn forms_in_sources(&self, sources: Vec<SourceId>) -> Arc<HashSet<FormId>>;
    fn forms_in_authors(&self, authors: Vec<AuthorId>) -> Arc<HashSet<FormId>>;

    fn lemmas_in_source(&self, source: SourceId) -> Arc<HashSet<LemmaId>>;
    fn lemmas_in_sources(&self, sources: Vec<SourceId>) -> Arc<HashSet<LemmaId>>;
    fn lemmas_in_authors(&self, authors: Vec<AuthorId>) -> Arc<HashSet<LemmaId>>;
}

// Sum sets of sources together
fn combine<'a, T: Hash + Eq + Clone + 'a>(
    sets: impl IntoIterator<Item = &'a HashSet<T>>,
) -> HashSet<T> {
    // TODO, most of these constr are then Arc, might be worthwhile to work with it here to remove allocations
    let res = HashSet::new();

    for s in sets.into_iter() {
        res.extend(s.into_iter().cloned())
    }

    res
}

fn parse_sources(
    db: &impl IntermediateDatabase,
    sources: Vec<SourceId>,
) -> Arc<HashSet<FormDataId>> {
    // TODO, I think this allocates twice?
    Arc::new(combine(sources.into_iter().map(|s| &*db.parse_source(s))))
}

fn authors_sources(
    db: &impl IntermediateDatabase,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<SourceId>> {
    Arc::new(combine(
        authors.into_iter().map(|a| &*db.associated_sources(a)),
    ))
}

fn parse_authors(
    db: &impl IntermediateDatabase,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<FormDataId>> {
    db.parse_sources(Vec::from_iter(db.authors_sources(authors).into_iter()))
}

fn forms_in_source(db: &impl IntermediateDatabase, source: SourceId) -> Arc<HashSet<FormId>> {
    let res = HashSet::new();
    for fd_id in db.parse_source(source).iter() {
        res.insert(db.lookup_intern_form_data(*fd_id).form());
    }
    Arc::new(res)
}

fn forms_in_sources(
    db: &impl IntermediateDatabase,
    sources: Vec<SourceId>,
) -> Arc<HashSet<FormId>> {
    Arc::new(combine(
        sources.into_iter().map(|s| &*db.forms_in_source(s)),
    ))
}

fn forms_in_authors(
    db: &impl IntermediateDatabase,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<FormId>> {
    db.forms_in_sources(Vec::from_iter(db.authors_sources(authors).into_iter()))
}

fn lemmas_in_source(db: &impl IntermediateDatabase, source: SourceId) -> Arc<HashSet<LemmaId>> {
    let forms = db.forms_in_source(source);
    let lemm = db.as_ref();
    let mut res = HashSet::new();

    for &fd_id in forms.iter() {
        let form = db.lookup_intern_form(fd_id).0;
        let possible_lemmas_o = lemm.get_possible_lemmas(&form);
        // Skip empty (these could go in not found?)
        if possible_lemmas_o.is_none() {
            continue;
        }
        let possible_lemmas = possible_lemmas_o.unwrap();
        res.extend(
            possible_lemmas
                .into_iter()
                .cloned()
                .map(|e| db.intern_lemma(crate::types::Lemma(e))),
        )
    }

    Arc::new(res)
}

fn lemmas_in_sources(
    db: &impl IntermediateDatabase,
    sources: Vec<SourceId>,
) -> Arc<HashSet<LemmaId>> {
    Arc::new(combine(
        sources.into_iter().map(|s| &*db.lemmas_in_source(s)),
    ))
}

fn lemmas_in_authors(
    db: &impl IntermediateDatabase,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<LemmaId>> {
    db.lemmas_in_sources(Vec::from_iter(db.authors_sources(authors).into_iter()))
}
