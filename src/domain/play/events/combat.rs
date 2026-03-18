use crate::domain::play::ids::{CardInstanceId, GameId, PlayerId};

#[derive(Debug, Clone)]
pub struct AttackersDeclared {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub attackers: Vec<CardInstanceId>,
}

impl AttackersDeclared {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, attackers: Vec<CardInstanceId>) -> Self {
        Self {
            game_id,
            player_id,
            attackers,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockersDeclared {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub assignments: Vec<(CardInstanceId, CardInstanceId)>,
}

impl BlockersDeclared {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        assignments: Vec<(CardInstanceId, CardInstanceId)>,
    ) -> Self {
        Self {
            game_id,
            player_id,
            assignments,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CombatDamageResolved {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub damage_events: Vec<DamageEvent>,
}

impl CombatDamageResolved {
    #[must_use]
    pub const fn new(
        game_id: GameId,
        player_id: PlayerId,
        damage_events: Vec<DamageEvent>,
    ) -> Self {
        Self {
            game_id,
            player_id,
            damage_events,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreatureDied {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl CreatureDied {
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
pub struct DamageEvent {
    pub source: CardInstanceId,
    pub target: DamageTarget,
    pub damage_amount: u32,
}

#[derive(Debug, Clone)]
pub enum DamageTarget {
    Creature(CardInstanceId),
    Player(PlayerId),
}
