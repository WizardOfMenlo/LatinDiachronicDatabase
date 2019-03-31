//! Parser for data in the lemlat format

use super::error::ParsingError;
use super::{ParserBuilder, ParserWrapper};
use crate::{Mapping, NaiveLemmatizer, StandardLatinConverter};
use std::collections::HashSet;

#[derive(Debug)]
pub struct LemlatFormatParserBuilder {
    mapping: Mapping,
    converter: StandardLatinConverter,
}

pub type LemlatFormatParser = ParserWrapper<LemlatFormatParserBuilder>;

/// Instantiate a parser to be used when working with data in the lemlat format
pub fn new() -> LemlatFormatParser {
    LemlatFormatParser::new()
}

impl ParserBuilder for LemlatFormatParserBuilder {
    type ErrorTy = ParsingError;

    fn new() -> Self {
        LemlatFormatParserBuilder {
            mapping: Mapping::new(),
            converter: StandardLatinConverter::default(),
        }
    }

    // Used to reduce the cost of calling BufReader::new
    fn read_line_as_str(&mut self, line: impl AsRef<str>) -> Result<(), Self::ErrorTy> {
        let header_body: Vec<&str> = line.as_ref().split('\t').collect();
        if header_body.len() < 2 {
            return Err(ParsingError::LineFormatError);
        }

        // Start from 2 to avoid id field
        let lemma = header_body[0];
        let body = &header_body[2..];

        for record in body {
            let form = record
                .split(' ')
                .next()
                .ok_or(ParsingError::LineFormatError)?;

            // Convert to normal form
            let (lemma, form) = (self.converter.convert(lemma), self.converter.convert(form));

            // Update the mapping
            self.mapping
                .entry(form)
                .or_insert_with(HashSet::new)
                .insert(lemma);
        }

        Ok(())
    }

    fn build(self) -> NaiveLemmatizer {
        NaiveLemmatizer::new(self.mapping)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_line() {
        let mut parser = new();
        let res =
            parser.read_line("Aaron	28308	Aaron (masc nom sg)	Aaroni (masc dat sg)".as_bytes());
        res.unwrap();

        let lemmatizer = parser.build();
        assert_eq!(lemmatizer.num_forms(), 2);
        assert!(lemmatizer.has_form(&"Aaron".into()));
        assert!(lemmatizer.has_form(&"Aaroni".into()));
    }
}
