use interner::{impl_arena_id, RawId};
use std::sync::Arc;

#[derive(Debug, Hash, Eq, Copy, PartialEq, Clone)]
pub struct SourceId(RawId);
impl_arena_id!(SourceId);

#[salsa::query_group(SourceQueryGroup)]
pub trait SourceDatabase {
    #[salsa::input]
    fn source_text(&self, source_id: SourceId) -> Arc<String>;

    fn get_line(&self, source_id: SourceId, line: usize) -> Option<Arc<String>>;
}

fn get_line(db: &impl SourceDatabase, source_id: SourceId, line: usize) -> Option<Arc<String>> {
    let text = db.source_text(source_id);
    text.lines().nth(line).map(|l| Arc::new(l.to_string()))
}
