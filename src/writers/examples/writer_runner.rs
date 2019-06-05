use query_driver::driver_init;
use query_system::lit_subset::LitSubset;
use query_system::traits::*;
use runner::load_configuration;

fn main() {
    let db = driver_init(load_configuration()).unwrap();
    let all_authors = LitSubset::from_authors(db.authors().values(), &db.snapshot());
}
