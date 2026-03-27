//! Supports rules stack priority pending scry effects.

use crate::domain::play::{
    commands::ResolvePendingScryCommand,
    errors::{DomainError, GameError},
    events::{SpellCast, SpellCastOutcome, StackTopResolved},
    game::{model::StackObjectKind, PendingDecision, PriorityState},
};

use super::{ResolvePendingScryOutcome, StackPriorityContext};

fn build_spell_resolution_events(
    game_id: &crate::domain::play::ids::GameId,
    controller_id: &crate::domain::play::ids::PlayerId,
    stack_object_number: u32,
    source_card_id: &crate::domain::play::ids::CardInstanceId,
    card_type: crate::domain::play::cards::CardType,
    mana_cost_paid: u32,
    outcome: SpellCastOutcome,
) -> (StackTopResolved, SpellCast) {
    let stack_object_id =
        crate::domain::play::ids::StackObjectId::for_stack_object(game_id, stack_object_number);
    (
        StackTopResolved::new(
            game_id.clone(),
            controller_id.clone(),
            stack_object_id,
            source_card_id.clone(),
        ),
        SpellCast::new(
            game_id.clone(),
            controller_id.clone(),
            source_card_id.clone(),
            card_type,
            mana_cost_paid,
            outcome,
        ),
    )
}

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

    let stack_object = stack.remove_by_number(stack_object_number).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "pending scry spell must still exist on the stack".to_string(),
        ))
    })?;
    let controller_id = players[controller_index].id().clone();
    let StackObjectKind::Spell(spell) = stack_object.into_kind() else {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            "pending scry requires a spell stack object".to_string(),
        )));
    };
    let source_card_id = spell.source_card_id().clone();
    let card_type = *spell.card_type();
    let mana_cost_paid = spell.mana_cost_paid();
    players[controller_index].receive_graveyard_card(spell.into_payload().into_card_instance());
    let (stack_top_resolved, spell_cast) = build_spell_resolution_events(
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
        moved_cards,
        game_ended: None,
        priority_still_open: true,
    })
}
