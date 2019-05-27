mod context;
mod inputs;
mod types;

use crate::context::Context;
use crate::inputs::AuthorsInput;
use crate::types::Lemma;
use juniper::FieldResult;
use latin_utilities::NormalizedLatinString;

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
        let lemma = NormalizedLatinString::from(lemma.as_str());
        let authors = authors.get_authors(context);

        Ok(Lemma::new( lemma, authors ))
    }
}
