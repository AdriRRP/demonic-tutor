//! Supports rules stack priority pending scry effects.

use crate::domain::play::{
    commands::ResolvePendingScryCommand,
    errors::{DomainError, GameError},
    events::SpellCastOutcome,
    game::{PendingDecision, PriorityState},
};

use super::{
    deferred_resolution::{build_spell_resolution_events_from_parts, remove_pending_spell},
    ResolvePendingScryOutcome, StackPriorityContext,
};

/// Resolves a pending scry decision.
///
/// # Errors
/// Returns an error if no scry decision is pending or if the caller is not its controller.
pub fn resolve_pending_scry(
    ctx: StackPriorityContext<'_>,
    cmd: ResolvePendingScryCommand,
) -> Result<ResolvePendingScryOutcome, DomainError> {
    let StackPriorityContext {
        game_id,
        players,
        active_player,
        stack,
        priority,
        pending_decision,
        ..
    } = ctx;

    if priority.is_some() {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "pending scry resolution requires the priority window to be closed".to_string(),
        )));
    }

    let ResolvePendingScryCommand {
        player_id,
        move_to_bottom,
    } = cmd;

    let (controller_index, stack_object_number, amount) = match pending_decision
        .take()
        .ok_or(DomainError::Game(GameError::NoPendingScry))?
    {
        PendingDecision::Scry {
            controller_index,
            stack_object_number,
            amount,
        } => (controller_index, stack_object_number, amount),
        other => {
            *pending_decision = Some(other);
            return Err(DomainError::Game(GameError::NoPendingScry));
        }
    };

    let current_controller = players[controller_index].id().clone();
    if current_controller != player_id {
        *pending_decision = Some(PendingDecision::scry(
            controller_index,
            stack_object_number,
            amount,
        ));
        return Err(DomainError::Game(GameError::NotPendingScryController {
            current: current_controller,
            requested: player_id,
        }));
    }

    let moved_cards = if move_to_bottom {
        players[controller_index]
            .move_top_library_card_to_bottom()
            .into_iter()
            .collect()
    } else {
        Vec::new()
    };

    let pending_spell = remove_pending_spell(
        players,
        stack,
        controller_index,
        stack_object_number,
        "pending scry spell must still exist on the stack",
        "pending scry requires a spell stack object",
    )?;
    let controller_id = pending_spell.controller_id().clone();
    let source_card_id = pending_spell.source_card_id().clone();
    let card_type = pending_spell.card_type();
    let mana_cost_paid = pending_spell.mana_cost_paid();
    let stack_object_number = pending_spell.stack_object_number();
    players[controller_index]
        .receive_graveyard_card(pending_spell.into_payload().into_card_instance());
    let (stack_top_resolved, spell_cast) = build_spell_resolution_events_from_parts(
        game_id,
        &controller_id,
        stack_object_number,
        &source_card_id,
        card_type,
        mana_cost_paid,
        SpellCastOutcome::ResolvedToGraveyard,
    );

    *priority = Some(PriorityState::opened(active_player.clone()));

    Ok(ResolvePendingScryOutcome {
        stack_top_resolved: Some(stack_top_resolved),
        spell_cast: Some(spell_cast),
        zone_changes: Vec::new(),
        moved_cards,
        game_ended: None,
        priority_still_open: true,
    })
}
