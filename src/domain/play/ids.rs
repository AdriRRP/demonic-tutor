//! Supports domain play ids with compact inline storage for common short ids.

use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    sync::Arc,
};

const INLINE_ID_CAPACITY: usize = 22;

#[derive(Clone, Copy)]
struct InlineIdStr {
    len: u8,
    bytes: [u8; INLINE_ID_CAPACITY],
}

impl InlineIdStr {
    fn new(value: &str) -> Option<Self> {
        let bytes = value.as_bytes();
        if bytes.len() > INLINE_ID_CAPACITY {
            return None;
        }

        let mut inline = [0; INLINE_ID_CAPACITY];
        inline[..bytes.len()].copy_from_slice(bytes);
        Some(Self {
            len: u8::try_from(bytes.len()).ok()?,
            bytes: inline,
        })
    }

    fn as_str(&self) -> &str {
        let bytes = &self.bytes[..usize::from(self.len)];
        // `InlineIdStr` is built only from valid UTF-8 `&str` inputs.
        unsafe { std::str::from_utf8_unchecked(bytes) }
    }
}

impl fmt::Debug for InlineIdStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq for InlineIdStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for InlineIdStr {}

impl Hash for InlineIdStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

#[derive(Clone)]
pub struct SharedIdStr(SharedIdStrRepr);

#[derive(Clone)]
enum SharedIdStrRepr {
    Inline(InlineIdStr),
    Shared(Arc<str>),
}

impl SharedIdStr {
    fn new(value: &str) -> Self {
        InlineIdStr::new(value).map_or_else(
            || Self(SharedIdStrRepr::Shared(Arc::from(value))),
            |inline| Self(SharedIdStrRepr::Inline(inline)),
        )
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        match &self.0 {
            SharedIdStrRepr::Inline(inline) => inline.as_str(),
            SharedIdStrRepr::Shared(shared) => shared.as_ref(),
        }
    }
}

impl From<String> for SharedIdStr {
    fn from(value: String) -> Self {
        InlineIdStr::new(&value).map_or_else(
            || Self(SharedIdStrRepr::Shared(Arc::from(value))),
            |inline| Self(SharedIdStrRepr::Inline(inline)),
        )
    }
}

impl From<&str> for SharedIdStr {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl Deref for SharedIdStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl fmt::Debug for SharedIdStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq for SharedIdStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for SharedIdStr {}

impl Hash for SharedIdStr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl fmt::Display for SharedIdStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

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

#[cfg(test)]
mod tests {
    //! Verifies compact inline ids remain deterministic at the public surface.

    use super::{GameId, PlayerId, SharedIdStr, StackObjectId};

    #[test]
    fn short_ids_round_trip_through_inline_storage() {
        let game_id = GameId::new("game-1");
        let player_id = PlayerId::new("player-2");

        assert_eq!(game_id.as_str(), "game-1");
        assert_eq!(player_id.as_str(), "player-2");
    }

    #[test]
    fn stack_object_ids_keep_stable_public_strings() {
        let stack_id = StackObjectId::for_stack_object(&GameId::new("game-9"), 4);

        assert_eq!(stack_id.as_str(), "game-9-stack-4");
    }

    #[test]
    fn long_ids_fall_back_without_changing_public_text() {
        let long_id = SharedIdStr::from("this-id-is-longer-than-inline-capacity".to_string());

        assert_eq!(long_id.as_str(), "this-id-is-longer-than-inline-capacity");
    }
}
