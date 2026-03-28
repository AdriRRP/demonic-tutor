//! Supports game rules lifecycle.

use {
    super::super::{
        model::{Player, OPENING_HAND_SIZE},
        Game, TerminalState,
    },
    crate::domain::play::{
        cards::CardInstance,
        commands::{DealOpeningHandsCommand, MulliganCommand, PlayerLibrary, StartGameCommand},
        errors::{DomainError, GameError, PlayerError},
        events::{GameStarted, MulliganTaken, OpeningHandDealt},
        ids::{CardInstanceId, GameId, PlayerId},
        phase::Phase,
    },
    std::collections::HashSet,
};

type RuntimeLibrary = (PlayerId, Vec<CardInstance>);

const fn validate_player_count(player_count: usize) -> Result<(), DomainError> {
    if player_count < 2 {
        return Err(DomainError::Player(PlayerError::NotEnoughPlayers {
            actual: player_count,
        }));
    }

    if player_count > 2 {
        return Err(DomainError::Player(PlayerError::TooManyPlayers {
            actual: player_count,
        }));
    }

    Ok(())
}

fn build_players_and_ids(
    cmd: &StartGameCommand,
) -> Result<(Vec<Player>, Vec<PlayerId>), DomainError> {
    let mut seen_players = HashSet::new();
    let mut players = Vec::with_capacity(cmd.players.len());
    let mut player_ids = Vec::with_capacity(cmd.players.len());

    for player_deck in &cmd.players {
        if !seen_players.insert(player_deck.player_id.clone()) {
            return Err(DomainError::Game(GameError::DuplicatePlayer(
                player_deck.player_id.clone(),
            )));
        }

        players.push(Player::new(player_deck.player_id.clone()));
        player_ids.push(player_deck.player_id.clone());
    }

    Ok((players, player_ids))
}

fn first_player_id(player_ids: &[PlayerId]) -> Result<PlayerId, DomainError> {
    player_ids.first().cloned().ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "player list should not be empty after validation".to_string(),
        ))
    })
}

fn validate_player_exists(players: &[Player], player_id: &PlayerId) -> Result<(), DomainError> {
    if players.iter().any(|player| player.id() == player_id) {
        Ok(())
    } else {
        Err(DomainError::Game(GameError::PlayerNotFound(
            player_id.clone(),
        )))
    }
}

fn validate_opening_hand_size(player_library: &PlayerLibrary) -> Result<(), DomainError> {
    if player_library.cards.len() < OPENING_HAND_SIZE {
        Err(DomainError::Game(GameError::NotEnoughCardsInLibrary {
            player: player_library.player_id.clone(),
            available: player_library.cards.len(),
            requested: OPENING_HAND_SIZE,
        }))
    } else {
        Ok(())
    }
}

fn validate_curated_library_card_profiles(
    player_library: &PlayerLibrary,
) -> Result<(), DomainError> {
    for card in &player_library.cards {
        if card.supported_limited_set_profile().is_none() {
            return Err(DomainError::Game(
                GameError::UnsupportedCuratedCardProfile {
                    player: player_library.player_id.clone(),
                    definition: card.definition_id().clone(),
                },
            ));
        }
    }

    Ok(())
}

fn validate_player_libraries(
    players: &[Player],
    cmd: &DealOpeningHandsCommand,
) -> Result<(), DomainError> {
    let mut seen_players = HashSet::new();

    for player_library in &cmd.player_libraries {
        if !seen_players.insert(player_library.player_id.clone()) {
            return Err(DomainError::Game(GameError::DuplicatePlayerLibrary(
                player_library.player_id.clone(),
            )));
        }
        validate_player_exists(players, &player_library.player_id)?;
        validate_opening_hand_size(player_library)?;
        validate_curated_library_card_profiles(player_library)?;
    }

    if cmd.player_libraries.len() != players.len() {
        for player in players {
            if !seen_players.contains(player.id()) {
                return Err(DomainError::Game(GameError::MissingPlayerLibrary(
                    player.id().clone(),
                )));
            }
        }
    }

    Ok(())
}

fn require_opening_hands_not_dealt(players: &[Player]) -> Result<(), DomainError> {
    if players.iter().any(|player| {
        !player.hand_is_empty()
            || player.library_size() > 0
            || !player.battlefield_is_empty()
            || !player.graveyard_is_empty()
    }) {
        Err(DomainError::Game(GameError::OpeningHandsAlreadyDealt))
    } else {
        Ok(())
    }
}

fn find_player_index(players: &[Player], player_id: &PlayerId) -> Result<usize, DomainError> {
    players
        .iter()
        .position(|player| player.id() == player_id)
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "player {} should exist after validation",
                player_id.as_str()
            )))
        })
}

