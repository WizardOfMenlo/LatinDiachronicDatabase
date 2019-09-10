use crate::{Author, TimeSpan};
use chrono::{Date, TimeZone, Utc};
use regex::Regex;
use std::collections::BTreeSet;
use std::error::Error;
use std::io::{self, prelude::*, BufReader};

/// A parser for our homemade and well loved format
#[derive(Debug, Default)]
pub struct WeirdParser {
    authors: BTreeSet<Author>,
}

/// The line where the parsing failed
#[derive(Debug)]
pub struct LineNo(usize);

/// The various way the parsing can fail
#[derive(Debug)]
pub enum ParsingError {
    /// Too many hashtags chars in the line specified
    InvalidNumberOfChunks(usize, LineNo),
    /// Either 0, or 3+ dates in the line
    InvalidNumberOfDates(usize, LineNo),
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
    /// Read the source to completion
    pub fn read_all(&mut self, read: impl io::Read) -> Result<(), ParsingError> {
        let bufreader = BufReader::new(read);
        for (i, line) in bufreader.lines().enumerate() {
            self.read_line(&line.unwrap(), LineNo(i))?;
        }
        Ok(())
    }

    /// Get the resulting representation
    pub fn build(self) -> BTreeSet<Author> {
        self.authors
    }

    fn read_line(&mut self, line: &str, num: LineNo) -> Result<(), ParsingError> {
        // We skip these lines
        if line.contains('~') {
            return Ok(());
        }

        let chunks: Vec<_> = line.split('#').collect();
        if chunks.len() != 2 {
            return Err(ParsingError::InvalidNumberOfChunks(chunks.len(), num));
        }

        let author_name = chunks[0].trim();
        let span = chunks[1];

        // No hist info
        if span.contains('?') {
            self.authors.insert(Author::new(author_name));
            return Ok(());
        }

        let inner = span.trim_end_matches(')').trim_start_matches('(');

        let segments: Vec<_> = inner.split(',').collect();

        if segments.is_empty() || segments.len() > 2 {
            return Err(ParsingError::InvalidNumberOfDates(segments.len(), num));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let parser = WeirdParser::default();
        let res = parser.build();
        assert_eq!(res.len(), 0);
    }
}
