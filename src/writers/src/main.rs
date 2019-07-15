use authors_chrono::Author;
use query_driver::driver_init;
use query_system::ids::*;
use query_system::lit_subset::LitSubset;
use query_system::traits::*;

use std::fs::File;

use runner::load_configuration;
use std::collections::HashSet;
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
    Full,
    NumberOnly,
    Nothing,
}

#[derive(Debug, Clone, Copy)]
struct Configuration {
    sorting_mode: SortingMode,
    ref_mode: ReferenceMode,
    author_mode: AuthorMode,
    form_mode: FormMode,
}

fn main() -> Result<(), Box<std::error::Error>> {
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
            author_mode: AuthorMode::Nothing,
            form_mode: FormMode::IncludeForms,
        },
    );

    let freq = Dictionary::new(
        &db,
        lit.clone(),
        Configuration {
            sorting_mode: SortingMode::ByFrequency,
            ref_mode: ReferenceMode::Identity,
            author_mode: AuthorMode::Full,
            form_mode: FormMode::IncludeForms,
        },
    );

    alpha.write(&db, &mut File::create("alpha.txt")?)?;
    freq.write(&db, &mut File::create("freq.txt")?)?;
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
    ) -> io::Result<()> {
        let resolved_lemma = db.lookup_intern_lemma(self.lemma);
        writeln!(
            w,
            "-------{:6}------{} count: {} (C: {}, A: {})",
            self.corresponding_index,
            resolved_lemma.0.inner(),
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

        let mut authors: Vec<&Author> = self
            .authors
            .iter()
            .map(|e| db.lookup_intern_author(*e))
            .collect();
        authors.sort_by(|a, b| a.name().cmp(b.name()));

        match config.author_mode {
            AuthorMode::Nothing => (),
            AuthorMode::Full => {
                writeln!(w, "\t\tUsed by {} authors", authors.len())?;
                let buckets = authors_chrono::split_by_century(authors.into_iter());
                for (cent, size) in buckets.into_iter().map(|(cent, auth)| (cent, auth.len())) {
                    writeln!(
                        w,
                        "\t\t\t{} {}: {}",
                        cent.abs(),
                        if cent > 0 { "BCE" } else { "ACE" },
                        size
                    )?;
                }
            }
            AuthorMode::NumberOnly => writeln!(w, "\t\tUsed by {} authors", authors.len())?,
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

    fn write(&self, db: &impl MainDatabase, w: &mut impl Write) -> io::Result<()> {
        for entry in &self.ls {
            entry.write(w, db, self.config)?;
        }
        Ok(())
    }
}
