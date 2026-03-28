//! Supports rules stack priority pending surveil effects.

use crate::domain::play::{
    commands::ResolvePendingSurveilCommand,
    errors::{DomainError, GameError},
    events::{CardMovedZone, ZoneType},
    game::{PendingDecision, PriorityState},
};

use super::{
    deferred_resolution::{remove_pending_spell, resolve_pending_spell_to_default_destination},
    ResolvePendingSurveilOutcome, StackPriorityContext,
};

fn build_surveil_zone_changes(
    game_id: &crate::domain::play::ids::GameId,
    player_id: &crate::domain::play::ids::PlayerId,
    moved_cards: &[crate::domain::play::ids::CardInstanceId],
) -> Vec<CardMovedZone> {
    moved_cards
        .iter()
        .cloned()
        .map(|card_id| {
            CardMovedZone::new(
                game_id.clone(),
                player_id.clone(),
                card_id,
                ZoneType::Library,
                ZoneType::Graveyard,
            )
        })
        .collect()
}

/// Resolves a pending surveil decision.
///
/// # Errors
/// Returns an error if no surveil decision is pending or if the caller is not its controller.
pub fn resolve_pending_surveil(
    ctx: StackPriorityContext<'_>,
    cmd: ResolvePendingSurveilCommand,
) -> Result<ResolvePendingSurveilOutcome, DomainError> {
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
            "pending surveil resolution requires the priority window to be closed".to_string(),
        )));
    }

    let ResolvePendingSurveilCommand {
        player_id,
        move_to_graveyard,
    } = cmd;

    let (controller_index, stack_object_number, amount) = match pending_decision
        .take()
        .ok_or(DomainError::Game(GameError::NoPendingSurveil))?
    {
        PendingDecision::Surveil {
            controller_index,
            stack_object_number,
            amount,
        } => (controller_index, stack_object_number, amount),
        other => {
            *pending_decision = Some(other);
            return Err(DomainError::Game(GameError::NoPendingSurveil));
        }
    };

    let current_controller = players[controller_index].id().clone();
    if current_controller != player_id {
        *pending_decision = Some(PendingDecision::surveil(
            controller_index,
            stack_object_number,
            amount,
        ));
        return Err(DomainError::Game(GameError::NotPendingSurveilController {
            current: current_controller,
            requested: player_id,
        }));
    }

    let moved_cards = if move_to_graveyard {
        players[controller_index]
            .mill_cards_to_graveyard(1)
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let zone_changes = if move_to_graveyard {
        build_surveil_zone_changes(game_id, players[controller_index].id(), &moved_cards)
    } else {
        Vec::new()
    };

    let pending_spell = remove_pending_spell(
        players,
        stack,
        controller_index,
        stack_object_number,
        "pending surveil spell must still exist on the stack",
        "pending surveil requires a spell stack object",
    )?;
    let (stack_top_resolved, spell_cast, _) = resolve_pending_spell_to_default_destination(
        game_id,
        players,
        controller_index,
        pending_spell,
    )?;

    *priority = Some(PriorityState::opened(active_player.clone()));

    Ok(ResolvePendingSurveilOutcome {
        stack_top_resolved: Some(stack_top_resolved),
        spell_cast: Some(spell_cast),
        zone_changes,
        moved_cards,
        game_ended: None,
        priority_still_open: true,
    })
}
