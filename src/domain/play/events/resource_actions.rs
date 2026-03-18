use crate::domain::play::ids::{CardInstanceId, GameId, PlayerId};

#[derive(Debug, Clone)]
pub struct LandPlayed {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl LandPlayed {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LifeChanged {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub from_life: u32,
    pub to_life: u32,
}

impl LifeChanged {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, from_life: u32, to_life: u32) -> Self {
        Self {
            game_id,
            player_id,
            from_life,
            to_life,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LandTapped {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl LandTapped {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, card_id: CardInstanceId) -> Self {
        Self {
            game_id,
            player_id,
            card_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManaAdded {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub amount: u32,
    pub new_mana_total: u32,
}

impl ManaAdded {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        amount: u32,
        new_mana_total: u32,
    ) -> Self {
        Self {
            game_id,
            player_id,
            amount,
            new_mana_total,
        }
    }
}
