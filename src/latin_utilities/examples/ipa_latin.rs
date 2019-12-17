use latin_utilities::ipa::*;
use std::fs::File;
use std::io::Write;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    color_backtrace::install();

    let args: Vec<_> = std::env::args().collect();
    let path = &args[1];
    let input = std::fs::read_to_string(path).unwrap();
    let input: Vec<_> = input
        .lines()
        .map(|s| s.trim().to_lowercase().to_string())
        .collect();

    let mut c = File::create("classical.txt").unwrap();

    println!("Classical -------------------------");

    for line in &input {
        writeln!(c, "{}", ipa_latin(line, true)).unwrap();
    }

    let mut e = File::create("eccl.txt").unwrap();

    println!("Ecclesiastic-------------------------");

    for line in input {
        writeln!(e, "{}", ipa_latin(&line, false)).unwrap();
    }
}
