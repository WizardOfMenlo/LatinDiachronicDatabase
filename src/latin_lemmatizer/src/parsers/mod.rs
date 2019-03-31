//! A module containing various ways to parse a representation of a lemmatizer
//! In particular, with this we can build a [`NaiveLemmatizer`](struct.NaiveLemmatizer.html)

use super::NaiveLemmatizer;
use error::CompositeParsingError;
use std::io::{prelude::*, BufReader};

pub mod csv_format;
pub mod error;
pub mod lemlat_format;

/// A trait that is used to build parser for lemmatizers
pub trait ParserImpl {
    type ErrorTy;

    fn new() -> Self;
    fn read_line_as_str(&mut self, line: &str) -> Result<(), Self::ErrorTy>;
    fn build(self) -> NaiveLemmatizer;
}

/// Auxiliary type 
type ErrorTy<T> = CompositeParsingError<<T as ParserImpl>::ErrorTy>;

#[derive(Debug)]
pub struct ParserWrapper<T: ParserImpl>(T);

impl<T: ParserImpl> ParserWrapper<T> {
    pub fn new() -> Self {
        ParserWrapper(T::new())
    }

    pub fn read_line(&mut self, reader: impl Read) -> Result<(), ErrorTy<T>> {
        let mut reader = BufReader::new(reader);
        let mut s = String::new();
        reader.read_line(&mut s)?;
        self.0
            .read_line_as_str(&s)
            .map_err(CompositeParsingError::Inner)
    }

    pub fn read_all(mut self, reader: impl Read) -> Result<Self, ErrorTy<T>> {
        let reader = BufReader::new(reader);
        for line in reader.lines() {
            let line = line?;
            self.0
                .read_line_as_str(&line)
                .map_err(CompositeParsingError::Inner)?;
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
