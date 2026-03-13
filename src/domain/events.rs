use crate::domain::ids::{CardInstanceId, GameId, PlayerId};

#[derive(Debug, Clone)]
pub enum DomainEvent {
    GameStarted(GameStarted),
    OpeningHandDealt(OpeningHandDealt),
    LandPlayed(LandPlayed),
    TurnAdvanced(TurnAdvanced),
    CardDrawn(CardDrawn),
    MulliganTaken(MulliganTaken),
}

impl DomainEvent {
    #[must_use]
    pub const fn as_game_started(&self) -> Option<&GameStarted> {
        if let Self::GameStarted(e) = self {
            Some(e)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_opening_hand_dealt(&self) -> Option<&OpeningHandDealt> {
        if let Self::OpeningHandDealt(e) = self {
            Some(e)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_land_played(&self) -> Option<&LandPlayed> {
        if let Self::LandPlayed(e) = self {
            Some(e)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_turn_advanced(&self) -> Option<&TurnAdvanced> {
        if let Self::TurnAdvanced(e) = self {
            Some(e)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_card_drawn(&self) -> Option<&CardDrawn> {
        if let Self::CardDrawn(e) = self {
            Some(e)
        } else {
            None
        }
    }

    #[must_use]
    pub const fn as_mulligan_taken(&self) -> Option<&MulliganTaken> {
        if let Self::MulliganTaken(e) = self {
            Some(e)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameStarted {
    pub game_id: GameId,
    pub players: Vec<PlayerId>,
}

impl GameStarted {
    #[must_use]
    pub const fn new(game_id: GameId, players: Vec<PlayerId>) -> Self {
        Self { game_id, players }
    }
}

#[derive(Debug, Clone)]
pub struct OpeningHandDealt {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub cards: Vec<CardInstanceId>,
}

impl OpeningHandDealt {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId, cards: Vec<CardInstanceId>) -> Self {
        Self {
            game_id,
            player_id,
            cards,
        }
    }
}

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
pub struct TurnAdvanced {
    pub game_id: GameId,
    pub new_active_player: PlayerId,
}

impl TurnAdvanced {
    #[must_use]
    pub const fn new(game_id: GameId, new_active_player: PlayerId) -> Self {
        Self {
            game_id,
            new_active_player,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardDrawn {
    pub game_id: GameId,
    pub player_id: PlayerId,
    pub card_id: CardInstanceId,
}

impl CardDrawn {
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
pub struct MulliganTaken {
    pub game_id: GameId,
    pub player_id: PlayerId,
}

impl MulliganTaken {
    #[must_use]
    pub const fn new(game_id: GameId, player_id: PlayerId) -> Self {
        Self { game_id, player_id }
    }
}

impl From<GameStarted> for DomainEvent {
    fn from(event: GameStarted) -> Self {
        Self::GameStarted(event)
    }
}

impl From<OpeningHandDealt> for DomainEvent {
    fn from(event: OpeningHandDealt) -> Self {
        Self::OpeningHandDealt(event)
    }
}

impl From<LandPlayed> for DomainEvent {
    fn from(event: LandPlayed) -> Self {
        Self::LandPlayed(event)
    }
}

impl From<TurnAdvanced> for DomainEvent {
    fn from(event: TurnAdvanced) -> Self {
        Self::TurnAdvanced(event)
    }
}

impl From<CardDrawn> for DomainEvent {
    fn from(event: CardDrawn) -> Self {
        Self::CardDrawn(event)
    }
}

impl From<MulliganTaken> for DomainEvent {
    fn from(event: MulliganTaken) -> Self {
        Self::MulliganTaken(event)
    }
}
