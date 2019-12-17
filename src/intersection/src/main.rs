use authors_chrono::TimeSpan;
use query_driver::driver_init;
use query_system::ids::*;
use query_system::lit_subset::LitSubset;
use query_system::traits::*;
use query_system::types::Author;
use runner::load_configuration;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    color_backtrace::install();
    env_logger::init();
    let db = driver_init(load_configuration())?;

    let mut map = HashMap::new();

    for (auth, id) in db.authors() {
        if auth.tspan().is_some() && auth.tspan().unwrap().get_century().0 >= -1 {
            map.insert(auth, uniquely_his(auth.clone(), *id, &db));
        }
    }

    let mut arr: Vec<_> = map
        .into_iter()
        .filter(|(_, s)| s.is_some())
        .map(|(a, s)| (a, s.unwrap()))
        .collect();

    arr.sort_by(|(_, f), (_, s)| s.len().cmp(&f.len()));

    for (auth, list) in arr {
        println!("{} \t ({})", auth.name(), list.len());
    }

    Ok(())
}

fn uniquely_his(
    auth: Author,
    id: AuthorId,
    db: &query_driver::MainDatabase,
) -> Option<Arc<HashSet<LemmaId>>> {
    if auth.tspan().is_none() {
        return None;
    }

    let tspan = auth.tspan().clone().unwrap();
    let (_, end) = tspan.get_century();
    let rest_of_lit =
        LitSubset::from_timespan(&TimeSpan::new_cent(-10, end), db.authors(), &db.snapshot());

    let intersection = db.intersect_sources(
        LitSubset::from_authors(vec![id].iter(), &db.snapshot()),
        rest_of_lit,
    );

    Some(intersection)
}

fn lemma_id_to_string(db: &impl IntermediateDatabase, lemma: query_system::ids::LemmaId) -> String {
    db.lookup_intern_lemma(lemma).0.inner().to_string()
}
