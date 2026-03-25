//! Supports dual-layer play ids with numeric core identity and stable public text.

use std::{
    collections::HashMap,
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    sync::{Arc, Mutex, OnceLock},
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct NumericCoreId(u64);

impl NumericCoreId {
    const fn new(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Default)]
struct IdInterner {
    next: u64,
    by_public: HashMap<SharedIdStr, NumericCoreId>,
    by_core: HashMap<NumericCoreId, SharedIdStr>,
}

impl IdInterner {
    fn intern(&mut self, public: &SharedIdStr) -> NumericCoreId {
        if let Some(core) = self.by_public.get(public).copied() {
            return core;
        }

        self.next += 1;
        let core = NumericCoreId::new(self.next);
        self.by_public.insert(public.clone(), core);
        self.by_core.insert(core, public.clone());
        core
    }

    fn public_for_core(&self, core: NumericCoreId) -> Option<SharedIdStr> {
        self.by_core.get(&core).cloned()
    }
}

fn shared_interner() -> &'static Mutex<IdInterner> {
    static SHARED_INTERNER: OnceLock<Mutex<IdInterner>> = OnceLock::new();
    SHARED_INTERNER.get_or_init(|| Mutex::new(IdInterner::default()))
}

fn intern_numeric_core(public: &SharedIdStr) -> NumericCoreId {
    let mut interner = match shared_interner().lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    interner.intern(public)
}

fn public_for_numeric_core(core: NumericCoreId) -> Option<SharedIdStr> {
    let interner = match shared_interner().lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    interner.public_for_core(core)
}

macro_rules! dual_layer_id {
    ($name:ident, display) => {
        #[derive(Clone)]
        pub struct $name {
            core: NumericCoreId,
            public: SharedIdStr,
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                let public = SharedIdStr::from(value.into());
                let core = intern_numeric_core(&public);
                Self { core, public }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.public
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.core == other.core
            }
        }

        impl Eq for $name {}

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.core.hash(state);
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.public)
            }
        }
    };
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name {
            core: NumericCoreId,
            public: SharedIdStr,
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                let public = SharedIdStr::from(value.into());
                let core = intern_numeric_core(&public);
                Self { core, public }
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.public)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.core == other.core
            }
        }

        impl Eq for $name {}

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.core.hash(state);
            }
        }
    };
}

dual_layer_id!(GameId, display);
dual_layer_id!(PlayerId, display);
dual_layer_id!(DeckId);
dual_layer_id!(CardInstanceId, display);
dual_layer_id!(CardDefinitionId, display);

impl CardInstanceId {
    pub(crate) const fn core_u64(&self) -> u64 {
        self.core.0
    }

    pub(crate) fn from_core_u64(core: u64) -> Option<Self> {
        let core = NumericCoreId::new(core);
        let public = public_for_numeric_core(core)?;
        Some(Self { core, public })
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
    core: NumericCoreId,
    public: SharedIdStr,
}

impl StackObjectId {
    pub fn new(value: impl Into<String>) -> Self {
        let public = SharedIdStr::from(value.into());
        let core = intern_numeric_core(&public);
        Self { core, public }
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
}

impl From<String> for StackObjectId {
    fn from(value: String) -> Self {
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
        self.core == other.core
    }
}

impl Eq for StackObjectId {}

impl Hash for StackObjectId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.core.hash(state);
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

    use super::{GameId, PlayerId, SharedIdStr, StackObjectId};

    #[test]
    fn short_ids_round_trip_through_inline_storage() {
        let game_id = GameId::new("game-1");
        let player_id = PlayerId::new("player-2");

        assert_eq!(game_id.as_str(), "game-1");
        assert_eq!(player_id.as_str(), "player-2");
    }

    #[test]
    fn same_public_text_reuses_the_same_numeric_core() {
        let left = GameId::new("game-42");
        let right = GameId::new("game-42");

        assert_eq!(left, right);
        assert_eq!(left.core.0, right.core.0);
    }

    #[test]
    fn different_public_texts_receive_distinct_numeric_cores() {
        let left = PlayerId::new("player-a");
        let right = PlayerId::new("player-b");

        assert_ne!(left, right);
        assert_ne!(left.core.0, right.core.0);
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
        assert_eq!(left.core.0, right.core.0);
    }

    #[test]
    fn long_ids_fall_back_without_changing_public_text() {
        let long_id = SharedIdStr::from("this-id-is-longer-than-inline-capacity".to_string());

        assert_eq!(long_id.as_str(), "this-id-is-longer-than-inline-capacity");
    }
}
