use super::latin_utilities::NormalizedLatinString;
use bimap::BiMap;
use std::cell::{Ref, RefCell};
use std::sync::{Arc, Mutex};

pub use super::query_system::ids::WordId;

pub trait WordDatabase {
    fn intern_word(&self, s: NormalizedLatinString) -> WordId;
    fn lookup_word(&self, id: WordId) -> NormalizedLatinString;
    fn lookup_interned_word(&self, s: NormalizedLatinString) -> Option<WordId>;
}

type Mapping = BiMap<WordId, NormalizedLatinString>;

#[derive(Debug, Default, Clone)]
pub struct WordDb {
    words: Arc<Mutex<RefCell<Mapping>>>,
}

impl WordDb {
    fn next_id(words: Ref<Mapping>) -> WordId {
        // TODO: Check
        let mut candidate = words.len() as u32;
        while words.contains_left(&WordId::from_integer(candidate)) {
            candidate += 1;
        }

        WordId::from_integer(candidate)
    }

    pub fn len(&self) -> usize {
        self.words.lock().unwrap().borrow().len()
    }
}

impl WordDatabase for WordDb {
    fn intern_word(&self, s: NormalizedLatinString) -> WordId {
        let lock = self.words.lock().unwrap();

        if let Some(id) = lock.borrow().get_by_right(&s).cloned() {
            return id;
        }

        let id = WordDb::next_id(lock.borrow());
        lock.borrow_mut().insert(id, s);
        id
    }

    fn lookup_word(&self, id: WordId) -> NormalizedLatinString {
        self.words
            .lock()
            .unwrap()
            .borrow()
            .get_by_left(&id)
            .expect("Shouldn't be possible to get a spurious id")
            .clone()
    }

    fn lookup_interned_word(&self, s: NormalizedLatinString) -> Option<WordId> {
        self.words
            .lock()
            .unwrap()
            .borrow()
            .get_by_right(&s)
            .cloned()
    }
}
