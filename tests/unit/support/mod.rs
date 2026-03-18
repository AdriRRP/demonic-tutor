#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]
#![allow(dead_code)]

use demonictutor::{
    AdvanceTurnCommand, AdvanceTurnOutcome, CardDefinitionId, CastSpellCommand,
    DealOpeningHandsCommand, DeckId, DeclareAttackersCommand, DiscardForCleanupCommand, Game,
    GameId, GameService, InMemoryEventBus, InMemoryEventStore, LibraryCard, NonCreatureCardType,
    PassPriorityCommand, Phase, PlayerDeck, PlayerId, PlayerLibrary, ResolveCombatDamageCommand,
    StartGameCommand,
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

pub fn sorcery_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::non_creature(
        CardDefinitionId::new(name),
        NonCreatureCardType::Sorcery,
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

pub fn enchantment_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::non_creature(
        CardDefinitionId::new(name),
        NonCreatureCardType::Enchantment,
        mana_cost,
    )
}

pub fn planeswalker_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::non_creature(
        CardDefinitionId::new(name),
        NonCreatureCardType::Planeswalker,
        mana_cost,
    )
}

pub fn vanilla_creature(name: &str) -> LibraryCard {
    LibraryCard::creature(CardDefinitionId::new(name), 0, 2, 2)
}

pub fn creature_card(name: &str, mana_cost: u32, power: u32, toughness: u32) -> LibraryCard {
    LibraryCard::creature(CardDefinitionId::new(name), mana_cost, power, toughness)
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

pub fn advance_to_phase_satisfying_cleanup(service: &TestService, game: &mut Game, phase: Phase) {
    for _ in 0..64 {
        if game.phase() == &phase {
            return;
        }

        advance_turn_satisfying_cleanup(service, game);
    }

    panic!("failed to reach phase {phase}");
}

pub fn advance_to_player_phase_satisfying_cleanup(
    service: &TestService,
    game: &mut Game,
    player_id: &str,
    phase: Phase,
) {
    let player_id = PlayerId::new(player_id);

    for _ in 0..64 {
        if game.active_player() == &player_id && game.phase() == &phase {
            return;
        }

        advance_turn_satisfying_cleanup(service, game);
    }

    panic!("failed to reach {phase} for {player_id}");
}

pub fn advance_turn_raw(service: &TestService, game: &mut Game) {
    close_empty_priority_window(service, game);

    let outcome = service
        .advance_turn(game, AdvanceTurnCommand::new())
        .unwrap();
    assert!(matches!(outcome, AdvanceTurnOutcome::Progressed { .. }));
}

pub fn close_empty_priority_window(service: &TestService, game: &mut Game) {
    if !game.has_open_priority_window() {
        return;
    }

    assert!(
        game.stack().is_empty(),
        "cannot close a priority window while the stack is still non-empty"
    );

    let first_holder = game.priority().map_or_else(
        || panic!("priority window should be open"),
        |p| p.current_holder().clone(),
    );
    service
        .pass_priority(game, PassPriorityCommand::new(first_holder))
        .unwrap();

    let second_holder = game.priority().map_or_else(
        || panic!("priority window should remain open after one pass"),
        |p| p.current_holder().clone(),
    );
    service
        .pass_priority(game, PassPriorityCommand::new(second_holder))
        .unwrap();
}

pub fn pass_priority_to_non_active_player_in_end_of_combat(service: &TestService, game: &mut Game) {
    advance_to_player_first_main_satisfying_cleanup(service, game, "player-1");

    let attacker_id = game
        .players()
        .iter()
        .find(|player| player.id() == &PlayerId::new("player-1"))
        .unwrap_or_else(|| panic!("player-1 should exist"))
        .hand()
        .cards()
        .iter()
        .find(|card| card.definition_id() == &CardDefinitionId::new("attacker"))
        .unwrap_or_else(|| panic!("attacker should exist in player-1 hand"))
        .id()
        .clone();

    service
        .cast_spell(
            game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(service, game);

    advance_to_player_first_main_satisfying_cleanup(service, game, "player-2");
    advance_to_player_first_main_satisfying_cleanup(service, game, "player-1");
    advance_turn_raw(service, game);
    close_empty_priority_window(service, game);
    advance_turn_raw(service, game);
    service
        .declare_attackers(
            game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id]),
        )
        .unwrap();
    close_empty_priority_window(service, game);
    advance_turn_raw(service, game);
    service
        .resolve_combat_damage(
            game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(game, PassPriorityCommand::new(PlayerId::new("player-1")))
        .unwrap();
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

pub fn resolve_top_stack_with_passes(service: &TestService, game: &mut Game) {
    let first_holder = game.priority().map_or_else(
        || panic!("priority window should be open"),
        |priority| priority.current_holder().clone(),
    );
    service
        .pass_priority(game, PassPriorityCommand::new(first_holder))
        .unwrap();

    let second_holder = game.priority().map_or_else(
        || panic!("priority window should remain open after one pass"),
        |priority| priority.current_holder().clone(),
    );
    service
        .pass_priority(game, PassPriorityCommand::new(second_holder))
        .unwrap();
}

pub fn cast_spell_and_resolve(
    service: &TestService,
    game: &mut Game,
    player_id: &str,
    card_id: demonictutor::CardInstanceId,
) {
    service
        .cast_spell(
            game,
            demonictutor::CastSpellCommand::new(PlayerId::new(player_id), card_id),
        )
        .unwrap();
    resolve_top_stack_with_passes(service, game);
    close_empty_priority_window(service, game);
}
