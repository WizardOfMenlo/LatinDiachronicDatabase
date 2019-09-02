use query_driver::driver_init;
use query_system::ids::*;
use query_system::lit_subset::LitSubset;
use query_system::traits::*;
use runner::load_configuration;

use std::fs::File;

use std::collections::{HashMap, HashSet};
use std::io::{self, prelude::*};

#[derive(Debug, Clone, Copy)]
enum SortingMode {
    Alphabetical,
    ByFrequency,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum FormMode {
    IncludeForms,
    HideForms,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum ReferenceMode {
    AlphaLocation,
    FreqLocation,
    Identity,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum AuthorMode {
    Full(AuthorConfig),
    Nothing,
}

#[derive(Debug, Clone, Copy)]
struct AuthorConfig {
    include_header: bool,
    include_authors: (bool, usize),
    include_centuries: (bool, CenturySettings, usize),
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum CenturySettings {
    IncludeAuthors(usize),
    Nothing,
}

#[derive(Debug, Clone, Copy)]
struct Configuration {
    sorting_mode: SortingMode,
    ref_mode: ReferenceMode,
    author_mode: AuthorMode,
    form_mode: FormMode,
}

const AUTHOR_SCALE_FACTOR: usize = 1_000;
const HISTORIC_SCALE_FACTOR: usize = 1_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    color_backtrace::install();
    env_logger::init();
    let db = driver_init(load_configuration())?;
    let lit = LitSubset::from_authors(db.authors().right_values(), &db.snapshot());
    let alpha = Dictionary::new(
        &db,
        lit.clone(),
        Configuration {
            sorting_mode: SortingMode::Alphabetical,
            ref_mode: ReferenceMode::FreqLocation,
            author_mode: AuthorMode::Full(AuthorConfig {
                include_header: true,
                include_authors: (false, AUTHOR_SCALE_FACTOR),
                include_centuries: (
                    true,
                    CenturySettings::IncludeAuthors(AUTHOR_SCALE_FACTOR),
                    HISTORIC_SCALE_FACTOR,
                ),
            }),
            form_mode: FormMode::HideForms,
        },
    );

    let freq = Dictionary::new(
        &db,
        lit.clone(),
        Configuration {
            sorting_mode: SortingMode::ByFrequency,
            ref_mode: ReferenceMode::Identity,
            author_mode: AuthorMode::Nothing,
            form_mode: FormMode::IncludeForms,
        },
    );

    let author_count = author_count(&db, lit);

    alpha.write(&db, &mut File::create("alpha.txt")?, &author_count)?;
    freq.write(&db, &mut File::create("freq.txt")?, &author_count)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Entry {
    lemma: LemmaId,
    count: usize,
    ambig_count: usize,
    corresponding_index: usize,
    forms: Vec<(FormId, Vec<FormDataId>)>,
    authors: HashSet<AuthorId>,
}

impl Entry {
    fn write(
        &self,
        w: &mut impl Write,
        db: &impl MainDatabase,
        config: Configuration,
        global_authors_count: &HashMap<AuthorId, usize>,
    ) -> io::Result<()> {
        let resolved_lemma = db.lookup_intern_lemma(self.lemma);
        writeln!(
            w,
            "-------{:6}------{} count: {} (C: {}, A: {})",
            self.corresponding_index,
            resolved_lemma.0.inner().to_uppercase(),
            self.count,
            self.count - self.ambig_count,
            self.ambig_count
        )?;

        if let FormMode::IncludeForms = config.form_mode {
            for (form, count) in self.forms.iter().map(|(k, v)| (k, v.len())) {
                let resolved_form = db.lookup_intern_form(*form);
                writeln!(
                    w,
                    "\t{}: {} {}",
                    resolved_form.0.inner(),
                    count,
                    if db.lemmatizer().is_ambig(&resolved_form.0) {
                        "(*)"
                    } else {
                        ""
                    }
                )?;
            }
        }

        let mut authors: Vec<_> = self
            .authors
            .iter()
            .map(|e| (e, db.lookup_intern_author(*e)))
            .collect();
        authors.sort_by(|(_, a), (_, b)| a.name().cmp(b.name()));

        // How many times was it used by an author
        let mut authors_count = HashMap::new();
        for &fd in self.forms.iter().map(|(_, fds)| fds).flatten() {
            let auth_id = db.lookup_intern_form_data(fd).author(db);
            *authors_count
                .entry(db.lookup_intern_author(auth_id))
                .or_insert(0usize) += 1;
        }

        match config.author_mode {
            AuthorMode::Nothing => (),
            AuthorMode::Full(config) => {
                if config.include_header {
                    writeln!(w, "\t\tUsed by {} authors", authors.len())?;
                }

                if config.include_authors.0 {
                    let scale = config.include_authors.1;
                    write!(w, "\t\t")?;
                    for (id, author) in &authors {
                        let relative_freq = (authors_count.get(author).unwrap() * scale) as f64
                            / *global_authors_count.get(&id).unwrap() as f64;
                        write!(w, "{} ({:.2}) ", author.name(), relative_freq)?;
                    }
                    writeln!(w)?;
                }

                if config.include_centuries.0 {
                    let scale = config.include_centuries.2;

                    let buckets =
                        authors_chrono::split_by_century(authors.iter().map(|(_, a)| a).cloned());
                    for (cent, mut authors_b) in buckets.into_iter() {
                        let aggregated = authors_b
                            .iter()
                            .flat_map(|&a| authors_count.get(&a))
                            .sum::<usize>();
                        let relative_freq = (aggregated * scale) as f64 / self.count as f64;

                        write!(
                            w,
                            "\t\t\t{} {}: {} ({:.2}) ",
                            cent.abs(),
                            if cent > 0 { "CE" } else { "BCE" },
                            authors_b.len(),
                            relative_freq
                        )?;

                        match config.include_centuries.1 {
                            CenturySettings::IncludeAuthors(scale) => {
                                authors_b.sort_by(|a, b| a.name().cmp(b.name()));
                                for author in authors_b {
                                    let count = authors_count.get(author).unwrap();
                                    let id = authors
                                        .iter()
                                        .find(|(_, a)| a == &author)
                                        .map(|(id, _)| id)
                                        .unwrap();
                                    let global_count = *global_authors_count.get(id).unwrap();
                                    let relative_freq =
                                        (count * scale) as f64 / global_count as f64;
                                    write!(
                                        w,
                                        "{} {} ({:.2}) ",
                                        author.name(),
                                        count,
                                        relative_freq
                                    )?;
                                }
                            }
                            CenturySettings::Nothing => (),
                        }
                        writeln!(w)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Dictionary {
    ls: Vec<Entry>,
    config: Configuration,
}

// TODO: Move to DB
fn author_count(db: &impl MainDatabase, sub: LitSubset) -> HashMap<AuthorId, usize> {
    let tree = db.subset_tree(sub);
    let mut res = HashMap::new();
    for author in tree
        .iter()
        .flat_map(|(_, forms)| forms.values().flatten())
        .map(|fd_id| db.lookup_intern_form_data(*fd_id).author(db))
    {
        *res.entry(author).or_insert(0) += 1;
    }
    res
}

impl Dictionary {
    fn new(db: &impl MainDatabase, sub: LitSubset, config: Configuration) -> Self {
        let tree = db.subset_tree(sub);
        let mut ls = Vec::with_capacity(tree.len());
        for (&lemma, forms) in tree.iter() {
            let count = forms.values().map(|v| v.len()).sum();
            let ambig_count = forms
                .iter()
                .filter(|(&k, _)| db.lemmatizer().is_ambig(&db.lookup_intern_form(k).0))
                .map(|(_, v)| v.len())
                .sum();

            ls.push(Entry {
                lemma,
                count,
                ambig_count,
                forms: forms.iter().map(|(a, b)| (*a, b.clone())).collect(),
                // TODO, this is a bit inefficient, as many double lookups
                authors: forms
                    .values()
                    .flatten()
                    .map(|f| db.lookup_intern_form_data(*f).author(db))
                    .collect(),

                // Will be set later on
                corresponding_index: 0,
            })
        }

        let mut res = Dictionary {
            ls: ls.clone(),
            config,
        };
        let mut aux = Dictionary { ls, config };

        match config.sorting_mode {
            SortingMode::Alphabetical => res.sort_alpha(db),
            SortingMode::ByFrequency => res.sort_freq(db),
        };

        match (config.ref_mode, config.sorting_mode) {
            (ReferenceMode::AlphaLocation, _) => aux.sort_alpha(db),
            (ReferenceMode::FreqLocation, _) => aux.sort_freq(db),
            (ReferenceMode::Identity, SortingMode::Alphabetical) => aux.sort_alpha(db),
            (ReferenceMode::Identity, SortingMode::ByFrequency) => aux.sort_freq(db),
        };

        // Set the index accordingly
        for entry in &mut res.ls {
            /*
            entry.corresponding_index = match (config.ref_mode, config.sorting_mode) {
                (ReferenceMode::AlphaLocation, _)
                | (ReferenceMode::Identity, SortingMode::Alphabetical) => {
                    let entry_lemma = db.lookup_intern_lemma(entry.lemma);
                    aux.ls.binary_search_by(|probe| {
                        let probe_lemma = db.lookup_intern_lemma(probe.lemma);
                        probe_lemma.cmp(&entry_lemma)
                    })
                }
                (ReferenceMode::FreqLocation, _)
                | (ReferenceMode::Identity, SortingMode::ByFrequency) => aux
                    .ls
                    .binary_search_by(|probe| probe.count.cmp(&entry.count)),
            }
            .unwrap();
            */

            // This is simpler, but the above is probably more efficient
            entry.corresponding_index = aux.ls.iter().position(|l| l.lemma == entry.lemma).unwrap();
        }

        res
    }

    fn sort_alpha(&mut self, db: &impl MainDatabase) {
        self.ls.sort_by(|a, b| {
            let lemm_a = db.lookup_intern_lemma(a.lemma);
            let lemm_b = db.lookup_intern_lemma(b.lemma);

            lemm_a.0.cmp(&lemm_b.0)
        });

        for entry in &mut self.ls {
            entry.forms.sort_by(|(a, _), (b, _)| {
                let form_a = db.lookup_intern_form(*a);
                let form_b = db.lookup_intern_form(*b);
                form_a.0.cmp(&form_b.0)
            });
        }
    }

    fn sort_freq(&mut self, _: &impl MainDatabase) {
        // Note b, a instead of a, b to reverse ordering
        self.ls.sort_by(|b, a| a.count.cmp(&b.count));
        for entry in &mut self.ls {
            entry.forms.sort_by(|(_, b), (_, a)| a.len().cmp(&b.len()));
        }
    }

    fn write(
        &self,
        db: &impl MainDatabase,
        w: &mut impl Write,
        author_count: &HashMap<AuthorId, usize>,
    ) -> io::Result<()> {
        for entry in &self.ls {
            entry.write(w, db, self.config, author_count)?;
        }
        Ok(())
    }
}
