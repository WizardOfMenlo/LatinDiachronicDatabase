use std::collections::HashMap;
use std::hash::Hash;

/// A simple struct that wraps a integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawId(pub u32);

impl From<RawId> for u32 {
    fn from(raw: RawId) -> u32 {
        raw.0
    }
}

impl From<u32> for RawId {
    fn from(id: u32) -> RawId {
        RawId(id)
    }
}

/// A trait satisfied by all types that are used as our ids.
pub trait InternerId: Hash + Eq + Clone + Copy {
    fn into_raw(self) -> RawId;
    fn from_raw(i: RawId) -> Self;
}

// Shamelessly stolen to simplify stuff
#[macro_export]
macro_rules! impl_arena_id {
    ($name:ident) => {
        impl $crate::InternerId for $name {
            fn from_raw(raw: $crate::RawId) -> Self {
                $name(raw)
            }
            fn into_raw(self) -> $crate::RawId {
                self.0
            }
        }
    };
}

/// A bidirectional mapping ID <-> Type
#[derive(Debug)]
pub struct Interner<ID, T>
where
    ID: InternerId,
    T: Hash + Eq + Clone,
{
    // For performance, this could be replaced for arena
    id_to_type: HashMap<ID, T>,
    type_to_id: HashMap<T, ID>,
}

impl<ID: InternerId, T> Interner<ID, T>
where
    ID: InternerId,
    T: Hash + Eq + Clone,
{
    /// Get the id for a particular item
    pub fn to_id(&self, id: &T) -> ID {
        self.type_to_id[id]
    }

    /// Get the item for a particular id
    pub fn fetch(&self, id: ID) -> &T {
        &self.id_to_type[&id]
    }

    pub fn len(&self) -> usize {
        // Should be equivalent if well constructed
        self.id_to_type.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Default)]
pub struct InternerBuilder<ID, T>
where
    ID: InternerId,
    T: Hash + Eq + Clone,
{
    id_to_type: HashMap<ID, T>,
    type_to_id: HashMap<T, ID>,
}

impl<ID, T> InternerBuilder<ID, T>
where
    ID: InternerId,
    T: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self {
            id_to_type: HashMap::new(),
            type_to_id: HashMap::new(),
        }
    }

    pub fn add_mapping(mut self, id: ID, t: impl Into<T>) -> Self {
        let t = t.into();
        self.type_to_id.insert(t.clone(), id);
        self.id_to_type.insert(id, t);
        self
    }

    pub fn add_all(mut self, v: impl IntoIterator<Item = (ID, T)>) -> Self {
        for (id, value) in v {
            self = self.add_mapping(id, value);
        }
        self
    }

    pub fn build(self) -> Interner<ID, T> {
        // TODO, maybe this error checking is too mynimalistic?
        assert_eq!(self.id_to_type.len(), self.type_to_id.len());
        Interner {
            id_to_type: self.id_to_type,
            type_to_id: self.type_to_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Hash, Eq, Clone, Copy, PartialEq)]
    struct TestId(RawId);
    impl_arena_id!(TestId);

    #[test]
    fn test_empty() {
        let interner: Interner<TestId, RawId> = InternerBuilder::new().build();
        assert_eq!(interner.len(), 0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_access_empty() {
        let interner: Interner<TestId, RawId> = InternerBuilder::new().build();
        let id = TestId(RawId(0));
        interner.fetch(id);
    }

    #[test]
    fn test_single_construction() {
        let exp_id = TestId(RawId(42));
        let exp_val = RawId(21);

        let interner = InternerBuilder::new().add_mapping(exp_id, exp_val).build();

        assert_eq!(interner.len(), 1);
        assert_eq!(*interner.fetch(exp_id), exp_val);
        assert_eq!(interner.to_id(&exp_val), exp_id);
    }

    #[test]
    fn test_multiple_construction() {
        let mut input_v = Vec::new();
        for i in 0..10 {
            input_v.push((TestId(RawId(i)), RawId(i + 1)));
        }
        let interner = InternerBuilder::new().add_all(input_v).build();

        assert_eq!(interner.len(), 10);

        for i in 0..10 {
            let exp_id = TestId(RawId(i));
            let exp_val = RawId(i + 1);

            assert_eq!(*interner.fetch(exp_id), exp_val);
            assert_eq!(interner.to_id(&exp_val), exp_id);
        }
    }
}
