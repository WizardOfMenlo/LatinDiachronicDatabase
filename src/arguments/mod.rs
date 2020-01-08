use crate::query_driver::{Configuration, LemmMode};
use clap::{load_yaml, App};

pub fn load_configuration() -> Configuration {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).get_matches();

    Configuration::new(
        app.value_of("data_path").unwrap(),
        app.value_of("lemmatizer").unwrap(),
        app.value_of("authors_path"),
        if app.value_of("useLemlat").is_some() {
            LemmMode::LemlatFormat
        } else {
            LemmMode::CSVFormat
        },
    )
    .unwrap()
}
