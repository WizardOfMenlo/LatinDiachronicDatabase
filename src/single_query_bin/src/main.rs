use query_driver::driver_init;
use query_system::lit_subset::LitSubset;
use query_system::traits::*;
use latin_utilities::NormalizedLatinString;
use runner::load_configuration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    color_backtrace::install();
    env_logger::init();
    let db = driver_init(load_configuration())?;
    let lit = LitSubset::from_authors(db.authors().right_values(), &db.snapshot());
    let lemma_id = db.intern_lemma(query_system::types::Lemma(NormalizedLatinString::from("puella")));
    let res = db.lemma_occurrences_subset(lemma_id, lit);
    println!("{:?}", res.len());

    Ok(())
}