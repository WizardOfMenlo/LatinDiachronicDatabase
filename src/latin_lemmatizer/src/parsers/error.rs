//! Common error handling facilities

use std::error::Error;
use std::fmt::{Debug, Display};
use std::io::Error as ioError;

/// A enum error type communicated by parsers (this could feasibly be substituted)
#[derive(Debug)]
pub enum CompositeParsingError<T>
where
    T: Debug,
{
    Inner(T),
    IOError(ioError),
}

impl<T: Display + Debug> Display for CompositeParsingError<T> {
    fn fmt(&self, form: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompositeParsingError::Inner(t) => write!(form, "Cause: {}", t),
            CompositeParsingError::IOError(t) => write!(form, "IO: {}", t),
        }
    }
}

impl<T: Debug> From<ioError> for CompositeParsingError<T> {
    fn from(err: ioError) -> Self {
        CompositeParsingError::IOError(err)
    }
}

impl<T: Display + Debug> Error for CompositeParsingError<T> {}

/// A common ErrorType used for reporting
#[derive(Debug)]
pub enum ParsingError {
    LineFormatError(String),
}

impl Display for ParsingError {
    fn fmt(&self, form: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParsingError::LineFormatError(l) => write!(form, "Error in line: {}", l),
        }
    }
}
