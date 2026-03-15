use super::player::Player;
use super::Phase;
use crate::domain::{
    commands::AdvanceTurnCommand,
    errors::{DomainError, GameError},
    events::{PhaseChanged, TurnAdvanced, TurnNumberChanged},
    ids::PlayerId,
};

pub fn advance_turn(
    players: &mut [Player],
    active_player: &mut PlayerId,
    phase: &mut Phase,
    turn_number: &mut u32,
    _cmd: AdvanceTurnCommand,
) -> Result<(TurnAdvanced, TurnNumberChanged, PhaseChanged), DomainError> {
    let from_phase = *phase;

    let from_turn = *turn_number;
    let (to_phase, change_player) = match phase {
        Phase::Setup | Phase::Ending => (Phase::Main, true),
        Phase::Main => (Phase::Ending, false),
    };

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
            for card in player.battlefield_mut().cards_mut() {
                card.remove_summoning_sickness();
            }
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
        ));
    }

    *phase = to_phase;
    let from_turn = *turn_number;

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
    ))
}
