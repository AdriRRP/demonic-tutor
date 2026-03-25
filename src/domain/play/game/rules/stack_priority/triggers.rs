//! Supports rules stack priority triggered abilities.

use crate::domain::play::{
    cards::TriggeredAbilityEvent,
    errors::{DomainError, GameError},
    events::TriggeredAbilityPutOnStack,
    game::{
        model::{StackCardRef, StackObject, StackObjectKind, StackZone, TriggeredAbilityOnStack},
        Player,
    },
    ids::{GameId, PlayerCardHandle},
};

fn enqueue_trigger_from_handle(
    game_id: &GameId,
    players: &[Player],
    controller_index: usize,
    handle: PlayerCardHandle,
    expected_event: TriggeredAbilityEvent,
    stack: &mut StackZone,
) -> Result<Option<TriggeredAbilityPutOnStack>, DomainError> {
    let player = players.get(controller_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "missing trigger controller at index {controller_index}"
        )))
    })?;
    let Some(card) = player.card_by_handle(handle) else {
        return Ok(None);
    };
    let Some(triggered_ability) = card.triggered_ability() else {
        return Ok(None);
    };
    if triggered_ability.event() != expected_event {
        return Ok(None);
    }

    let stack_object_number = stack.next_object_number();
    stack.push(StackObject::new(
        stack_object_number,
        controller_index,
        StackObjectKind::TriggeredAbility(TriggeredAbilityOnStack::new(
            StackCardRef::new(controller_index, handle),
            card.id().core_u64(),
            triggered_ability,
        )),
    ));

    Ok(Some(TriggeredAbilityPutOnStack::new(
        game_id.clone(),
        player.id().clone(),
        card.id().clone(),
        triggered_ability.event(),
        triggered_ability.effect(),
        crate::domain::play::ids::StackObjectId::for_stack_object(game_id, stack_object_number),
    )))
}

pub fn enqueue_trigger_for_card_handle(
    game_id: &GameId,
    players: &[Player],
    controller_index: usize,
    handle: PlayerCardHandle,
    expected_event: TriggeredAbilityEvent,
    stack: &mut StackZone,
) -> Result<Vec<TriggeredAbilityPutOnStack>, DomainError> {
    Ok(enqueue_trigger_from_handle(
        game_id,
        players,
        controller_index,
        handle,
        expected_event,
        stack,
    )?
    .into_iter()
    .collect())
}

pub fn enqueue_battlefield_step_triggers(
    game_id: &GameId,
    players: &[Player],
    controller_index: usize,
    expected_event: TriggeredAbilityEvent,
    stack: &mut StackZone,
) -> Result<Vec<TriggeredAbilityPutOnStack>, DomainError> {
    let player = players.get(controller_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "missing trigger controller at index {controller_index}"
        )))
    })?;
    let handles = player.battlefield_handles().collect::<Vec<_>>();
    let mut events = Vec::new();

    for handle in handles {
        if let Some(event) = enqueue_trigger_from_handle(
            game_id,
            players,
            controller_index,
            handle,
            expected_event,
            stack,
        )? {
            events.push(event);
        }
    }

    Ok(events)
}

pub fn enqueue_battlefield_step_triggers_apnap(
    game_id: &GameId,
    players: &[Player],
    active_player_index: usize,
    expected_event: TriggeredAbilityEvent,
    stack: &mut StackZone,
) -> Result<Vec<TriggeredAbilityPutOnStack>, DomainError> {
    let mut events = Vec::new();

    for offset in 0..players.len() {
        let controller_index = (active_player_index + offset) % players.len();
        events.extend(enqueue_battlefield_step_triggers(
            game_id,
            players,
            controller_index,
            expected_event,
            stack,
        )?);
    }

    Ok(events)
}
