use super::context::Context;
use crate::query_system::ids::AuthorId;
use chrono::NaiveDate;
use std::collections::BTreeSet;
use std::collections::HashSet;

pub trait Filter {
    /// Return a filter which matches everything
    fn all() -> Self;

    /// Get the authors that the filters restricts to
    fn get_authors(&self, context: &Context) -> BTreeSet<AuthorId>;

    /// Given filters with author sets A B, make a filter which computes A intersect B
    fn intersect<T>(self, other: T) -> FilterIntersect<Self, T>
    where
        T: Filter,
        Self: Sized,
    {
        FilterIntersect(self, other)
    }
}

#[derive(juniper::GraphQLInputObject, Debug)]
#[graphql(
    name = "Authors",
    description = "The authors to filter a research with"
)]
pub struct AuthorsInput {
    #[graphql(description = "Use all authors in the database")]
    use_all: bool,
    list: Option<Vec<String>>,
}

impl Filter for AuthorsInput {
    fn all() -> Self {
        Self {
            use_all: true,
            list: None,
        }
    }

    // Get the list of authors to apply the query to
    fn get_authors(&self, context: &Context) -> BTreeSet<AuthorId> {
        let db = context.get();
        if self.use_all {
            return db.authors().right_values().cloned().collect();
        }

        // TODO, this can be probably done better
        let hashset: HashSet<String> = self
            .list
            .clone()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .collect();

        db.authors()
            .iter()
            .filter(|(k, _)| hashset.contains(k.name()))
            .map(|(_, v)| v)
            .cloned()
            .collect()
    }
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
pub struct Span {
    start_year: NaiveDate,
    end_year: NaiveDate,
}

#[derive(juniper::GraphQLInputObject, Debug)]
#[graphql(
    name = "SpanInput",
    description = "The time span to filter a research with"
)]
pub struct SpanInput {
    #[graphql(description = "Use all authors in the database")]
    use_all: bool,
    span: Option<Span>,
}

impl Filter for SpanInput {
    fn all() -> Self {
        SpanInput {
            use_all: true,
            span: None,
        }
    }

    fn get_authors(&self, context: &Context) -> BTreeSet<AuthorId> {
        let db = context.get();
        if self.use_all {
            return db.authors().right_values().cloned().collect();
        }
        let span = self.span.as_ref().cloned().unwrap();
        let timespan = crate::authors_chrono::TimeSpan::new(span.start_year, span.end_year);

        db.authors()
            .iter()
            .filter(|(k, _)| k.in_timespan(&timespan))
            .map(|(_, v)| *v)
            .collect()
    }
}

#[derive(Debug)]
pub struct FilterIntersect<A, B>(A, B);

impl<A, B> Filter for FilterIntersect<A, B>
where
    A: Filter,
    B: Filter,
{
    fn all() -> Self {
        FilterIntersect(A::all(), B::all())
    }

    fn get_authors(&self, context: &Context) -> BTreeSet<AuthorId> {
        self.0
            .get_authors(context)
            .intersection(&self.1.get_authors(context))
            .cloned()
            .collect()
    }
}
