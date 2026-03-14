#![allow(clippy::unwrap_used)]

use demonictutor::{
    CardDefinitionId, CardType, DealOpeningHandsCommand, DeckId, DomainError, GameId, GameService,
    InMemoryEventBus, InMemoryEventStore, PlayerDeck, PlayerDeckContents, PlayerId,
    StartGameCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

fn player_deck_contents(player: &str, cards: Vec<(String, CardType)>) -> PlayerDeckContents {
    PlayerDeckContents::new(
        PlayerId::new(player),
        cards
            .into_iter()
            .map(|(c, ct)| (CardDefinitionId::new(c), ct))
            .collect(),
    )
}

fn non_land_cards(count: usize) -> Vec<(String, CardType)> {
    (0..count)
        .map(|i| (format!("card-{i}"), CardType::NonLand))
        .collect()
}

fn create_service() -> GameService<InMemoryEventStore, InMemoryEventBus> {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
}

#[test]
fn deal_opening_hands_moves_cards_to_hand() {
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

    let cmd = DealOpeningHandsCommand::new(vec![
        player_deck_contents("player-1", non_land_cards(7)),
        player_deck_contents("player-2", non_land_cards(7)),
    ]);

    let result = service.deal_opening_hands(&mut game, &cmd);

    assert!(result.is_ok());
    let events = result.unwrap();
    assert_eq!(events.len(), 2);

    let p1_hand = game.players()[0].hand().cards();
    assert_eq!(p1_hand.len(), 7);

    let p2_hand = game.players()[1].hand().cards();
    assert_eq!(p2_hand.len(), 7);
}

#[test]
fn deal_opening_hands_emits_event_per_player() {
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

    let cmd = DealOpeningHandsCommand::new(vec![
        player_deck_contents("player-1", non_land_cards(7)),
        player_deck_contents("player-2", non_land_cards(7)),
    ]);

    let result = service.deal_opening_hands(&mut game, &cmd);

    assert!(result.is_ok());
    let events = result.unwrap();
    assert_eq!(events.len(), 2);
}

#[test]
fn deal_opening_hands_fails_when_not_enough_cards() {
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

    let cmd =
        DealOpeningHandsCommand::new(vec![player_deck_contents("player-1", non_land_cards(6))]);

    let result = service.deal_opening_hands(&mut game, &cmd);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::NotEnoughCardsInLibrary { .. }
    ));
}

#[test]
fn deal_opening_hands_does_not_affect_other_player() {
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

    let cmd =
        DealOpeningHandsCommand::new(vec![player_deck_contents("player-1", non_land_cards(7))]);

    let result = service.deal_opening_hands(&mut game, &cmd);

    assert!(result.is_ok());
    let p1_hand = game.players()[0].hand().cards();
    assert_eq!(p1_hand.len(), 7);

    let p2_hand = game.players()[1].hand().cards();
    assert_eq!(p2_hand.len(), 0);
}
