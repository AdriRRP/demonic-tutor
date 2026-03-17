#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdjustLifeCommand, DeckId, DomainError, GameEndReason, GameError, GameId, GameService,
    InMemoryEventBus, InMemoryEventStore, PlayerDeck, PlayerId, StartGameCommand,
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
fn adjust_life_deltas_player_life() {
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

    let cmd = AdjustLifeCommand::new(PlayerId::new("player-1"), -5);
    let result = service.adjust_life(&mut game, cmd);

    assert!(result.is_ok());
    let outcome = result.unwrap();
    let event = outcome.life_changed;
    assert_eq!(event.player_id.as_str(), "player-1");
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 15);
    assert!(outcome.game_ended.is_none());
    assert_eq!(game.players()[0].life(), 15);
    assert_eq!(game.players()[1].life(), 20);
}

#[test]
fn adjust_life_gains_life() {
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

    let cmd = AdjustLifeCommand::new(PlayerId::new("player-1"), 3);
    let result = service.adjust_life(&mut game, cmd);

    assert!(result.is_ok());
    let outcome = result.unwrap();
    let event = outcome.life_changed;
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 23);
    assert!(outcome.game_ended.is_none());
    assert_eq!(game.players()[0].life(), 23);
}

#[test]
fn adjust_life_cannot_go_below_zero_and_ends_the_game() {
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

    let cmd = AdjustLifeCommand::new(PlayerId::new("player-1"), -30);
    let result = service.adjust_life(&mut game, cmd);

    assert!(result.is_ok());
    let outcome = result.unwrap();
    let event = outcome.life_changed;
    assert_eq!(event.from_life, 20);
    assert_eq!(event.to_life, 0);
    assert!(outcome.game_ended.is_some());
    let Some(game_ended) = outcome.game_ended else {
        return;
    };
    assert_eq!(game_ended.loser_id, PlayerId::new("player-1"));
    assert_eq!(game_ended.winner_id, PlayerId::new("player-2"));
    assert_eq!(game_ended.reason, GameEndReason::ZeroLife);
    assert_eq!(game.players()[0].life(), 0);
    assert!(game.is_over());
}

#[test]
fn gameplay_actions_fail_after_zero_life_game_end() {
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

    let outcome = service
        .adjust_life(
            &mut game,
            AdjustLifeCommand::new(PlayerId::new("player-1"), -20),
        )
        .unwrap();
    assert!(outcome.game_ended.is_some());

    let result = service.adjust_life(
        &mut game,
        AdjustLifeCommand::new(PlayerId::new("player-2"), 1),
    );

    assert_eq!(
        result.unwrap_err(),
        DomainError::Game(GameError::GameAlreadyEnded)
    );
}

#[test]
fn adjust_life_fails_for_unknown_player() {
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

    let cmd = AdjustLifeCommand::new(PlayerId::new("unknown-player"), 10);
    let result = service.adjust_life(&mut game, cmd);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::Game(GameError::PlayerNotFound { .. })
    ));
}
