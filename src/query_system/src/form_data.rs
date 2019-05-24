use crate::ids::{FormDataId, FormId, SourceId};

#[salsa::query_group(FormDataQueryGroup)]
pub trait FormDataDatabase {
    #[salsa::interned]
    fn intern_form_data(&self, fd: FormData) -> FormDataId;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct FormData {
    source: SourceId,
    line_no: usize,
    form: FormId,
}

impl FormData {
    pub fn new(source: SourceId, line_no: usize, form: FormId) -> Self {
        Self {
            source,
            line_no,
            form,
        }
    }

    pub fn source(&self) -> SourceId {
        self.source
    }

    pub fn line_no(&self) -> usize {
        self.line_no
    }

    pub fn form(&self) -> FormId {
        self.form
    }
}
