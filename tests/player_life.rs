#![allow(clippy::unwrap_used)]

use demonictutor::{
    DeckId, DomainError, GameId, GameService, InMemoryEventBus, InMemoryEventStore, PlayerDeck,
    PlayerId, SetLifeCommand, StartGameCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

fn create_service() -> GameService<InMemoryEventStore, InMemoryEventBus> {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
}

#[test]
fn players_start_with_20_life() {
    let service = create_service();
    let (game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    assert_eq!(game.players()[0].life(), 20);
    assert_eq!(game.players()[1].life(), 20);
}

#[test]
fn set_life_changes_player_life() {
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

    let cmd = SetLifeCommand::new(PlayerId::new("player-1"), -5);
    let result = service.set_life(&mut game, cmd);

    assert!(result.is_ok());
    let event = result.unwrap();
    assert_eq!(event.player_id.0, "player-1");
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 15);
    assert_eq!(game.players()[0].life(), 15);
    assert_eq!(game.players()[1].life(), 20);
}

#[test]
fn set_life_gains_life() {
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

    let cmd = SetLifeCommand::new(PlayerId::new("player-1"), 3);
    let result = service.set_life(&mut game, cmd);

    assert!(result.is_ok());
    let event = result.unwrap();
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 23);
    assert_eq!(game.players()[0].life(), 23);
}

#[test]
fn set_life_cannot_go_below_zero() {
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

    let cmd = SetLifeCommand::new(PlayerId::new("player-1"), -30);
    let result = service.set_life(&mut game, cmd);

    assert!(result.is_ok());
    let event = result.unwrap();
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 0);
    assert_eq!(game.players()[0].life(), 0);
}

#[test]
fn set_life_fails_for_unknown_player() {
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

    let cmd = SetLifeCommand::new(PlayerId::new("unknown-player"), 10);
    let result = service.set_life(&mut game, cmd);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::PlayerNotFound { .. }
    ));
}
