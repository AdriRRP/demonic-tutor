use crate::domain::play::{
    commands::DrawCardsEffectCommand,
    errors::{DomainError, GameError},
    events::{CardDrawn, DrawKind, GameEnded},
    game::{helpers, invariants, model::Player, TerminalState},
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};

#[derive(Debug, Clone)]
pub struct DrawCardsEffectOutcome {
    pub cards_drawn: Vec<CardDrawn>,
    pub game_ended: Option<GameEnded>,
}

impl DrawCardsEffectOutcome {
    #[must_use]
    pub const fn new(cards_drawn: Vec<CardDrawn>, game_ended: Option<GameEnded>) -> Self {
        Self {
            cards_drawn,
            game_ended,
        }
    }
}

pub(super) fn draw_one_card(player: &mut Player) -> Option<CardInstanceId> {
    let card = player.library_mut().draw_one()?;

    let card_id = card.id().clone();
    player.hand_mut().receive(vec![card]);
    Some(card_id)
}

/// Resolves an explicit draw effect by moving one or more cards from library to hand.
///
/// # Errors
/// Returns an error if:
/// - The caster is not the active player
/// - The phase is not valid for drawing
/// - The requested draw count is zero
pub fn draw_cards_effect(
    game_id: &GameId,
    players: &mut [Player],
    active_player: &PlayerId,
    phase: &Phase,
    terminal_state: &mut TerminalState,
    cmd: &DrawCardsEffectCommand,
) -> Result<DrawCardsEffectOutcome, DomainError> {
    invariants::require_game_active(terminal_state.is_over())?;
    invariants::require_active_player(active_player, &cmd.caster_id)?;

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(
            crate::domain::play::errors::PhaseError::InvalidForDraw { phase: *phase },
        ));
    }

    if cmd.draw_count == 0 {
        return Err(DomainError::Game(GameError::InvalidDrawCount(0)));
    }

    let target_player_idx = helpers::find_player_index(players, &cmd.target_player_id)?;
    let mut cards_drawn = Vec::new();

    for _ in 0..cmd.draw_count {
        let Some(card_id) = draw_one_card(&mut players[target_player_idx]) else {
            let game_ended =
                crate::domain::play::game::rules::game_effects::end_game_for_empty_library_draw(
                    game_id,
                    players,
                    terminal_state,
                    &cmd.target_player_id,
                )?;
            return Ok(DrawCardsEffectOutcome::new(cards_drawn, Some(game_ended)));
        };

        cards_drawn.push(CardDrawn::new(
            game_id.clone(),
            cmd.target_player_id.clone(),
            card_id,
            DrawKind::ExplicitEffect,
        ));
    }

    Ok(DrawCardsEffectOutcome::new(cards_drawn, None))
}
