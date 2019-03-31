//! A crate that encompasses the concept of a lemmatizer  
//! A lemmatizer, formally speaking, is a way to resolve a form to a determinate lemma  
//! For example, a lemmatizer could feasibly resolve the word `rosae` to the lemma `rosa`

pub mod parsers;

use latin_utilities::{NormalizedLatinString, StandardLatinConverter};
use std::collections::{HashMap, HashSet};

// TODO Instead of this, could it be worthwile to have a mapping W -> Id?
type Mapping = HashMap<NormalizedLatinString, HashSet<NormalizedLatinString>>;

/// A lemmatizer that uses a simple hashmap lookup to resolve lemmas
#[derive(Debug, Default, Clone)]
pub struct NaiveLemmatizer {
    mapping: Mapping,
    converter: StandardLatinConverter,
}

impl NaiveLemmatizer {
    pub fn new(mapping: Mapping) -> Self {
        NaiveLemmatizer {
            mapping,
            converter: StandardLatinConverter::default(),
        }
    }

    pub fn num_lemmas(&self) -> usize {
        self.mapping.iter().map(|(_, v)| v.len()).sum()
    }

    pub fn num_forms(&self) -> usize {
        self.mapping.len()
    }

    pub fn has_form(&self, form: &NormalizedLatinString) -> bool {
        self.mapping.contains_key(form)
    }

    pub fn get_possible_lemmas(
        &self,
        key: &NormalizedLatinString,
    ) -> Option<&HashSet<NormalizedLatinString>> {
        self.mapping.get(key)
    }

    pub fn convert_and_get_possible_lemmas(
        &self,
        key: &str,
    ) -> Option<&HashSet<NormalizedLatinString>> {
        self.get_possible_lemmas(&self.converter.convert(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    fn hashset_from_vec(v: Vec<NormalizedLatinString>) -> HashSet<NormalizedLatinString> {
        HashSet::from_iter(v.into_iter())
    }

    fn default_lemmatizer() -> NaiveLemmatizer {
        let mut mapping = Mapping::new();
        mapping.insert(
            "first".into(),
            hashset_from_vec(vec!["vaa".into(), "vab".into()]),
        );
        mapping.insert(
            "second".into(),
            hashset_from_vec(vec!["vba".into(), "vbb".into()]),
        );

        NaiveLemmatizer::new(mapping)
    }

    #[test]
    fn test_default() {
        let lemmatizer = NaiveLemmatizer::default();
        assert_eq!(lemmatizer.num_lemmas(), 0);
        assert_eq!(lemmatizer.num_forms(), 0);
        assert!(lemmatizer.get_possible_lemmas(&"test".into()).is_none());
    }

    #[test]
    fn test_querying() {
        let lemmatizer = default_lemmatizer();

        assert_eq!(lemmatizer.num_lemmas(), 4);
        assert_eq!(lemmatizer.num_forms(), 2);

        let values = ["first", "second"];
        let results = [
            hashset_from_vec(vec!["vaa".into(), "vab".into()]),
            hashset_from_vec(vec!["vba".into(), "vbb".into()]),
        ];

        for (i, &v) in values.iter().enumerate() {
            let query = lemmatizer.get_possible_lemmas(&v.into()).unwrap();
            assert_eq!(query.len(), results[i].len());
            assert_eq!(*query, results[i]);
        }
    }
}
