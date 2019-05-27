mod context;
mod inputs;
mod types;

use crate::context::Context;
use crate::inputs::AuthorsInput;
use crate::types::{Form, Lemma};
use juniper::FieldResult;
use latin_utilities::NormalizedLatinString;

use query_system::traits::InternDatabase;

struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "0.1"
    }

    #[graphql(
        arguments(
            lemma(
                description = "The lemma to lookup"
            ),
            authors(
                description = "The authors to query",
                default = AuthorsInput::all(),
            )
        )
    )]
    fn lemma(context: &Context, lemma: String, authors: AuthorsInput) -> FieldResult<Lemma> {
        let lemma = query_system::types::Lemma(NormalizedLatinString::from(lemma.as_str()));
        let authors = authors.get_authors(context);

        Ok(Lemma::new(context.as_ref().intern_lemma(lemma), authors))
    }

    #[graphql(
        arguments(
            form(
                description = "The form to lookup"
            ),
            authors(
                description = "The authors to query",
                default = AuthorsInput::all(),
            )
        )
    )]
    fn form(context: &Context, form: String, authors: AuthorsInput) -> FieldResult<Form> {
        let form = query_system::types::Form(NormalizedLatinString::from(form.as_str()));
        let authors = authors.get_authors(context);

        Ok(Form::new(context.as_ref().intern_form(form), authors))
    }
}
