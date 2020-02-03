use crate::authors_chrono::Author;
use crate::filesystem::{FileSystem, GetFileSystem, InternerFileSystem};
use crate::latin_lemmatizer::compressed::CompressedLemmatizer;
use crate::latin_lemmatizer::NaiveLemmatizer;
use crate::latin_utilities::NormalizedLatinString;
use crate::query_system::ids::*;
use crate::query_system::middle::IntermediateDatabase;
use crate::query_system::middle::IntermediateQueries;
use crate::query_system::sources::SourcesDatabase;
use crate::query_system::sources::SourcesQueryGroup;
use crate::query_system::traits::AuthorInternDatabase;
use crate::query_system::types::InternersGroup;
use crate::query_system::MainQueries;
use crate::word_db::{WordDatabase, WordDb};

use bimap::BiMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use walkdir::WalkDir;

pub mod memory;

#[salsa::database(MainQueries, SourcesQueryGroup, InternersGroup, IntermediateQueries)]
#[derive(Default, Debug)]
pub struct MainDatabase {
    runtime: salsa::Runtime<MainDatabase>,
    authors: BiMap<Author, AuthorId>,
    fs: InternerFileSystem,
    word_db: WordDb,
}

impl MainDatabase {
    fn new() -> Self {
        Self {
            runtime: Default::default(),
            authors: BiMap::new(),
            fs: InternerFileSystem::new(),
            word_db: WordDb::default(),
        }
    }

    pub fn authors(&self) -> &BiMap<Author, AuthorId> {
        &self.authors
    }

    pub fn sources(&self) -> &BiMap<PathBuf, SourceId> {
        &self.fs.sources()
    }
}

impl AuthorInternDatabase for MainDatabase {
    fn intern_author(&mut self, auth: Author) -> AuthorId {
        if self.authors.contains_left(&auth) {
            return *self.authors.get_by_left(&auth).unwrap();
        }

        // Guarantee no ovewrite, while having a sensible default
        let mut new_id = self.authors.len() as u32;
        while self.authors.contains_right(&AuthorId::from_integer(new_id)) {
            new_id += 1;
        }

        let id = AuthorId::from_integer(new_id);

        self.authors.insert(auth, id);

        id
    }

    fn lookup_intern_author(&self, id: AuthorId) -> &Author {
        // If invalid id, we panic
        self.authors().get_by_right(&id).unwrap()
    }
}

impl GetFileSystem for MainDatabase {
    type Fs = InternerFileSystem;

    fn filesystem(&self) -> &Self::Fs {
        &self.fs
    }

    fn filesystem_mut(&mut self) -> &mut Self::Fs {
        &mut self.fs
    }
}

impl WordDatabase for MainDatabase {
    fn intern_word(&self, s: NormalizedLatinString) -> WordId {
        self.word_db.intern_word(s)
    }

    fn lookup_word(&self, id: WordId) -> NormalizedLatinString {
        self.word_db.lookup_word(id)
    }

    fn lookup_interned_word(&self, s: NormalizedLatinString) -> Option<WordId> {
        self.word_db.lookup_interned_word(s)
    }
}

impl salsa::Database for MainDatabase {
    fn salsa_runtime(&self) -> &salsa::Runtime<Self> {
        &self.runtime
    }

    fn salsa_runtime_mut(&mut self) -> &mut salsa::Runtime<Self> {
        &mut self.runtime
    }
}

impl salsa::ParallelDatabase for MainDatabase {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(MainDatabase {
            runtime: self.runtime.snapshot(self),
            authors: self.authors.clone(),
            fs: self.fs.clone(),
            word_db: self.word_db.clone(),
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
    authors_path: Option<PathBuf>,
    lemm_mode: LemmMode,
}

impl Configuration {
    pub fn new(
        data_dir: impl Into<PathBuf>,
        lemmatizer_path: impl Into<PathBuf>,
        authors_path: Option<impl Into<PathBuf>>,
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

        let authors_path = authors_path.map(Into::into);

        if authors_path.is_some() && !authors_path.as_ref().unwrap().exists() {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }

        Ok(Configuration {
            data_dir,
            lemmatizer_path,
            authors_path,
            lemm_mode,
        })
    }

    pub(crate) fn make_lemm(&self) -> Result<NaiveLemmatizer, Box<dyn Error>> {
        Ok(match self.lemm_mode {
            LemmMode::CSVFormat => crate::latin_lemmatizer::parsers::csv_format::new()
                .read_all(File::open(&self.lemmatizer_path)?)?
                .build(),
            LemmMode::LemlatFormat => crate::latin_lemmatizer::parsers::lemlat_format::new()
                .read_all(File::open(&self.lemmatizer_path)?)?
                .build(),
        })
    }
}

pub fn driver_init(config: Configuration) -> Result<MainDatabase, Box<dyn Error>> {
    let mut current_author_id = None;
    let mut author_associations = HashMap::new();

    let mut db = MainDatabase::new();
    memory::set_lru_sizes(&mut db);

    // First, load lemmatizer
    let lemm = config.make_lemm()?;
    let compressed = CompressedLemmatizer::new(lemm, &db);

    for entry in WalkDir::new(config.data_dir).max_depth(2) {
        let entry = entry?;
        let ft = entry.file_type();
        // Branch: Add a new author (maybe check for non folder on 2nd level?)
        if ft.is_dir() {
            // We create authors from file mapping
            let file_name = entry.file_name().to_string_lossy().into_owned();

            let new_id = db.intern_author(Author::new(file_name));
            current_author_id = Some(new_id);
        }
        // Branch, load into db (skip if no author appeared first)
        else if ft.is_file() && current_author_id.is_some() {
            let path = entry.path();

            let new_id = db.intern_source(path.to_path_buf());
            // Add the source to the author
            author_associations
                .entry(current_author_id.unwrap())
                .or_insert_with(HashSet::new)
                .insert(new_id);
        }
    }

    // Ensure no childless authors arise
    db.authors = db
        .authors
        .into_iter()
        .filter(|(_, v)| author_associations.contains_key(v) && !author_associations[v].is_empty())
        .collect();

    // Update, so that we can get the authors with metadata
    if let Some(authors_path) = config.authors_path {
        let mut authors_hist = crate::authors_chrono::parsers::WeirdParser::default();
        let authors_file = File::open(authors_path)?;
        authors_hist.read_all(authors_file)?;
        let authors_list = authors_hist.build();
        db.authors = db
            .authors
            .into_iter()
            // Skip all authors we didn't find
            .flat_map(|(a, k)| {
                if let Some(author) = authors_list.get(&a).cloned() {
                    Some((author, k))
                } else {
                    None
                }
            })
            .collect();
    }

    // Load the authors assoc
    author_associations.into_iter().for_each(|(k, v)| {
        db.set_associated_sources(k, Arc::new(v.clone()));
        v.iter().for_each(|&s| db.set_associated_author(s, k))
    });

    db.set_lemmatizer(Arc::new(compressed));

    Ok(db)
}
