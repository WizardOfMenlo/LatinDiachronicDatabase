use hashbrown::HashMap;
use latin_utilities::NormalizedLatinString;

#[derive(Debug, Clone, Copy)]
pub struct SourceId(usize);

// Strong typedefs for more intuitive api
pub type Form = NormalizedLatinString;
pub type Lemma = NormalizedLatinString;

#[derive(Debug, Clone, Default)]
pub struct WordDatabase {
    word_mapping: HashMap<Lemma, Vec<FormData>>,
}

impl WordDatabase {
    pub fn get_forms_for_lemma(&self, lemma: &Lemma) -> Option<impl Iterator<Item = &Form>> {
        self.word_mapping
            .get(lemma)
            .map(|form_vec| form_vec.iter().map(|f| &f.form))
    }

    pub fn get_number_of_lemmas(&self) -> usize {
        self.word_mapping.len()
    }

    pub fn get_number_of_forms(&self) -> usize {
        self.word_mapping.iter().map(|(_, f)| f.len()).sum()
    }

    pub fn get_total_num_occurrences_of_lemma(&self, lemma: &Lemma) -> usize {
        self.word_mapping
            .get(lemma)
            .map_or(0, |form_v| form_v.iter().map(|f| f.records.len()).sum())
    }
}

// TODO Add more robust Record -> Source File mapping (Maybe the interner can help from ra_syntax?)

#[derive(Debug, Clone)]
struct FormData {
    form: Form,
    records: Vec<Record>,
}

#[derive(Debug, Clone, Copy)]
struct Record {
    source_id: SourceId,
    offset: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_word_db() {
        let db = WordDatabase::default();
        assert!(db.get_forms_for_lemma(&"test".into()).is_none());
        assert_eq!(db.get_number_of_forms(), 0);
        assert_eq!(db.get_number_of_lemmas(), 0);
    }
}
