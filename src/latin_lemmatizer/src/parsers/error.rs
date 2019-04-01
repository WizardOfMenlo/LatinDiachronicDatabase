//! Common error handling facilities

use std::io::Error;

/// A enum error type communicated by parsers (this could feasibly be substituted)
#[derive(Debug)]
pub enum CompositeParsingError<T> {
    Inner(T),
    IOError(Error),
}

impl<T> From<Error> for CompositeParsingError<T> {
    fn from(err: Error) -> Self {
        CompositeParsingError::IOError(err)
    }
}

/// A common ErrorType used for reporting
#[derive(Debug)]
pub enum ParsingError {
    LineFormatError(String),
}
