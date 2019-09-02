//! A crate that encompasses the concept of a lemmatizer  
//! A lemmatizer, formally speaking, is a way to resolve a form to a determinate lemma  
//! For example, a lemmatizer could feasibly resolve the word `rosae` to the lemma `rosa`

pub mod parsers;

use latin_utilities::{NormalizedLatinString, StandardLatinConverter};
use std::collections::{HashMap, HashSet};

type Mapping = HashMap<NormalizedLatinString, HashSet<NormalizedLatinString>>;

/// A lemmatizer that uses a simple hashmap lookup to resolve lemmas
#[derive(Debug, Default, Clone)]
pub struct NaiveLemmatizer {
    form_to_lemma: Mapping,
    lemma_to_form: Mapping,
    converter: StandardLatinConverter,
}

impl NaiveLemmatizer {
    pub fn new(form_to_lemma: Mapping) -> Self {
        // TODO, deduplicate similar mappings
        NaiveLemmatizer {
            lemma_to_form: Self::invert_mapping(&form_to_lemma),
            form_to_lemma,
            converter: StandardLatinConverter::default(),
        }
    }

    fn invert_mapping(form_to_lemma: &Mapping) -> Mapping {
        // Invert the mapping
        let lemma_to_form_pairs = form_to_lemma
            .iter()
            .map(|(k, v)| v.iter().map(|e| (e.clone(), k.clone())).collect::<Vec<_>>())
            .flatten();

        let mut lemma_to_form = HashMap::new();
        for (k, v) in lemma_to_form_pairs {
            lemma_to_form
                .entry(k)
                .or_insert_with(HashSet::new)
                .insert(v);
        }

        lemma_to_form
    }

    pub fn num_lemmas(&self) -> usize {
        self.lemma_to_form.len()
    }

    pub fn num_forms(&self) -> usize {
        self.form_to_lemma.len()
    }

    pub fn has_form(&self, form: &NormalizedLatinString) -> bool {
        self.form_to_lemma.contains_key(form)
    }

    pub fn has_lemma(&self, lemma: &NormalizedLatinString) -> bool {
        self.lemma_to_form.contains_key(lemma)
    }

    pub fn get_possible_lemmas(
        &self,
        key: &NormalizedLatinString,
    ) -> Option<&HashSet<NormalizedLatinString>> {
        self.form_to_lemma.get(key)
    }

    pub fn get_possible_forms(
        &self,
        lemma: &NormalizedLatinString,
    ) -> Option<&HashSet<NormalizedLatinString>> {
        self.lemma_to_form.get(lemma)
    }

    pub fn convert_and_get_possible_lemmas(
        &self,
        key: &str,
    ) -> Option<&HashSet<NormalizedLatinString>> {
        self.get_possible_lemmas(&self.converter.convert(key))
    }

    pub fn convert_and_get_possible_forms(
        &self,
        key: &str,
    ) -> Option<&HashSet<NormalizedLatinString>> {
        self.get_possible_forms(&self.converter.convert(key))
    }

    pub fn is_ambig(&self, form: &NormalizedLatinString) -> bool {
        match self.get_possible_lemmas(form) {
            Some(v) => v.len() > 1,
            None => false,
        }
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
        let mut form_to_lemma = Mapping::new();
        form_to_lemma.insert(
            "first".into(),
            hashset_from_vec(vec!["vaa".into(), "vab".into()]),
        );
        form_to_lemma.insert(
            "second".into(),
            hashset_from_vec(vec!["vba".into(), "vbb".into()]),
        );

        NaiveLemmatizer::new(form_to_lemma)
    }

    #[test]
    fn test_default() {
        let lemmatizer = NaiveLemmatizer::default();
        assert_eq!(lemmatizer.num_lemmas(), 0);
        assert_eq!(lemmatizer.num_forms(), 0);
        assert!(lemmatizer.get_possible_lemmas(&"test".into()).is_none());
        assert!(lemmatizer.get_possible_forms(&"test".into()).is_none());
    }

    #[test]
    fn test_querying() {
        let lemmatizer = default_lemmatizer();

        assert_eq!(lemmatizer.num_lemmas(), 4);
        assert_eq!(lemmatizer.num_forms(), 2);

        let values = ["first", "second"];
        let results_lemmas = [
            hashset_from_vec(vec!["vaa".into(), "vab".into()]),
            hashset_from_vec(vec!["vba".into(), "vbb".into()]),
        ];

        for (i, &v) in values.iter().enumerate() {
            let query = lemmatizer.get_possible_lemmas(&v.into()).unwrap();
            assert_eq!(query.len(), results_lemmas[i].len());
            assert_eq!(*query, results_lemmas[i]);
        }

        let forms = ["vaa", "vab", "vba", "vbb"];
        let results_forms = [
            hashset_from_vec(vec!["first".into()]),
            hashset_from_vec(vec!["first".into()]),
            hashset_from_vec(vec!["second".into()]),
            hashset_from_vec(vec!["second".into()]),
        ];

        for (i, &v) in forms.iter().enumerate() {
            let query = lemmatizer.get_possible_forms(&v.into()).unwrap();
            assert_eq!(query.len(), results_forms[i].len());
            assert_eq!(*query, results_forms[i]);
        }
    }
}
