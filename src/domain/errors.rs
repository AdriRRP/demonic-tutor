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
    NotACreature {
        card_id: CardInstanceId,
    },
    NotYourTurn {
        current_player: PlayerId,
        requested_player: PlayerId,
    },
    InvalidPhaseForLand,
    InvalidPhaseForPlayingCard {
        phase: Phase,
    },
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
    CannotCastLand {
        card_id: CardInstanceId,
    },
    InsufficientMana {
        player_id: PlayerId,
        required: u32,
        available: u32,
    },
    MulliganAlreadyUsed {
        player_id: PlayerId,
    },
    InvalidPhaseForMulligan,
    InvalidPhaseForCombat,
    CreatureAlreadyTapped {
        player_id: PlayerId,
        card_id: CardInstanceId,
    },
    CreatureHasSummoningSickness {
        player_id: PlayerId,
        card_id: CardInstanceId,
    },
    CreatureNotControlledByAttacker {
        player_id: PlayerId,
        card_id: CardInstanceId,
    },
    NotACreatureForAttack {
        card_id: CardInstanceId,
    },
    InternalInvariantViolation {
        message: String,
    },
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::NotEnoughPlayers { actual } => {
                return write!(f, "not enough players: expected at least 2, got {actual}")
            }
            Self::TooManyPlayers { actual } => {
                return write!(f, "too many players: expected at most 2, got {actual}")
            }
            Self::DuplicatePlayer(pid) => return write!(f, "duplicate player: {}", pid.0),
            Self::PlayerNotFound(pid) => return write!(f, "player not found: {}", pid.0),
            Self::NotEnoughCardsInLibrary {
                player_id,
                available,
                requested,
            } => {
                return write!(
                    f,
                    "not enough cards in library for player {}: have {available}, need {requested}",
                    player_id.0
                )
            }
            Self::CardNotInHand { player_id, card_id } => {
                return write!(f, "card {card_id} not in hand of player {player_id}")
            }
            Self::NotALand { card_id } => return write!(f, "card {card_id} is not a land"),
            Self::NotACreature { card_id } => return write!(f, "card {card_id} is not a creature"),
            Self::NotYourTurn {
                current_player,
                requested_player,
            } => {
                return write!(
                    f,
                    "not {requested_player}'s turn, it's {current_player}'s turn"
                )
            }
            Self::InvalidPhaseForLand => "cannot play land in current phase",
            Self::InvalidPhaseForPlayingCard { phase } => {
                return write!(f, "cannot play card in phase {phase:?}")
            }
            Self::InvalidPhaseForDraw { phase } => {
                return write!(f, "cannot draw card in phase {phase:?}")
            }
            Self::AlreadyPlayedLandThisTurn { player_id } => {
                return write!(f, "player {player_id} already played a land this turn")
            }
            Self::CardAlreadyTapped { player_id, card_id } => {
                return write!(f, "card {card_id} is already tapped for player {player_id}")
            }
            Self::CardNotOnBattlefield { player_id, card_id } => {
                return write!(
                    f,
                    "card {card_id} not on battlefield for player {player_id}"
                )
            }
            Self::CannotCastLand { card_id } => {
                return write!(f, "cannot cast land {card_id} as a spell")
            }
            Self::InsufficientMana {
                player_id,
                required,
                available,
            } => {
                return write!(
                    f,
                    "player {} has insufficient mana: required {required}, available {available}",
                    player_id.0
                )
            }
            Self::MulliganAlreadyUsed { player_id } => {
                return write!(f, "player {player_id} has already used mulligan")
            }
            Self::InvalidPhaseForMulligan => "cannot perform mulligan in current phase",
            Self::InvalidPhaseForCombat => "cannot declare attackers in current phase",
            Self::CreatureAlreadyTapped { player_id, card_id } => {
                return write!(
                    f,
                    "creature {card_id} is already tapped for player {player_id}"
                )
            }
            Self::CreatureHasSummoningSickness { card_id, .. } => {
                return write!(
                    f,
                    "creature {card_id} has summoning sickness and cannot attack"
                )
            }
            Self::CreatureNotControlledByAttacker { player_id, card_id } => {
                return write!(
                    f,
                    "creature {card_id} is not controlled by player {player_id}"
                )
            }
            Self::NotACreatureForAttack { card_id } => {
                return write!(f, "card {card_id} is not a creature and cannot attack")
            }
            Self::InternalInvariantViolation { message } => {
                return write!(f, "internal invariant violated: {message}")
            }
        };
        write!(f, "{s}")
    }
}

impl std::error::Error for DomainError {}
