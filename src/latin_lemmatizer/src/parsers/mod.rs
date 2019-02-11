use super::NaiveLemmatizer;
use std::io::{self, BufRead};

pub mod csv_format;
pub mod lemlat_format;

// START Error handling utilities ===================================
#[derive(Debug)]
pub enum ParsingError {
    LineFormatError,
    IOError(io::Error),
}

impl From<io::Error> for ParsingError {
    fn from(err: io::Error) -> Self {
        ParsingError::IOError(err)
    }
}

type PResult<T> = Result<T, ParsingError>;

// END Error handling utilities  ===================================

pub trait ParserImpl {
    fn new() -> Self;
    fn read_line_as_str(&mut self, line: &str) -> PResult<()>;
    fn build(self) -> NaiveLemmatizer;
}

#[derive(Debug)]
pub struct ParserWrapper<T: ParserImpl>(T);

impl<T: ParserImpl> ParserWrapper<T> {
    pub fn new() -> Self {
        ParserWrapper(T::new())
    }

    pub fn read_line(&mut self, reader: impl io::Read) -> PResult<()> {
        let mut reader = io::BufReader::new(reader);
        let mut s = String::new();
        reader.read_line(&mut s)?;
        self.0.read_line_as_str(&s)
    }

    pub fn read_all(mut self, reader: impl io::Read) -> PResult<Self> {
        let reader = io::BufReader::new(reader);
        for line in reader.lines() {
            let line = line?;
            self.0.read_line_as_str(&line)?;
        }
        Ok(self)
    }

    pub fn build(self) -> NaiveLemmatizer {
        self.0.build()
    }
}

impl<T: ParserImpl> Default for ParserWrapper<T> {
    fn default() -> Self {
        Self::new()
    }
}
