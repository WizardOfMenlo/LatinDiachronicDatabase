use std::fs::File;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

use latin_db::authors_chrono::parsers::WeirdParser;

#[derive(StructOpt)]
#[structopt(
    name = "Author Converter",
    about = "Utility to convert authors to JSON",
    author = "Giacomo Fenzi <giacomofenzi@outlook.com>"
)]
struct Arguments {
    #[structopt(short, long)]
    authors_file: PathBuf,

    #[structopt(short, long)]
    output: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Arguments::from_args();

    let f = File::open(args.authors_file)?;
    let mut authors_parser = WeirdParser::default();
    authors_parser
        .read_all(f)
        .expect("Building the lemmatizer failed");

    let authors = authors_parser.build();

    println!("Authors correctly parsed");
    println!("Num authors {}", authors.len());

    /*
    let json = serde_json::to_string(&authors)?;

    if let Some(filename) = args.output {
        std::fs::write(filename, json)?;
    } else {
        println!("{}", json);
    }
    */

    Ok(())
}
