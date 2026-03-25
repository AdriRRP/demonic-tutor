//! Supports domain play commands.

mod combat;
mod lifecycle;
mod resource_actions;
mod setup;
mod stack_priority;
mod turn_flow;
mod zones;

pub use combat::{DeclareAttackersCommand, DeclareBlockersCommand, ResolveCombatDamageCommand};
pub use lifecycle::{DealOpeningHandsCommand, MulliganCommand, StartGameCommand};
pub use resource_actions::{AdjustPlayerLifeEffectCommand, PlayLandCommand, TapLandCommand};
pub use setup::{LibraryCard, LibraryCreature, PlayerDeck, PlayerLibrary};
pub use stack_priority::{
    ActivateAbilityCommand, CastSpellCommand, ModalSpellMode, PassPriorityCommand, SpellChoice,
};
pub use turn_flow::{AdvanceTurnCommand, DiscardForCleanupCommand, DrawCardsEffectCommand};
pub use zones::ExileCardCommand;
