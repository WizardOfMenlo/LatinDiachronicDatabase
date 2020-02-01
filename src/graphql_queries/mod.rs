pub mod context;
mod inputs;
mod types;

use crate::latin_utilities::NormalizedLatinString;
use crate::query_system::traits::*;
use context::Context;
use inputs::{AuthorsInput, Filter, SpanInput};
use types::{Author, Form, Lemma, WordType};

use juniper::{graphql_value, EmptyMutation, FieldError, FieldResult, RootNode};

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, EmptyMutation::<Context>::new())
}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "0.1"
    }

    fn authors(context: &Context, first: Option<i32>) -> FieldResult<Vec<Author>> {
        let db = context.get();

        // Limit a number of authors
        let limit = match first {
            Some(i) if i >= 0 => i as usize,
            Some(_) => {
                return Err(FieldError::new(
                    "Invalid number of records",
                    graphql_value!({ "input_error" : "i"}),
                ))
            }
            None => db.authors().len(),
        };

        Ok(db
            .authors()
            .right_values()
            .map(|v| Author::new(*v))
            .take(limit)
            .collect())
    }

    fn word_type(context: &Context, word: String) -> FieldResult<WordType> {
        let lemm = context.get().lemmatizer();
        let word = NormalizedLatinString::from(word.as_str());
        Ok(if lemm.has_lemma(&word) {
            WordType::Lemma
        } else if lemm.has_form(&word) {
            WordType::Form
        } else {
            WordType::NotFound
        })
    }

    #[graphql(
        arguments(
            lemma(
                description = "The lemma to lookup"
            ),
            authors(
                description = "The authors to query",
                default = AuthorsInput::all(),
            ),
            span(
                description = "The timespan to search",
                default = SpanInput::all()
            )
        )
    )]
    fn lemma(
        context: &Context,
        lemma: String,
        authors: AuthorsInput,
        span: SpanInput,
    ) -> FieldResult<Lemma> {
        let lemma = crate::query_system::types::Lemma(NormalizedLatinString::from(lemma.as_str()));
        let authors = authors.intersect(span).get_authors(context);

        Ok(Lemma::from_iter(context.get().intern_lemma(lemma), authors))
    }

    #[graphql(
        arguments(
            form(
                description = "The form to lookup"
            ),
            authors(
                description = "The authors to query",
                default = AuthorsInput::all(),
            ),
            span(
                description = "The timespan to search",
                default = SpanInput::all()
            )
        )
    )]
    fn form(
        context: &Context,
        form: String,
        authors: AuthorsInput,
        span: SpanInput,
    ) -> FieldResult<Form> {
        let form = crate::query_system::types::Form(NormalizedLatinString::from(form.as_str()));
        let authors = authors.intersect(span).get_authors(context);

        Ok(Form::from_iter(context.get().intern_form(form), authors))
    }
}
