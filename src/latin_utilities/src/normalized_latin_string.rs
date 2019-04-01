use super::StandardLatinConverter;

/// A checked and normalized string type, which can only
/// be instantiated by the [`StandardLatinConverter`](struct.StandardLatinConverter.html) type
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct NormalizedLatinString(String);

impl NormalizedLatinString {
    /// Accessor method to get the inner value of the nlstring
    pub fn inner(&self) -> &str {
        &self.0
    }

    // Crate-local way to create nlstring
    pub(crate) fn instantiate(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl PartialEq<&str> for NormalizedLatinString {
    fn eq(&self, rhs: &&str) -> bool {
        self.0 == *rhs
    }
}

impl PartialEq<String> for NormalizedLatinString {
    fn eq(&self, rhs: &String) -> bool {
        self.0 == *rhs
    }
}

impl From<&str> for NormalizedLatinString {
    fn from(other: &str) -> Self {
        // Note, this could possibly be expensive if the converter does
        // some extra book-keeping, atm its pretty cheap
        StandardLatinConverter::default().convert(other)
    }
}

// This tests really are trivial, but I wanted to ensure functionality
#[cfg(test)]
mod tests {
    use super::*;

    fn make_nlstr(s: &str) -> NormalizedLatinString {
        NormalizedLatinString(s.to_string())
    }

    #[test]
    fn test_comparision_empty() {
        assert_eq!(make_nlstr(""), "");
    }

    #[test]
    fn test_comparision_str() {
        assert_eq!(make_nlstr("hello there"), "hello there");
    }

    #[test]
    fn test_comparision_string() {
        assert_eq!(make_nlstr("A String"), String::from("A String"));
    }

    #[test]
    fn test_comparison_nlstring() {
        assert_eq!(make_nlstr("A word"), make_nlstr("A word"));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(NormalizedLatinString::from("some"), "some");
    }

    #[test]
    fn test_str_into() {
        let s: NormalizedLatinString = "some".into();
        assert_eq!(s, "some");
    }

}
