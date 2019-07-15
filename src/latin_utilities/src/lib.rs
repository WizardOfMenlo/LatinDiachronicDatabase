//! A crate that encompasses various utilities for working with Latin String  
//! In particular, it aims to standardize latin string representation to a
//! format that resembles closely Italian.
//! The high level concept is to normalize unicode, remove all non alphabetical
//! characters, and to then replace j, v to i, u

mod normalized_latin_string;
pub use self::normalized_latin_string::NormalizedLatinString;

use lazy_static::lazy_static;
use std::collections::HashSet;
use unicode_normalization::UnicodeNormalization;

/// A converter which can be use to turn a `&str` into a
///  [`NormalizedLatingString`](struct.NormalizedLatinString.html)
#[derive(Debug, Clone, Default)]
pub struct StandardLatinConverter;

lazy_static! {
    static ref ALLOWED: HashSet<char> = {
        let mut m = HashSet::new();
        // We use this to disambiguate
        m.insert('/');
        m.insert('#');
        m
    };
}

impl StandardLatinConverter {
    /// Convert a str to the correctly parsed form, converting j -> i, v -> u
    /// ```
    /// use latin_utilities::StandardLatinConverter;
    /// let res = StandardLatinConverter::default().convert("dura lex, sed lex");
    /// assert_eq!(res, "dura lex sed lex");
    /// ```
    pub fn convert(&self, input: impl AsRef<str>) -> NormalizedLatinString {
        // Unicode normalisation
        let mut res: String = input
            .as_ref()
            .nfd()
            .filter(|c| {
                ALLOWED.contains(c) || c.is_whitespace() || (c.is_alphanumeric() && !c.is_digit(10))
            })
            .collect();

        // Lowercase
        res = res.to_lowercase();

        // Last round of replacements
        const TO_REPLACE: [&str; 10] = ["j", "v", "[", "]", "{", "}", "(", ")", "<", ">"];
        const REPLACEMENT: [&str; 10] = ["i", "u", "", "", "", "", "", "", "", ""];

        for i in 0..TO_REPLACE.len() {
            res = res.replace(TO_REPLACE[i], REPLACEMENT[i]);
        }

        NormalizedLatinString::instantiate(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn make() -> StandardLatinConverter {
        StandardLatinConverter::default()
    }

    #[test]
    fn test_empty() {
        let converter = make();
        assert_eq!(converter.convert(""), "");
    }

    #[test]
    fn test_identical() {
        let converter = make();
        assert_eq!(converter.convert("hello"), "hello");
        assert_eq!(converter.convert("HELLO"), "hello");
    }

    #[test]
    fn test_single_letter_replacement() {
        let converter = make();
        assert_eq!(converter.convert("v"), "u");
        assert_eq!(converter.convert("V"), "u");
        assert_eq!(converter.convert("u"), "u");
        assert_eq!(converter.convert("U"), "u");
        assert_eq!(converter.convert("J"), "i");
        assert_eq!(converter.convert("j"), "i");
        assert_eq!(converter.convert("I"), "i");
        assert_eq!(converter.convert("i"), "i");
    }

    #[test]
    fn test_phrase_replacement() {
        let converter = make();
        assert_eq!(converter.convert("dvra lex, sed lex"), "dura lex sed lex");
        assert_eq!(converter.convert("Julius Caesar"), "iulius caesar");
    }

    #[test]
    fn test_exotic_letters() {
        let converter = make();
        let test_strings = [
            "á, é, í, ó, ú, ü, ñ",
            "In amóre inermus",
            "Hell<o> t<[here]>",
        ];
        let model_outputs = ["a e i o u u n", "in amore inermus", "hello there"];

        for i in 0..test_strings.len() {
            assert_eq!(converter.convert(test_strings[i]), model_outputs[i]);
        }
    }

    #[test]
    fn test_numbers() {
        let converter = make();
        assert_eq!(converter.convert("123something456"), "something");
    }

    #[test]
    fn test_apostrophe() {
        let converter = make();
        assert_eq!(converter.convert("e'do"), "edo");
        assert_eq!(converter.convert("a'vitus puella"), "auitus puella");
    }

    #[test]
    fn test_underscore() {
        let converter = make();
        assert_eq!(converter.convert("e-do"), "edo");
        assert_eq!(converter.convert("a-vitus puella"), "auitus puella");
    }

    proptest! {
        #[test]
        fn doesnt_crash(s in "\\PC*") {
            let converter = make();
            converter.convert(s)
        }
    }
}
