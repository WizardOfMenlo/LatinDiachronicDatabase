use crate::ids::{FormDataId, SourceId, AuthorId};
use crate::types::{Form, FormData, InternDatabase};
use latin_utilities::StandardLatinConverter;

use std::sync::Arc;

#[salsa::query_group(SourcesQueryGroup)]
pub trait SourcesDatabase: InternDatabase {
    #[salsa::input]
    fn source_text(&self, source_id: SourceId) -> Arc<String>;

    #[salsa::input]
    fn associated_sources(&self, author_id: AuthorId) -> Arc<Vec<SourceId>>;

    // Low level
    fn num_lines(&self, source_id: SourceId) -> usize;
    fn get_line(&self, source_id: SourceId, line: usize) -> Option<Arc<String>>;

    fn parse_source(&self, source_id: SourceId) -> Arc<Vec<FormDataId>>;
}

fn num_lines(db: &impl SourcesDatabase, source_id: SourceId) -> usize {
    let text = db.source_text(source_id);
    text.lines().count()
}

fn get_line(db: &impl SourcesDatabase, source_id: SourceId, line: usize) -> Option<Arc<String>> {
    let text = db.source_text(source_id);
    text.lines().nth(line).map(|l| Arc::new(l.to_string()))
}

fn parse_source(db: &impl SourcesDatabase, source_id: SourceId) -> Arc<Vec<FormDataId>> {
    let num_lines = db.num_lines(source_id);
    let converter = StandardLatinConverter::default();
    let mut form_data_ids = Vec::new();

    for i in 0..num_lines {
        let line = db.get_line(source_id, i).expect("should always succeed");
        for word in line.split(' ') {
            let lw = converter.convert(word);
            let form = Form(lw);
            let form_id = db.intern_form(form);
            let form_data = FormData::new(source_id, i, form_id);
            let form_data_id = db.intern_form_data(form_data);
            form_data_ids.push(form_data_id);
        }
    }

    Arc::new(form_data_ids)
}