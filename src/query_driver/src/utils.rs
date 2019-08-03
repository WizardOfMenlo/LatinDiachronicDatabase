use latin_lemmatizer::NaiveLemmatizer;

use query_system::ids::*;
use query_system::traits::*;
use std::collections::HashSet;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::Arc;

/// Load database given a list of authors and some sources
pub fn load_database<S, T: Read>(
    db: &mut impl MainDatabase,
    authors: impl IntoIterator<Item = (AuthorId, HashSet<SourceId>)>,
    sources: impl IntoIterator<Item = (SourceId, S)>,
    extractor: impl Fn(S) -> io::Result<T>,
    lemma: NaiveLemmatizer,
) -> io::Result<()> {
    // Load the authors assoc
    authors.into_iter().for_each(|(k, v)| {
        db.set_associated_sources(k, Arc::new(v.clone()));
        v.iter().for_each(|&s| db.set_associated_author(s, k))
    });

    for (source, reader) in sources.into_iter() {
        let mut read = BufReader::new(extractor(reader)?);
        let mut s = String::new();
        read.read_to_string(&mut s)?;
        db.set_source_text(source, Arc::new(s));
    }

    db.set_lemmatizer(Arc::new(lemma));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use latin_lemmatizer::NaiveLemmatizer;
    use query_system::mock::make_mock;
    #[test]
    fn empty_load() {
        let mut db = make_mock();
        let authors = vec![];
        let sources = vec![];

        // Note, needed to allow for gap in type inf
        #[allow(clippy::redundant_closure)]
        load_database(
            &mut db,
            authors.into_iter(),
            sources.into_iter(),
            |s: &[u8]| Ok(s),
            NaiveLemmatizer::default(),
        )
        .unwrap();
    }
}
