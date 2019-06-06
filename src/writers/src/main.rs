use query_system::ids::{FormId, LemmaId};
use query_system::lit_subset::LitSubset;
use query_system::traits::*;

use query_driver::driver_init;
use runner::load_configuration;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let db = driver_init(load_configuration()).unwrap();
    let all_authors = LitSubset::from_authors(db.authors().values(), &db.snapshot());
    let lemmas = db.lemmas_in_subset(all_authors);
    write_dictionary(FormattingMode::Alphabetical, lemmas.iter().cloned())?;
    write_dictionary(FormattingMode::Historical, lemmas.iter().cloned())?;
    write_dictionary(FormattingMode::Frequential, lemmas.iter().cloned())?;

    Ok(())
}

#[derive(Clone, Copy)]
pub enum FormattingMode {
    Alphabetical,
    Historical,
    Frequential,
}

fn write_dictionary(
    form: FormattingMode,
    lemmas: impl IntoIterator<Item = LemmaId>,
    w: &impl Write,
) -> io::Result<()> {
    Ok(())
}

fn lemma_sorting_func<'a, T>(
    form: FormattingMode,
    sub: LitSubset,
    db: &'a impl MainDatabase,
) -> Box<dyn (Fn(LemmaId, LemmaId) -> std::cmp::Ordering) + 'a> {
    match form {
        FormattingMode::Alphabetical | FormattingMode::Historical => {
            Box::new(move |f, s| db.lookup_intern_lemma(f).cmp(&db.lookup_intern_lemma(s)))
        }
        FormattingMode::Frequential => Box::new(move |f, s| {
            let f = db.count_lemma_occurrences_subset(f, sub.clone());
            let s = db.count_lemma_occurrences_subset(s, sub.clone());
            f.cmp(&s)
        }),
    }
}
