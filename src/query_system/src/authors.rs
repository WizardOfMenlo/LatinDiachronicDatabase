use crate::sources::SourceId;
use salsa::InternId;
use std::sync::Arc;

#[derive(Debug, Hash, Eq, Copy, PartialEq, Clone)]
pub struct AuthorId(InternId);

impl salsa::InternKey for AuthorId {
    fn from_intern_id(v: InternId) -> Self {
        AuthorId(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
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

#[salsa::query_group(AuthorsQueryGroup)]
pub trait AuthorsDatabase {
    #[salsa::interned]
    fn intern_author(&self, auth: Author) -> AuthorId;

    #[salsa::input]
    fn associated_sources(&self, author_id: AuthorId) -> Arc<Vec<SourceId>>;
}
