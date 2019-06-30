use authors_chrono::Author;
use query_driver::driver_init;
use query_system::ids::*;
use query_system::lit_subset::LitSubset;
use query_system::traits::*;

use std::fs::File;

use runner::load_configuration;
use std::collections::HashSet;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    color_backtrace::install();
    env_logger::init();
    let db = driver_init(load_configuration())?;
    let lit = LitSubset::from_authors(db.authors().right_values(), &db.snapshot());
    let mut d_a = Dictionary::new(&db, lit.clone());
    d_a.sort_alpha(&db);
    let mut d_b = d_a.clone();
    d_b.sort_freq(&db);

    d_a.write(&db, &mut File::create("alpha.txt")?, &d_b)?;
    d_b.write(&db, &mut File::create("freq.txt")?, &d_a)?;
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
        other: &Dictionary,
    ) -> io::Result<()> {
        let corresponding_index = other.ls.iter().position(|l| l.lemma == self.lemma).unwrap();
        let resolved_lemma = db.lookup_intern_lemma(self.lemma);
        writeln!(
            w,
            "-------{:6}------{} count: {} (C: {}, A: {})",
            corresponding_index,
            resolved_lemma.0.inner(),
            self.count,
            self.count - self.ambig_count,
            self.ambig_count
        )?;

        for (form, count) in self.forms.iter().map(|(k, v)| (k, v.len())) {
            let resolved_form = db.lookup_intern_form(*form);
            writeln!(
                w,
                "\t{}: {} {}",
                resolved_form.0.inner(),
                count,
                if db.as_ref().is_ambig(&resolved_form.0) {
                    "(*)"
                } else {
                    ""
                }
            )?;
        }

        let mut authors: Vec<&Author> = self
            .authors
            .iter()
            .map(|e| db.lookup_intern_author(*e))
            .collect();
        authors.sort_by(|a, b| a.name().cmp(b.name()));

        writeln!(w, "\t\tUsed by {} authors", authors.len())?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
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
                .filter(|(&k, _)| db.as_ref().is_ambig(&db.lookup_intern_form(k).0))
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

        Dictionary { ls }
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
        other: &Dictionary,
    ) -> io::Result<()> {
        for entry in &self.ls {
            entry.write(w, db, other)?;
        }
        Ok(())
    }
}
