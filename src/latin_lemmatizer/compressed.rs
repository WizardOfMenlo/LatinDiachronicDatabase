use super::NaiveLemmatizer;
use crate::word_db::{WordDatabase, WordId};

use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, Clone)]
pub struct CompressedLemmatizer {
    form_to_lemma: HashMap<WordId, HashSet<WordId>>,
    lemma_to_form: HashMap<WordId, HashSet<WordId>>,
}

impl CompressedLemmatizer {
    pub fn new(lemm: NaiveLemmatizer, db: &impl WordDatabase) -> Self {
        let mut form_to_lemma = HashMap::new();

        for (form, lemmas) in lemm.form_to_lemma {
            let form = db.intern_word(form);
            let lemmas = lemmas.into_iter().map(|l| db.intern_word(l)).collect();
            form_to_lemma.insert(form, lemmas);
        }

        let lemma_to_form = super::invert_mapping(&form_to_lemma);

        CompressedLemmatizer {
            form_to_lemma,
            lemma_to_form,
        }
    }

    pub fn num_lemmas(&self) -> usize {
        self.lemma_to_form.len()
    }

    pub fn num_forms(&self) -> usize {
        self.form_to_lemma.len()
    }

    pub fn has_form(&self, form: WordId) -> bool {
        self.form_to_lemma.contains_key(&form)
    }

    pub fn has_lemma(&self, lemma: WordId) -> bool {
        self.lemma_to_form.contains_key(&lemma)
    }

    pub fn get_possible_lemmas(&self, key: WordId) -> Option<&HashSet<WordId>> {
        self.form_to_lemma.get(&key)
    }

    pub fn get_possible_forms(&self, lemma: WordId) -> Option<&HashSet<WordId>> {
        self.lemma_to_form.get(&lemma)
    }

    pub fn is_ambig(&self, form: WordId) -> bool {
        match self.get_possible_lemmas(form) {
            Some(v) => v.len() > 1,
            None => false,
        }
    }
}
