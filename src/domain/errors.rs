use crate::domain::{
    game::Phase,
    ids::{CardInstanceId, PlayerId},
};

#[derive(Debug, PartialEq, Eq)]
pub enum DomainError {
    NotEnoughPlayers {
        actual: usize,
    },
    TooManyPlayers {
        actual: usize,
    },
    DuplicatePlayer(PlayerId),
    PlayerNotFound(PlayerId),
    NotEnoughCardsInLibrary {
        player_id: PlayerId,
        available: usize,
        requested: usize,
    },
    CardNotInHand {
        player_id: PlayerId,
        card_id: CardInstanceId,
    },
    NotALand {
        card_id: CardInstanceId,
    },
    NotYourTurn {
        current_player: PlayerId,
        requested_player: PlayerId,
    },
    InvalidPhaseForLand,
    InvalidPhaseForDraw {
        phase: Phase,
    },
    AlreadyPlayedLandThisTurn {
        player_id: PlayerId,
    },
    CardAlreadyTapped {
        player_id: PlayerId,
        card_id: CardInstanceId,
    },
    CardNotOnBattlefield {
        player_id: PlayerId,
        card_id: CardInstanceId,
    },
    MulliganAlreadyUsed {
        player_id: PlayerId,
    },
    InvalidPhaseForMulligan,
    InternalInvariantViolation {
        message: String,
    },
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughPlayers { actual } => {
                write!(f, "not enough players: expected at least 2, got {actual}")
            }
            Self::TooManyPlayers { actual } => {
                write!(f, "too many players: expected at most 2, got {actual}")
            }
            Self::DuplicatePlayer(pid) => {
                write!(f, "duplicate player: {}", pid.0)
            }
            Self::PlayerNotFound(pid) => {
                write!(f, "player not found: {}", pid.0)
            }
            Self::NotEnoughCardsInLibrary {
                player_id,
                available,
                requested,
            } => {
                write!(
                    f,
                    "not enough cards in library for player {}: have {available}, need {requested}",
                    player_id.0
                )
            }
            Self::CardNotInHand { player_id, card_id } => {
                write!(f, "card {card_id} not in hand of player {player_id}")
            }
            Self::NotALand { card_id } => {
                write!(f, "card {card_id} is not a land")
            }
            Self::NotYourTurn {
                current_player,
                requested_player,
            } => {
                write!(
                    f,
                    "not {requested_player}'s turn, it's {current_player}'s turn"
                )
            }
            Self::InvalidPhaseForLand => {
                write!(f, "cannot play land in current phase")
            }
            Self::InvalidPhaseForDraw { phase } => {
                write!(f, "cannot draw card in phase {phase:?}")
            }
            Self::AlreadyPlayedLandThisTurn { player_id } => {
                write!(f, "player {player_id} already played a land this turn")
            }
            Self::CardAlreadyTapped { player_id, card_id } => {
                write!(f, "card {card_id} is already tapped for player {player_id}")
            }
            Self::CardNotOnBattlefield { player_id, card_id } => {
                write!(
                    f,
                    "card {card_id} not on battlefield for player {player_id}"
                )
            }
            Self::MulliganAlreadyUsed { player_id } => {
                write!(f, "player {player_id} has already used mulligan")
            }
            Self::InvalidPhaseForMulligan => {
                write!(f, "cannot perform mulligan in current phase")
            }
            Self::InternalInvariantViolation { message } => {
                write!(f, "internal invariant violated: {message}")
            }
        }
    }
}

impl std::error::Error for DomainError {}
