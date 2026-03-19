use super::PriorityState;
use crate::domain::play::{
    errors::{DomainError, GameError},
    ids::PlayerId,
};

pub(super) fn require_active_player(
    active_player: &PlayerId,
    requested_player: &PlayerId,
) -> Result<(), DomainError> {
    if active_player != requested_player {
        return Err(DomainError::Game(GameError::NotYourTurn {
            current: active_player.clone(),
            requested: requested_player.clone(),
        }));
    }

    Ok(())
}

pub(super) const fn require_game_active(game_is_over: bool) -> Result<(), DomainError> {
    if game_is_over {
        Err(DomainError::Game(GameError::GameAlreadyEnded))
    } else {
        Ok(())
    }
}

pub(super) fn require_no_open_priority_window(
    priority: Option<&PriorityState>,
) -> Result<(), DomainError> {
    priority.map_or(Ok(()), |priority| {
        Err(DomainError::Game(GameError::PriorityWindowOpen {
            current_holder: priority.current_holder().clone(),
        }))
    })
}

pub(super) fn require_priority_holder(
    priority: Option<&PriorityState>,
    requested_player: &PlayerId,
) -> Result<(), DomainError> {
    let priority = priority.ok_or(DomainError::Game(GameError::NoPriorityWindow))?;
    if priority.current_holder() != requested_player {
        return Err(DomainError::Game(GameError::NotPriorityHolder {
            current: priority.current_holder().clone(),
            requested: requested_player.clone(),
        }));
    }

    Ok(())
}

pub(super) fn require_empty_stack_priority_action_window(
    priority: Option<&PriorityState>,
    stack_is_empty: bool,
    requested_player: &PlayerId,
) -> Result<(), DomainError> {
    let Some(priority) = priority else {
        return Ok(());
    };

    if !stack_is_empty {
        return Err(DomainError::Game(GameError::PriorityWindowOpen {
            current_holder: priority.current_holder().clone(),
        }));
    }

    require_priority_holder(Some(priority), requested_player)
}

pub(super) fn require_no_priority_with_pending_stack(
    priority: Option<&PriorityState>,
    stack_is_empty: bool,
) -> Result<(), DomainError> {
    let Some(priority) = priority else {
        return Ok(());
    };

    if stack_is_empty {
        return Ok(());
    }

    Err(DomainError::Game(GameError::PriorityWindowOpen {
        current_holder: priority.current_holder().clone(),
    }))
}
