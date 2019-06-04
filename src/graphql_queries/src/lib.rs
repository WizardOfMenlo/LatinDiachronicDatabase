pub mod context;
mod inputs;
mod types;

use crate::context::Context;
use crate::inputs::{AuthorsInput, Filter, SpanInput};
use crate::types::{Author, Form, Lemma};
use juniper::{graphql_value, EmptyMutation, FieldError, FieldResult, RootNode};
use latin_utilities::NormalizedLatinString;

use query_system::traits::*;

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
            .iter()
            .take(limit)
            .map(|(_, v)| Author::new(*v))
            .collect())
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
        let lemma = query_system::types::Lemma(NormalizedLatinString::from(lemma.as_str()));
        let authors = authors.intersect(span).get_authors(context);

        Ok(Lemma::new(context.get().intern_lemma(lemma), authors))
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
        let form = query_system::types::Form(NormalizedLatinString::from(form.as_str()));
        let authors = authors.intersect(span).get_authors(context);

        Ok(Form::new(context.get().intern_form(form), authors))
    }
}
