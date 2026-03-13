#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardInstanceId, CardType, DealOpeningHandsCommand,
    DeckId, GameId, GameService, PlayLandCommand, PlayerDeck, PlayerDeckContents, PlayerId,
    StartGameCommand,
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

fn create_game_with_land_in_hand() -> demonictutor::Game {
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
fn advance_turn_changes_active_player() {
    let mut game = create_game_with_land_in_hand();

    assert_eq!(game.active_player().0, "player-1");

    let cmd = AdvanceTurnCommand::new();
    let result = GameService::advance_turn(&mut game, cmd).unwrap();

    assert_eq!(result.new_active_player.0, "player-2");
}

#[test]
fn advance_turn_emits_event() {
    let mut game = create_game_with_land_in_hand();

    let cmd = AdvanceTurnCommand::new();
    let result = GameService::advance_turn(&mut game, cmd).unwrap();

    assert_eq!(result.game_id.0, "game-1");
    assert_eq!(result.new_active_player.0, "player-2");
}

#[test]
fn advance_turn_resets_lands_played() {
    let mut game = create_game_with_land_in_hand();

    let advance_cmd = AdvanceTurnCommand::new();
    GameService::advance_turn(&mut game, advance_cmd).unwrap();

    let land_cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-0"),
    );
    GameService::play_land(&mut game, land_cmd).unwrap();

    assert_eq!(game.players()[1].lands_played_this_turn(), 1);

    let advance_cmd2 = AdvanceTurnCommand::new();
    GameService::advance_turn(&mut game, advance_cmd2).unwrap();

    assert_eq!(game.players()[0].lands_played_this_turn(), 0);
    assert_eq!(game.players()[1].lands_played_this_turn(), 0);
}

#[test]
fn advance_turn_allows_playing_land_after_turn_change() {
    let mut game = create_game_with_land_in_hand();

    let advance_cmd = AdvanceTurnCommand::new();
    GameService::advance_turn(&mut game, advance_cmd).unwrap();

    let land_cmd = PlayLandCommand::new(
        PlayerId::new("player-2"),
        CardInstanceId::new("game-1-player-2-0"),
    );
    let result = GameService::play_land(&mut game, land_cmd);

    assert!(result.is_ok());

    let advance_cmd2 = AdvanceTurnCommand::new();
    GameService::advance_turn(&mut game, advance_cmd2).unwrap();

    let land_cmd_player2 = PlayLandCommand::new(
        PlayerId::new("player-1"),
        CardInstanceId::new("nonexistent"),
    );
    let result2 = GameService::play_land(&mut game, land_cmd_player2);

    assert!(result2.is_err());
}
