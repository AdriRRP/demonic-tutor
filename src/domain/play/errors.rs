use crate::domain::play::{
    ids::{CardInstanceId, PlayerId},
    phase::Phase,
};

#[derive(Debug, PartialEq, Eq)]
pub enum DomainError {
    Game(GameError),
    Card(CardError),
    Phase(PhaseError),
    Player(PlayerError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameError {
    NotYourTurn {
        current: PlayerId,
        requested: PlayerId,
    },
    DuplicatePlayer(PlayerId),
    PlayerNotFound(PlayerId),
    DuplicateBlockerAssignment(CardInstanceId),
    MultipleBlockersPerAttackerNotSupported(CardInstanceId),
    InsufficientMana {
        player: PlayerId,
        required: u32,
        available: u32,
    },
    PriorityWindowOpen {
        current_holder: PlayerId,
    },
    OnlyInstantSpellsSupportedAsResponses(CardInstanceId),
    NoPriorityWindow,
    NotPriorityHolder {
        current: PlayerId,
        requested: PlayerId,
    },
    NotEnoughCardsInLibrary {
        player: PlayerId,
        available: usize,
        requested: usize,
    },
    MissingPlayerLibrary(PlayerId),
    DuplicatePlayerLibrary(PlayerId),
    OpeningHandsAlreadyDealt,
    GameAlreadyEnded,
    InvalidDrawCount(u32),
    MissingSpellTarget(CardInstanceId),
    SpellDoesNotUseTargets(CardInstanceId),
    InvalidCreatureTarget(CardInstanceId),
    NoAttackersDeclared,
    MulliganAlreadyUsed(PlayerId),
    HandSizeLimitExceeded {
        player: PlayerId,
        hand_size: usize,
        max_hand_size: usize,
    },
    DiscardNotRequired {
        player: PlayerId,
        hand_size: usize,
        max_hand_size: usize,
    },
    InternalInvariantViolation(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CardError {
    NotInHand {
        player: PlayerId,
        card: CardInstanceId,
    },
    NotALand(CardInstanceId),
    NotACreature(CardInstanceId),
    AlreadyTapped {
        player: PlayerId,
        card: CardInstanceId,
    },
    NotOnBattlefield {
        player: PlayerId,
        card: CardInstanceId,
    },
    CannotCastLand(CardInstanceId),
    CreatureHasSummoningSickness {
        player: PlayerId,
        card: CardInstanceId,
    },
    NotControlledBy {
        player: PlayerId,
        card: CardInstanceId,
    },
    NotAttacking(CardInstanceId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PhaseError {
    InvalidForLand,
    InvalidForPlayingCard {
        phase: Phase,
    },
    InvalidForDraw {
        phase: Phase,
    },
    InvalidForDiscard {
        phase: Phase,
    },
    InvalidForMulligan,
    InvalidForCombat,
    AlreadyPlayedLandThisTurn(PlayerId),
    NotDefendingPlayer {
        current: PlayerId,
        requested: PlayerId,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerError {
    NotEnoughPlayers { actual: usize },
    TooManyPlayers { actual: usize },
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Game(e) => write!(f, "{e}"),
            Self::Card(e) => write!(f, "{e}"),
            Self::Phase(e) => write!(f, "{e}"),
            Self::Player(e) => write!(f, "{e}"),
        }
    }
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotYourTurn { current, requested } => {
                write!(f, "not {requested}'s turn, it's {current}'s turn")
            }
            Self::DuplicatePlayer(pid) => write!(f, "duplicate player: {}", pid.as_str()),
            Self::PlayerNotFound(pid) => write!(f, "player not found: {}", pid.as_str()),
            Self::DuplicateBlockerAssignment(card_id) => {
                write!(f, "blocking creature {card_id} cannot be assigned more than once")
            }
            Self::MultipleBlockersPerAttackerNotSupported(card_id) => {
                write!(
                    f,
                    "attacking creature {card_id} cannot be assigned more than one blocker in the current combat model"
                )
            }
            Self::InsufficientMana {
                player,
                required,
                available,
            } => write!(
                f,
                "player {} has insufficient mana: required {required}, available {available}",
                player.as_str()
            ),
            Self::PriorityWindowOpen { current_holder } => write!(
                f,
                "a priority window is currently open and waiting on {current_holder}"
            ),
            Self::OnlyInstantSpellsSupportedAsResponses(card_id) => write!(
                f,
                "current stack timing only supports instant response spells; card {card_id} is not an instant"
            ),
            Self::NoPriorityWindow => write!(f, "no priority window is currently open"),
            Self::NotPriorityHolder { current, requested } => {
                write!(f, "not {requested}'s priority, current holder is {current}")
            }
            Self::NotEnoughCardsInLibrary {
                player,
                available,
                requested,
            } => write!(
                f,
                "not enough cards in library for player {}: have {available}, need {requested}",
                player.as_str()
            ),
            Self::MissingPlayerLibrary(pid) => write_player_library_error(f, "missing", pid),
            Self::DuplicatePlayerLibrary(pid) => write_player_library_error(f, "duplicate", pid),
            Self::OpeningHandsAlreadyDealt => write!(f, "opening hands have already been dealt"),
            Self::GameAlreadyEnded => write!(f, "the game has already ended"),
            Self::InvalidDrawCount(requested) => {
                write!(f, "draw effect must request at least one card, got {requested}")
            }
            Self::MissingSpellTarget(card_id) => {
                write!(f, "spell {card_id} requires an explicit target")
            }
            Self::SpellDoesNotUseTargets(card_id) => {
                write!(f, "spell {card_id} does not use explicit targets in the current model")
            }
            Self::InvalidCreatureTarget(card_id) => {
                write!(f, "creature target {card_id} is not on the battlefield")
            }
            Self::NoAttackersDeclared => write!(f, "no attackers have been declared"),
            Self::MulliganAlreadyUsed(pid) => {
                write!(f, "player {} has already used mulligan", pid.as_str())
            }
            Self::HandSizeLimitExceeded {
                player,
                hand_size,
                max_hand_size,
            } => write!(
                f,
                "player {} must discard down to {max_hand_size} cards before the turn can end (currently {hand_size})",
                player.as_str()
            ),
            Self::DiscardNotRequired {
                player,
                hand_size,
                max_hand_size,
            } => write!(
                f,
                "player {} cannot discard for cleanup at hand size {hand_size}; maximum is {max_hand_size}",
                player.as_str()
            ),
            Self::InternalInvariantViolation(msg) => {
                write!(f, "internal invariant violated: {msg}")
            }
        }
    }
}

fn write_player_library_error(
    f: &mut std::fmt::Formatter<'_>,
    adjective: &str,
    pid: &PlayerId,
) -> std::fmt::Result {
    write!(
        f,
        "{adjective} opening-hand library for player {}",
        pid.as_str()
    )
}

impl std::fmt::Display for CardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInHand { player, card } => {
                write!(f, "card {card} not in hand of player {player}")
            }
            Self::NotALand(card) => write!(f, "card {card} is not a land"),
            Self::NotACreature(card) => write!(f, "card {card} is not a creature"),
            Self::AlreadyTapped { player, card } => {
                write!(f, "card {card} is already tapped for player {player}")
            }
            Self::NotOnBattlefield { player, card } => {
                write!(f, "card {card} not on battlefield for player {player}")
            }
            Self::CannotCastLand(card) => write!(f, "cannot cast land {card} as a spell"),
            Self::CreatureHasSummoningSickness { player: _, card } => {
                write!(
                    f,
                    "creature {card} has summoning sickness and cannot attack"
                )
            }
            Self::NotControlledBy { player, card } => {
                write!(f, "creature {card} is not controlled by player {player}")
            }
            Self::NotAttacking(card) => write!(f, "creature {card} is not an attacking creature"),
        }
    }
}

impl std::fmt::Display for PhaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidForLand => write!(f, "cannot play land in current phase"),
            Self::InvalidForPlayingCard { phase } => {
                write!(f, "cannot play card in phase {phase}")
            }
            Self::InvalidForDraw { phase } => write!(f, "cannot draw card in phase {phase}"),
            Self::InvalidForDiscard { phase } => {
                write!(f, "cannot discard card in phase {phase}")
            }
            Self::InvalidForMulligan => write!(f, "cannot perform mulligan in current phase"),
            Self::InvalidForCombat => {
                write!(
                    f,
                    "cannot declare attackers, declare blockers, or resolve combat damage in current phase"
                )
            }
            Self::AlreadyPlayedLandThisTurn(pid) => {
                write!(f, "player {pid} already played a land this turn")
            }
            Self::NotDefendingPlayer { current, requested } => {
                write!(f, "not {requested}'s turn to block, it's {current}'s turn")
            }
        }
    }
}

impl std::fmt::Display for PlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughPlayers { actual } => {
                write!(f, "not enough players: expected at least 2, got {actual}")
            }
            Self::TooManyPlayers { actual } => {
                write!(f, "too many players: expected at most 2, got {actual}")
            }
        }
    }
}

impl std::error::Error for DomainError {}

impl From<GameError> for DomainError {
    fn from(e: GameError) -> Self {
        Self::Game(e)
    }
}

impl From<CardError> for DomainError {
    fn from(e: CardError) -> Self {
        Self::Card(e)
    }
}

impl From<PhaseError> for DomainError {
    fn from(e: PhaseError) -> Self {
        Self::Phase(e)
    }
}

impl From<PlayerError> for DomainError {
    fn from(e: PlayerError) -> Self {
        Self::Player(e)
    }
}
