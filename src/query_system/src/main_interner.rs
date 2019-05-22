use crate::sources::SourceId;
use interner::Interner;
use std::path::PathBuf;

pub struct MainInterner {
    pub source_interner: Interner<SourceId, PathBuf>,
}

impl MainInterner {
    pub fn new(source_interner: Interner<SourceId, PathBuf>) -> Self {
        Self { source_interner }
    }
}
