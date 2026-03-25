//! Supports play commands stack priority.

use crate::domain::play::{
    game::SpellTarget,
    ids::{CardInstanceId, PlayerId},
};

#[derive(Debug, Clone)]
pub enum SpellChoice {
    HandCard(CardInstanceId),
}

#[derive(Debug, Clone)]
pub struct PassPriorityCommand {
    pub player_id: PlayerId,
}

impl PassPriorityCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}

#[derive(Debug, Clone)]
pub struct CastSpellCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub target: Option<SpellTarget>,
    pub choice: Option<SpellChoice>,
}

impl CastSpellCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self {
            player_id,
            card_id,
            target: None,
            choice: None,
        }
    }

    #[must_use]
    pub fn with_target(mut self, target: SpellTarget) -> Self {
        self.target = Some(target);
        self
    }

    #[must_use]
    pub fn with_choice(mut self, choice: SpellChoice) -> Self {
        self.choice = Some(choice);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ActivateAbilityCommand {
    pub player_id: PlayerId,
    pub source_card_id: CardInstanceId,
    pub target: Option<SpellTarget>,
}

impl ActivateAbilityCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, source_card_id: CardInstanceId) -> Self {
        Self {
            player_id,
            source_card_id,
            target: None,
        }
    }

    #[must_use]
    pub fn with_target(mut self, target: SpellTarget) -> Self {
        self.target = Some(target);
        self
    }
}
