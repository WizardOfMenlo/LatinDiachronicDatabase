use latin_utilities::NormalizedLatinString;
use salsa::InternId;

#[salsa::query_group(FormsQueryGroup)]
pub trait FormsDatabase {
    #[salsa::interned]
    fn intern_form(&self, p: Form) -> FormId;
}

#[derive(Debug, Hash, Eq, Copy, PartialEq, Clone)]
pub struct FormId(InternId);

impl salsa::InternKey for FormId {
    fn from_intern_id(v: InternId) -> Self {
        FormId(v)
    }

    fn as_intern_id(&self) -> InternId {
        self.0
    }
}

#[derive(shrinkwraprs::Shrinkwrap, Debug, Hash, Clone, Eq, PartialEq)]
pub struct Form(pub NormalizedLatinString);
