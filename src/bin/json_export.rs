use latin_db::arguments::load_configuration;
use latin_db::query_driver::driver_init;
use latin_db::query_system::ids::*;
use latin_db::query_system::lit_subset::LitSubset;
use latin_db::query_system::traits::*;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    color_backtrace::install();
    env_logger::init();
    let db = driver_init(load_configuration())?;
    let lit = LitSubset::from_authors(db.authors().right_values(), &db.snapshot());

    let source_tree = db.subset_tree(lit);

    let res: HashMap<String, HashMap<String, Vec<FormData>>> = source_tree
        .iter()
        .map(|(l, inner)| {
            (
                lemma_id_to_string(&db, *l),
                inner
                    .iter()
                    .map(|(f, inner)| {
                        (
                            form_id_to_string(&db, *f),
                            inner
                                .iter()
                                .map(|fd| form_data_normalize(&db, *fd))
                                .collect(),
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    let string = serde_json::to_string_pretty(&res)?;

    std::fs::write("result.json", string)?;

    Ok(())
}

fn lemma_id_to_string(db: &impl IntermediateDatabase, lemma: LemmaId) -> String {
    let id = db.lookup_intern_lemma(lemma).0;
    db.lookup_word(id).inner().to_string()
}

fn form_id_to_string(db: &impl IntermediateDatabase, form: FormId) -> String {
    let id = db.lookup_intern_form(form).0;
    db.lookup_word(id).inner().to_string()
}

fn form_data_normalize(db: &latin_db::query_driver::MainDatabase, fd: FormDataId) -> FormData {
    let fd = db.lookup_intern_form_data(fd);
    FormData {
        line_no: fd.line_no(),
        author: db.lookup_intern_author(fd.author(db)).name().to_string(),
        source: db.sources().get_by_right(&fd.source()).unwrap().clone(),
    }
}

#[derive(Debug, Serialize)]
struct FormData {
    author: String,
    line_no: usize,
    source: PathBuf,
}
