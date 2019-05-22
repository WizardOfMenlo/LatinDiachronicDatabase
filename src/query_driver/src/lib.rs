use interner::SequentialInternerBuilder;
use query_system::main_interner::MainInterner;
use query_system::sources::{SourceDatabase, SourceQueryGroup};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

pub struct MainDatabase {
    inner: InnerDatabase,
    interner: MainInterner,
}

impl MainDatabase {
    fn new(inner: InnerDatabase, interner: MainInterner) -> Self {
        Self { inner, interner }
    }
}

#[salsa::database(SourceQueryGroup)]
#[derive(Default)]
struct InnerDatabase {
    runtime: salsa::Runtime<InnerDatabase>,
}

impl salsa::Database for InnerDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }
}

// Helper to load a file to string
fn load_to_string(p: &Path) -> std::io::Result<String> {
    let mut f = File::open(p)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

// TODO, make async
pub fn driver_init(data_dir: &Path) -> Result<MainDatabase, Box<Error>> {
    let mut db = InnerDatabase::default();
    let mut source_interner = SequentialInternerBuilder::new();

    for entry in WalkDir::new(data_dir).max_depth(1) {
        let entry = entry?;
        let ft = entry.file_type();
        let path = entry.path();
        // Branch: Add a new author
        if ft.is_dir() {

        }
        // Branch, load into db
        else if ft.is_file() {
            let new_source_id = source_interner.next_id();
            source_interner = source_interner.add_mapping(path.to_owned());

            db.set_source_text(new_source_id, Arc::new(load_to_string(path)?));
        }
    }

    let source_interner = source_interner.build();
    Ok(MainDatabase::new(db, MainInterner::new(source_interner)))
}
