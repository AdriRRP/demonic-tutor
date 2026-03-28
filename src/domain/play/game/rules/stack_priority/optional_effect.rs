//! Supports rules stack priority optional-effect decisions.

use {
    super::{resolution::resolve_stack_object, ResolveOptionalEffectOutcome, StackPriorityContext},
    crate::domain::play::{
        commands::ResolveOptionalEffectCommand,
        errors::{DomainError, GameError},
        game::{model::PriorityState, PendingDecision},
    },
};

fn stack_top_resolved_without_effect(
    game_id: &crate::domain::play::ids::GameId,
    players: &[crate::domain::play::game::Player],
    stack_object: &crate::domain::play::game::StackObject,
) -> crate::domain::play::events::StackTopResolved {
    crate::domain::play::events::StackTopResolved::new(
        game_id.clone(),
        players[stack_object.controller_index()].id().clone(),
        crate::domain::play::ids::StackObjectId::for_stack_object(game_id, stack_object.number()),
        stack_object.source_card_id(),
    )
}

/// Resolves or declines the currently pending optional stack effect.
///
/// # Errors
/// Returns an error if no optional choice is pending, if the caller is not the
/// controller of that choice, or if the stack object cannot be resolved.
pub fn resolve_optional_effect(
    ctx: StackPriorityContext<'_>,
    cmd: ResolveOptionalEffectCommand,
) -> Result<ResolveOptionalEffectOutcome, DomainError> {
    let StackPriorityContext {
        game_id,
        players,
        card_locations,
        active_player,
        stack,
        priority,
        pending_decision,
        terminal_state,
        ..
    } = ctx;

    let ResolveOptionalEffectCommand { player_id, accept } = cmd;

    if priority.is_some() {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "optional effect decisions require the priority window to be closed".to_string(),
        )));
    }

    let (controller_index, stack_object_number) = match pending_decision
        .take()
        .ok_or(DomainError::Game(GameError::NoPendingOptionalEffect))?
    {
        PendingDecision::OptionalEffect {
            controller_index,
            stack_object_number,
        } => (controller_index, stack_object_number),
        other => {
            *pending_decision = Some(other);
            return Err(DomainError::Game(GameError::NoPendingOptionalEffect));
        }
    };

    let current_controller = players[controller_index].id().clone();
    if current_controller != player_id {
        *pending_decision = Some(PendingDecision::optional_effect(
            controller_index,
            stack_object_number,
        ));
        return Err(DomainError::Game(GameError::NotOptionalEffectController {
            current: current_controller,
            requested: player_id,
        }));
    }

    let stack_object = stack.remove_by_number(stack_object_number).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "pending optional effect must still exist on the stack".to_string(),
        ))
    })?;

    let (
        stack_top_resolved,
        triggered_abilities_put_on_stack,
        spell_cast,
        card_discarded,
        zone_changes,
        life_changed,
        creatures_died,
        _moved_cards,
        game_ended,
    ) = if accept {
        resolve_stack_object(
            game_id,
            players,
            card_locations,
            terminal_state,
            stack,
            stack_object,
        )?
    } else {
        (
            stack_top_resolved_without_effect(game_id, players, &stack_object),
            Vec::new(),
            None,
            None,
            Vec::new(),
            None,
            Vec::new(),
            Vec::new(),
            None,
        )
    };

    if terminal_state.is_over() {
        *priority = None;
    } else {
        *priority = Some(PriorityState::opened(active_player.clone()));
    }

    Ok(ResolveOptionalEffectOutcome {
        stack_top_resolved: Some(stack_top_resolved),
        triggered_abilities_put_on_stack,
        spell_cast,
        card_discarded,
        zone_changes,
        life_changed,
        creatures_died,
        game_ended,
        priority_still_open: priority.is_some(),
    })
}
