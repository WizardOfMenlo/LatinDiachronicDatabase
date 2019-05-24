use salsa::InternId;

// Convenience, since all of these are repeated almost exactly
macro_rules! create_ids {
    ($($name:ident),*) => {
        $(
        #[derive(Debug, Hash, Eq, Copy, PartialEq, Clone)]
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
    };
}

create_ids!(AuthorId, FormDataId, FormId, LemmaId, SourceId);
