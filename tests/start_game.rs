#![allow(clippy::unwrap_used)]

use demonictutor::{
    DeckId, DomainError, GameId, GameService, PlayerDeck, PlayerId, StartGameCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

#[test]
fn start_game_creates_valid_game() {
    let cmd = StartGameCommand::new(
        GameId::new("game-1"),
        vec![
            player_deck("player-1", "deck-1"),
            player_deck("player-2", "deck-2"),
        ],
    );

    let result = GameService::start_game(cmd);

    assert!(result.is_ok());
    let (game, event) = result.unwrap();
    assert_eq!(game.id().0, "game-1");
    assert_eq!(event.players.len(), 2);
}

#[test]
fn start_game_rejects_single_player() {
    let cmd = StartGameCommand::new(
        GameId::new("game-1"),
        vec![player_deck("player-1", "deck-1")],
    );

    let result = GameService::start_game(cmd);

    assert!(matches!(
        result,
        Err(DomainError::NotEnoughPlayers { actual: 1 })
    ));
}

#[test]
fn start_game_rejects_too_many_players() {
    let cmd = StartGameCommand::new(
        GameId::new("game-1"),
        vec![
            player_deck("player-1", "deck-1"),
            player_deck("player-2", "deck-2"),
            player_deck("player-3", "deck-3"),
        ],
    );

    let result = GameService::start_game(cmd);

    assert!(matches!(
        result,
        Err(DomainError::TooManyPlayers { actual: 3 })
    ));
}

#[test]
fn start_game_rejects_duplicate_players() {
    let cmd = StartGameCommand::new(
        GameId::new("game-1"),
        vec![
            player_deck("player-1", "deck-1"),
            player_deck("player-1", "deck-2"),
        ],
    );

    let result = GameService::start_game(cmd);

    assert!(matches!(result, Err(DomainError::DuplicatePlayer(_))));
}
