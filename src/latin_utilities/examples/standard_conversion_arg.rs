use clap::{App, Arg};
use latin_utilities::StandardLatinConverter;

fn main() {
    let matches = App::new("Argument Converter")
        .version("0.1")
        .author("Giacomo Fenzi <giacomofenzi@outlook.com>")
        .about("A simple utility to test the converter")
        .arg(
            Arg::with_name("input")
                .required(true)
                .index(1)
                .takes_value(true)
                .value_name("INPUT")
                .help("The input provided"),
        )
        .get_matches();

    let converter = StandardLatinConverter::default();
    let w = matches.value_of("input").expect("Value is required");
    println!("{}", converter.convert(w).inner());
}
