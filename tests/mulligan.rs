#![allow(clippy::unwrap_used)]

use demonictutor::{
    AdvanceTurnCommand, CardDefinitionId, CardType, DealOpeningHandsCommand, DeckId, DomainError,
    GameId, GameService, InMemoryEventBus, InMemoryEventStore, MulliganCommand, PlayerDeck,
    PlayerDeckContents, PlayerId, StartGameCommand,
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

fn non_land_cards(count: usize) -> Vec<(String, CardType, u32)> {
    (0..count)
        .map(|i| (format!("card-{i}"), CardType::Creature, 0))
        .collect()
}

fn create_service() -> GameService<InMemoryEventStore, InMemoryEventBus> {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
}

#[test]
fn mulligan_succeeds() {
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

    assert_eq!(game.phase(), &demonictutor::Phase::Setup);

    let cmd = DealOpeningHandsCommand::new(vec![
        player_deck_contents("player-1", non_land_cards(14)),
        player_deck_contents("player-2", non_land_cards(14)),
    ]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    assert_eq!(game.phase(), &demonictutor::Phase::Setup);

    let mulligan_cmd = MulliganCommand::new(PlayerId::new("player-1"));
    let result = service.mulligan(&mut game, mulligan_cmd);

    assert!(result.is_ok());
}

#[test]
fn mulligan_fails_already_used() {
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

    assert_eq!(game.phase(), &demonictutor::Phase::Setup);

    let cmd = DealOpeningHandsCommand::new(vec![
        player_deck_contents("player-1", non_land_cards(14)),
        player_deck_contents("player-2", non_land_cards(14)),
    ]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    let mulligan_cmd = MulliganCommand::new(PlayerId::new("player-1"));
    service.mulligan(&mut game, mulligan_cmd).unwrap();

    let mulligan_cmd2 = MulliganCommand::new(PlayerId::new("player-1"));
    let result = service.mulligan(&mut game, mulligan_cmd2);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        DomainError::MulliganAlreadyUsed {
            player_id: PlayerId::new("player-1")
        }
    );
}

#[test]
fn mulligan_fails_not_enough_cards() {
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

    assert_eq!(game.phase(), &demonictutor::Phase::Setup);

    let mulligan_cmd = MulliganCommand::new(PlayerId::new("player-1"));
    let result = service.mulligan(&mut game, mulligan_cmd);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        DomainError::NotEnoughCardsInLibrary {
            available: 0,
            requested: 7,
            ..
        }
    ));
}

#[test]
fn mulligan_fails_not_setup_phase() {
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
        player_deck_contents("player-1", non_land_cards(14)),
        player_deck_contents("player-2", non_land_cards(14)),
    ]);

    service.deal_opening_hands(&mut game, &cmd).unwrap();

    assert_eq!(game.phase(), &demonictutor::Phase::Setup);

    let advance_cmd = AdvanceTurnCommand::new();
    service.advance_turn(&mut game, advance_cmd).unwrap();

    assert_eq!(game.phase(), &demonictutor::Phase::Main);

    let mulligan_cmd = MulliganCommand::new(PlayerId::new("player-1"));
    let result = service.mulligan(&mut game, mulligan_cmd);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DomainError::InvalidPhaseForMulligan);
}
