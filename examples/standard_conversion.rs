use clap::{App, Arg};
use latin_db::latin_utilities::StandardLatinConverter;
use std::io::{self, prelude::*};

fn main() {
    let matches = App::new("Argument Converter")
        .version("0.1")
        .author("Giacomo Fenzi <giacomofenzi@outlook.com>")
        .about("A simple utility to test the converter")
        .arg(
            Arg::with_name("input")
                .index(1)
                .takes_value(true)
                .value_name("INPUT")
                .help("The input provided"),
        )
        .get_matches();

    let converter = StandardLatinConverter::default();

    match matches.value_of("input") {
        Some(w) => println!("{}", converter.convert(w).inner()),
        None => {
            println!("Interactive mode");
            println!("===============");
            for line in io::stdin().lock().lines() {
                let res = converter.convert(line.expect("Invalid line read"));
                println!("{}", res.inner());
                println!("===============");
            }
        }
    }
}
