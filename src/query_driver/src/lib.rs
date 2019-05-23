use query_system::authors::{Author, AuthorId};
use query_system::authors::{AuthorsDatabase, AuthorsQueryGroup};
use query_system::form_data::FormDataQueryGroup;
use query_system::forms::FormsQueryGroup;
use query_system::sources::{SourcesDatabase, SourcesQueryGroup};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

#[salsa::database(
    SourcesQueryGroup,
    AuthorsQueryGroup,
    FormDataQueryGroup,
    FormsQueryGroup
)]
#[derive(Default, Debug)]
pub struct MainDatabase {
    // TODO, just for testing
    pub authors: Vec<AuthorId>,
    runtime: salsa::Runtime<MainDatabase>,
}

impl salsa::Database for MainDatabase {
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
    let mut db = MainDatabase::default();
    let mut current_author_id = None;
    let mut author_associations = HashMap::new();

    for entry in WalkDir::new(data_dir).max_depth(2) {
        let entry = entry?;
        let ft = entry.file_type();
        // Branch: Add a new author
        if ft.is_dir() {
            // We create authors from file mapping
            let file_name = entry.file_name().to_string_lossy().into_owned();
            current_author_id = Some(db.intern_author(Author::new(file_name)));
            db.set_associated_sources(current_author_id.unwrap(), Arc::new(vec![]));
        }
        // Branch, load into db (skip if no author appeared first)
        else if ft.is_file() && current_author_id.is_some() {
            let path = entry.path();
            let new_source_id = db.intern_source(path.to_path_buf());
            // Add the source to the author
            author_associations
                .entry(current_author_id.unwrap())
                .or_insert_with(Vec::new)
                .push(new_source_id);

            db.set_source_text(new_source_id, Arc::new(load_to_string(path)?));
        }
    }

    // Should be copied
    db.authors = author_associations.keys().cloned().collect();

    for (auth_id, sources) in author_associations {
        db.set_associated_sources(auth_id, Arc::new(sources));
    }

    Ok(db)
}
