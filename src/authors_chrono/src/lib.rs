use chrono::prelude::*;
use chrono::Utc;
use std::cmp::Ordering;
use std::collections::BTreeMap;

pub mod parsers;

/// A struct representing the span between two dates
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeSpan {
    start: Date<Utc>,
    end: Date<Utc>,
}

impl TimeSpan {
    /// Instantiate the TimeSpan
    pub fn new(start: Date<Utc>, end: Date<Utc>) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    /// Does `self` contain `other`? Note that contains is only a partial order, resembling set inclusion
    /// where  `not contains a b` does not necessarily imply `contains b a`
    pub fn contains(&self, other: &TimeSpan) -> bool {
        // Note, we consider [a,b] intervals, rather than [a,b)
        self.start <= other.start && other.end <= self.end
    }

    /// Get the start of the interval
    pub fn start(&self) -> &Date<Utc> {
        &self.start
    }

    /// Get the end of the interval
    pub fn end(&self) -> &Date<Utc> {
        &self.end
    }

    /// Get a tuple with the centuries spanned
    pub fn get_century(&self) -> (i32, i32) {
        (self.start().year() / 100, self.end().year() / 100)
    }
}

impl PartialOrd for TimeSpan {
    fn partial_cmp(&self, other: &TimeSpan) -> Option<Ordering> {
        if self.contains(other) && other.contains(self) {
            return Some(Ordering::Equal);
        }

        if self.contains(other) {
            return Some(Ordering::Greater);
        }

        if other.contains(self) {
            return Some(Ordering::Less);
        }

        None
    }
}

/// Our representation of an Author
#[derive(Debug, Clone, Eq)]
pub struct Author {
    name: String,
    time_span: Option<TimeSpan>,
}

impl Author {
    /// Instantiate a new author, with no historical information
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            time_span: None,
        }
    }

    /// Instantiate author with historical info
    pub fn new_with_tspan(name: impl ToString, time_span: TimeSpan) -> Self {
        Self {
            name: name.to_string(),
            time_span: Some(time_span),
        }
    }

    /// Get author name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get author's historical info
    pub fn tspan(&self) -> Option<&TimeSpan> {
        self.time_span.as_ref()
    }

    /// Was this author present in this time, if we know?
    pub fn in_timespan(&self, time_span: &TimeSpan) -> bool {
        match &self.time_span {
            Some(t) => time_span.contains(t),
            None => false,
        }
    }
}

/// Given an iterator over authors, construct a mapping that buckets authors by date
pub fn split_by_century<'a>(
    iter: impl IntoIterator<Item = &'a Author>,
) -> BTreeMap<i32, Vec<&'a Author>> {
    let mut res = BTreeMap::new();
    for author in iter.into_iter() {
        // Skip null
        if author.tspan().is_none() {
            continue;
        }
        let span = author.tspan().unwrap();
        let (s, e) = span.get_century();
        let mut possible_centuries = Vec::new();
        for i in s..=e {
            // Skip 0 since there is no zeroth cent
            if i == 0 {
                continue;
            }
            possible_centuries.push(i);
        }

        for cent in possible_centuries {
            res.entry(cent).or_insert_with(Vec::new).push(author);
        }
    }

    res
}

// All these impls ensure comparision are only ever done by name

impl PartialEq for Author {
    fn eq(&self, other: &Author) -> bool {
        self.name().eq(other.name())
    }
}

impl PartialOrd for Author {
    fn partial_cmp(&self, other: &Author) -> Option<Ordering> {
        Some(self.name().cmp(other.name()))
    }
}

impl Ord for Author {
    fn cmp(&self, other: &Author) -> Ordering {
        self.name().cmp(other.name())
    }
}

impl std::hash::Hash for Author {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.name().hash(hasher);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_century(century: i32) -> Date<Utc> {
        Utc.ymd(century * 100, 1, 1)
    }

    #[test]
    fn contains() {
        let firsts = [(1, 4), (1, 1), (-4, -1), (-1, 1), (2, 3), (2, 3)];
        let seconds = [(2, 3), (1, 1), (-3, -2), (0, 0), (4, 5), (1, 3)];
        let exp = [true, true, true, true, false, false];

        assert_eq!(firsts.len(), seconds.len());
        assert_eq!(seconds.len(), exp.len());

        for ((first, second), exp) in firsts.iter().zip(seconds.iter()).zip(exp.iter()) {
            let f = TimeSpan::new(make_century(first.0), make_century(first.1));
            let s = TimeSpan::new(make_century(second.0), make_century(second.1));
            assert_eq!(f.contains(&s), *exp);
        }
    }

    #[test]
    fn get_century() {
        for i in -10..10 {
            for j in i..10 {
                assert_eq!(
                    TimeSpan::new(make_century(i), make_century(j)).get_century(),
                    (i, j)
                )
            }
        }
    }

    fn author_single_cent(name: &str, cent: i32) -> Author {
        Author::new_with_tspan(name, TimeSpan::new(make_century(cent), make_century(cent)))
    }

    #[test]
    fn split() {
        let f = |n, c| author_single_cent(n, c);
        let vec = vec![f("first", 1), f("second", 2)];
        let res = split_by_century(vec.iter());
        let first_cen = res.get(&1).unwrap();

        assert_eq!(first_cen.len(), 1);
        assert_eq!(first_cen[0].name(), "first");
    }
}
