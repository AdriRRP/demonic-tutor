use crate::domain::play::ids::{CardInstanceId, PlayerId};

#[derive(Debug, Clone)]
pub struct PlayLandCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl PlayLandCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self { player_id, card_id }
    }
}

#[derive(Debug, Clone)]
pub struct AdjustLifeCommand {
    pub player_id: PlayerId,
    pub life_delta: i32,
}

impl AdjustLifeCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, life_delta: i32) -> Self {
        Self {
            player_id,
            life_delta,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TapLandCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl TapLandCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self { player_id, card_id }
    }
}
