use crate::{Author, TimeSpan};
use chrono::{Date, TimeZone, Utc};
use regex::Regex;
use std::collections::BTreeSet;
use std::error::Error;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Default)]
pub struct WeirdParser {
    authors: BTreeSet<Author>,
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidNumberOfChunks(usize),
    InvalidNumberOfDates(usize),
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParsingError {}

fn parse_segment(s: &str) -> Date<Utc> {
    let re = Regex::new("(\\d)(a|d)").unwrap();
    let captures = re.captures(s).unwrap();
    // Note the first is the whole str
    let century = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let avanti_o_dietro = captures.get(2).unwrap().as_str();
    match avanti_o_dietro {
        "a" => Utc.ymd(century * -100, 1, 1),
        "d" => Utc.ymd(century * 100, 1, 1),
        _ => unreachable!(),
    }
}

impl WeirdParser {
    pub fn read_all(&mut self, read: impl io::Read) -> Result<(), ParsingError> {
        let bufreader = BufReader::new(read);
        for line in bufreader.lines() {
            self.read_line(&line.unwrap())?;
        }
        Ok(())
    }

    pub fn build(self) -> BTreeSet<Author> {
        self.authors
    }

    fn read_line(&mut self, line: &str) -> Result<(), ParsingError> {
        let chunks: Vec<_> = line.split('#').collect();
        if chunks.len() != 2 {
            return Err(ParsingError::InvalidNumberOfChunks(chunks.len()));
        }

        let author_name = chunks[0].trim();
        let span = chunks[1];

        if span.contains('?') {
            self.authors.insert(Author::new(author_name));
            return Ok(());
        }

        let inner = span.trim_end_matches(')').trim_start_matches('(');

        let segments: Vec<_> = inner.split(',').collect();

        if segments.is_empty() || segments.len() > 2 {
            return Err(ParsingError::InvalidNumberOfDates(segments.len()));
        }

        let start = parse_segment(segments[0]);
        let end = if segments.len() == 2 {
            parse_segment(segments[1])
        } else {
            start
        };

        self.authors.insert(Author::new_with_tspan(
            author_name,
            TimeSpan::new(start, end),
        ));

        Ok(())
    }
}
