use crate::ids::{AuthorId, SourceId};
use std::collections::HashSet;
use std::sync::Arc;

#[salsa::query_group(AuthorsQueryGroup)]
pub trait AuthorsDatabase {
    #[salsa::interned]
    fn intern_author(&self, auth: Author) -> AuthorId;

    #[salsa::input]
    fn associated_sources(&self, author_id: AuthorId) -> Arc<Vec<SourceId>>;

    fn union_author_sources(&self, authors: Vec<AuthorId>) -> Arc<HashSet<SourceId>>;
}

fn union_author_sources(
    db: &impl AuthorsDatabase,
    authors: Vec<AuthorId>,
) -> Arc<HashSet<SourceId>> {
    let mut hs = HashSet::new();
    for auth in authors {
        hs.extend(db.associated_sources(auth).iter());
    }
    Arc::new(hs)
}

// TODO, might refactor in its own crate, depending on the time involved (chrono)
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Author {
    name: String,
}

impl Author {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
