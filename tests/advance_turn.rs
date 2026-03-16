#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardInstanceId, CardType, CardWithCost,
    DealOpeningHandsCommand, DeckId, GameId, GameService, InMemoryEventBus, InMemoryEventStore,
    Phase, PlayLandCommand, PlayerDeck, PlayerDeckContents, PlayerId, StartGameCommand,
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

fn create_game_with_land_in_hand() -> demonictutor::Game {
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

    game
}

#[test]
fn advance_turn_changes_active_player() {
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

    assert_eq!(game.active_player().0, "player-1");
    assert_eq!(game.phase(), &Phase::Setup);

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap();

    // After Setup -> Untap (player stays same)
    assert_eq!(game.active_player().0, "player-1");
    assert_eq!(game.phase(), &Phase::Untap);

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap();

    // After Untap -> Upkeep (player stays same)
    assert_eq!(game.active_player().0, "player-1");
    assert_eq!(game.phase(), &Phase::Upkeep);

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap();

    // After Upkeep -> Draw (player stays same)
    assert_eq!(game.active_player().0, "player-1");
    assert_eq!(game.phase(), &Phase::Draw);

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap();

    // After Draw -> FirstMain (player stays same)
    assert_eq!(game.active_player().0, "player-1");
    assert_eq!(game.phase(), &Phase::FirstMain);

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap();

    // After FirstMain -> Combat (player stays same)
    assert_eq!(game.active_player().0, "player-1");
    assert_eq!(game.phase(), &Phase::Combat);

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap();

    // After Combat -> SecondMain (player stays same)
    assert_eq!(game.active_player().0, "player-1");
    assert_eq!(game.phase(), &Phase::SecondMain);

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap();

    // After SecondMain -> EndStep (player stays same)
    assert_eq!(game.active_player().0, "player-1");
    assert_eq!(game.phase(), &Phase::EndStep);

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap();

    // After EndStep -> Untap (player changes)
    assert_eq!(game.active_player().0, "player-2");
    assert_eq!(game.phase(), &Phase::Untap);
}

#[test]
fn advance_turn_emits_event() {
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

    // Advance to first main phase
    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap(); // Setup -> Untap

    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, cmd).unwrap(); // Untap -> Upkeep

    let cmd = AdvanceTurnCommand::new();
    let result = service.advance_turn(&mut game, cmd);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().new_active_player.0, "player-1");
}

#[test]
fn advance_turn_resets_lands_played() {
    let mut game = create_game_with_land_in_hand();

    let service = create_service();

    // First advance: Setup -> Untap
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Second advance: Untap -> Upkeep
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Third advance: Upkeep -> Draw
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Fourth advance: Draw -> FirstMain
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Now in FirstMain phase, player-1 can play land
    // But player-2 has land in hand, let's advance to player-2's turn

    // FirstMain -> Combat
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Combat -> SecondMain
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // SecondMain -> EndStep
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // EndStep -> Untap (player-2)
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Untap -> Upkeep (player-2)
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Upkeep -> Draw (player-2)
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Draw -> FirstMain (player-2)
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Now player-2 can play land
    let land_cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-0"),
    );
    service.play_land(&mut game, land_cmd).unwrap();

    assert_eq!(game.players()[1].lands_played_this_turn(), 1);

    // Advance through remaining phases to next turn
    // FirstMain -> Combat
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Combat -> SecondMain
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // SecondMain -> EndStep
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // EndStep -> Untap (player-1)
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Untap -> Draw (player-1)
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    // Draw -> FirstMain (player-1) - lands reset here
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    assert_eq!(game.players()[0].lands_played_this_turn(), 0);
    assert_eq!(game.players()[1].lands_played_this_turn(), 0);
}

#[test]
fn advance_turn_allows_playing_land_after_turn_change() {
    let mut game = create_game_with_land_in_hand();

    let service = create_service();

    // Advance to FirstMain phase (player-1)
    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap(); // Setup -> Untap

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap(); // Untap -> Upkeep

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap(); // Upkeep -> Draw

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap(); // Draw -> FirstMain

    // Player-1 can play land in FirstMain
    let land_cmd = PlayLandCommand::new(
        PlayerId::new("player-1"),
        CardInstanceId::new("game-1-player-1-0"),
    );
    let result = service.play_land(&mut game, land_cmd);

    assert!(result.is_ok());

    // Advance to player-2's FirstMain
    let advance_cmd2 = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd2).unwrap(); // FirstMain -> Combat

    let advance_cmd3 = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd3).unwrap(); // Combat -> SecondMain

    let advance_cmd4 = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd4).unwrap(); // SecondMain -> EndStep

    let advance_cmd5 = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd5).unwrap(); // EndStep -> Untap (player-2)

    let advance_cmd6 = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd6).unwrap(); // Untap -> Upkeep

    let advance_cmd7 = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd7).unwrap(); // Upkeep -> Draw

    let advance_cmd8 = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd8).unwrap(); // Draw -> FirstMain (player-2)

    // Player-2 can play land in FirstMain
    let land_cmd2 = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-0"),
    );
    let result2 = service.play_land(&mut game, land_cmd2);

    assert!(result2.is_ok());
}
