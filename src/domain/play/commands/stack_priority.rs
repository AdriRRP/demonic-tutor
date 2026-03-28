//! Supports play commands stack priority.

use crate::domain::play::{
    game::SpellTarget,
    ids::{CardInstanceId, PlayerId},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalSpellMode {
    TargetPlayerGainLife,
    TargetPlayerLoseLife,
}

#[derive(Debug, Clone)]
pub enum SpellChoice {
    HandCard(CardInstanceId),
    ModalMode(ModalSpellMode),
    SecondaryCreatureTarget(Option<CardInstanceId>),
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
pub struct ResolveOptionalEffectCommand {
    pub player_id: PlayerId,
    pub accept: bool,
}

impl ResolveOptionalEffectCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, accept: bool) -> Self {
        Self { player_id, accept }
    }

    #[must_use]
    pub const fn accept(player_id: PlayerId) -> Self {
        Self {
            player_id,
            accept: true,
        }
    }

    #[must_use]
    pub const fn decline(player_id: PlayerId) -> Self {
        Self {
            player_id,
            accept: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvePendingHandChoiceCommand {
    pub player_id: PlayerId,
    pub chosen_card_id: CardInstanceId,
}

impl ResolvePendingHandChoiceCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, chosen_card_id: CardInstanceId) -> Self {
        Self {
            player_id,
            chosen_card_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvePendingScryCommand {
    pub player_id: PlayerId,
    pub move_to_bottom: bool,
}

impl ResolvePendingScryCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, move_to_bottom: bool) -> Self {
        Self {
            player_id,
            move_to_bottom,
        }
    }

    #[must_use]
    pub const fn keep_on_top(player_id: PlayerId) -> Self {
        Self::new(player_id, false)
    }

    #[must_use]
    pub const fn move_to_bottom(player_id: PlayerId) -> Self {
        Self::new(player_id, true)
    }
}

#[derive(Debug, Clone)]
pub struct ResolvePendingSurveilCommand {
    pub player_id: PlayerId,
    pub move_to_graveyard: bool,
}

impl ResolvePendingSurveilCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, move_to_graveyard: bool) -> Self {
        Self {
            player_id,
            move_to_graveyard,
        }
    }

    #[must_use]
    pub const fn keep_on_top(player_id: PlayerId) -> Self {
        Self::new(player_id, false)
    }

    #[must_use]
    pub const fn move_to_graveyard(player_id: PlayerId) -> Self {
        Self::new(player_id, true)
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
