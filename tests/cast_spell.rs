#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardError, CardInstanceId, CardType, CardWithCost,
    CastSpellCommand, DealOpeningHandsCommand, DeckId, DomainError, GameId, GameService,
    InMemoryEventBus, InMemoryEventStore, Phase, PlayLandCommand, PlayerDeck, PlayerDeckContents,
    PlayerId, StartGameCommand, TapLandCommand,
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

fn advance_to_first_main(
    service: &GameService<InMemoryEventStore, InMemoryEventBus>,
    game: &mut demonictutor::Game,
) {
    // Advance to Untap
    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(game, cmd).unwrap();
    // Advance to Upkeep
    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(game, cmd).unwrap();
    // Advance to Draw
    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(game, cmd).unwrap();
    // Advance to FirstMain
    let cmd = AdvanceTurnCommand::new();
    service.advance_turn(game, cmd).unwrap();
    assert_eq!(game.phase(), &Phase::FirstMain);
}

#[test]
fn cast_spell_moves_card_from_hand_to_battlefield() {
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
                CardWithCost::new(CardDefinitionId::new("giant-growth"), CardType::Instant, 0),
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

    advance_to_first_main(&service, &mut game);

    let _hand_before = game.players()[0].hand().cards().len();
    let _battlefield_before = game.players()[0].battlefield().cards().len();

    let card_id = CardInstanceId::new("game-1-player-1-0");
    let cmd = CastSpellCommand::new(PlayerId::new("player-1"), card_id.clone());
    let event = service.cast_spell(&mut game, cmd).unwrap();

    assert_eq!(event.card_id, card_id);

    let hand_after = game.players()[0].hand().cards().len();
    let battlefield_after = game.players()[0].battlefield().cards().len();

    assert_eq!(hand_after, 7);
    assert_eq!(battlefield_after, 1);
}

#[test]
fn cast_spell_rejected_land_card_stays_in_hand() {
    // Regression: card must not disappear when cast is rejected due to wrong type.
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
    advance_to_first_main(&service, &mut game);

    let hand_before = game.players()[0].hand().cards().len();

    let card_id = CardInstanceId::new("game-1-player-1-0");
    let cmd = CastSpellCommand::new(PlayerId::new("player-1"), card_id);
    let result = service.cast_spell(&mut game, cmd);

    assert!(result.is_err());
    // Card must still be in the hand after a failed cast
    assert_eq!(game.players()[0].hand().cards().len(), hand_before);
}

#[test]
fn cast_spell_fails_for_land_card() {
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

    advance_to_first_main(&service, &mut game);

    let card_id = CardInstanceId::new("game-1-player-1-0");
    let cmd = CastSpellCommand::new(PlayerId::new("player-1"), card_id);
    let result = service.cast_spell(&mut game, cmd);

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::CannotCastLand { .. }))
    ));
}

#[test]
fn cast_spell_fails_when_not_player_turn() {
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
                CardWithCost::new(CardDefinitionId::new("card-1"), CardType::Instant, 0),
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
                CardWithCost::new(CardDefinitionId::new("card-1"), CardType::Instant, 0),
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

    let card_id = CardInstanceId::new("game-1-player-2-0");
    let cmd = CastSpellCommand::new(PlayerId::new("player-2"), card_id);
    let result = service.cast_spell(&mut game, cmd);

    assert!(matches!(
        result,
        Err(DomainError::Game(
            demonictutor::GameError::NotYourTurn { .. }
        ))
    ));
}

#[test]
fn cast_spell_fails_when_card_not_in_hand() {
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
                CardWithCost::new(CardDefinitionId::new("card-1"), CardType::Creature, 0),
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

    advance_to_first_main(&service, &mut game);

    let card_id = CardInstanceId::new("game-1-player-1-99");
    let cmd = CastSpellCommand::new(PlayerId::new("player-1"), card_id);
    let result = service.cast_spell(&mut game, cmd);

    assert!(matches!(
        result,
        Err(DomainError::Card(CardError::NotInHand { .. }))
    ));
}

#[test]
fn cast_spell_fails_with_insufficient_mana() {
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
                CardWithCost::new(
                    CardDefinitionId::new("expensive-spell"),
                    CardType::Instant,
                    3,
                ),
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

    advance_to_first_main(&service, &mut game);

    let card_id = CardInstanceId::new("game-1-player-1-0");
    let cmd = CastSpellCommand::new(PlayerId::new("player-1"), card_id);
    let result = service.cast_spell(&mut game, cmd);

    assert!(matches!(
        result,
        Err(DomainError::Game(
            demonictutor::GameError::InsufficientMana { .. }
        ))
    ));
}

#[test]
fn cast_spell_succeeds_with_sufficient_mana() {
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
                CardWithCost::new(CardDefinitionId::new("card-2"), CardType::Instant, 1),
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
                CardWithCost::new(CardDefinitionId::new("forest"), CardType::Land, 0),
                CardWithCost::new(CardDefinitionId::new("card-2"), CardType::Instant, 1),
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

    // Need 11 advances to reach player-2's FirstMain:
    // Setup -> Untap -> Upkeep -> Draw -> FirstMain -> Combat -> SecondMain -> EndStep -> Untap(player-2) -> Upkeep -> Draw -> FirstMain
    for _ in 0..11 {
        let cmd = demonictutor::AdvanceTurnCommand::new();
        service.advance_turn(&mut game, cmd).unwrap();
    }
    assert_eq!(*game.phase(), Phase::FirstMain);

    let play_land_cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-0"),
    );
    service.play_land(&mut game, play_land_cmd).unwrap();

    let tap_land_cmd = TapLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-0"),
    );
    service.tap_land(&mut game, tap_land_cmd).unwrap();

    assert_eq!(game.players()[1].mana(), 1);

    let cast_cmd = CastSpellCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-1"),
    );
    let result = service.cast_spell(&mut game, cast_cmd);

    assert!(result.is_ok());
    assert_eq!(game.players()[1].mana(), 0);
}
