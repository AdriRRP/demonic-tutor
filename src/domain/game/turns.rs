use super::player::Player;
use super::Phase;
use crate::domain::{
    commands::AdvanceTurnCommand,
    errors::{DomainError, GameError},
    events::{CardDrawn, PhaseChanged, TurnAdvanced, TurnNumberChanged},
    ids::PlayerId,
};

/// Advances the turn to the next phase and player.
/// Also handles auto-untap at start and auto-draw in Draw phase.
///
/// # Errors
/// Returns an error if auto-draw fails (empty library).
pub fn advance_turn(
    players: &mut [Player],
    active_player: &mut PlayerId,
    phase: &mut Phase,
    turn_number: &mut u32,
    _cmd: AdvanceTurnCommand,
) -> Result<
    (
        TurnAdvanced,
        TurnNumberChanged,
        PhaseChanged,
        Option<CardDrawn>,
    ),
    DomainError,
> {
    let from_phase = *phase;
    let from_turn = *turn_number;

    let (to_phase, change_player, auto_draw) = match phase {
        Phase::Setup => (Phase::Untap, false, false),
        Phase::EndStep => (Phase::Untap, true, false),
        Phase::Untap => (Phase::Upkeep, false, false),
        Phase::Upkeep => (Phase::Draw, false, false),
        Phase::Draw => (Phase::FirstMain, false, true),
        Phase::FirstMain => (Phase::Combat, false, false),
        Phase::Combat => (Phase::SecondMain, false, false),
        Phase::SecondMain => (Phase::EndStep, false, false),
    };

    let mut card_drawn_event: Option<CardDrawn> = None;

    if change_player {
        let current_idx = players
            .iter()
            .position(|p| p.id() == active_player)
            .ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(
                    "active player should exist in player list".to_string(),
                ))
            })?;

        let next_idx = (current_idx + 1) % players.len();
        *active_player = players[next_idx].id().clone();
        *turn_number += 1;

        for player in players.iter_mut() {
            *player.lands_played_this_turn_mut() = 0;
            player
                .battlefield_mut()
                .cards_mut()
                .iter_mut()
                .for_each(|card| {
                    card.untap();
                    card.remove_summoning_sickness();
                    card.clear_damage();
                });
        }

        *phase = to_phase;

        return Ok((
            TurnAdvanced::new(
                super::Game::id_from_player_id(active_player),
                active_player.clone(),
            ),
            TurnNumberChanged::new(
                super::Game::id_from_player_id(active_player),
                from_turn,
                *turn_number,
            ),
            PhaseChanged::new(
                super::Game::id_from_player_id(active_player),
                from_phase,
                to_phase,
            ),
            card_drawn_event,
        ));
    }

    if auto_draw {
        let player_idx = players
            .iter()
            .position(|p| p.id() == active_player)
            .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(active_player.clone())))?;

        let player = &mut players[player_idx];
        let drawn_cards = player.library_mut().draw(1).ok_or_else(|| {
            DomainError::Game(GameError::NotEnoughCardsInLibrary {
                player: active_player.clone(),
                available: player.library().len(),
                requested: 1,
            })
        })?;
        let card = drawn_cards.into_iter().next().ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "draw(1) should return exactly one card".to_string(),
            ))
        })?;
        player.hand_mut().receive(vec![card.clone()]);
        card_drawn_event = Some(CardDrawn::new(
            super::Game::id_from_player_id(active_player),
            active_player.clone(),
            card.id().clone(),
        ));
    }

    *phase = to_phase;

    Ok((
        TurnAdvanced::new(
            super::Game::id_from_player_id(active_player),
            active_player.clone(),
        ),
        TurnNumberChanged::new(
            super::Game::id_from_player_id(active_player),
            from_turn,
            *turn_number,
        ),
        PhaseChanged::new(
            super::Game::id_from_player_id(active_player),
            from_phase,
            to_phase,
        ),
        card_drawn_event,
    ))
}
