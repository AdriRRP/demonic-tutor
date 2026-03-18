use crate::domain::play::ids::{CardInstanceId, PlayerId};

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
}

impl CastSpellCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self { player_id, card_id }
    }
}
