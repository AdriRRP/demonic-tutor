use super::super::{invariants, model::Player};
use crate::domain::play::{
    commands::{AdvanceTurnCommand, DrawCardEffectCommand},
    errors::{DomainError, GameError},
    events::{CardDrawn, DrawKind, TurnProgressed},
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};

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

fn draw_one_card(player: &mut Player, player_id: &PlayerId) -> Result<CardInstanceId, DomainError> {
    let card = player.library_mut().draw_one().ok_or_else(|| {
        DomainError::Game(GameError::NotEnoughCardsInLibrary {
            player: player_id.clone(),
            available: player.library().len(),
            requested: 1,
        })
    })?;

    let card_id = card.id().clone();
    player.hand_mut().receive(vec![card]);
    Ok(card_id)
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
    let card_id = draw_one_card(&mut players[player_idx], active_player)?;

    Ok(Some(CardDrawn::new(
        game_id.clone(),
        active_player.clone(),
        card_id,
        DrawKind::TurnStep,
    )))
}

fn clear_all_mana(players: &mut [Player]) {
    for player in players {
        player.clear_mana();
    }
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
    cmd: DrawCardEffectCommand,
) -> Result<CardDrawn, DomainError> {
    invariants::require_active_player(active_player, &cmd.player_id)?;

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(
            crate::domain::play::errors::PhaseError::InvalidForDraw { phase: *phase },
        ));
    }

    let player_idx = invariants::find_player_index(players, &cmd.player_id)?;
    let card_id = draw_one_card(&mut players[player_idx], &cmd.player_id)?;

    Ok(CardDrawn::new(
        game_id.clone(),
        cmd.player_id,
        card_id,
        DrawKind::ExplicitEffect,
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
    _cmd: AdvanceTurnCommand,
) -> Result<(TurnProgressed, Option<CardDrawn>), DomainError> {
    let from_phase = *phase;
    let from_turn = *turn_number;

    let current_phase_behavior = get_phase_behavior(from_phase);
    current_phase_behavior.on_exit(players, active_player)?;
    clear_all_mana(players);

    let to_phase = current_phase_behavior.next_phase();
    let to_phase_behavior = get_phase_behavior(to_phase);

    if current_phase_behavior.requires_player_change() {
        rotate_to_next_player(players, active_player, turn_number)?;
        *phase = to_phase;
        to_phase_behavior.on_enter(players, active_player)?;

        return Ok(build_events(
            game_id,
            active_player,
            from_phase,
            to_phase,
            from_turn,
            *turn_number,
            None,
        ));
    }

    let card_drawn_event = if current_phase_behavior.triggers_auto_draw() {
        auto_draw_card(game_id, players, active_player)?
    } else {
        None
    };

    *phase = to_phase;
    to_phase_behavior.on_enter(players, active_player)?;

    Ok(build_events(
        game_id,
        active_player,
        from_phase,
        to_phase,
        from_turn,
        *turn_number,
        card_drawn_event,
    ))
}
