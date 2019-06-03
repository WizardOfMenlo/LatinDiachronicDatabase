use chrono::Date;
use chrono::Utc;
use std::cmp::Ordering;

pub mod parsers;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeSpan {
    start: Date<Utc>,
    end: Date<Utc>,
}

impl TimeSpan {
    pub fn new(start: Date<Utc>, end: Date<Utc>) -> Self {
        assert!(start < end);
        Self { start, end }
    }

    pub fn contains(&self, other: &TimeSpan) -> bool {
        // Note, we consider [a,b] intervals, rather than [a,b)
        self.start <= other.start && other.end <= self.end
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

    pub fn in_timespan(&self, time_span: &TimeSpan) -> bool {
        match &self.time_span {
            Some(t) => time_span.contains(t),
            None => false,
        }
    }
}

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
