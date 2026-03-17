use super::{
    draw_effects::draw_one_card,
    phase_behavior::{active_player_hand_size, clear_all_mana, get_phase_behavior, max_hand_size},
};
use crate::domain::play::{
    commands::AdvanceTurnCommand,
    errors::{DomainError, GameError},
    events::{CardDrawn, DrawKind, GameEnded, TurnProgressed},
    game::{invariants, model::Player, PriorityState, TerminalState},
    ids::{GameId, PlayerId},
    phase::Phase,
};

#[derive(Debug, Clone)]
pub enum AdvanceTurnOutcome {
    Progressed {
        turn_progressed: TurnProgressed,
        card_drawn: Option<CardDrawn>,
    },
    GameEnded(GameEnded),
}

pub struct TurnProgressionContext<'a> {
    pub game_id: &'a GameId,
    pub players: &'a mut [Player],
    pub active_player: &'a mut PlayerId,
    pub phase: &'a mut Phase,
    pub priority: &'a mut Option<PriorityState>,
    pub turn_number: &'a mut u32,
    pub terminal_state: &'a mut TerminalState,
}

fn rotate_to_next_player(
    players: &[Player],
    active_player: &mut PlayerId,
    turn_number: &mut u32,
) -> Result<(), DomainError> {
    let current_idx = players
        .iter()
        .position(|player| player.id() == active_player)
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "active player should exist in player list".to_string(),
            ))
        })?;

    let next_idx = (current_idx + 1) % players.len();
    *active_player = players[next_idx].id().clone();
    *turn_number += 1;
    Ok(())
}

fn auto_draw_card(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
) -> Result<Option<CardDrawn>, DomainError> {
    let player_idx = invariants::find_player_index(players, active_player)?;
    let Some(card_id) = draw_one_card(&mut players[player_idx]) else {
        return Ok(None);
    };

    Ok(Some(CardDrawn::new(
        game_id.clone(),
        active_player.clone(),
        card_id,
        DrawKind::TurnStep,
    )))
}

fn build_events(
    game_id: &GameId,
    active_player: &PlayerId,
    from_phase: Phase,
    to_phase: Phase,
    from_turn: u32,
    turn_number: u32,
    card_drawn_event: Option<CardDrawn>,
) -> (TurnProgressed, Option<CardDrawn>) {
    (
        TurnProgressed::new(
            game_id.clone(),
            active_player.clone(),
            from_turn,
            turn_number,
            from_phase,
            to_phase,
        ),
        card_drawn_event,
    )
}

const fn opens_priority_window(phase: Phase) -> bool {
    matches!(
        phase,
        Phase::Upkeep | Phase::FirstMain | Phase::Combat | Phase::SecondMain
    )
}

/// Advances the turn to the next phase and player.
///
/// # Errors
/// Returns an error if auto-draw fails.
pub fn advance_turn(
    ctx: TurnProgressionContext<'_>,
    _cmd: AdvanceTurnCommand,
) -> Result<AdvanceTurnOutcome, DomainError> {
    let TurnProgressionContext {
        game_id,
        players,
        active_player,
        phase,
        priority,
        turn_number,
        terminal_state,
    } = ctx;

    invariants::require_game_active(terminal_state.is_over())?;
    let from_phase = *phase;
    let from_turn = *turn_number;

    if matches!(from_phase, Phase::EndStep) {
        let hand_size = active_player_hand_size(players, active_player)?;
        if hand_size > max_hand_size() {
            return Err(DomainError::Game(GameError::HandSizeLimitExceeded {
                player: active_player.clone(),
                hand_size,
                max_hand_size: max_hand_size(),
            }));
        }
    }

    let current_phase_behavior = get_phase_behavior(from_phase);
    current_phase_behavior.on_exit(players, active_player)?;
    clear_all_mana(players);

    let to_phase = current_phase_behavior.next_phase();
    let to_phase_behavior = get_phase_behavior(to_phase);

    if current_phase_behavior.requires_player_change() {
        rotate_to_next_player(players, active_player, turn_number)?;
        *phase = to_phase;
        to_phase_behavior.on_enter(players, active_player)?;
        *priority = if opens_priority_window(to_phase) {
            Some(PriorityState::new(active_player.clone()))
        } else {
            None
        };

        let (turn_progressed, card_drawn) = build_events(
            game_id,
            active_player,
            from_phase,
            to_phase,
            from_turn,
            *turn_number,
            None,
        );

        return Ok(AdvanceTurnOutcome::Progressed {
            turn_progressed,
            card_drawn,
        });
    }

    let card_drawn_event = if current_phase_behavior.triggers_auto_draw() {
        auto_draw_card(game_id, players, active_player)?
    } else {
        None
    };

    if current_phase_behavior.triggers_auto_draw() && card_drawn_event.is_none() {
        return crate::domain::play::game::rules::game_effects::end_game_for_empty_library_draw(
            game_id,
            players,
            terminal_state,
            active_player,
        )
        .map(AdvanceTurnOutcome::GameEnded);
    }

    *phase = to_phase;
    to_phase_behavior.on_enter(players, active_player)?;
    *priority = if opens_priority_window(to_phase) {
        Some(PriorityState::new(active_player.clone()))
    } else {
        None
    };

    let (turn_progressed, card_drawn) = build_events(
        game_id,
        active_player,
        from_phase,
        to_phase,
        from_turn,
        *turn_number,
        card_drawn_event,
    );

    Ok(AdvanceTurnOutcome::Progressed {
        turn_progressed,
        card_drawn,
    })
}
