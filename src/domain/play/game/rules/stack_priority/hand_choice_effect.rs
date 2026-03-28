//! Supports rules stack priority pending hand-choice effects.

use crate::domain::play::{
    commands::ResolvePendingHandChoiceCommand,
    errors::{DomainError, GameError},
    events::{CardDiscarded, CardDrawn, DiscardKind, DrawKind, GameEnded},
    game::{model::PlayerCardZone, PendingDecision, PendingHandChoiceKind, PriorityState},
    ids::PlayerCardHandle,
};

use super::{
    deferred_resolution::{
        build_spell_resolution_events_from_parts, move_spell_to_resolution_destination,
        remove_pending_spell,
    },
    ResolvePendingHandChoiceOutcome, StackPriorityContext,
};

const fn restore_pending_hand_choice(
    pending_decision: &mut Option<PendingDecision>,
    controller_index: usize,
    stack_object_number: u32,
    kind: PendingHandChoiceKind,
) {
    *pending_decision = Some(PendingDecision::hand_choice(
        controller_index,
        stack_object_number,
        kind,
    ));
}

pub(super) fn draw_cards_for_pending_effect(
    game_id: &crate::domain::play::ids::GameId,
    players: &mut [crate::domain::play::game::Player],
    terminal_state: &mut crate::domain::play::game::TerminalState,
    controller_index: usize,
    draw_count: u32,
) -> Result<(Vec<CardDrawn>, Option<GameEnded>), DomainError> {
    let controller_id = players[controller_index].id().clone();
    let mut cards_drawn = Vec::new();

    for _ in 0..draw_count {
        let Some(card_id) = players[controller_index].draw_one_into_hand() else {
            let game_ended =
                crate::domain::play::game::rules::game_effects::end_game_for_empty_library_draw(
                    game_id,
                    players,
                    terminal_state,
                    controller_index,
                )?;
            return Ok((cards_drawn, Some(game_ended)));
        };

        cards_drawn.push(CardDrawn::new(
            game_id.clone(),
            controller_id.clone(),
            card_id,
            DrawKind::ExplicitEffect,
        ));
    }

    Ok((cards_drawn, None))
}

fn validate_pending_hand_choice(
    players: &[crate::domain::play::game::Player],
    pending_decision: &mut Option<PendingDecision>,
    controller_index: usize,
    stack_object_number: u32,
    kind: PendingHandChoiceKind,
    player_id: &crate::domain::play::ids::PlayerId,
    chosen_card_id: &crate::domain::play::ids::CardInstanceId,
) -> Result<PlayerCardHandle, DomainError> {
    let current_controller = players[controller_index].id().clone();
    if current_controller != *player_id {
        restore_pending_hand_choice(
            pending_decision,
            controller_index,
            stack_object_number,
            kind,
        );
        return Err(DomainError::Game(
            GameError::NotPendingHandChoiceController {
                current: current_controller,
                requested: player_id.clone(),
            },
        ));
    }

    let Some(handle) = players[controller_index].resolve_public_card_handle(chosen_card_id) else {
        restore_pending_hand_choice(
            pending_decision,
            controller_index,
            stack_object_number,
            kind,
        );
        return Err(DomainError::Game(GameError::InvalidHandCardChoice(
            chosen_card_id.clone(),
        )));
    };

    if players[controller_index].card_zone(chosen_card_id) != Some(PlayerCardZone::Hand) {
        restore_pending_hand_choice(
            pending_decision,
            controller_index,
            stack_object_number,
            kind,
        );
        return Err(DomainError::Game(GameError::InvalidHandCardChoice(
            chosen_card_id.clone(),
        )));
    }

    Ok(handle)
}

/// Finishes a pending hand-choice spell effect.
///
/// # Errors
/// Returns an error if no hand-choice effect is pending, if the caller is not
/// its controller, or if the selected card is not currently in that player's hand.
#[allow(clippy::too_many_lines)]
pub fn resolve_pending_hand_choice(
    ctx: StackPriorityContext<'_>,
    cmd: ResolvePendingHandChoiceCommand,
) -> Result<ResolvePendingHandChoiceOutcome, DomainError> {
    let StackPriorityContext {
        game_id,
        players,
        active_player,
        stack,
        priority,
        pending_decision,
        terminal_state,
        ..
    } = ctx;

    let ResolvePendingHandChoiceCommand {
        player_id,
        chosen_card_id,
    } = cmd;

    if priority.is_some() {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "pending hand-choice resolution requires the priority window to be closed".to_string(),
        )));
    }

    let (controller_index, stack_object_number, kind) = match pending_decision
        .take()
        .ok_or(DomainError::Game(GameError::NoPendingHandChoice))?
    {
        PendingDecision::HandChoice {
            controller_index,
            stack_object_number,
            kind,
        } => (controller_index, stack_object_number, kind),
        other => {
            *pending_decision = Some(other);
            return Err(DomainError::Game(GameError::NoPendingHandChoice));
        }
    };

    let handle = validate_pending_hand_choice(
        players,
        pending_decision,
        controller_index,
        stack_object_number,
        kind,
        &player_id,
        &chosen_card_id,
    )?;

    let pending_spell = remove_pending_spell(
        players,
        stack,
        controller_index,
        stack_object_number,
        "pending hand-choice spell must still exist on the stack",
        "pending hand choice requires a spell stack object",
    )?;
    let controller_id = pending_spell.controller_id().clone();

    players[controller_index]
        .move_hand_handle_to_graveyard(handle)
        .ok_or_else(|| {
            DomainError::Game(GameError::InvalidHandCardChoice(chosen_card_id.clone()))
        })?;
    let card_discarded = Some(CardDiscarded::new(
        game_id.clone(),
        controller_id.clone(),
        chosen_card_id,
        DiscardKind::SpellEffect,
    ));

    let (card_drawn, game_ended) = match kind {
        PendingHandChoiceKind::Loot { .. } => (Vec::new(), None),
        PendingHandChoiceKind::Rummage { draw_count } => draw_cards_for_pending_effect(
            game_id,
            players,
            terminal_state,
            controller_index,
            draw_count,
        )?,
    };

    let source_card_id = pending_spell.source_card_id().clone();
    let card_type = pending_spell.card_type();
    let mana_cost_paid = pending_spell.mana_cost_paid();
    let stack_object_number = pending_spell.stack_object_number();
    let (spell_outcome, moved_cards) = move_spell_to_resolution_destination(
        players,
        controller_index,
        pending_spell.into_payload(),
        card_type,
    )?;
    let (stack_top_resolved, spell_cast) = build_spell_resolution_events_from_parts(
        game_id,
        &controller_id,
        stack_object_number,
        &source_card_id,
        card_type,
        mana_cost_paid,
        spell_outcome,
    );

    if terminal_state.is_over() {
        *priority = None;
    } else {
        *priority = Some(PriorityState::opened(active_player.clone()));
    }

    Ok(ResolvePendingHandChoiceOutcome {
        stack_top_resolved: Some(stack_top_resolved),
        spell_cast: Some(spell_cast),
        card_drawn,
        card_discarded,
        moved_cards,
        game_ended,
        priority_still_open: priority.is_some(),
    })
}
