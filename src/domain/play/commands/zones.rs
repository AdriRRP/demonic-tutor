//! Supports play commands zones.

use crate::domain::play::ids::{CardInstanceId, PlayerId};

#[derive(Debug, Clone)]
pub struct ExileCardCommand {
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
    pub from_battlefield: bool, // if false, from graveyard
}

impl ExileCardCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, card_id: CardInstanceId, from_battlefield: bool) -> Self {
        Self {
            player_id,
            card_id,
            from_battlefield,
        }
    }
}
