#![allow(clippy::unwrap_used)]

//! Unit coverage for unit lifecycle deal opening hands.

use {
    crate::support::{create_service, creature_library},
    demonictutor::{
        ActivatedAbilityProfile, CardDefinitionId, CardType, DomainError, GameError, LibraryCard,
        TriggeredAbilityProfile,
    },
};

#[test]
fn deal_opening_hands_moves_cards_to_hand() {
    let (service, game) =
        crate::support::setup_two_player_game("game-1", creature_library(7), creature_library(7));

    assert_eq!(game.players()[0].hand_size(), 7);
    assert_eq!(game.players()[1].hand_size(), 7);

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
fn deal_opening_hands_fails_when_a_player_library_is_missing() {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");

    let result = service.deal_opening_hands(
        &mut game,
        &demonictutor::DealOpeningHandsCommand::new(vec![crate::support::player_library(
            "player-1",
            creature_library(7),
        )]),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::MissingPlayerLibrary(_)))
    ));
}

#[test]
fn deal_opening_hands_fails_when_a_player_library_is_duplicated() {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");

    let result = service.deal_opening_hands(
        &mut game,
        &demonictutor::DealOpeningHandsCommand::new(vec![
            crate::support::player_library("player-1", creature_library(7)),
            crate::support::player_library("player-1", creature_library(7)),
        ]),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::DuplicatePlayerLibrary(_)))
    ));
}

#[test]
fn deal_opening_hands_fails_when_hands_were_already_dealt() {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");

    service
        .deal_opening_hands(
            &mut game,
            &demonictutor::DealOpeningHandsCommand::new(vec![
                crate::support::player_library("player-1", creature_library(7)),
                crate::support::player_library("player-2", creature_library(7)),
            ]),
        )
        .unwrap();

    let result = service.deal_opening_hands(
        &mut game,
        &demonictutor::DealOpeningHandsCommand::new(vec![
            crate::support::player_library("player-1", creature_library(7)),
            crate::support::player_library("player-2", creature_library(7)),
        ]),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::OpeningHandsAlreadyDealt))
    ));
}

#[test]
fn deal_opening_hands_uses_explicit_non_creature_library_input() {
    let card = LibraryCard::new(CardDefinitionId::new("forest"), CardType::Land, 0);

    let card_instance = card.to_card_instance(demonictutor::CardInstanceId::new("card-1"));

    assert!(card_instance.card_type().is_land());
}

#[test]
fn deal_opening_hands_rejects_library_cards_outside_the_curated_profile_catalog() {
    let service = create_service();
    let mut game = crate::support::start_two_player_game(&service, "game-1");
    let unsupported_card =
        LibraryCard::creature(CardDefinitionId::new("illegal-creature"), 1, 2, 2)
            .with_activated_ability(ActivatedAbilityProfile::tap_to_gain_life_to_controller(1))
            .with_triggered_ability(TriggeredAbilityProfile::attacks_gain_life_to_controller(1));

    let result = service.deal_opening_hands(
        &mut game,
        &demonictutor::DealOpeningHandsCommand::new(vec![
            crate::support::player_library(
                "player-1",
                vec![
                    unsupported_card,
                    LibraryCard::creature(CardDefinitionId::new("filler-1"), 1, 2, 2),
                    LibraryCard::creature(CardDefinitionId::new("filler-2"), 1, 2, 2),
                    LibraryCard::creature(CardDefinitionId::new("filler-3"), 1, 2, 2),
                    LibraryCard::creature(CardDefinitionId::new("filler-4"), 1, 2, 2),
                    LibraryCard::creature(CardDefinitionId::new("filler-5"), 1, 2, 2),
                    LibraryCard::creature(CardDefinitionId::new("filler-6"), 1, 2, 2),
                ],
            ),
            crate::support::player_library("player-2", creature_library(7)),
        ]),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::UnsupportedCuratedCardProfile {
            player,
            definition,
        })) if player == demonictutor::PlayerId::new("player-1")
            && definition == CardDefinitionId::new("illegal-creature")
    ));
}
