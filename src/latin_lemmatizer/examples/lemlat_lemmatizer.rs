use clap::{App, Arg};
use latin_lemmatizer::{parsers, NaiveLemmatizer};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let matches = App::new("CSV Lemmatizer")
        .version("0.1")
        .author("Giacomo Fenzi <giacomofenzi@outlook.com>")
        .about("A simple utility to test the CSV lemmatizer")
        .arg(
            Arg::with_name("lemmafile")
                .required(true)
                .index(1)
                .takes_value(true)
                .value_name("INPUTFILE")
                .help("The input file provided"),
        )
        .arg(
            Arg::with_name("interactive")
                .short("I")
                .conflicts_with("input")
                .help("Uses interactive mode"),
        )
        .arg(
            Arg::with_name("input")
                .short("i")
                .conflicts_with("interactive")
                .required_unless("interactive")
                .takes_value(true),
        )
        .get_matches();

    let lemma_path_str = matches.value_of("lemmafile").unwrap();
    let f = File::open(lemma_path_str)?;
    let lemmatizer = parsers::lemlat_format::new()
        .read_all(f)
        .expect("Building the lemmatizer failed")
        .build();

    println!("Lemmatizer correctly parsed");
    println!(
        "Lemmas: {}. Forms: {}",
        lemmatizer.num_lemmas(),
        lemmatizer.num_forms()
    );

    if matches.is_present("interactive") {
        println!("Interactive mode");
        println!("===============");
        for line in io::stdin().lock().lines() {
            let arg = line.expect("Invalid line read");
            handle_input(&arg, &lemmatizer);
            println!("===============");
        }
    } else {
        let arg = matches.value_of("input").unwrap();
        handle_input(arg, &lemmatizer);
    }

    Ok(())
}

fn handle_input(s: &str, lemmatizer: &NaiveLemmatizer) {
    let res_o = lemmatizer.convert_and_get_possible_lemmas(s);
    match res_o {
        Some(value) => {
            let mut v: Vec<_> = value.iter().collect();
            v.sort();
            println!("{:?}", v);
        }
        None => println!("Not Found"),
    }
}
