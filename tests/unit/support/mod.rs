#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]
#![allow(dead_code)]

use demonictutor::{
    AdvanceTurnCommand, AdvanceTurnOutcome, CardDefinitionId, DealOpeningHandsCommand, DeckId,
    DiscardForCleanupCommand, Game, GameId, GameService, InMemoryEventBus, InMemoryEventStore,
    LibraryCard, NonCreatureCardType, Phase, PlayerDeck, PlayerId, PlayerLibrary, StartGameCommand,
};

pub type TestService = GameService<InMemoryEventStore, InMemoryEventBus>;

pub fn create_service() -> TestService {
    GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new())
}

pub fn player_deck(player: &str, deck: &str) -> PlayerDeck {
    PlayerDeck::new(PlayerId::new(player), DeckId::new(deck))
}

pub fn player_library(player: &str, cards: Vec<LibraryCard>) -> PlayerLibrary {
    PlayerLibrary::new(PlayerId::new(player), cards)
}

pub fn land_card(name: &str) -> LibraryCard {
    LibraryCard::non_creature(CardDefinitionId::new(name), NonCreatureCardType::Land, 0)
}

pub fn instant_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::non_creature(
        CardDefinitionId::new(name),
        NonCreatureCardType::Instant,
        mana_cost,
    )
}

pub fn artifact_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::non_creature(
        CardDefinitionId::new(name),
        NonCreatureCardType::Artifact,
        mana_cost,
    )
}

pub fn vanilla_creature(name: &str) -> LibraryCard {
    LibraryCard::creature(CardDefinitionId::new(name), 0, 2, 2)
}

pub fn filled_library(seed_cards: Vec<LibraryCard>, total_cards: usize) -> Vec<LibraryCard> {
    assert!(seed_cards.len() <= total_cards);

    let mut cards = seed_cards;
    for i in cards.len() + 1..=total_cards {
        cards.push(vanilla_creature(&format!("card-{i}")));
    }

    cards
}

pub fn creature_library(total_cards: usize) -> Vec<LibraryCard> {
    filled_library(Vec::new(), total_cards)
}

pub fn start_two_player_game(service: &TestService, game_id: &str) -> Game {
    service
        .start_game(StartGameCommand::new(
            GameId::new(game_id),
            vec![
                player_deck("player-1", "deck-1"),
                player_deck("player-2", "deck-2"),
            ],
        ))
        .unwrap()
        .0
}

pub fn deal_opening_hands(
    service: &TestService,
    game: &mut Game,
    player_one_cards: Vec<LibraryCard>,
    player_two_cards: Vec<LibraryCard>,
) {
    service
        .deal_opening_hands(
            game,
            &DealOpeningHandsCommand::new(vec![
                player_library("player-1", player_one_cards),
                player_library("player-2", player_two_cards),
            ]),
        )
        .unwrap();
}

pub fn setup_two_player_game(
    game_id: &str,
    player_one_cards: Vec<LibraryCard>,
    player_two_cards: Vec<LibraryCard>,
) -> (TestService, Game) {
    let service = create_service();
    let mut game = start_two_player_game(&service, game_id);
    deal_opening_hands(&service, &mut game, player_one_cards, player_two_cards);
    (service, game)
}

pub fn advance_n_raw(service: &TestService, game: &mut Game, turns: usize) {
    for _ in 0..turns {
        advance_turn_raw(service, game);
    }
}

pub fn advance_n_satisfying_cleanup(service: &TestService, game: &mut Game, turns: usize) {
    for _ in 0..turns {
        advance_turn_satisfying_cleanup(service, game);
    }
}

pub fn advance_to_first_main_raw(service: &TestService, game: &mut Game) {
    while game.phase() != &Phase::FirstMain {
        advance_turn_raw(service, game);
    }
}

pub fn advance_to_first_main_satisfying_cleanup(service: &TestService, game: &mut Game) {
    while game.phase() != &Phase::FirstMain {
        advance_turn_satisfying_cleanup(service, game);
    }
}

pub fn advance_to_player_first_main_raw(service: &TestService, game: &mut Game, player_id: &str) {
    let player_id = PlayerId::new(player_id);

    for _ in 0..32 {
        if game.active_player() == &player_id && game.phase() == &Phase::FirstMain {
            return;
        }

        advance_turn_raw(service, game);
    }

    panic!("failed to reach FirstMain for {player_id}");
}

pub fn advance_to_player_first_main_satisfying_cleanup(
    service: &TestService,
    game: &mut Game,
    player_id: &str,
) {
    let player_id = PlayerId::new(player_id);

    for _ in 0..32 {
        if game.active_player() == &player_id && game.phase() == &Phase::FirstMain {
            return;
        }

        advance_turn_satisfying_cleanup(service, game);
    }

    panic!("failed to reach FirstMain for {player_id}");
}

pub fn advance_turn_raw(service: &TestService, game: &mut Game) {
    let outcome = service
        .advance_turn(game, AdvanceTurnCommand::new())
        .unwrap();
    assert!(matches!(outcome, AdvanceTurnOutcome::Progressed { .. }));
}

pub fn satisfy_cleanup_discard(service: &TestService, game: &mut Game) {
    while game.phase() == &Phase::EndStep {
        let active_player = game.active_player().clone();
        let active_hand_size = game
            .players()
            .iter()
            .find(|player| player.id() == &active_player)
            .unwrap_or_else(|| panic!("active player should exist: {active_player}"))
            .hand()
            .cards()
            .len();

        if active_hand_size <= 7 {
            break;
        }

        let card_id = game
            .players()
            .iter()
            .find(|player| player.id() == &active_player)
            .unwrap_or_else(|| panic!("active player should exist: {active_player}"))
            .hand()
            .cards()[0]
            .id()
            .clone();

        service
            .discard_for_cleanup(game, DiscardForCleanupCommand::new(active_player, card_id))
            .unwrap();
    }
}

pub fn advance_turn_satisfying_cleanup(service: &TestService, game: &mut Game) {
    satisfy_cleanup_discard(service, game);
    advance_turn_raw(service, game);
}
