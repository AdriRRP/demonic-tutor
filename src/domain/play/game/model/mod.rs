//! Supports play game model.

mod location_index;
mod player;
mod priority;
mod stack;
mod terminal_state;

pub use location_index::{AggregateCardLocation, AggregateCardLocationIndex};
pub use player::{
    Player, PlayerCardZone, PrepareHandSpellCastError, PreparedHandSpellCast, MAX_HAND_SIZE,
    OPENING_HAND_SIZE,
};
pub use priority::PriorityState;
pub use stack::{
    ActivatedAbilityOnStack, SpellOnStack, SpellTarget, StackObject, StackObjectKind, StackZone,
};
pub use terminal_state::TerminalState;
