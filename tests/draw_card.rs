#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardInstanceId, CardType, DealOpeningHandsCommand,
    DeckId, DomainError, DrawCardCommand, GameId, GameService, PlayLandCommand, PlayerDeck,
    PlayerDeckContents, PlayerId, StartGameCommand,
};

fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

fn player_deck_contents(player: &str, cards: Vec<(String, CardType)>) -> PlayerDeckContents {
    PlayerDeckContents::new(
        PlayerId::new(player),
        cards
            .into_iter()
            .map(|(c, ct)| (CardDefinitionId::new(c), ct))
            .collect(),
    )
}

fn create_game_with_library_cards() -> demonictutor::Game {
    let (mut game, _) = GameService::start_game(StartGameCommand::new(
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
                (String::from("forest"), CardType::Land),
                (String::from("card-2"), CardType::NonLand),
                (String::from("card-3"), CardType::NonLand),
                (String::from("card-4"), CardType::NonLand),
                (String::from("card-5"), CardType::NonLand),
                (String::from("card-6"), CardType::NonLand),
                (String::from("card-7"), CardType::NonLand),
                (String::from("card-8"), CardType::NonLand),
                (String::from("card-9"), CardType::NonLand),
                (String::from("card-10"), CardType::NonLand),
                (String::from("card-11"), CardType::NonLand),
                (String::from("card-12"), CardType::NonLand),
            ],
        ),
        player_deck_contents(
            "player-2",
            vec![
                (String::from("mountain"), CardType::Land),
                (String::from("card-2"), CardType::NonLand),
                (String::from("card-3"), CardType::NonLand),
                (String::from("card-4"), CardType::NonLand),
                (String::from("card-5"), CardType::NonLand),
                (String::from("card-6"), CardType::NonLand),
                (String::from("card-7"), CardType::NonLand),
            ],
        ),
    ]);

    GameService::deal_opening_hands(&mut game, &cmd).unwrap();

    game
}

#[test]
fn draw_card_moves_card_from_library_to_hand() {
    let mut game = create_game_with_library_cards();

    let hand_before = game.players()[0].hand().cards().len();
    let library_before = game.players()[0].library().len();

    let cmd = DrawCardCommand::new(PlayerId::new("player-1"));
    let result = GameService::draw_card(&mut game, cmd);

    assert!(result.is_ok());

    let hand_after = game.players()[0].hand().cards().len();
    let library_after = game.players()[0].library().len();

    assert_eq!(hand_before + 1, hand_after);
    assert_eq!(library_before - 1, library_after);
}

#[test]
fn draw_card_emits_event() {
    let mut game = create_game_with_library_cards();

    let cmd = DrawCardCommand::new(PlayerId::new("player-1"));
    let result = GameService::draw_card(&mut game, cmd);

    assert!(result.is_ok());
    let event = result.unwrap();
    assert_eq!(event.player_id.0, "player-1");
    assert_eq!(event.game_id.0, "game-1");
}

#[test]
fn draw_card_fails_when_not_player_turn() {
    let mut game = create_game_with_library_cards();

    let cmd = DrawCardCommand::new(PlayerId::new("player-2"));
    let result = GameService::draw_card(&mut game, cmd);

    assert!(matches!(result, Err(DomainError::NotYourTurn { .. })));
}

#[test]
fn draw_card_fails_when_library_empty() {
    let (mut game, _) = GameService::start_game(StartGameCommand::new(
        GameId::new("game-1"),
        vec![
            player_deck("player-1", "deck-1"),
            player_deck("player-2", "deck-2"),
        ],
    ))
    .unwrap();

    let cmd = DrawCardCommand::new(PlayerId::new("player-1"));
    let result = GameService::draw_card(&mut game, cmd);

    assert!(matches!(
        result,
        Err(DomainError::NotEnoughCardsInLibrary { .. })
    ));
}

#[test]
fn draw_card_works_in_main_phase() {
    let mut game = create_game_with_library_cards();

    let cmd = DrawCardCommand::new(PlayerId::new("player-1"));
    let result = GameService::draw_card(&mut game, cmd);

    assert!(result.is_ok());
}

#[test]
fn draw_card_allows_playing_land_after_draw() {
    let mut game = create_game_with_library_cards();

    let draw_cmd = DrawCardCommand::new(PlayerId::new("player-1"));
    GameService::draw_card(&mut game, draw_cmd).unwrap();

    let advance_cmd = AdvanceTurnCommand::new();
    GameService::advance_turn(&mut game, advance_cmd).unwrap();

    let land_cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-0"),
    );
    let result = GameService::play_land(&mut game, land_cmd);

    assert!(result.is_ok());
}
