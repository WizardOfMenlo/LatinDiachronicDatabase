use crate::context::Context;
use latin_utilities::NormalizedLatinString;
use query_system::ids::AuthorId;
use query_system::ids::FormDataId;
use query_system::ids::FormId;
use query_system::ids::LemmaId;
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

pub struct Occurrence {
    id: FormDataId,
}

#[juniper::object(Context = Context)]
impl Occurrence {
    fn line(&self, context: &Context) -> String {
        let fd = context.as_ref().lookup_intern_form_data(self.id);
        context
            .as_ref()
            .get_line(fd.source(), fd.line_no())
            .unwrap()
            .to_string()
    }

    fn source(&self, context: &Context) -> Source {
        let fd = context.as_ref().lookup_intern_form_data(self.id);

        // TODO, this code is duplicated, extract and refactor
        context
            .as_ref()
            .sources()
            .iter()
            .find(|(_, &v)| v == fd.source())
            .map(|(k, _)| Source::new(k))
            .unwrap()
    }
}

pub struct Form {
    form: FormId,
    authors: Vec<AuthorId>,
}

impl Form {
    pub(crate) fn new(form: FormId, authors: Vec<AuthorId>) -> Self {
        Form { form, authors }
    }
}

#[juniper::object(Context = Context)]
impl Form {
    fn form(&self, context: &Context) -> String {
        context
            .as_ref()
            .lookup_intern_form(self.form)
            .0
            .inner()
            .to_string()
    }

    fn count(&self, context: &Context) -> i32 {
        let db = context.as_ref();
        db.count_form_occurrences_authors(self.form, self.authors.clone()) as i32
    }

    fn occurrences(&self, context: &Context) -> Vec<Occurrence> {
        let db = context.as_ref();
        db.form_occurrences_authors(self.form, self.authors.clone())
            .iter()
            .map(|s| Occurrence { id: *s })
            .collect()
    }
}

pub struct Lemma {
    lemma: LemmaId,
    authors: Vec<AuthorId>,
}

impl Lemma {
    pub(crate) fn new(lemma: LemmaId, authors: Vec<AuthorId>) -> Self {
        Lemma { lemma, authors }
    }
}

#[juniper::object(Context = Context)]
impl Lemma {
    fn lemma(&self, context: &Context) -> String {
        context
            .as_ref()
            .lookup_intern_lemma(self.lemma)
            .0
            .inner()
            .to_string()
    }

    fn count(&self, context: &Context) -> i32 {
        let db = context.as_ref();
        db.count_lemma_occurrences_authors(self.lemma, self.authors.clone()) as i32
    }

    fn occurrences(&self, context: &Context) -> Vec<Occurrence> {
        let db = context.as_ref();
        db.lemma_occurrences_authors(self.lemma, self.authors.clone())
            .iter()
            .map(|s| Occurrence { id: *s })
            .collect()
    }
}
