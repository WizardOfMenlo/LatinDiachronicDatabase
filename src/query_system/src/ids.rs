//! The mod where all the `ids` live
//! This encompasses both the interned ids and the ones used throught, such as `AuthorId` and `SourceId`

use salsa::InternId;
use salsa::InternKey;

// Convenience, since all of these are repeated almost exactly
macro_rules! create_ids {
    ($($name:ident),*) => {
        $(
        #[derive(Debug, Hash, Eq, Copy, PartialEq, Clone, PartialOrd, Ord)]
        pub struct $name(InternId);
        impl_intern_key!($name);
        )*
    };
}

macro_rules! impl_intern_key {
    ($name:ident) => {
        impl salsa::InternKey for $name {
            fn from_intern_id(v: InternId) -> Self {
                $name(v)
            }

            fn as_intern_id(&self) -> InternId {
                self.0
            }
        }

        impl $name {
            pub fn from_integer(v: u32) -> Self {
                $name::from_intern_id(InternId::from(v))
            }
        }
    };
}

create_ids!(AuthorId, FormDataId, FormId, LemmaId, SourceId);
