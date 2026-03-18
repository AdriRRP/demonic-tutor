use crate::domain::play::ids::{CardInstanceId, PlayerId};

#[derive(Debug, Clone, Default)]
pub struct AdvanceTurnCommand;

impl AdvanceTurnCommand {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct DrawCardsEffectCommand {
    pub caster_id: PlayerId,
    pub target_player_id: PlayerId,
    pub draw_count: u32,
}

impl DrawCardsEffectCommand {
    #[must_use]
    pub const fn new(caster_id: PlayerId, target_player_id: PlayerId, draw_count: u32) -> Self {
        Self {
            caster_id,
            target_player_id,
            draw_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiscardForCleanupCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl DiscardForCleanupCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self { player_id, card_id }
    }
}
