pub mod context;
mod inputs;
mod types;

use crate::latin_utilities::NormalizedLatinString;
use crate::query_system::traits::*;
use crate::word_db::WordDatabase;
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
        let word = NormalizedLatinString::from(word.as_str());
        let db = context.get();
        let possible_id = db.lookup_interned_word(word);

        if let Some(id) = possible_id {
            let lemm = db.lemmatizer();
            Ok(if lemm.has_lemma(id) {
                WordType::Lemma
            } else if lemm.has_form(id) {
                WordType::Form
            } else {
                WordType::NotFound
            })
        } else {
            Ok(WordType::NotFound)
        }
    }

    fn intersection(
        context: &Context,
        authors: AuthorsInput,
        rest_of_lit: SpanInput,
    ) -> FieldResult<Vec<Lemma>> {
        use super::query_system::lit_subset::LitSubset;

        let authors = authors.get_authors(context);
        let rest_of_lit = rest_of_lit.get_authors(context);
        let db = context.get();

        let lemmas = db.intersect_sources(
            LitSubset::from_authors(authors.iter(), &db.snapshot()),
            LitSubset::from_authors(rest_of_lit.iter(), &db.snapshot()),
        );

        Ok(lemmas
            .iter()
            .map(|l| Lemma::from_iter(*l, authors.iter().cloned()))
            .collect())
    }

    fn intersection_hist(context: &Context, authors: AuthorsInput) -> FieldResult<Vec<Lemma>> {
        use super::authors_chrono::TimeSpan;
        use super::query_system::lit_subset::LitSubset;
        let authors = authors.get_authors(context);
        let max_timespan = authors
            .iter()
            .cloned()
            .map(Author::new)
            .flat_map(|a| a.tspan(context))
            .map(|t| t.time_span.get_century().1)
            .max();

        if max_timespan.is_none() {
            return Err(FieldError::new(
                "Intersection Historical Called with no historical author",
                graphql_value!({ "input_error" : "i"}),
            ));
        }

        let max_timespan = max_timespan.unwrap();
        let db = context.get();
        let rest_of_lit = LitSubset::from_timespan(
            &TimeSpan::new_cent(-10, max_timespan),
            db.authors(),
            &db.snapshot(),
        );

        let lemmas = db.intersect_sources(
            LitSubset::from_authors(authors.iter(), &db.snapshot()),
            rest_of_lit,
        );

        Ok(lemmas
            .iter()
            .map(|l| Lemma::from_iter(*l, authors.iter().cloned()))
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
        let nw = NormalizedLatinString::from(lemma.as_str());
        let authors = authors.intersect(span).get_authors(context);

        let db = context.get();
        let id = db.intern_word(nw);
        let lemma = crate::query_system::types::Lemma(id);

        Ok(Lemma::from_iter(db.intern_lemma(lemma), authors))
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
        let nw = NormalizedLatinString::from(form.as_str());
        let authors = authors.intersect(span).get_authors(context);

        let db = context.get();
        let id = db.intern_word(nw);
        let form = crate::query_system::types::Form(id);

        Ok(Form::from_iter(db.intern_form(form), authors))
    }
}
