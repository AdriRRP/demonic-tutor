//! Supports play game invariants.

use {
    super::{model::Player, PriorityState},
    crate::domain::play::{
        errors::{DomainError, GameError},
        ids::PlayerId,
    },
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

pub(super) fn require_active_player_index(
    players: &[Player],
    active_player_index: usize,
    requested_player: &PlayerId,
) -> Result<(), DomainError> {
    let active_player = players.get(active_player_index).ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "active player index should point to an existing player".to_string(),
        ))
    })?;

    require_active_player(active_player.id(), requested_player)
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
    if stack_is_empty {
        return Ok(());
    }

    if let Some(priority) = priority {
        return Err(DomainError::Game(GameError::PriorityWindowOpen {
            current_holder: priority.current_holder().clone(),
        }));
    }

    Err(DomainError::Game(GameError::InternalInvariantViolation(
        "stack cannot remain pending without an open priority window".to_string(),
    )))
}
