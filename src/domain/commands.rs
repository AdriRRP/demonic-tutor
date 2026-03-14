use crate::domain::{
    cards::CardType,
    ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerId},
};

#[derive(Debug, Clone)]
pub struct PlayerDeck {
    pub player_id: PlayerId,
    pub deck_id: DeckId,
}

impl PlayerDeck {
    #[must_use]
    pub const fn new(player_id: PlayerId, deck_id: DeckId) -> Self {
        Self { player_id, deck_id }
    }
}

#[derive(Debug, Clone)]
pub struct StartGameCommand {
    pub game_id: GameId,
    pub players: Vec<PlayerDeck>,
}

impl StartGameCommand {
    #[must_use]
    pub const fn new(game_id: GameId, players: Vec<PlayerDeck>) -> Self {
        Self { game_id, players }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerDeckContents {
    pub player_id: PlayerId,
    pub cards: Vec<(CardDefinitionId, CardType)>,
}

impl PlayerDeckContents {
    #[must_use]
    pub const fn new(player_id: PlayerId, cards: Vec<(CardDefinitionId, CardType)>) -> Self {
        Self { player_id, cards }
    }
}

#[derive(Debug, Clone)]
pub struct DealOpeningHandsCommand {
    pub player_cards: Vec<PlayerDeckContents>,
}

impl DealOpeningHandsCommand {
    #[must_use]
    pub const fn new(player_cards: Vec<PlayerDeckContents>) -> Self {
        Self { player_cards }
    }
}

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

#[derive(Debug, Clone, Default)]
pub struct AdvanceTurnCommand;

impl AdvanceTurnCommand {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

#[derive(Debug, Clone)]
pub struct DrawCardCommand {
    pub player_id: PlayerId,
}

impl DrawCardCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}

#[derive(Debug, Clone)]
pub struct MulliganCommand {
    pub player_id: PlayerId,
}

impl MulliganCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}

#[derive(Debug, Clone)]
pub struct SetLifeCommand {
    pub player_id: PlayerId,
    pub life_change: i32,
}

impl SetLifeCommand {
    #[must_use]
    pub const fn new(player_id: PlayerId, life_change: i32) -> Self {
        Self {
            player_id,
            life_change,
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
