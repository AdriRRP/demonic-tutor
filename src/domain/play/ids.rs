use std::sync::Arc;

// Centralize the backing storage for domain ids so we can revisit the choice
// later without touching every id type. We intentionally keep `Arc<str>` today
// because ids are cloned pervasively into events and tests, and we do not yet
// have profiling that justifies a broader representation change.
type SharedIdStr = Arc<str>;

macro_rules! shared_string_id {
    ($name:ident, display) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(pub SharedIdStr);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(SharedIdStr::from(value.into()))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(SharedIdStr::from(value))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(pub SharedIdStr);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(SharedIdStr::from(value.into()))
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(SharedIdStr::from(value))
            }
        }
    };
}

shared_string_id!(GameId, display);
shared_string_id!(PlayerId, display);
shared_string_id!(DeckId);
shared_string_id!(CardInstanceId, display);
shared_string_id!(CardDefinitionId, display);
shared_string_id!(StackObjectId, display);
