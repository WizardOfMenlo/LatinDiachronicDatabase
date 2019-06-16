use query_system::ids::{FormDataId, FormId, LemmaId};
use query_system::lit_subset::LitSubset;
use query_system::traits::*;

use query_driver::driver_init;
use runner::load_configuration;
use std::fs::File;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let db = driver_init(load_configuration())?;
    let lit = LitSubset::from_authors(db.authors().values(), &db.snapshot());
    let mut d_a = Dictionary::new(&db, lit.clone());
    d_a.sort_alpha(&db);
    let mut d_b = d_a.clone();
    d_b.sort_freq(&db);

    d_a.write(&db, &mut File::create("test1")?, &d_b)?;
    d_b.write(&db, &mut File::create("test2")?, &d_a)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Entry {
    lemma: LemmaId,
    count: usize,
    forms: Vec<(FormId, Vec<FormDataId>)>,
}

impl Entry {
    fn write(&self, )
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
            ls.push(Entry {
                lemma,
                count,
                forms: forms.iter().map(|(a, b)| (*a, b.clone())).collect(),
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
    ) -> Result<(), Box<std::error::Error>> {
        for entry in self.ls {
            entry.write(w)?;
        }
        Ok(())
    }
}
