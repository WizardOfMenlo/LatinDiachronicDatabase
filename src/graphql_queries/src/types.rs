use crate::context::Context;
use latin_utilities::NormalizedLatinString;
use query_system::ids::AuthorId;
use query_system::traits::*;
use query_system::types;

pub struct Author {
    id: AuthorId,
}

impl Author {
    pub(crate) fn new(id: AuthorId) -> Self {
        Author { id }
    }

    fn author<'a>(&self, context: &'a Context) -> &'a types::Author {
        // TODO We could cache this?
        context
            .as_ref()
            .authors()
            .iter()
            .find(|(_, &v)| v == self.id)
            .map(|(k, _)| k)
            .expect("No authorid should be created")
    }
}

#[juniper::object(Context = Context)]
impl Author {
    fn name(&self, context: &Context) -> &str {
        self.author(context).name()
    }

    fn sources(&self, context: &Context) -> Vec<Source> {
        let sources = context.as_ref().associated_sources(self.id);
        context
            .as_ref()
            .sources()
            .iter()
            .filter(|(_, v)| sources.contains(v))
            .map(|(k, _)| Source::new(k.as_path()))
            .collect()
    }
}

#[derive(juniper::GraphQLObject)]
pub struct Source {
    name: String,
}

impl Source {
    fn new(p: &std::path::Path) -> Self {
        Source {
            name: p.file_name().unwrap().to_string_lossy().to_string(),
        }
    }
}

pub struct Form {
    form: NormalizedLatinString,
    authors: Vec<AuthorId>,
}

impl Form {
    pub(crate) fn new(form: NormalizedLatinString, authors: Vec<AuthorId>) -> Self {
        Form { form, authors }
    }
}

#[juniper::object(Context = Context)]
impl Form {
    fn form(&self) -> &str {
        self.form.inner()
    }

    fn count(&self, context: &Context) -> i32 {
        let db = context.as_ref();
        db.count_form_occurrences_authors(
            db.intern_form(types::Form(self.form.clone())),
            self.authors.clone(),
        ) as i32
    }
}

pub struct Lemma {
    lemma: NormalizedLatinString,
    authors: Vec<AuthorId>,
}

impl Lemma {
    pub(crate) fn new(lemma: NormalizedLatinString, authors: Vec<AuthorId>) -> Self {
        Lemma { lemma, authors }
    }
}

#[juniper::object(Context = Context)]
impl Lemma {
    fn lemma(&self) -> &str {
        self.lemma.inner()
    }

    fn count(&self, context: &Context) -> i32 {
        let db = context.as_ref();
        db.count_lemma_occurrences_authors(
            db.intern_lemma(types::Lemma(self.lemma.clone())),
            self.authors.clone(),
        ) as i32
    }
}
