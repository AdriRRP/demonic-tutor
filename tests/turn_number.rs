#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardType, CardWithCost, DealOpeningHandsCommand, DeckId,
    DomainEvent, GameId, GameLogProjection, GameService, InMemoryEventBus, InMemoryEventStore,
    PlayerDeck, PlayerDeckContents, PlayerId, StartGameCommand,
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

#[test]
fn game_starts_with_turn_number_1() {
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

    assert_eq!(game.turn_number(), 1);
}

#[test]
fn advance_turn_increments_turn_number() {
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
                CardWithCost::new(CardDefinitionId::new("card-8"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-9"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-10"), CardType::Creature, 0),
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
                CardWithCost::new(CardDefinitionId::new("card-8"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-9"), CardType::Creature, 0),
                CardWithCost::new(CardDefinitionId::new("card-10"), CardType::Creature, 0),
            ],
        ),
    ]);
    service.deal_opening_hands(&mut game, &cmd).unwrap();

    assert_eq!(game.turn_number(), 1);

    // Need 8 advances to get to turn 2 (Setup adds 2 more phases)
    for _ in 0..8 {
        let cmd = AdvanceTurnCommand::new();
        service.advance_turn(&mut game, cmd).unwrap();
    }

    assert_eq!(game.turn_number(), 2);
}

#[test]
fn advance_turn_emits_turn_number_changed_event() {
    let projection = Arc::new(GameLogProjection::new());
    let projection_clone = Arc::clone(&projection);

    let mut bus = InMemoryEventBus::new();
    bus.subscribe(Arc::new(move |event: &DomainEvent| {
        projection_clone.handle(event);
    }));

    let service = GameService::new(InMemoryEventStore::new(), bus);

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-1"),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap();

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap();

    let logs = projection.logs();
    let turn_log = logs.iter().find(|l| l.contains("Turn changed"));
    assert!(turn_log.is_some());
}
