use super::{model::Player, PriorityState};
use crate::domain::play::{
    cards::{CardInstance, CardType},
    errors::{CardError, DomainError, GameError},
    ids::{CardInstanceId, PlayerId},
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

pub(super) fn find_player_index(
    players: &[Player],
    player_id: &PlayerId,
) -> Result<usize, DomainError> {
    players
        .iter()
        .position(|player| player.id() == player_id)
        .ok_or_else(|| DomainError::Game(GameError::PlayerNotFound(player_id.clone())))
}

pub(super) fn opposing_player_id(
    players: &[Player],
    player_id: &PlayerId,
) -> Result<PlayerId, DomainError> {
    players
        .iter()
        .find(|player| player.id() != player_id)
        .map(|player| player.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "a two-player game should always produce a winner when one player loses"
                    .to_string(),
            ))
        })
}

pub(super) fn find_player_mut<'a>(
    players: &'a mut [Player],
    player_id: &PlayerId,
) -> Result<&'a mut Player, DomainError> {
    let player_index = find_player_index(players, player_id)?;
    Ok(&mut players[player_index])
}

pub(super) fn hand_card_type(
    player: &Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<CardType, DomainError> {
    hand_card(player, player_id, card_id).map(|card| card.card_type().clone())
}

pub(super) fn hand_card<'a>(
    player: &'a Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<&'a CardInstance, DomainError> {
    player
        .hand()
        .cards()
        .iter()
        .find(|card| card.id() == card_id)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotInHand {
                player: player_id.clone(),
                card: card_id.clone(),
            })
        })
}

pub(super) fn remove_card_from_hand(
    player: &mut Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<CardInstance, DomainError> {
    player.hand_mut().remove(card_id).ok_or_else(|| {
        DomainError::Card(CardError::NotInHand {
            player: player_id.clone(),
            card: card_id.clone(),
        })
    })
}
