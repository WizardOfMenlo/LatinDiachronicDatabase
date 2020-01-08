use latin_db::arguments::load_configuration;
use latin_db::query_driver::driver_init;
use latin_db::query_system::ids::*;
use latin_db::query_system::lit_subset::LitSubset;
use latin_db::query_system::traits::*;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*};

const AUTHOR_SCALE_FACTOR: usize = 1_000;
const HISTORIC_SCALE_FACTOR: usize = 1_000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    color_backtrace::install();
    env_logger::init();
    let db = driver_init(load_configuration())?;
    let lit = LitSubset::from_authors(db.authors().right_values(), &db.snapshot());
    let alpha = Dictionary::new(&db, lit.clone());

    let author_count = db.authors_count(lit);
    let author_names: BTreeMap<_, _> = author_count
        .keys()
        .map(|a| (db.lookup_intern_author(*a).name(), *a))
        .collect();

    let file = &mut File::create("export.csv")?;
    write!(file, "lemma,count,certain,ambigous,")?;
    for name in author_names.keys() {
        write!(file, "{},", name)?;
        write!(file, "{} Freq,", name)?;
    }

    for cent in -6..=6 {
        write!(file, "{} cent,{} cent rel,", cent, cent)?;
    }

    writeln!(file)?;

    alpha.write(&db, file, &author_count, &author_names)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Entry {
    lemma: LemmaId,
    count: usize,
    ambig_count: usize,
    forms: Vec<(FormId, Vec<FormDataId>)>,
    authors: HashSet<AuthorId>,
}

impl Entry {
    fn write(
        &self,
        w: &mut impl Write,
        db: &impl MainDatabase,
        global_authors_count: &HashMap<AuthorId, usize>,
        authors_names: &BTreeMap<&str, AuthorId>,
    ) -> io::Result<()> {
        let resolved_lemma = db.lookup_intern_lemma(self.lemma);
        write!(
            w,
            "{},{},{},{},",
            resolved_lemma.0.inner().to_uppercase(),
            self.count,
            self.count - self.ambig_count,
            self.ambig_count
        )?;

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

        // For each author, compute the count and the relative freq
        for id in authors_names.values() {
            let count = *authors_count
                .get(db.lookup_intern_author(*id))
                .unwrap_or(&0);
            let relative_count = *global_authors_count.get(id).unwrap_or(&1);
            let freq = ((count * AUTHOR_SCALE_FACTOR) as f64) / relative_count as f64;
            write!(w, "{},{:.2},", count, freq)?;
        }
        // Split the authors by century
        let buckets =
            latin_db::authors_chrono::split_by_century(authors.iter().map(|(_, a)| a).cloned());
        let mut centuries: BTreeMap<_, _> = (-6..=6_i32).map(|i| (i, (0, 0.0))).collect();
        for (cent, authors_b) in buckets.into_iter() {
            // How many we had for each century
            let aggregated = authors_b
                .iter()
                .flat_map(|&a| authors_count.get(&a))
                .sum::<usize>();
            let relative_freq = (aggregated * HISTORIC_SCALE_FACTOR) as f64 / self.count as f64;

            centuries.insert(cent, (aggregated, relative_freq));
        }

        for (aggr, relative) in centuries.values() {
            write!(w, "{},{:.2},", aggr, relative)?;
        }

        writeln!(w)?;

        Ok(())
    }
}

#[derive(Debug)]
struct Dictionary {
    ls: Vec<Entry>,
}

impl Dictionary {
    fn new(db: &impl MainDatabase, sub: LitSubset) -> Self {
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
            })
        }

        let mut res = Dictionary { ls: ls.clone() };

        res.sort_alpha(db);

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

    fn write(
        &self,
        db: &impl MainDatabase,
        w: &mut impl Write,
        author_count: &HashMap<AuthorId, usize>,
        author_names: &BTreeMap<&str, AuthorId>,
    ) -> io::Result<()> {
        for entry in &self.ls {
            entry.write(w, db, author_count, author_names)?;
        }
        Ok(())
    }
}
