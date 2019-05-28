use latin_lemmatizer::NaiveLemmatizer;
use query_system::ids::{AuthorId, SourceId};
use query_system::middle::IntermediateQueries;
use query_system::sources::{SourcesDatabase, SourcesQueryGroup};
use query_system::types::{Author, InternersGroup};
use query_system::MainQueries;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use walkdir::WalkDir;

#[salsa::database(MainQueries, SourcesQueryGroup, InternersGroup, IntermediateQueries)]
#[derive(Default, Debug)]
pub struct MainDatabase {
    runtime: salsa::Runtime<MainDatabase>,
    // TODO, bidirectionaize this? Use the old interner impl
    sources: HashMap<PathBuf, SourceId>,
    authors: HashMap<Author, AuthorId>,
    lemmatizer: NaiveLemmatizer,
}

impl MainDatabase {
    fn new(lemmatizer: NaiveLemmatizer) -> Self {
        Self {
            runtime: Default::default(),
            sources: HashMap::new(),
            authors: HashMap::new(),
            lemmatizer,
        }
    }

    pub fn authors(&self) -> &HashMap<Author, AuthorId> {
        &self.authors
    }

    pub fn sources(&self) -> &HashMap<PathBuf, SourceId> {
        &self.sources
    }
}

impl AsRef<NaiveLemmatizer> for MainDatabase {
    fn as_ref(&self) -> &NaiveLemmatizer {
        &self.lemmatizer
    }
}

impl salsa::Database for MainDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }
}

impl salsa::ParallelDatabase for MainDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(MainDatabase {
            runtime: self.runtime.snapshot(self),
            sources: self.sources.clone(),
            authors: self.authors.clone(),
            lemmatizer: self.lemmatizer.clone(),
        })
    }
}

pub enum LemmMode {
    CSVFormat,
    LemlatFormat,
}

pub struct Configuration {
    data_dir: PathBuf,
    lemmatizer_path: PathBuf,
    lemm_mode: LemmMode,
}

impl Configuration {
    pub fn new(
        data_dir: impl Into<PathBuf>,
        lemmatizer_path: impl Into<PathBuf>,
        lemm_mode: LemmMode,
    ) -> io::Result<Self> {
        let data_dir = data_dir.into();
        let lemmatizer_path = lemmatizer_path.into();

        if !lemmatizer_path.exists() {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }

        if !(data_dir.exists() && data_dir.is_dir()) {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }

        Ok(Configuration {
            data_dir,
            lemmatizer_path,
            lemm_mode,
        })
    }

    pub(crate) fn make_lemm(&self) -> Result<NaiveLemmatizer, Box<Error>> {
        Ok(match self.lemm_mode {
            LemmMode::CSVFormat => latin_lemmatizer::parsers::csv_format::new()
                .read_all(File::open(&self.lemmatizer_path)?)?
                .build(),
            LemmMode::LemlatFormat => latin_lemmatizer::parsers::lemlat_format::new()
                .read_all(File::open(&self.lemmatizer_path)?)?
                .build(),
        })
    }
}

// Helper to load a file to string
fn load_to_string(p: &Path) -> io::Result<String> {
    let mut f = File::open(p)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

// TODO, make async
pub fn driver_init(config: Configuration) -> Result<MainDatabase, Box<Error>> {
    let mut db = MainDatabase::new(config.make_lemm()?);
    let mut current_author_id = None;
    let mut author_associations = HashMap::new();
    let mut author_counter = 0;
    let mut source_counter = 0;

    for entry in WalkDir::new(config.data_dir).max_depth(2) {
        let entry = entry?;
        let ft = entry.file_type();
        // Branch: Add a new author (maybe check for non folder on 2nd level?)
        if ft.is_dir() {
            // We create authors from file mapping
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let new_id = AuthorId::from_integer(author_counter);

            current_author_id = Some(new_id);

            db.authors.insert(Author::new(file_name), new_id);

            author_counter += 1;
        }
        // Branch, load into db (skip if no author appeared first)
        else if ft.is_file() && current_author_id.is_some() {
            let path = entry.path();
            let new_id = SourceId::from_integer(source_counter);

            db.sources.insert(path.to_path_buf(), new_id);
            // Add the source to the author
            author_associations
                .entry(current_author_id.unwrap())
                .or_insert_with(Vec::new)
                .push(new_id);

            db.set_source_text(new_id, Arc::new(load_to_string(path)?));
            source_counter += 1;
        }
    }

    // Ensure no childless authors arise
    db.authors = db
        .authors
        .into_iter()
        .filter(|(_, v)| author_associations.contains_key(v) && !author_associations[v].is_empty())
        .collect();

    for (auth_id, sources) in author_associations {
        // TODO the conversion is pretty bad
        db.set_associated_sources(auth_id, Arc::new(sources.into_iter().collect()));
    }

    Ok(db)
}
