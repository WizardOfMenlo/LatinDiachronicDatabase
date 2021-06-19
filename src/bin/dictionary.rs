use latin_db::query_driver::driver_init;
use latin_db::query_system::ids::*;
use latin_db::query_system::lit_subset::LitSubset;
use latin_db::query_system::traits::*;
use latin_db::query_system::types::{Form, Lemma};
use latin_db::{arguments::load_configuration, authors_chrono::Author};

use std::collections::{HashMap, HashSet};
use std::fs::File;
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
    OnlyAmbig,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
enum LemmaMode {
    Full,
    OnlyAmbig,
    Lean,
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
    spotlight: Option<AuthorId>,
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
    lemma_mode: LemmaMode,
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
    let epigraph_id = *db.authors().get_by_left(&Author::new("Epigraphs")).unwrap();
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
                spotlight: Some(epigraph_id),
            }),
            form_mode: FormMode::HideForms,
            lemma_mode: LemmaMode::Full,
        },
    );

    let alpha_only_ambig = Dictionary::new(
        &db,
        lit.clone(),
        Configuration {
            sorting_mode: SortingMode::Alphabetical,
            ref_mode: ReferenceMode::Identity,
            author_mode: AuthorMode::Nothing,
            form_mode: FormMode::OnlyAmbig,
            lemma_mode: LemmaMode::OnlyAmbig,
        },
    );

    let freq_with_forms = Dictionary::new(
        &db,
        lit.clone(),
        Configuration {
            sorting_mode: SortingMode::ByFrequency,
            ref_mode: ReferenceMode::Identity,
            author_mode: AuthorMode::Nothing,
            form_mode: FormMode::IncludeForms,
            lemma_mode: LemmaMode::Full,
        },
    );

    let freq_without_forms = Dictionary::new(
        &db,
        lit.clone(),
        Configuration {
            sorting_mode: SortingMode::ByFrequency,
            ref_mode: ReferenceMode::Identity,
            author_mode: AuthorMode::Nothing,
            form_mode: FormMode::HideForms,
            lemma_mode: LemmaMode::Full,
        },
    );

    let author_count = db.authors_count(lit);

    alpha.write(&db, &mut File::create("alpha.txt")?, &author_count)?;
    alpha_only_ambig.write(&db, &mut File::create("alpha_ambig.txt")?, &author_count)?;
    freq_with_forms.write(&db, &mut File::create("freq_forms.txt")?, &author_count)?;
    freq_without_forms.write(&db, &mut File::create("freq_no_forms.txt")?, &author_count)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Entry {
    lemma: Lemma,
    count: usize,
    ambig_count: usize,
    corresponding_index: usize,
    forms: Vec<(Form, Vec<FormDataId>)>,
    authors: HashSet<AuthorId>,
}

fn id_to_str(db: &impl MainDatabase, id: WordId) -> String {
    db.lookup_word(id).inner().to_string()
}

impl Entry {
    fn write(
        &self,
        w: &mut impl Write,
        db: &impl MainDatabase,
        config: Configuration,
        global_authors_count: &HashMap<AuthorId, usize>,
    ) -> io::Result<()> {
        match config.lemma_mode {
            LemmaMode::Full => {
                writeln!(
            w,
            "-{}: {} total occurrence{} [certain: {}, ambiguous: {}, frequential_order: {}]",
            id_to_str(db, self.lemma.0).to_uppercase(),
            self.count,
            if self.count == 1 { "" } else { "s" },
            self.count - self.ambig_count,
            self.ambig_count,
            self.corresponding_index
        )?;
            }
            // Note sure if this binds thightly or not
            LemmaMode::Lean | LemmaMode::OnlyAmbig
                if db.lemmatizer().is_ambig_lemma(self.lemma.0) =>
            {
                writeln!(w, "-{}", id_to_str(db, self.lemma.0).to_uppercase())?;
            }
            _ => {}
        }

        match config.form_mode {
            FormMode::HideForms => {}
            FormMode::IncludeForms => {
                let forms: Vec<_> = self
                    .forms
                    .iter()
                    .map(|(k, v)| (k, v.len()))
                    .map(|(f, count)| {
                        format!(
                            "{}: {} {}",
                            id_to_str(db, f.0),
                            count,
                            if db.lemmatizer().is_ambig(f.0) {
                                "(*)"
                            } else {
                                ""
                            }
                        )
                    })
                    .collect();

                writeln!(w, "\t {} @", forms.join(", "))?;
            }
            FormMode::OnlyAmbig => {
                let forms: Vec<_> = self
                    .forms
                    .iter()
                    .map(|(k, v)| (k, v.len()))
                    .filter(|(k, _)| db.lemmatizer().is_ambig(k.0))
                    .map(|(f, _)| {
                        let mut ambig: Vec<_> = db
                            .lemmatizer()
                            .get_possible_lemmas(f.0)
                            .unwrap()
                            .iter()
                            .map(|f| id_to_str(db, *f))
                            .collect();
                        ambig.sort();
                        format!("{} ({})", id_to_str(db, f.0), ambig.join(", "))
                    })
                    .collect();

                if forms.len() > 0 {
                    writeln!(w, "\t {} @", forms.join(", "),)?;
                }
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
                    if let Some(spot_id) = config.spotlight {
                        let spot = db.lookup_intern_author(spot_id);

                        let spot_count = authors_count.get(spot).copied().unwrap_or_default();
                        writeln!(
                            w,
                            "\t\tAttested in {} author{}, {} occ. in {} $",
                            authors.len(),
                            if authors.len() == 1 { "" } else { "s" },
                            spot_count,
                            spot.name().to_lowercase()
                        )?;
                    } else {
                        writeln!(
                            w,
                            "\t\tAttested in {} author{}",
                            authors.len(),
                            if authors.len() == 1 { "" } else { "s" }
                        )?;
                    }
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

                    let buckets = latin_db::authors_chrono::split_by_century(
                        authors.iter().map(|(_, a)| a).cloned(),
                    );
                    for (cent, mut authors_b) in buckets.into_iter() {
                        let aggregated = authors_b
                            .iter()
                            .flat_map(|&a| authors_count.get(&a))
                            .sum::<usize>();
                        let relative_freq = (aggregated * scale) as f64 / self.count as f64;

                        write!(
                            w,
                            "\t\t\t•{} {}: {} author{} ({:.2}), ",
                            cent.abs(),
                            if cent > 0 { "CE" } else { "BCE" },
                            authors_b.len(),
                            if authors_b.len() == 1 { "" } else { "s" },
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
                        writeln!(w, "!")?;
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

impl Dictionary {
    fn new(db: &impl MainDatabase, sub: LitSubset, config: Configuration) -> Self {
        let tree = db.subset_tree(sub);
        let mut ls = Vec::with_capacity(tree.len());
        for (&lemma, forms) in tree.iter() {
            let count = forms.values().map(|v| v.len()).sum();
            let ambig_count = forms
                .iter()
                .filter(|(&k, _)| db.lemmatizer().is_ambig(k.0))
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
            let lemm_a = id_to_str(db, a.lemma.0);
            let lemm_b = id_to_str(db, b.lemma.0);

            lemm_a.cmp(&lemm_b)
        });

        for entry in &mut self.ls {
            entry.forms.sort_by(|(a, _), (b, _)| {
                let form_a = id_to_str(db, a.0);
                let form_b = id_to_str(db, b.0);
                form_a.cmp(&form_b)
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
