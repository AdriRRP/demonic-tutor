use crate::domain::play::{
    commands::DrawCardEffectCommand,
    errors::DomainError,
    events::{CardDrawn, DrawKind, GameEnded},
    game::{invariants, model::Player, TerminalState},
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};

#[derive(Debug, Clone)]
pub enum DrawCardEffectOutcome {
    CardDrawn(CardDrawn),
    GameEnded(GameEnded),
}

pub(super) fn draw_one_card(player: &mut Player) -> Option<CardInstanceId> {
    let card = player.library_mut().draw_one()?;

    let card_id = card.id().clone();
    player.hand_mut().receive(vec![card]);
    Some(card_id)
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
        return crate::domain::play::game::rules::game_effects::end_game_for_empty_library_draw(
            game_id,
            players,
            terminal_state,
            &cmd.player_id,
        )
        .map(DrawCardEffectOutcome::GameEnded);
    };

    Ok(DrawCardEffectOutcome::CardDrawn(CardDrawn::new(
        game_id.clone(),
        cmd.player_id,
        card_id,
        DrawKind::ExplicitEffect,
    )))
}
