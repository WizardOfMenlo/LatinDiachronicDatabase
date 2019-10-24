use query_driver::driver_init;
use query_system::lit_subset::LitSubset;
use query_system::traits::*;
use query_system::types::Author;
use authors_chrono::TimeSpan;
use runner::load_configuration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    color_backtrace::install();
    env_logger::init();
    let mut db = driver_init(load_configuration())?;
    let lit = LitSubset::from_timespan(&TimeSpan::new_cent(-10, 1), db.authors(), &db.snapshot());

    let authors = LitSubset::from_authors(
        vec![
            Author::new("Publius Papinius Statius"),
            Author::new("Publius Ovidius Naso"),
        ]
        .into_iter()
        .map(|a| db.intern_author(a))
        .collect::<Vec<_>>()
        .iter(),
        &db.snapshot(),
    );

    let intersection : Vec<_> = db.intersect_sources(authors, lit).iter().map(|l| lemma_id_to_string(&db,*l)).collect();

    println!("{:?}", intersection);

    Ok(())
}

fn lemma_id_to_string(db: &impl IntermediateDatabase, lemma: query_system::ids::LemmaId) -> String {
    db.lookup_intern_lemma(lemma).0.inner().to_string()
}
