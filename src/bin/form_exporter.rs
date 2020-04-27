use latin_db::arguments::load_configuration;
use latin_db::query_driver::driver_init;
use latin_db::query_system::lit_subset::LitSubset;
use latin_db::query_system::traits::*;

use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    color_backtrace::install();
    env_logger::init();
    let db = driver_init(load_configuration())?;
    let lit = LitSubset::from_authors(db.authors().right_values(), &db.snapshot());
    let subset_tree = db.subset_tree(lit);

    let forms = subset_tree
        .values()
        .flat_map(|map| map.keys())
        .map(|form| db.lookup_word(form.0).inner().to_string());

    let mut out = File::create("forms.txt")?;

    for form in forms {
        writeln!(out, "{}", form)?;
    }

    Ok(())
}
