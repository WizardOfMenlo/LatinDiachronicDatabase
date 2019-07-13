use chrono::prelude::*;
use chrono::Utc;
use std::cmp::Ordering;
use std::collections::BTreeMap;


pub mod parsers;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeSpan {
    start: Date<Utc>,
    end: Date<Utc>,
}

impl TimeSpan {
    pub fn new(start: Date<Utc>, end: Date<Utc>) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    pub fn contains(&self, other: &TimeSpan) -> bool {
        // Note, we consider [a,b] intervals, rather than [a,b)
        self.start <= other.start && other.end <= self.end
    }

    pub fn start(&self) -> &Date<Utc> {
        &self.start
    }

    pub fn end(&self) -> &Date<Utc> {
        &self.end
    }

    pub fn get_century(&self) -> (i32, i32) {
        (
            century_helper(self.start().year()),
            century_helper(self.end().year()),
        )
    }
}

fn century_helper(year: i32) -> i32 {
    if year >= 0 {
        (year / 100) + 1
    } else {
        (year / 100) - 1
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

#[derive(Debug, Clone, Eq)]
pub struct Author {
    name: String,
    time_span: Option<TimeSpan>,
}

impl Author {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            time_span: None,
        }
    }

    pub fn new_with_tspan(name: impl ToString, time_span: TimeSpan) -> Self {
        Self {
            name: name.to_string(),
            time_span: Some(time_span),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tspan(&self) -> Option<&TimeSpan> {
        self.time_span.as_ref()
    }

    pub fn in_timespan(&self, time_span: &TimeSpan) -> bool {
        match &self.time_span {
            Some(t) => time_span.contains(t),
            None => false,
        }
    }
}

/// Given an iterator over authors, construct a mapping that buckets authors by date
pub fn split_by_century<'a>(
    iter: impl Iterator<Item = &'a Author>,
) -> BTreeMap<i32, Vec<&'a Author>> {
    let mut res = BTreeMap::new();
    for author in iter {
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
