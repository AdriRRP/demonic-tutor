//! Supports domain play ids.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerCardHandle(usize);

impl PlayerCardHandle {
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StackObjectId(pub SharedIdStr);

impl StackObjectId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(SharedIdStr::from(value.into()))
    }

    #[must_use]
    pub fn for_stack_object(game_id: &GameId, object_number: u32) -> Self {
        let object_number = object_number.to_string();
        let mut value =
            String::with_capacity(game_id.as_str().len() + "-stack-".len() + object_number.len());
        value.push_str(game_id.as_str());
        value.push_str("-stack-");
        value.push_str(&object_number);
        Self(SharedIdStr::from(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for StackObjectId {
    fn from(value: String) -> Self {
        Self(SharedIdStr::from(value))
    }
}

impl std::fmt::Display for StackObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
