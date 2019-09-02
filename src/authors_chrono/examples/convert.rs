use authors_chrono::parsers::WeirdParser;
use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let matches = App::new("Author Converter")
        .version("0.1")
        .author("Giacomo Fenzi <giacomofenzi@outlook.com>")
        .about("A simple utility to convert the author file to JSON")
        .arg(
            Arg::with_name("authorsfile")
                .required(true)
                .index(1)
                .takes_value(true)
                .value_name("INPUTFILE")
                .help("The input file provided"),
        )
        .arg(
            Arg::with_name("output")
                .required(false)
                .short("o")
                .takes_value(true)
                .value_name("OUTPUT")
                .help("The output file to write to"),
        )
        .get_matches();

    let authors_path_str = matches.value_of("authorsfile").unwrap();
    let f = File::open(authors_path_str)?;
    let mut authors_parser = WeirdParser::default();
    authors_parser
        .read_all(f)
        .expect("Building the lemmatizer failed");

    let authors = authors_parser.build();

    println!("Authors correctly parsed");
    println!("Num authors {}", authors.len());

    /*
    let json = serde_json::to_string(&authors)?;

    if let Some(filename) = matches.value_of("output") {
        let f = File::open(filename)?;
        write!(f, "{}", json);
    } else {
        println!("{}", json);
    }
    */

    Ok(())
}
