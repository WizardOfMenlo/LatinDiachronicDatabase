use authors_chrono::Author;
use bimap::BiMap;
use latin_lemmatizer::NaiveLemmatizer;
use query_system::ids::*;
use query_system::middle::IntermediateQueries;
use query_system::sources::SourcesQueryGroup;
use query_system::traits::AuthorInternDatabase;
use query_system::types::InternersGroup;
use query_system::MainQueries;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

pub mod utils;

#[salsa::database(MainQueries, SourcesQueryGroup, InternersGroup, IntermediateQueries)]
#[derive(Default, Debug)]
pub struct MainDatabase {
    runtime: salsa::Runtime<MainDatabase>,
    // TODO, bidirectionaize this? Use the old interner impl
    sources: BiMap<PathBuf, SourceId>,
    authors: BiMap<Author, AuthorId>,
}

impl MainDatabase {
    fn new() -> Self {
        Self {
            runtime: Default::default(),
            sources: BiMap::new(),
            authors: BiMap::new(),
        }
    }

    pub fn authors(&self) -> &BiMap<Author, AuthorId> {
        &self.authors
    }

    pub fn sources(&self) -> &BiMap<PathBuf, SourceId> {
        &self.sources
    }
}

impl AuthorInternDatabase for MainDatabase {
    fn intern_author(&mut self, auth: Author) -> AuthorId {
        let authors = self.authors();
        if authors.contains_left(&auth) {
            return *authors.get_by_left(&auth).unwrap();
        }

        // TODO, we could probably find a better selection
        let mut new_id = 0;
        while authors.contains_right(&AuthorId::from_integer(new_id)) {
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
            LemmMode::CSVFormat => latin_lemmatizer::parsers::csv_format::new()
                .read_all(File::open(&self.lemmatizer_path)?)?
                .build(),
            LemmMode::LemlatFormat => latin_lemmatizer::parsers::lemlat_format::new()
                .read_all(File::open(&self.lemmatizer_path)?)?
                .build(),
        })
    }
}

// TODO, make async
pub fn driver_init(config: Configuration) -> Result<MainDatabase, Box<dyn Error>> {
    let mut db = MainDatabase::new();
    let mut current_author_id = None;
    let mut author_associations = HashMap::new();
    let mut author_counter = 0;
    let mut source_counter = 0;

    // First, load lemmatizer
    let lemm = config.make_lemm()?;

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
                .or_insert_with(HashSet::new)
                .insert(new_id);

            source_counter += 1;
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
        let mut authors_hist = authors_chrono::parsers::WeirdParser::default();
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

    let aux_sources = db.sources.clone();

    utils::load_database(
        &mut db,
        author_associations,
        aux_sources.into_iter().map(|(k, v)| (v, k)),
        File::open,
        lemm,
    )?;

    Ok(db)
}
