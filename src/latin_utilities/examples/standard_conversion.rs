use latin_utilities::StandardLatinConverter;
use std::io::{self, prelude::*};

fn main() {
    let converter = StandardLatinConverter::default();
    println!("===============");
    for line in io::stdin().lock().lines() {
        let res = converter.convert(line.expect("Invalid line read"));
        println!("{}", res.inner());
        println!("===============");
    }
}
