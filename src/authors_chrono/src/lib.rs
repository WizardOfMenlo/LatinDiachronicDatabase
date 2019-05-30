#[derive(Debug, Clone, Eq)]
pub struct Author {
    name: String,
}

impl Author {
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for Author {
    fn eq(&self, other: &Author) -> bool {
        self.name().eq(other.name())
    }
}

impl std::hash::Hash for Author {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.name().hash(hasher);
    }
}
