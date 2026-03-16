#![allow(clippy::unwrap_used)]

use demonictutor::{
    CardDefinitionId, CardError, CardInstanceId, CardType, CardWithCost, DealOpeningHandsCommand,
    DeckId, DomainError, GameId, GameService, InMemoryEventBus, InMemoryEventStore,
    PlayLandCommand, PlayerDeck, PlayerDeckContents, PlayerId, StartGameCommand, TapLandCommand,
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

fn create_game_with_land_on_battlefield() -> (
    demonictutor::Game,
    GameService<InMemoryEventStore, InMemoryEventBus>,
) {
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

    // Advance: Setup -> Main (player-2)
    let advance_cmd = demonictutor::AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Advance: Main -> Combat (player-2)
    let advance_cmd = demonictutor::AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Advance: Combat -> Ending (player-2)
    let advance_cmd = demonictutor::AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Advance: Ending -> Main (player-1)
    let advance_cmd = demonictutor::AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Player-1 plays a land
    let play_land_cmd = PlayLandCommand::new(
        PlayerId::new("player-1"),
        CardInstanceId::new("game-1-player-1-0"),
    );
    service.play_land(&mut game, play_land_cmd).unwrap();

    (game, service)
}

#[test]
fn players_start_with_zero_mana() {
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

    assert_eq!(game.players()[0].mana(), 0);
    assert_eq!(game.players()[1].mana(), 0);
}

#[test]
fn tap_land_adds_mana() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    let cmd = TapLandCommand::new(
        PlayerId::new("player-1"),
        CardInstanceId::new("game-1-player-1-0"),
    );
    let result = service.tap_land(&mut game, cmd);

    assert!(result.is_ok());
    let (_, mana_event) = result.unwrap();
    assert_eq!(mana_event.amount, 1);
    assert_eq!(mana_event.new_mana_total, 1);
    assert_eq!(game.players()[0].mana(), 1);
}

#[test]
fn tap_land_fails_for_untapped_land() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    // Try to tap player's own land
    let cmd = TapLandCommand::new(
        PlayerId::new("player-1"),
        CardInstanceId::new("game-1-player-1-0"),
    );
    let result = service.tap_land(&mut game, cmd);

    assert!(result.is_ok());

    // Try to tap again - should fail
    let cmd2 = TapLandCommand::new(
        PlayerId::new("player-1"),
        CardInstanceId::new("game-1-player-1-0"),
    );
    let result2 = service.tap_land(&mut game, cmd2);

    assert!(result2.is_err());
    assert!(matches!(
        result2.unwrap_err(),
        DomainError::Card(CardError::AlreadyTapped { .. })
    ));
}

#[test]
fn tap_land_fails_for_non_land_card() {
    let (mut game, service) = create_game_with_land_on_battlefield();

    // Try to tap a non-land card (not on battlefield)
    let cmd = TapLandCommand::new(
        PlayerId::new("player-1"),
        CardInstanceId::new("game-1-player-1-1"), // This is in hand
    );
    let result = service.tap_land(&mut game, cmd);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::Card(CardError::NotOnBattlefield { .. })
    ));
}

#[test]
fn tap_land_fails_for_unknown_card() {
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

    let cmd = TapLandCommand::new(
        PlayerId::new("player-1"),
        CardInstanceId::new("nonexistent"),
    );
    let result = service.tap_land(&mut game, cmd);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::Card(CardError::NotOnBattlefield { .. })
    ));
}
