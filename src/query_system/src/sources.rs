//! The lowest level interfacing with source files directly

use crate::ids::{AuthorId, FormDataId, SourceId};
use crate::types::{Form, FormData, InternDatabase};
use latin_utilities::StandardLatinConverter;

use std::collections::HashSet;
use std::sync::Arc;

/// The trait that is used to parse sources
/// Usage: set the source text, define the relation between sources and authors
#[salsa::query_group(SourcesQueryGroup)]
pub trait SourcesDatabase: InternDatabase {
    /// Get the source text for a specified source
    #[salsa::input]
    fn source_text(&self, source_id: SourceId) -> Arc<String>;

    /// Get the sources for an author
    #[salsa::input]
    fn associated_sources(&self, author_id: AuthorId) -> Arc<HashSet<SourceId>>;

    /// Get the author for a source
    #[salsa::input]
    fn associated_author(&self, source_id: SourceId) -> AuthorId;

    // Low level
    /// Get a determined line in a source, if possible
    fn get_line(&self, source_id: SourceId, line: usize) -> Option<Arc<String>>;

    // TODO, benchmark and see if hashset actually worth it
    /// Parse a source, returning the FormData that it generates
    fn parse_source(&self, source_id: SourceId) -> Arc<HashSet<FormDataId>>;
}

// Note, this function is O(line), so it should be used scarcely
fn get_line(db: &impl SourcesDatabase, source_id: SourceId, line: usize) -> Option<Arc<String>> {
    let text = db.source_text(source_id);
    text.lines().nth(line).map(|l| Arc::new(l.to_string()))
}

fn parse_source(db: &impl SourcesDatabase, source_id: SourceId) -> Arc<HashSet<FormDataId>> {
    info!("Parsing source {:?}", source_id);
    let converter = StandardLatinConverter::default();
    let mut form_data_ids = HashSet::new();

    let text = db.source_text(source_id);

    for (i, line) in text.lines().enumerate() {
        for word in line.split(' ') {
            let lw = converter.convert(word);
            let form = Form(lw);
            let form_id = db.intern_form(form);
            let form_data = FormData::new(source_id, i, form_id);
            let form_data_id = db.intern_form_data(form_data);
            form_data_ids.insert(form_data_id);
        }
    }

    Arc::new(form_data_ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::MockDatabase;
    use proptest::prelude::*;
    use std::iter;

    fn create_mock_database() -> MockDatabase {
        MockDatabase::new()
    }

    fn generate_source_repeated_n_lines(s: &str, n: usize) -> Arc<String> {
        // TODO, might want to make the string dyn generated
        Arc::new(iter::repeat(s).take(n).collect::<Vec<&str>>().join("\n"))
    }

    #[test]
    #[should_panic]
    fn test_panic_non_set_source_text() {
        let db = create_mock_database();
        db.source_text(SourceId::from_integer(0));
    }

    #[test]
    #[should_panic]
    fn test_panic_non_set_author_sources() {
        let db = create_mock_database();
        db.associated_sources(AuthorId::from_integer(0));
    }

    #[test]
    fn test_empty_source_parsing() {
        let mut db = create_mock_database();
        let source = SourceId::from_integer(0);
        db.set_source_text(source, Arc::new(String::new()));
        let res = db.parse_source(source);

        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_source_parsing() {
        let mut db = create_mock_database();
        let source = SourceId::from_integer(0);
        db.set_source_text(source, generate_source_repeated_n_lines("puella", 100));
        let parse_res = db.parse_source(source);

        assert_eq!(parse_res.len(), 100);

        let form_data: Vec<_> = parse_res
            .iter()
            .map(|&fd| db.lookup_intern_form_data(fd))
            .collect();
        let form_id = form_data[0].form();

        assert!(form_data.iter().all(|fd| fd.source() == source));
        assert!(form_data.iter().all(|fd| fd.form() == form_id));
    }

    use insta::assert_debug_snapshot;
    use std::collections::BTreeSet;

    #[test]
    fn parse_lorem_ipsum() {
        let mut db = create_mock_database();
        let source = SourceId::from_integer(0);
        db.set_source_text(source, Arc::new(String::from(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")));
        let parse_res = db.parse_source(source);
        let form_data: BTreeSet<_> = parse_res
            .iter()
            .map(|&fd| db.lookup_intern_form_data(fd))
            .map(|fd| (db.lookup_intern_form(fd.form()), fd.source(), fd.line_no()))
            .collect();

        assert_debug_snapshot!("lorem_ipusm", form_data)
    }

    #[test]
    fn parse_segment() {
        let text = r#"Notum est omnibus, Celse, penes te studiorum
nostrorum manere summam, ideoque primum sedulitatis meae
inpendium iudiciis tuis offerre proposui. nam cum sibi
inter aequales quendam locum deposcat aemulatio,
neminem magis conatibus nostris profuturum credidi quam
qui inter eos in hac parte plurimum possit. itaque quo
cultior in quorundam notitiam ueniat, omnia tibi nota
perlaturus ad te primum liber iste festinet, apud te"#;

        let mut db = create_mock_database();
        let source = SourceId::from_integer(0);
        db.set_source_text(source, Arc::new(text.to_string()));
        let parse_res = db.parse_source(source);
        let form_data: BTreeSet<_> = parse_res
            .iter()
            .map(|&fd| db.lookup_intern_form_data(fd))
            .map(|fd| (db.lookup_intern_form(fd.form()), fd.source(), fd.line_no()))
            .collect();

        assert_debug_snapshot!("notum_est_omnibus", form_data)
    }

    proptest! {
        #[test]
        fn doesnt_crash(s in "\\PC*") {
            let mut db = create_mock_database();
            let source = SourceId::from_integer(0);
            db.set_source_text(source, Arc::new(s));
            let _parse_res = db.parse_source(source);
        }
    }

}
