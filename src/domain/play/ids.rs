//! Supports stable play ids with compact shared text storage.

use std::{
    fmt,
    hash::{Hash, Hasher},
    mem::size_of,
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

    #[must_use]
    pub(crate) fn estimated_heap_bytes(&self) -> usize {
        match &self.0 {
            SharedIdStrRepr::Inline(_) => 0,
            // `Arc<str>` stores the string bytes plus the strong/weak counters
            // in the shared heap allocation.
            SharedIdStrRepr::Shared(shared) => shared.len() + (2 * size_of::<usize>()),
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

macro_rules! shared_text_id {
    ($name:ident, display) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            public: SharedIdStr,
        }

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Self {
                Self {
                    public: SharedIdStr::from(value.as_ref()),
                }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.public
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self {
                    public: SharedIdStr::from(value),
                }
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.public)
            }
        }
    };
    ($name:ident) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name {
            public: SharedIdStr,
        }

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Self {
                Self {
                    public: SharedIdStr::from(value.as_ref()),
                }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.public
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self {
                    public: SharedIdStr::from(value),
                }
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.public)
            }
        }
    };
}

shared_text_id!(GameId, display);
shared_text_id!(PlayerId, display);
shared_text_id!(DeckId);
shared_text_id!(CardInstanceId, display);
shared_text_id!(CardDefinitionId, display);

impl GameId {
    #[must_use]
    pub(crate) fn estimated_heap_bytes(&self) -> usize {
        self.public.estimated_heap_bytes()
    }
}

impl PlayerId {
    #[must_use]
    pub(crate) fn estimated_heap_bytes(&self) -> usize {
        self.public.estimated_heap_bytes()
    }
}

impl CardInstanceId {
    #[must_use]
    pub(crate) fn estimated_heap_bytes(&self) -> usize {
        self.public.estimated_heap_bytes()
    }
}

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

#[derive(Clone)]
pub struct StackObjectId {
    public: SharedIdStr,
}

impl StackObjectId {
    pub fn new(value: impl AsRef<str>) -> Self {
        Self {
            public: SharedIdStr::from(value.as_ref()),
        }
    }

    #[must_use]
    pub fn for_stack_object(game_id: &GameId, object_number: u32) -> Self {
        let object_number = object_number.to_string();
        let mut value =
            String::with_capacity(game_id.as_str().len() + "-stack-".len() + object_number.len());
        value.push_str(game_id.as_str());
        value.push_str("-stack-");
        value.push_str(&object_number);
        Self::new(value)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.public
    }

    #[must_use]
    pub(crate) fn estimated_heap_bytes(&self) -> usize {
        self.public.estimated_heap_bytes()
    }

    #[must_use]
    pub fn object_number(&self) -> Option<u32> {
        self.as_str().rsplit_once("-stack-")?.1.parse().ok()
    }
}

impl From<String> for StackObjectId {
    fn from(value: String) -> Self {
        Self {
            public: SharedIdStr::from(value),
        }
    }
}

impl From<&str> for StackObjectId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl fmt::Debug for StackObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl PartialEq for StackObjectId {
    fn eq(&self, other: &Self) -> bool {
        self.public == other.public
    }
}

impl Eq for StackObjectId {}

impl Hash for StackObjectId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.public.hash(state);
    }
}

impl fmt::Display for StackObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.public)
    }
}

#[cfg(test)]
mod tests {
    //! Verifies numeric core ids remain stable while public text stays deterministic.

    use super::{CardInstanceId, GameId, PlayerId, SharedIdStr, StackObjectId};

    #[test]
    fn short_ids_round_trip_through_inline_storage() {
        let game_id = GameId::new("game-1");
        let player_id = PlayerId::new("player-2");

        assert_eq!(game_id.as_str(), "game-1");
        assert_eq!(player_id.as_str(), "player-2");
    }

    #[test]
    fn same_public_text_keeps_card_instance_ids_equal() {
        let left = CardInstanceId::new("card-42");
        let right = CardInstanceId::new("card-42");

        assert_eq!(left, right);
        assert_eq!(left.as_str(), right.as_str());
    }

    #[test]
    fn different_public_texts_keep_card_instance_ids_distinct() {
        let left = CardInstanceId::new("card-a");
        let right = CardInstanceId::new("card-b");

        assert_ne!(left, right);
        assert_ne!(left.as_str(), right.as_str());
    }

    #[test]
    fn stack_object_ids_keep_stable_public_strings() {
        let stack_id = StackObjectId::for_stack_object(&GameId::new("game-9"), 4);

        assert_eq!(stack_id.as_str(), "game-9-stack-4");
    }

    #[test]
    fn stack_object_ids_share_numeric_core_for_the_same_public_value() {
        let left = StackObjectId::for_stack_object(&GameId::new("game-9"), 4);
        let right = StackObjectId::new("game-9-stack-4");

        assert_eq!(left, right);
    }

    #[test]
    fn long_ids_fall_back_without_changing_public_text() {
        let long_id = SharedIdStr::from("this-id-is-longer-than-inline-capacity".to_string());

        assert_eq!(long_id.as_str(), "this-id-is-longer-than-inline-capacity");
    }
}
