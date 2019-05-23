use query_driver::driver_init;
use query_system::authors::AuthorsDatabase;

fn main() {
    let db = driver_init("./data/works/").unwrap();
    for &source in &db.authors {
        dbg!(db.lookup_intern_author(source));
        dbg!(db.associated_sources(source));
    }
}
