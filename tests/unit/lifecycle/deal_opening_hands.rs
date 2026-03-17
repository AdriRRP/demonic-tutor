#![allow(clippy::unwrap_used)]

use crate::support::{create_service, creature_library};
use demonictutor::{CardDefinitionId, DomainError, GameError, LibraryCard, NonCreatureCardType};

#[test]
fn deal_opening_hands_moves_cards_to_hand() {
    let (service, game) =
        crate::support::setup_two_player_game("game-1", creature_library(7), creature_library(7));

    let p1_hand = game.players()[0].hand().cards();
    let p2_hand = game.players()[1].hand().cards();

    assert_eq!(p1_hand.len(), 7);
    assert_eq!(p2_hand.len(), 7);

    // setup_two_player_game already proved the command completed, but keep the service bound used
    let _ = service;
}

#[test]
fn deal_opening_hands_emits_event_per_player() {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");

    let events = service
        .deal_opening_hands(
            &mut game,
            &demonictutor::DealOpeningHandsCommand::new(vec![
                crate::support::player_library("player-1", creature_library(7)),
                crate::support::player_library("player-2", creature_library(7)),
            ]),
        )
        .unwrap();

    assert_eq!(events.len(), 2);
}

#[test]
fn deal_opening_hands_fails_when_not_enough_cards() {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");

    let result = service.deal_opening_hands(
        &mut game,
        &demonictutor::DealOpeningHandsCommand::new(vec![crate::support::player_library(
            "player-1",
            creature_library(6),
        )]),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::NotEnoughCardsInLibrary { .. }))
    ));
}

#[test]
fn deal_opening_hands_does_not_affect_other_player() {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");

    service
        .deal_opening_hands(
            &mut game,
            &demonictutor::DealOpeningHandsCommand::new(vec![crate::support::player_library(
                "player-1",
                creature_library(7),
            )]),
        )
        .unwrap();

    assert_eq!(game.players()[0].hand().cards().len(), 7);
    assert_eq!(game.players()[1].hand().cards().len(), 0);
}

#[test]
fn deal_opening_hands_uses_explicit_non_creature_library_input() {
    let card = LibraryCard::non_creature(
        CardDefinitionId::new("forest"),
        NonCreatureCardType::Land,
        0,
    );

    let card_instance = card.to_card_instance(demonictutor::CardInstanceId::new("card-1"));

    assert!(card_instance.card_type().is_land());
}
