use interner::SequentialInternerBuilder;
use query_system::authors::Author;
use query_system::authors::{AuthorsDatabase, AuthorsQueryGroup};
use query_system::main_interner::MainInterner;
use query_system::sources::{SourcesDatabase, SourcesQueryGroup};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct MainDatabase {
    inner: InnerDatabase,
    interner: MainInterner,
}

impl MainDatabase {
    fn new(inner: InnerDatabase, interner: MainInterner) -> Self {
        Self { inner, interner }
    }
}

#[salsa::database(SourcesQueryGroup, AuthorsQueryGroup)]
#[derive(Default, Debug)]
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
pub fn driver_init(data_dir: impl AsRef<Path>) -> Result<MainDatabase, Box<Error>> {
    let mut db = InnerDatabase::default();
    let mut source_interner = SequentialInternerBuilder::new();
    let mut author_interner = SequentialInternerBuilder::new();
    let mut current_author_id = None;

    for entry in WalkDir::new(data_dir).max_depth(1) {
        let entry = entry?;
        let ft = entry.file_type();
        // Branch: Add a new author
        if ft.is_dir() {
            // We create authors from file mapping
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let new_author_id = author_interner.next_id();
            current_author_id = Some(new_author_id);
            author_interner = author_interner.add_mapping(Author::new(file_name));
            db.set_associated_sources(new_author_id, Arc::new(vec![]));
        }
        // Branch, load into db (skip if no author appeared first)
        else if ft.is_file() && current_author_id.is_some() {
            let path = entry.path();
            let new_source_id = source_interner.next_id();
            source_interner = source_interner.add_mapping(path.to_owned());

            // TODO, could be better with an hashmap? Less allocations for sure
            let old_sources = db.associated_sources(current_author_id.unwrap());
            let mut new_sources = Vec::new();
            new_sources.extend(old_sources.iter().cloned());
            new_sources.push(new_source_id);

            db.set_source_text(new_source_id, Arc::new(load_to_string(path)?));
            db.set_associated_sources(current_author_id.unwrap(), Arc::new(new_sources))
        }
    }

    let source_interner = source_interner.build();
    let author_interner = author_interner.build();
    Ok(MainDatabase::new(
        db,
        MainInterner::new(source_interner, author_interner),
    ))
}
