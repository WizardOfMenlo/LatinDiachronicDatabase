//! Parser for data in the CSV format

use super::error::ParsingError;
use super::{ParserBuilder, ParserWrapper};
use crate::{Mapping, NaiveLemmatizer, StandardLatinConverter};
use std::collections::HashSet;

#[derive(Debug)]
pub struct CSVFormatParserBuilder {
    mapping: Mapping,
    converter: StandardLatinConverter,
}

pub type CSVFormatParser = ParserWrapper<CSVFormatParserBuilder>;

/// Instantiate a parser for when working with data in CSV format
pub fn new() -> CSVFormatParser {
    CSVFormatParser::new()
}

impl ParserBuilder for CSVFormatParserBuilder {
    type ErrorTy = ParsingError;

    fn new() -> Self {
        CSVFormatParserBuilder {
            mapping: Mapping::new(),
            converter: StandardLatinConverter::default(),
        }
    }

    fn read_line_as_str(&mut self, line: impl AsRef<str>) -> Result<(), Self::ErrorTy> {
        let line = line.as_ref();
        let segments: Vec<&str> = line.split(',').collect();
        if segments.len() < 3 {
            return Err(ParsingError::LineFormatError(line.to_string()));
        }

        let lemma = segments[2];
        let form = segments[0];

        let (lemma, form) = (self.converter.convert(lemma), self.converter.convert(form));

        // Update the mapping
        self.mapping
            .entry(form)
            .or_insert_with(HashSet::new)
            .insert(lemma);

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
        let res = parser.read_line(b"iungam,iungam,iungo,V3,i3044,,VmH" as &[u8]);
        res.unwrap();

        let lemmatizer = parser.build();
        assert_eq!(lemmatizer.num_forms(), 1);
        assert!(lemmatizer.has_form(&"iungam".into()));
    }
}
