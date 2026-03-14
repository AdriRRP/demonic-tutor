#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardInstanceId, CardType, DealOpeningHandsCommand,
    DeckId, DomainError, DrawCardCommand, GameId, GameService, InMemoryEventBus,
    InMemoryEventStore, PlayLandCommand, PlayerDeck, PlayerDeckContents, PlayerId,
    StartGameCommand,
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

fn create_game_with_library_cards() -> demonictutor::Game {
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
                (String::from("forest"), CardType::Land, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
                (String::from("card-8"), CardType::Creature, 0),
                (String::from("card-9"), CardType::Creature, 0),
                (String::from("card-10"), CardType::Creature, 0),
                (String::from("card-11"), CardType::Creature, 0),
                (String::from("card-12"), CardType::Creature, 0),
            ],
        ),
        player_deck_contents(
            "player-2",
            vec![
                (String::from("mountain"), CardType::Land, 0),
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

    game
}

#[test]
fn draw_card_works_in_main_phase() {
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
                (String::from("forest"), CardType::Land, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
                (String::from("card-8"), CardType::Creature, 0),
                (String::from("card-9"), CardType::Creature, 0),
                (String::from("card-10"), CardType::Creature, 0),
            ],
        ),
        player_deck_contents(
            "player-2",
            vec![
                (String::from("mountain"), CardType::Land, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
                (String::from("card-8"), CardType::Creature, 0),
            ],
        ),
    ]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    let draw_cmd = DrawCardCommand::new(PlayerId::new("player-2"));
    let result = service.draw_card(&mut game, draw_cmd);

    assert!(result.is_ok());
}

#[test]
fn draw_card_moves_card_from_library_to_hand() {
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
                (String::from("forest"), CardType::Land, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
                (String::from("card-8"), CardType::Creature, 0),
                (String::from("card-9"), CardType::Creature, 0),
                (String::from("card-10"), CardType::Creature, 0),
            ],
        ),
        player_deck_contents(
            "player-2",
            vec![
                (String::from("mountain"), CardType::Land, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
                (String::from("card-8"), CardType::Creature, 0),
            ],
        ),
    ]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    let hand_before = game.players()[1].hand().cards().len();
    let lib_before = game.players()[1].library().len();

    let draw_cmd = DrawCardCommand::new(PlayerId::new("player-2"));
    service.draw_card(&mut game, draw_cmd).unwrap();

    let hand_after = game.players()[1].hand().cards().len();
    let lib_after = game.players()[1].library().len();

    assert_eq!(hand_before + 1, hand_after);
    assert_eq!(lib_before - 1, lib_after);
}

#[test]
fn draw_card_emits_event() {
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
                (String::from("forest"), CardType::Land, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
                (String::from("card-8"), CardType::Creature, 0),
            ],
        ),
        player_deck_contents(
            "player-2",
            vec![
                (String::from("mountain"), CardType::Land, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
                (String::from("card-8"), CardType::Creature, 0),
            ],
        ),
    ]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    let draw_cmd = DrawCardCommand::new(PlayerId::new("player-2"));
    let result = service.draw_card(&mut game, draw_cmd);

    assert!(result.is_ok());
    let event = result.unwrap();
    assert_eq!(event.player_id.0, "player-2");
}

#[test]
fn draw_card_fails_when_not_enough_cards() {
    let mut game = create_game_with_library_cards();
    let service = create_service();

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    let draw_cmd = DrawCardCommand::new(PlayerId::new("player-2"));
    let result = service.draw_card(&mut game, draw_cmd);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, DomainError::NotEnoughCardsInLibrary { .. }));
}

#[test]
fn draw_card_fails_when_not_player_turn() {
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
                (String::from("forest"), CardType::Land, 0),
                (String::from("card-2"), CardType::Creature, 0),
                (String::from("card-3"), CardType::Creature, 0),
                (String::from("card-4"), CardType::Creature, 0),
                (String::from("card-5"), CardType::Creature, 0),
                (String::from("card-6"), CardType::Creature, 0),
                (String::from("card-7"), CardType::Creature, 0),
                (String::from("card-8"), CardType::Creature, 0),
            ],
        ),
        player_deck_contents(
            "player-2",
            vec![
                (String::from("mountain"), CardType::Land, 0),
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

    let draw_cmd = DrawCardCommand::new(PlayerId::new("player-2"));
    let result = service.draw_card(&mut game, draw_cmd);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::NotYourTurn { .. }
    ));
}

#[test]
fn draw_card_allows_playing_land_after_draw() {
    let mut game = create_game_with_library_cards();
    let service = create_service();

    let draw_cmd = DrawCardCommand::new(PlayerId::new("player-1"));
    service.draw_card(&mut game, draw_cmd).unwrap();

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    let land_cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-0"),
    );
    let result = service.play_land(&mut game, land_cmd);

    assert!(result.is_ok());
}
