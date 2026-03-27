//! Supports tests for focused spell-resolution effects modules.

#![allow(clippy::expect_used)]

use super::stack_and_hand::return_permanent_to_owners_hand;
use crate::domain::play::{
    cards::{CardInstance, CardType},
    game::{AggregateCardLocationIndex, Player},
    ids::{CardDefinitionId, CardInstanceId, PlayerId},
};

#[test]
fn bounce_returns_foreign_owned_permanent_to_owners_hand() {
    let mut players = vec![
        Player::new(PlayerId::new("p1")),
        Player::new(PlayerId::new("p2")),
    ];
    let card_id = CardInstanceId::new("borrowed-relic");

    players[1].receive_graveyard_card(CardInstance::new(
        card_id.clone(),
        CardDefinitionId::new("borrowed-relic"),
        CardType::Artifact,
        0,
    ));
    let card = players[1]
        .remove_graveyard_card(&card_id)
        .expect("owner graveyard should contain the card");
    assert!(players[0].receive_battlefield_card(card).is_some());
    let card_locations = AggregateCardLocationIndex::from_players(&players);

    let moved = return_permanent_to_owners_hand(&mut players, &card_locations, &card_id);

    assert_eq!(moved, Some(card_id.clone()));
    assert!(players[0].battlefield_card(&card_id).is_none());
    assert!(players[1].hand_card(&card_id).is_some());
}
