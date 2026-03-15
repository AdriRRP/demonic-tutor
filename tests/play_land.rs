#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardInstanceId, CardType, CardWithCost,
    DealOpeningHandsCommand, DeckId, DomainError, GameId, GameService, InMemoryEventBus,
    InMemoryEventStore, PlayLandCommand, PlayerDeck, PlayerDeckContents, PlayerId,
    StartGameCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

fn player_deck_contents(player: &str, cards: Vec<CardWithCost>) -> PlayerDeckContents {
    PlayerDeckContents::new(PlayerId::new(player), cards)
}

fn create_service() -> GameService<InMemoryEventStore, InMemoryEventBus> {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
}

fn create_game_with_land_in_hand() -> (demonictutor::Game, CardInstanceId) {
    let service = create_service();
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let land_card_id = CardInstanceId::new("game-1-player-2-0");

    let cmd = DealOpeningHandsCommand::new(vec![
        player_deck_contents(
            "player-1",
            vec![
                CardWithCost::new(CardDefinitionId::new("forest"), CardType::Land, 0),
                CardWithCost::new(CardDefinitionId::new("card-2"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-3"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-4"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-5"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-6"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-7"), CardType::Creature, 0),
            ],
        ),
        player_deck_contents(
            "player-2",
            vec![
                CardWithCost::new(CardDefinitionId::new("mountain"), CardType::Land, 0),
                CardWithCost::new(CardDefinitionId::new("card-2"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-3"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-4"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-5"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-6"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-7"), CardType::Creature, 0),
            ],
        ),
    ]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    (game, land_card_id)
}

#[test]
fn play_land_moves_card_from_hand_to_battlefield() {
    let (mut game, land_card_id) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(PlayerId::new("player-2"), land_card_id.clone());
    let result = service.play_land(&mut game, cmd);

    assert!(result.is_ok());

    let p2_battlefield = game.players()[1].battlefield().cards();
    assert_eq!(p2_battlefield.len(), 1);
    assert_eq!(p2_battlefield[0].id(), &land_card_id);
}

#[test]
fn play_land_emits_event() {
    let (mut game, land_card_id) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(PlayerId::new("player-2"), land_card_id.clone());
    let result = service.play_land(&mut game, cmd);

    assert!(result.is_ok());
    let event = result.unwrap();
    assert_eq!(event.card_id, land_card_id);
    assert_eq!(event.player_id.0, "player-2");
}

#[test]
fn play_land_fails_when_card_not_in_hand() {
    let (mut game, _) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("nonexistent-card"),
    );
    let result = service.play_land(&mut game, cmd);

    assert!(matches!(result, Err(DomainError::CardNotInHand { .. })));
}

#[test]
fn play_land_fails_when_card_is_not_a_land() {
    let (mut game, _) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-1"),
    );
    let result = service.play_land(&mut game, cmd);

    assert!(matches!(result, Err(DomainError::NotALand { .. })));
}

#[test]
fn play_land_fails_when_not_player_turn() {
    let (mut game, land_card_id) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(PlayerId::new("player-1"), land_card_id);
    let result = service.play_land(&mut game, cmd);

    assert!(matches!(result, Err(DomainError::NotYourTurn { .. })));
}

#[test]
fn play_land_fails_when_land_already_played_this_turn() {
    let (mut game, land_card_id) = create_game_with_land_in_hand();
    let service = create_service();

    let cmd = PlayLandCommand::new(PlayerId::new("player-2"), land_card_id);
    let result = service.play_land(&mut game, cmd);
    assert!(result.is_ok());

    let second_land_id = CardInstanceId::new("game-1-player-2-5");
    let cmd2 = PlayLandCommand::new(PlayerId::new("player-2"), second_land_id);
    let result2 = service.play_land(&mut game, cmd2);

    assert!(matches!(
        result2,
        Err(DomainError::AlreadyPlayedLandThisTurn { .. })
    ));
}
