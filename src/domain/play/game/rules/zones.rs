use super::super::model::Player;
use crate::domain::play::{
    errors::DomainError,
    events::{CardExiled, ZoneType},
    ids::{CardInstanceId, GameId, PlayerId},
};

/// Exiles a card from the battlefield.
///
/// # Errors
/// Returns `DomainError::Game::PlayerNotFound` if the player is not found.
/// Returns `DomainError::Card::NotOnBattlefield` if the card is not on the battlefield.
pub fn exile_card_from_battlefield(
    game_id: &GameId,
    players: &mut [Player],
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<CardExiled, DomainError> {
    let player = players
        .iter_mut()
        .find(|p| p.id() == player_id)
        .ok_or_else(|| {
            DomainError::Game(crate::domain::play::errors::GameError::PlayerNotFound(
                player_id.clone(),
            ))
        })?;

    let card = player.battlefield_mut().remove(card_id).ok_or_else(|| {
        DomainError::Card(crate::domain::play::errors::CardError::NotOnBattlefield {
            player: player_id.clone(),
            card: card_id.clone(),
        })
    })?;

    player.exile_mut().add(card);

    Ok(CardExiled::new(
        game_id.clone(),
        player_id.clone(),
        card_id.clone(),
        ZoneType::Battlefield,
    ))
}

/// Exiles a card from the graveyard.
///
/// # Errors
/// Returns `DomainError::Game::PlayerNotFound` if the player is not found.
/// Returns `DomainError::Card::NotOnBattlefield` if the card is not in the graveyard.
pub fn exile_card_from_graveyard(
    game_id: &GameId,
    players: &mut [Player],
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<CardExiled, DomainError> {
    let player = players
        .iter_mut()
        .find(|p| p.id() == player_id)
        .ok_or_else(|| {
            DomainError::Game(crate::domain::play::errors::GameError::PlayerNotFound(
                player_id.clone(),
            ))
        })?;

    let card = player.graveyard_mut().remove(card_id);

    card.map_or_else(
        || {
            Err(DomainError::Card(
                crate::domain::play::errors::CardError::NotOnBattlefield {
                    player: player_id.clone(),
                    card: card_id.clone(),
                },
            ))
        },
        |card| {
            player.exile_mut().add(card);
            Ok(CardExiled::new(
                game_id.clone(),
                player_id.clone(),
                card_id.clone(),
                ZoneType::Graveyard,
            ))
        },
    )
}
