use crate::{Author, TimeSpan};
use chrono::{Date, TimeZone, Utc};
use regex::Regex;
use std::io::{self, prelude::*, BufReader};

pub struct WeirdParser {
    authors: Vec<Author>,
}

pub enum ParsingError {
    InvalidNumberOfChuncks(usize),
    InvalidNumberOfDates(usize),
}

fn parse_segment(s: &str) -> Date<Utc> {
    let re = Regex::new("(\\d)(a|d)").unwrap();
    let captures = re.captures(s).unwrap();
    let century = captures.get(0).unwrap().as_str().parse::<i32>().unwrap();
    let avanti_o_dietro = captures.get(1).unwrap().as_str();
    match avanti_o_dietro {
        "a" => Utc.ymd(century * 100, 1, 1),
        "d" => Utc.ymd(century * -100, 1, 1),
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

    pub fn build(self) -> Vec<Author> {
        self.authors
    }

    fn read_line(&mut self, line: &str) -> Result<(), ParsingError> {
        let chunks: Vec<_> = line.split('#').collect();
        if chunks.len() != 2 {
            return Err(ParsingError::InvalidNumberOfChuncks(chunks.len()));
        }

        let author_name = chunks[0].trim();
        let span = chunks[1];

        if span.contains('?') {
            self.authors.push(Author::new(author_name));
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

        self.authors.push(Author::new_with_tspan(
            author_name,
            TimeSpan::new(start, end),
        ));

        Ok(())
    }
}
