use super::latin_utilities::NormalizedLatinString;
use bimap::BiMap;

pub use super::query_system::ids::WordId;

pub trait WordDatabase {
    fn intern_word(&mut self, s: NormalizedLatinString) -> WordId;
    fn lookup_word(&self, id: WordId) -> Option<&NormalizedLatinString>;
}

#[derive(Debug, Default, Clone)]
pub struct WordDb {
    words: BiMap<WordId, NormalizedLatinString>,
}

impl WordDb {
    fn next_id(&self) -> WordId {
        // TODO: Check
        let mut candidate = self.words.len() as u32;
        while self.words.contains_left(&WordId::from_integer(candidate)) {
            candidate += 1;
        }

        WordId::from_integer(candidate)
    }
}

impl WordDatabase for WordDb {
    fn intern_word(&mut self, s: NormalizedLatinString) -> WordId {
        let id = self.next_id();
        self.words.insert(id, s);
        id
    }

    fn lookup_word(&self, id: WordId) -> Option<&NormalizedLatinString> {
        self.words.get_by_left(&id)
    }
}
