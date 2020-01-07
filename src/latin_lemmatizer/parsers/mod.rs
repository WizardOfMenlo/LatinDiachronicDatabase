//! A module containing various ways to parse a representation of a lemmatizer
//! In particular, with this we can build a [`NaiveLemmatizer`](struct.NaiveLemmatizer.html)

use super::NaiveLemmatizer;
use error::CompositeParsingError;
use std::fmt::Debug;
use std::io::{prelude::*, BufReader};

pub mod csv_format;
pub mod error;
pub mod lemlat_format;

/// A trait that is used to build parser for lemmatizers
pub trait ParserBuilder {
    /// A type that is used to communicate errors up the chain
    type ErrorTy;

    /// Initiate the builder
    fn new() -> Self;

    /// Read a single line
    fn read_line_as_str(&mut self, line: impl AsRef<str>) -> Result<(), Self::ErrorTy>;

    /// Build a [`NaiveLemmatizer`](struct.NaiveLemmatizer.html)
    fn build(self) -> NaiveLemmatizer;
}

// Auxiliary type
type ErrorTy<T> = CompositeParsingError<<T as ParserBuilder>::ErrorTy>;

#[derive(Debug)]
pub struct ParserWrapper<T: ParserBuilder>(T);

impl<T: ParserBuilder> ParserWrapper<T>
where
    T::ErrorTy: Debug,
{
    /// Create the builder
    pub fn new() -> Self {
        ParserWrapper(T::new())
    }

    /// Read a line from a general source
    pub fn read_line(&mut self, reader: impl Read) -> Result<(), ErrorTy<T>> {
        let mut reader = BufReader::new(reader);
        let mut s = String::new();
        reader.read_line(&mut s)?;
        self.0
            .read_line_as_str(&s)
            .map_err(CompositeParsingError::Inner)
    }

    /// Read a source until completion (i.e. EOF)
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

    /// Construct the lemmatizer
    pub fn build(self) -> NaiveLemmatizer {
        self.0.build()
    }
}

impl<T: ParserBuilder> Default for ParserWrapper<T>
where
    T::ErrorTy: Debug,
{
    fn default() -> Self {
        Self::new()
    }
}
