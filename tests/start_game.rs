#![allow(clippy::unwrap_used)]

use demonictutor::{
    DeckId, DomainError, GameError, GameId, GameService, InMemoryEventBus, InMemoryEventStore,
    PlayerDeck, PlayerError, PlayerId, StartGameCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

fn create_service() -> GameService<InMemoryEventStore, InMemoryEventBus> {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
}

#[test]
fn start_game_creates_valid_game() {
    let service = create_service();
    let cmd = StartGameCommand::new(
        GameId::new("game-1"),
        vec![
            player_deck("player-1", "deck-1"),
            player_deck("player-2", "deck-2"),
        ],
    );

    let result = service.start_game(cmd);

    assert!(result.is_ok());
    let (game, event) = result.unwrap();
    assert_eq!(game.id().0, "game-1");
    assert_eq!(event.players.len(), 2);
}

#[test]
fn start_game_rejects_single_player() {
    let service = create_service();
    let cmd = StartGameCommand::new(
        GameId::new("game-1"),
        vec![player_deck("player-1", "deck-1")],
    );

    let result = service.start_game(cmd);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        DomainError::Player(PlayerError::NotEnoughPlayers { actual: 1 })
    );
}

#[test]
fn start_game_rejects_too_many_players() {
    let service = create_service();
    let cmd = StartGameCommand::new(
        GameId::new("game-1"),
        vec![
            player_deck("player-1", "deck-1"),
            player_deck("player-2", "deck-2"),
            player_deck("player-3", "deck-3"),
        ],
    );

    let result = service.start_game(cmd);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        DomainError::Player(PlayerError::TooManyPlayers { actual: 3 })
    );
}

#[test]
fn start_game_rejects_duplicate_players() {
    let service = create_service();
    let cmd = StartGameCommand::new(
        GameId::new("game-1"),
        vec![
            player_deck("player-1", "deck-1"),
            player_deck("player-1", "deck-2"),
        ],
    );

    let result = service.start_game(cmd);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        DomainError::Game(GameError::DuplicatePlayer(PlayerId::new("player-1")))
    );
}
