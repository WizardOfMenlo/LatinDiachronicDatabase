use super::context::Context;
use crate::query_system::ids::AuthorId;
use crate::query_system::ids::FormDataId;
use crate::query_system::ids::FormId;
use crate::query_system::ids::LemmaId;
use crate::query_system::ids::SourceId;
use crate::query_system::lit_subset::LitSubset;
use crate::query_system::traits::*;
use crate::query_system::types;

use chrono::prelude::Datelike;
use std::sync::Arc;

pub struct Author {
    id: AuthorId,
}

pub struct TimeSpan {
    time_span: crate::authors_chrono::TimeSpan,
}

#[juniper::object]
impl TimeSpan {
    fn start(&self) -> i32 {
        self.time_span.start().year()
    }

    fn end(&self) -> i32 {
        self.time_span.end().year()
    }
}

impl Author {
    pub(crate) fn new(id: AuthorId) -> Self {
        Author { id }
    }

    fn author(&self, context: &Context) -> types::Author {
        context
            .get()
            .authors()
            .get_by_right(&self.id)
            .cloned()
            .expect("No authorid should be created")
    }
}

#[juniper::object(Context = Context)]
impl Author {
    fn name(&self, context: &Context) -> String {
        self.author(context).name().to_string()
    }

    fn sources(&self, context: &Context) -> Vec<Source> {
        let db = context.get();
        let sources = db.associated_sources(self.id);
        sources.iter().cloned().map(Source::new).collect()
    }

    fn time_span(&self, context: &Context) -> Option<TimeSpan> {
        let auth = self.author(context);
        auth.tspan()
            .cloned()
            .map(|time_span| TimeSpan { time_span })
    }
}

pub struct Source {
    source_id: SourceId,
}

impl Source {
    fn new(p: SourceId) -> Self {
        Source { source_id: p }
    }
}

#[juniper::object(Context = Context)]
impl Source {
    fn name(&self, context: &Context) -> String {
        let db = context.get();

        let p = db
            .sources()
            .get_by_right(&self.source_id)
            .expect("Invalid source file");

        p.file_name().unwrap().to_string_lossy().to_string()
    }

    fn author(&self, context: &Context) -> Author {
        let db = context.get();

        let author_id = db.associated_author(self.source_id);

        Author::new(author_id)
    }
}

pub struct Occurrence {
    id: FormDataId,
}

#[juniper::object(Context = Context)]
impl Occurrence {
    fn line(&self, context: &Context) -> String {
        let db = context.get();
        let fd = db.lookup_intern_form_data(self.id);
        db.get_line(fd.source(), fd.line_no()).unwrap().to_string()
    }

    fn source(&self, context: &Context) -> Source {
        let db = context.get();
        let fd = db.lookup_intern_form_data(self.id);
        Source::new(fd.source())
    }

    fn ambiguos(&self, context: &Context) -> bool {
        let db = context.get();
        let fd = db.lookup_intern_form_data(self.id);
        // TODO: Global empty vec
        let form = Form::new(fd.form(), Arc::new(Vec::new()));
        drop(db);
        form.is_ambig(context)
    }
}

pub struct Form {
    form: FormId,
    authors: Arc<Vec<AuthorId>>,
}

impl Form {
    pub(crate) fn new(form: FormId, authors: Arc<Vec<AuthorId>>) -> Self {
        Form { form, authors }
    }

    pub(crate) fn from_iter(form: FormId, authors: impl IntoIterator<Item = AuthorId>) -> Self {
        Form::new(form, Arc::new(authors.into_iter().collect()))
    }

    pub(crate) fn is_ambig(&self, context: &Context) -> bool {
        let db = context.get();
        let form = db.lookup_intern_form(self.form).0;
        let lemm = db.lemmatizer();

        lemm.get_possible_lemmas(&form)
            .map(|s| s.len())
            .unwrap_or(0)
            > 1
    }
}

#[juniper::object(Context = Context)]
impl Form {
    fn form(&self, context: &Context) -> String {
        context
            .get()
            .lookup_intern_form(self.form)
            .0
            .inner()
            .to_string()
    }

    fn lemmas(&self, context: &Context) -> Vec<Lemma> {
        let db = context.get();
        let form = db.lookup_intern_form(self.form).0;
        let lemm = db.lemmatizer();

        lemm.get_possible_lemmas(&form)
            .cloned()
            .map(|v| {
                v.into_iter()
                    .map(|l| db.intern_lemma(crate::query_system::types::Lemma(l)))
                    .map(|l| Lemma::new(l, self.authors.clone()))
                    .collect()
            })
            .unwrap_or_else(Vec::new)
    }

    fn ambiguos(&self, context: &Context) -> bool {
        self.is_ambig(context)
    }

    fn count(&self, context: &Context) -> i32 {
        let db = context.get();
        db.count_form_occurrences_subset(
            self.form,
            LitSubset::from_authors(self.authors.iter(), &db),
        ) as i32
    }

    fn occurrences(&self, context: &Context) -> Vec<Occurrence> {
        let db = context.get();
        db.form_occurrences_subset(self.form, LitSubset::from_authors(self.authors.iter(), &db))
            .iter()
            .map(|s| Occurrence { id: *s })
            .collect()
    }
}

pub struct Lemma {
    lemma: LemmaId,
    authors: Arc<Vec<AuthorId>>,
}

impl Lemma {
    pub(crate) fn new(lemma: LemmaId, authors: Arc<Vec<AuthorId>>) -> Self {
        Lemma { lemma, authors }
    }

    pub(crate) fn from_iter(lemma: LemmaId, authors: impl IntoIterator<Item = AuthorId>) -> Self {
        Lemma::new(lemma, Arc::new(authors.into_iter().collect()))
    }
}

#[juniper::object(Context = Context)]
impl Lemma {
    fn lemma(&self, context: &Context) -> String {
        context
            .get()
            .lookup_intern_lemma(self.lemma)
            .0
            .inner()
            .to_string()
    }

    fn forms(&self, context: &Context) -> Vec<Form> {
        let db = context.get();
        let lemma = db.lookup_intern_lemma(self.lemma).0;
        let lemm = db.lemmatizer();

        lemm.get_possible_forms(&lemma)
            .cloned()
            .map(|v| {
                v.into_iter()
                    .map(|f| db.intern_form(crate::query_system::types::Form(f)))
                    .map(|f| Form::new(f, self.authors.clone()))
                    .collect()
            })
            .unwrap_or_else(Vec::new)
    }

    fn count(&self, context: &Context) -> i32 {
        let db = context.get();
        db.count_lemma_occurrences_subset(
            self.lemma,
            LitSubset::from_authors(self.authors.iter(), &db),
        ) as i32
    }

    fn occurrences(&self, context: &Context) -> Vec<Occurrence> {
        let db = context.get();
        db.lemma_occurrences_subset(
            self.lemma,
            LitSubset::from_authors(self.authors.iter(), &db),
        )
        .iter()
        .map(|s| Occurrence { id: *s })
        .collect()
    }
}
