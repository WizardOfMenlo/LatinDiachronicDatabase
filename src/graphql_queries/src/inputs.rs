use crate::context::Context;
use query_system::ids::AuthorId;
use std::collections::HashSet;

#[derive(juniper::GraphQLInputObject)]
#[graphql(
    name = "Authors",
    description = "The authors to filter a research with"
)]
pub struct AuthorsInput {
    #[graphql(description = "Use all authors in the database")]
    use_all: bool,
    list: Option<Vec<String>>,
}

impl AuthorsInput {
    pub fn all() -> Self {
        Self {
            use_all: true,
            list: None,
        }
    }

    // Get the list of authors to apply the query to
    pub fn get_authors(&self, context: &Context) -> Vec<AuthorId> {
        let db = context.get();
        if self.use_all {
            return db.authors().values().cloned().collect();
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
