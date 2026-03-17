use super::super::model::MAX_HAND_SIZE;
use super::super::{invariants, model::Player, TerminalState};
use crate::domain::play::{
    commands::{AdvanceTurnCommand, DiscardForCleanupCommand, DrawCardEffectCommand},
    errors::{DomainError, GameError, PhaseError},
    events::{
        CardDiscarded, CardDrawn, DiscardKind, DrawKind, GameEndReason, GameEnded, TurnProgressed,
    },
    ids::{CardInstanceId, GameId, PlayerId},
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

#[derive(Debug, Clone)]
pub enum DrawCardEffectOutcome {
    CardDrawn(CardDrawn),
    GameEnded(GameEnded),
}

trait PhaseBehavior {
    fn next_phase(&self) -> Phase;
    fn requires_player_change(&self) -> bool;
    fn triggers_auto_draw(&self) -> bool;

    fn on_enter(
        &self,
        _players: &mut [Player],
        _active_player: &PlayerId,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    fn on_exit(
        &self,
        _players: &mut [Player],
        _active_player: &PlayerId,
    ) -> Result<(), DomainError> {
        Ok(())
    }
}

struct SetupPhase;
struct UntapPhase;
struct UpkeepPhase;
struct DrawPhase;
struct FirstMainPhase;
struct CombatPhase;
struct SecondMainPhase;
struct EndStepPhase;

impl PhaseBehavior for SetupPhase {
    fn next_phase(&self) -> Phase {
        Phase::Untap
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for UntapPhase {
    fn next_phase(&self) -> Phase {
        Phase::Upkeep
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }

    fn on_enter(
        &self,
        players: &mut [Player],
        active_player: &PlayerId,
    ) -> Result<(), DomainError> {
        let player = invariants::find_player_mut(players, active_player)?;
        player.battlefield_mut().iter_mut().for_each(|card| {
            card.untap();
            card.remove_summoning_sickness();
        });
        player.reset_lands_played();
        Ok(())
    }
}

impl PhaseBehavior for UpkeepPhase {
    fn next_phase(&self) -> Phase {
        Phase::Draw
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for DrawPhase {
    fn next_phase(&self) -> Phase {
        Phase::FirstMain
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        true
    }
}

impl PhaseBehavior for FirstMainPhase {
    fn next_phase(&self) -> Phase {
        Phase::Combat
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for CombatPhase {
    fn next_phase(&self) -> Phase {
        Phase::SecondMain
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for SecondMainPhase {
    fn next_phase(&self) -> Phase {
        Phase::EndStep
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for EndStepPhase {
    fn next_phase(&self) -> Phase {
        Phase::Untap
    }
    fn requires_player_change(&self) -> bool {
        true
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }

    fn on_exit(
        &self,
        players: &mut [Player],
        _active_player: &PlayerId,
    ) -> Result<(), DomainError> {
        for player in players.iter_mut() {
            player
                .battlefield_mut()
                .iter_mut()
                .for_each(crate::domain::play::cards::CardInstance::clear_damage);
        }
        Ok(())
    }
}

fn get_phase_behavior(phase: Phase) -> &'static dyn PhaseBehavior {
    match phase {
        Phase::Setup => &SetupPhase,
        Phase::Untap => &UntapPhase,
        Phase::Upkeep => &UpkeepPhase,
        Phase::Draw => &DrawPhase,
        Phase::FirstMain => &FirstMainPhase,
        Phase::Combat => &CombatPhase,
        Phase::SecondMain => &SecondMainPhase,
        Phase::EndStep => &EndStepPhase,
    }
}

fn draw_one_card(player: &mut Player) -> Option<CardInstanceId> {
    let card = player.library_mut().draw_one()?;

    let card_id = card.id().clone();
    player.hand_mut().receive(vec![card]);
    Some(card_id)
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

fn game_ended_for_empty_library_draw(
    game_id: &GameId,
    players: &[Player],
    terminal_state: &mut TerminalState,
    losing_player: &PlayerId,
) -> Result<GameEnded, DomainError> {
    let winning_player = invariants::opposing_player_id(players, losing_player)?;
    terminal_state.end(
        winning_player.clone(),
        losing_player.clone(),
        GameEndReason::EmptyLibraryDraw,
    );

    Ok(GameEnded::new(
        game_id.clone(),
        winning_player,
        losing_player.clone(),
        GameEndReason::EmptyLibraryDraw,
    ))
}

fn clear_all_mana(players: &mut [Player]) {
    for player in players {
        player.clear_mana();
    }
}

fn active_player_hand_size(
    players: &[Player],
    active_player: &PlayerId,
) -> Result<usize, DomainError> {
    let player_idx = invariants::find_player_index(players, active_player)?;
    Ok(players[player_idx].hand_size())
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

/// Resolves an explicit draw effect by moving one card from library to hand.
///
/// # Errors
/// Returns an error if:
/// - The player is not the active player
/// - The phase is not valid for drawing
/// - The player has no cards in their library
pub fn draw_card_effect(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    terminal_state: &mut TerminalState,
    cmd: DrawCardEffectCommand,
) -> Result<DrawCardEffectOutcome, DomainError> {
    invariants::require_game_active(terminal_state.is_over())?;
    invariants::require_active_player(active_player, &cmd.player_id)?;

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(
            crate::domain::play::errors::PhaseError::InvalidForDraw { phase: *phase },
        ));
    }

    let player_idx = invariants::find_player_index(players, &cmd.player_id)?;
    let Some(card_id) = draw_one_card(&mut players[player_idx]) else {
        return game_ended_for_empty_library_draw(game_id, players, terminal_state, &cmd.player_id)
            .map(DrawCardEffectOutcome::GameEnded);
    };

    Ok(DrawCardEffectOutcome::CardDrawn(CardDrawn::new(
        game_id.clone(),
        cmd.player_id,
        card_id,
        DrawKind::ExplicitEffect,
    )))
}

/// Discards one card from hand to graveyard as an explicit cleanup action.
///
/// # Errors
/// Returns an error if:
/// - The player is not the active player
/// - The phase is not `EndStep`
/// - The player is not above the maximum hand size
/// - The card is not in the player's hand
pub fn discard_for_cleanup(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    cmd: DiscardForCleanupCommand,
) -> Result<CardDiscarded, DomainError> {
    invariants::require_active_player(active_player, &cmd.player_id)?;

    if !matches!(phase, Phase::EndStep) {
        return Err(DomainError::Phase(PhaseError::InvalidForDiscard {
            phase: *phase,
        }));
    }

    let player = invariants::find_player_mut(players, &cmd.player_id)?;
    let hand_size = player.hand_size();
    if hand_size <= MAX_HAND_SIZE {
        return Err(DomainError::Game(GameError::DiscardNotRequired {
            player: cmd.player_id.clone(),
            hand_size,
            max_hand_size: MAX_HAND_SIZE,
        }));
    }

    let card = invariants::remove_card_from_hand(player, &cmd.player_id, &cmd.card_id)?;
    let card_id = card.id().clone();
    player.graveyard_mut().add(card);

    Ok(CardDiscarded::new(
        game_id.clone(),
        cmd.player_id,
        card_id,
        DiscardKind::CleanupHandSize,
    ))
}

/// Advances the turn to the next phase and player.
///
/// # Errors
/// Returns an error if auto-draw fails.
pub fn advance_turn(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &mut PlayerId,
    phase: &mut Phase,
    turn_number: &mut u32,
    terminal_state: &mut TerminalState,
    _cmd: AdvanceTurnCommand,
) -> Result<AdvanceTurnOutcome, DomainError> {
    invariants::require_game_active(terminal_state.is_over())?;
    let from_phase = *phase;
    let from_turn = *turn_number;

    if matches!(from_phase, Phase::EndStep) {
        let hand_size = active_player_hand_size(players, active_player)?;
        if hand_size > MAX_HAND_SIZE {
            return Err(DomainError::Game(GameError::HandSizeLimitExceeded {
                player: active_player.clone(),
                hand_size,
                max_hand_size: MAX_HAND_SIZE,
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
        return game_ended_for_empty_library_draw(game_id, players, terminal_state, active_player)
            .map(AdvanceTurnOutcome::GameEnded);
    }

    *phase = to_phase;
    to_phase_behavior.on_enter(players, active_player)?;

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
