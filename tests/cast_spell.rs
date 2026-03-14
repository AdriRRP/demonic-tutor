#![allow(clippy::unwrap_used)]

use demonictutor::{
    CardDefinitionId, CardInstanceId, CardType, CastSpellCommand, DealOpeningHandsCommand, DeckId,
    DomainError, GameId, GameService, InMemoryEventBus, InMemoryEventStore, PlayLandCommand,
    PlayerDeck, PlayerDeckContents, PlayerId, StartGameCommand, TapLandCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

fn player_deck_contents(player: &str, cards: Vec<(String, CardType, u32)>) -> PlayerDeckContents {
    PlayerDeckContents::new(
        PlayerId::new(player),
        cards
            .into_iter()
            .map(|(c, ct, mc)| (CardDefinitionId::new(c), ct, mc))
            .collect(),
    )
}

fn create_service() -> GameService<InMemoryEventStore, InMemoryEventBus> {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
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

    let cmd = DealOpeningHandsCommand::new(vec![player_deck_contents(
        "player-1",
        vec![
            (String::from("giant-growth"), CardType::Instant, 0),
            (String::from("card-2"), CardType::Creature, 0),
            (String::from("card-3"), CardType::Creature, 0),
            (String::from("card-4"), CardType::Creature, 0),
            (String::from("card-5"), CardType::Creature, 0),
            (String::from("card-6"), CardType::Creature, 0),
            (String::from("card-7"), CardType::Creature, 0),
        ],
    )]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let _hand_before = game.players()[0].hand().cards().len();
    let _battlefield_before = game.players()[0].battlefield().cards().len();

    let card_id = CardInstanceId::new("game-1-player-1-0");
    let cmd = CastSpellCommand::new(PlayerId::new("player-1"), card_id.clone());
    let event = service.cast_spell(&mut game, cmd).unwrap();

    assert_eq!(event.card_id, card_id);

    let hand_after = game.players()[0].hand().cards().len();
    let battlefield_after = game.players()[0].battlefield().cards().len();

    assert_eq!(hand_after, 6);
    assert_eq!(battlefield_after, 1);
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

    let cmd = DealOpeningHandsCommand::new(vec![player_deck_contents(
        "player-1",
        vec![
            (String::from("forest"), CardType::Land, 0),
            (String::from("card-2"), CardType::Creature, 0),
            (String::from("card-3"), CardType::Creature, 0),
            (String::from("card-4"), CardType::Creature, 0),
            (String::from("card-5"), CardType::Creature, 0),
            (String::from("card-6"), CardType::Creature, 0),
            (String::from("card-7"), CardType::Creature, 0),
        ],
    )]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let card_id = CardInstanceId::new("game-1-player-1-0");
    let cmd = CastSpellCommand::new(PlayerId::new("player-1"), card_id);
    let result = service.cast_spell(&mut game, cmd);

    assert!(matches!(result, Err(DomainError::CannotCastLand { .. })));
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
                (String::from("card-1"), CardType::Instant, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
            ],
        ),
        player_deck_contents(
            "player-2",
            vec![
                (String::from("card-1"), CardType::Instant, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
            ],
        ),
    ]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let card_id = CardInstanceId::new("game-1-player-2-0");
    let cmd = CastSpellCommand::new(PlayerId::new("player-2"), card_id);
    let result = service.cast_spell(&mut game, cmd);

    assert!(matches!(result, Err(DomainError::NotYourTurn { .. })));
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

    let cmd = DealOpeningHandsCommand::new(vec![player_deck_contents(
        "player-1",
        vec![
            (String::from("card-1"), CardType::Creature, 0),
            (String::from("card-2"), CardType::Creature, 0),
            (String::from("card-3"), CardType::Creature, 0),
            (String::from("card-4"), CardType::Creature, 0),
            (String::from("card-5"), CardType::Creature, 0),
            (String::from("card-6"), CardType::Creature, 0),
            (String::from("card-7"), CardType::Creature, 0),
        ],
    )]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let card_id = CardInstanceId::new("game-1-player-1-99");
    let cmd = CastSpellCommand::new(PlayerId::new("player-1"), card_id);
    let result = service.cast_spell(&mut game, cmd);

    assert!(matches!(result, Err(DomainError::CardNotInHand { .. })));
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

    let cmd = DealOpeningHandsCommand::new(vec![player_deck_contents(
        "player-1",
        vec![
            (String::from("expensive-spell"), CardType::Instant, 3),
            (String::from("card-2"), CardType::Creature, 0),
            (String::from("card-3"), CardType::Creature, 0),
            (String::from("card-4"), CardType::Creature, 0),
            (String::from("card-5"), CardType::Creature, 0),
            (String::from("card-6"), CardType::Creature, 0),
            (String::from("card-7"), CardType::Creature, 0),
        ],
    )]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let card_id = CardInstanceId::new("game-1-player-1-0");
    let cmd = CastSpellCommand::new(PlayerId::new("player-1"), card_id);
    let result = service.cast_spell(&mut game, cmd);

    assert!(matches!(result, Err(DomainError::InsufficientMana { .. })));
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

    let cmd = DealOpeningHandsCommand::new(vec![player_deck_contents(
        "player-1",
        vec![
            (String::from("forest"), CardType::Land, 0),
            (String::from("card-2"), CardType::Instant, 1),
            (String::from("card-3"), CardType::Creature, 0),
            (String::from("card-4"), CardType::Creature, 0),
            (String::from("card-5"), CardType::Creature, 0),
            (String::from("card-6"), CardType::Creature, 0),
            (String::from("card-7"), CardType::Creature, 0),
        ],
    )]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let deal_p2_cmd = DealOpeningHandsCommand::new(vec![player_deck_contents(
        "player-2",
        vec![
            (String::from("forest"), CardType::Land, 0),
            (String::from("card-2"), CardType::Instant, 1),
            (String::from("card-3"), CardType::Creature, 0),
            (String::from("card-4"), CardType::Creature, 0),
            (String::from("card-5"), CardType::Creature, 0),
            (String::from("card-6"), CardType::Creature, 0),
            (String::from("card-7"), CardType::Creature, 0),
        ],
    )]);
    service.deal_opening_hands(&mut game, &deal_p2_cmd).unwrap();

    let advance_cmd = demonictutor::AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

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
