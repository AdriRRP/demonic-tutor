#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority stack foundation.

use demonictutor::{
    DeckId, GameId, GameService, InMemoryEventBus, InMemoryEventStore, Phase, PlayerDeck, PlayerId,
    StartGameCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

fn create_service() -> GameService<InMemoryEventStore, InMemoryEventBus> {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
}

#[test]
fn start_game_initializes_empty_stack_and_closed_priority_window() {
    let service = create_service();
    let command = StartGameCommand::new(
        GameId::new("game-stack-foundation"),
        vec![
            player_deck("player-1", "deck-1"),
            player_deck("player-2", "deck-2"),
        ],
    );

    let (game, _) = service.start_game(command).unwrap();

    assert_eq!(game.phase(), &Phase::Setup);
    assert!(game.stack().is_empty());
    assert_eq!(game.stack().len(), 0);
    assert!(game.priority().is_none());
    assert!(!game.has_open_priority_window());
}
