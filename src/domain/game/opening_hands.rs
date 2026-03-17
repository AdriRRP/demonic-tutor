use super::player::Player;
use crate::domain::{
    cards::CardInstance,
    commands::DealOpeningHandsCommand,
    errors::{DomainError, GameError},
    events::OpeningHandDealt,
    ids::CardInstanceId,
};

/// Deals opening hands to all players.
///
/// # Errors
/// Returns an error if:
/// - A player is not found
/// - A player does not have enough cards in their library
pub fn deal_opening_hands(
    players: &mut [Player],
    cmd: &DealOpeningHandsCommand,
    game_id: &super::GameId,
) -> Result<Vec<OpeningHandDealt>, DomainError> {
    let hand_size = 7;

    for pc in &cmd.player_cards {
        let player_exists = players.iter().any(|p| p.id() == &pc.player_id);
        if !player_exists {
            return Err(DomainError::Game(GameError::PlayerNotFound(
                pc.player_id.clone(),
            )));
        }
    }

    for pc in &cmd.player_cards {
        if pc.cards.len() < hand_size {
            return Err(DomainError::Game(GameError::NotEnoughCardsInLibrary {
                player: pc.player_id.clone(),
                available: pc.cards.len(),
                requested: hand_size,
            }));
        }
    }

    let mut events: Vec<OpeningHandDealt> = Vec::new();

    for pc in &cmd.player_cards {
        let idx = players
            .iter()
            .position(|p| p.id() == &pc.player_id)
            .ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "player {} should exist after validation",
                    pc.player_id.0
                )))
            })?;

        let player_id_owned = pc.player_id.clone();

        let cards: Vec<CardInstance> = pc
            .cards
            .iter()
            .enumerate()
            .map(|(i, card)| {
                if card.card_type.is_creature() {
                    CardInstance::new_creature(
                        CardInstanceId::new(format!("{}-{}-{}", game_id.0, player_id_owned.0, i)),
                        card.definition_id.clone(),
                        card.mana_cost,
                        card.power.unwrap_or(0),
                        card.toughness.unwrap_or(0),
                    )
                } else {
                    CardInstance::new(
                        CardInstanceId::new(format!("{}-{}-{}", game_id.0, player_id_owned.0, i)),
                        card.definition_id.clone(),
                        card.card_type.clone(),
                        card.mana_cost,
                    )
                }
            })
            .collect();

        let player = &mut players[idx];

        player.library_mut().receive(cards);

        let drawn_cards = player.library_mut().draw(hand_size).ok_or_else(|| {
            DomainError::Game(GameError::NotEnoughCardsInLibrary {
                player: pc.player_id.clone(),
                available: player.library().len(),
                requested: hand_size,
            })
        })?;

        player.hand_mut().receive(drawn_cards);

        let hand = player.hand();
        let hand_cards: Vec<_> = hand.cards().iter().map(|c| c.id().clone()).collect();

        events.push(OpeningHandDealt::new(
            game_id.clone(),
            pc.player_id.clone(),
            hand_cards,
        ));
    }

    Ok(events)
}