fn build_library_cards(game_id: &GameId, player_library: &PlayerLibrary) -> Vec<CardInstance> {
    player_library
        .cards
        .iter()
        .enumerate()
        .map(|(index, card)| {
            card.to_card_instance(CardInstanceId::new(format!(
                "{}-{}-{}",
                game_id.as_str(),
                player_library.player_id.as_str(),
                index
            )))
        })
        .collect()
}

fn build_runtime_libraries(game_id: &GameId, cmd: &DealOpeningHandsCommand) -> Vec<RuntimeLibrary> {
    cmd.player_libraries
        .iter()
        .map(|player_library| {
            (
                player_library.player_id.clone(),
                build_library_cards(game_id, player_library),
            )
        })
        .collect()
}

fn draw_opening_hand(
    player: &mut Player,
    player_id: &PlayerId,
) -> Result<Vec<CardInstanceId>, DomainError> {
    player
        .draw_cards_into_hand(OPENING_HAND_SIZE)
        .ok_or_else(|| {
            DomainError::Game(GameError::NotEnoughCardsInLibrary {
                player: player_id.clone(),
                available: player.library_size(),
                requested: OPENING_HAND_SIZE,
            })
        })?;

    Ok(player.hand_card_ids())
}

/// Starts a new game with the given players.
///
/// # Errors
/// Returns an error if:
/// - Fewer than 2 players are provided
/// - More than 2 players are provided
/// - There are duplicate players
pub fn start(cmd: StartGameCommand) -> Result<(Game, GameStarted), DomainError> {
    let player_count = cmd.players.len();
    validate_player_count(player_count)?;
    let (players, player_ids) = build_players_and_ids(&cmd)?;

    let game_started = GameStarted::new(cmd.game_id.clone(), player_ids.clone());
    let active_player = first_player_id(&player_ids)?;

    let game = Game::new(
        cmd.game_id,
        &active_player,
        Phase::Setup,
        1,
        players,
        TerminalState::active(),
    )?;

    Ok((game, game_started))
}

/// Deals opening hands to all players.
///
/// # Errors
/// Returns an error if:
/// - A player is not found
/// - A player does not have enough cards in their library
pub fn deal_opening_hands(
    players: &mut [Player],
    cmd: &DealOpeningHandsCommand,
    game_id: &GameId,
) -> Result<Vec<OpeningHandDealt>, DomainError> {
    require_opening_hands_not_dealt(players)?;
    validate_player_libraries(players, cmd)?;
    let runtime_libraries = build_runtime_libraries(game_id, cmd);

    let mut events: Vec<OpeningHandDealt> = Vec::new();

    for (player_id, runtime_cards) in runtime_libraries {
        let player_index = find_player_index(players, &player_id)?;
        let player = &mut players[player_index];
        player.receive_library_cards(runtime_cards);
        let hand_cards = draw_opening_hand(player, &player_id)?;
        events.push(OpeningHandDealt::new(
            game_id.clone(),
            player_id,
            hand_cards,
        ));
    }

    Ok(events)
}

/// Performs a mulligan, shuffling hand back into library and drawing new hand.
///
/// # Errors
/// Returns an error if:
/// - The phase is not Setup
/// - The player has already used mulligan
/// - The player does not have enough cards in library
pub fn mulligan(
    game_id: &GameId,
    players: &mut [Player],
    _active_player: &PlayerId,
    phase: &Phase,
    cmd: MulliganCommand,
) -> Result<MulliganTaken, DomainError> {
    if !matches!(phase, Phase::Setup) {
        return Err(DomainError::Phase(
            crate::domain::play::errors::PhaseError::InvalidForMulligan,
        ));
    }

    let player = super::super::helpers::find_player_mut(players, &cmd.player_id)?;

    if player.mulligan_used() {
        return Err(DomainError::Game(GameError::MulliganAlreadyUsed(
            cmd.player_id,
        )));
    }

    if player.library_size() < OPENING_HAND_SIZE {
        return Err(DomainError::Game(GameError::NotEnoughCardsInLibrary {
            player: cmd.player_id,
            available: player.library_size(),
            requested: OPENING_HAND_SIZE,
        }));
    }

    player.recycle_hand_into_library();
    player.shuffle_library();

    player
        .draw_cards_into_hand(OPENING_HAND_SIZE)
        .ok_or_else(|| {
            DomainError::Game(GameError::NotEnoughCardsInLibrary {
                player: cmd.player_id.clone(),
                available: player.library_size(),
                requested: OPENING_HAND_SIZE,
            })
        })?;
    player.use_mulligan();

    Ok(MulliganTaken::new(game_id.clone(), cmd.player_id))
}
