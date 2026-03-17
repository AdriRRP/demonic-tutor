use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameId(pub Arc<str>);

impl GameId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(Arc::from(value.into()))
    }
}

impl From<String> for GameId {
    fn from(s: String) -> Self {
        Self(Arc::from(s))
    }
}

impl std::fmt::Display for GameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GameId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerId(pub Arc<str>);

impl PlayerId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(Arc::from(value.into()))
    }
}

impl From<String> for PlayerId {
    fn from(s: String) -> Self {
        Self(Arc::from(s))
    }
}

impl std::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PlayerId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeckId(pub Arc<str>);

impl DeckId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(Arc::from(value.into()))
    }
}

impl From<String> for DeckId {
    fn from(s: String) -> Self {
        Self(Arc::from(s))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CardInstanceId(Arc<str>);

impl CardInstanceId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(Arc::from(value.into()))
    }
}

impl std::fmt::Display for CardInstanceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CardInstanceId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CardDefinitionId(Arc<str>);

impl CardDefinitionId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(Arc::from(value.into()))
    }
}

impl std::fmt::Display for CardDefinitionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CardDefinitionId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackObjectId(Arc<str>);

impl StackObjectId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(Arc::from(value.into()))
    }
}

impl std::fmt::Display for StackObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StackObjectId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
