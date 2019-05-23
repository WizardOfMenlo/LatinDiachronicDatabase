use salsa::InternId;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Hash, Eq, Copy, PartialEq, Clone)]
pub struct SourceId(InternId);

impl salsa::InternKey for SourceId {
    fn from_intern_id(v: InternId) -> Self {
        SourceId(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[salsa::query_group(SourcesQueryGroup)]
pub trait SourcesDatabase {
    #[salsa::interned]
    fn intern_source(&self, p: PathBuf) -> SourceId;

    #[salsa::input]
    fn source_text(&self, source_id: SourceId) -> Arc<String>;

    fn get_line(&self, source_id: SourceId, line: usize) -> Option<Arc<String>>;
}

fn get_line(db: &impl SourcesDatabase, source_id: SourceId, line: usize) -> Option<Arc<String>> {
    let text = db.source_text(source_id);
    text.lines().nth(line).map(|l| Arc::new(l.to_string()))
}
