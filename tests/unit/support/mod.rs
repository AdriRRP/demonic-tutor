#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]
#![allow(dead_code)]

//! Unit coverage for unit support.

use {
    demonictutor::{ActivatedAbilityProfile, CastingRule, TriggeredAbilityProfile},
    demonictutor::{
        AdvanceTurnCommand, AdvanceTurnOutcome, CardDefinitionId, CardType, CastSpellCommand,
        DealOpeningHandsCommand, DeckId, DeclareAttackersCommand, DiscardForCleanupCommand, Game,
        GameId, GameService, InMemoryEventBus, InMemoryEventStore, KeywordAbility,
        KeywordAbilitySet, LibraryCard, ManaColor, ManaCost, PassPriorityCommand, Phase,
        PlayerDeck, PlayerId, PlayerLibrary, ResolveCombatDamageCommand, StartGameCommand,
        SupportedSpellRules,
    },
};

pub type TestService = GameService<InMemoryEventStore, InMemoryEventBus>;

pub fn player<'a>(game: &'a Game, player_id: &str) -> &'a demonictutor::domain::play::game::Player {
    let player_id = PlayerId::new(player_id);
    game.players()
        .iter()
        .find(|player| player.id() == &player_id)
        .unwrap_or_else(|| panic!("player should exist: {player_id}"))
}

pub fn first_hand_card_id(game: &Game, player_id: &str) -> demonictutor::CardInstanceId {
    player(game, player_id)
        .hand_card_at(0)
        .unwrap_or_else(|| panic!("first hand card should exist for {player_id}"))
        .id()
        .clone()
}

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
    LibraryCard::new(CardDefinitionId::new(name), CardType::Land, 0)
}

pub fn forest_card(name: &str) -> LibraryCard {
    LibraryCard::land(CardDefinitionId::new(name), ManaColor::Green)
}

pub fn mountain_card(name: &str) -> LibraryCard {
    LibraryCard::land(CardDefinitionId::new(name), ManaColor::Red)
}

pub fn plains_card(name: &str) -> LibraryCard {
    LibraryCard::land(CardDefinitionId::new(name), ManaColor::White)
}

pub fn island_card(name: &str) -> LibraryCard {
    LibraryCard::land(CardDefinitionId::new(name), ManaColor::Blue)
}

pub fn swamp_card(name: &str) -> LibraryCard {
    LibraryCard::land(CardDefinitionId::new(name), ManaColor::Black)
}

pub fn instant_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
}

pub fn green_instant_card(name: &str, green_requirement: u32) -> LibraryCard {
    LibraryCard::new(
        CardDefinitionId::new(name),
        CardType::Instant,
        green_requirement,
    )
    .with_mana_cost(ManaCost::green(green_requirement))
}

pub fn mixed_green_instant_card(name: &str, generic_amount: u32) -> LibraryCard {
    let total_cost = generic_amount + 1;
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, total_cost).with_mana_cost(
        ManaCost::generic_plus_single_color(generic_amount, ManaColor::Green, 1),
    )
}

pub fn double_green_instant_card(name: &str) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, 2)
        .with_mana_cost(ManaCost::green(2))
}

pub fn targeted_damage_instant_card(name: &str, mana_cost: u32, damage: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::deal_damage_to_any_target(damage))
}

pub fn targeted_player_damage_instant_card(name: &str, mana_cost: u32, damage: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::deal_damage_to_player(damage))
}

pub fn targeted_gain_life_instant_card(name: &str, mana_cost: u32, amount: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::gain_life_to_player(amount))
}

pub fn targeted_lose_life_instant_card(name: &str, mana_cost: u32, amount: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::lose_life_from_player(amount))
}

pub fn targeted_opponent_damage_instant_card(
    name: &str,
    mana_cost: u32,
    damage: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::deal_damage_to_opponent(damage))
}

pub fn targeted_controlled_creature_damage_instant_card(
    name: &str,
    mana_cost: u32,
    damage: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::deal_damage_to_controlled_creature(
            damage,
        ))
}

pub fn targeted_opponents_creature_damage_instant_card(
    name: &str,
    mana_cost: u32,
    damage: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::deal_damage_to_opponents_creature(
            damage,
        ))
}

pub fn targeted_attacking_creature_damage_instant_card(
    name: &str,
    mana_cost: u32,
    damage: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::deal_damage_to_attacking_creature(
            damage,
        ))
}

pub fn targeted_blocking_creature_damage_instant_card(
    name: &str,
    mana_cost: u32,
    damage: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::deal_damage_to_blocking_creature(
            damage,
        ))
}

pub fn targeted_controlled_blocking_creature_damage_instant_card(
    name: &str,
    mana_cost: u32,
    damage: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(
            SupportedSpellRules::deal_damage_to_controlled_blocking_creature(damage),
        )
}

pub fn targeted_opponents_attacking_creature_damage_instant_card(
    name: &str,
    mana_cost: u32,
    damage: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(
            SupportedSpellRules::deal_damage_to_opponents_attacking_creature(damage),
        )
}

pub fn targeted_controlled_attacking_creature_damage_instant_card(
    name: &str,
    mana_cost: u32,
    damage: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(
            SupportedSpellRules::deal_damage_to_controlled_attacking_creature(damage),
        )
}

pub fn targeted_opponents_blocking_creature_damage_instant_card(
    name: &str,
    mana_cost: u32,
    damage: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(
            SupportedSpellRules::deal_damage_to_opponents_blocking_creature(damage),
        )
}

pub fn targeted_destroy_creature_instant_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::destroy_target_creature())
}

pub fn counter_target_spell_instant_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::counter_target_spell())
}

pub fn return_target_permanent_to_hand_instant_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::return_target_permanent_to_hand())
}

pub fn destroy_target_artifact_or_enchantment_instant_card(
    name: &str,
    mana_cost: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::destroy_target_artifact_or_enchantment())
}

pub fn target_player_discards_chosen_card_sorcery_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Sorcery, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::target_player_discards_chosen_card())
}

pub fn targeted_exile_creature_instant_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::exile_target_creature())
}

pub fn targeted_exile_graveyard_card_instant_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::exile_target_card_from_graveyard())
}

pub fn targeted_pump_creature_instant_card(
    name: &str,
    mana_cost: u32,
    power: u32,
    toughness: u32,
) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Instant, mana_cost)
        .with_supported_spell_rules(SupportedSpellRules::pump_target_creature_until_end_of_turn(
            power, toughness,
        ))
}

pub fn sorcery_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Sorcery, mana_cost)
}

pub fn artifact_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Artifact, mana_cost)
}

pub fn life_gain_artifact_card(name: &str, mana_cost: u32, amount: u32) -> LibraryCard {
    artifact_card(name, mana_cost).with_activated_ability(
        ActivatedAbilityProfile::tap_to_gain_life_to_controller(amount),
    )
}

pub fn targeted_life_gain_artifact_card(name: &str, mana_cost: u32, amount: u32) -> LibraryCard {
    artifact_card(name, mana_cost).with_activated_ability(
        ActivatedAbilityProfile::tap_to_gain_life_to_target_player(amount),
    )
}

pub fn mana_costed_life_gain_artifact_card(
    name: &str,
    mana_cost: u32,
    activation_cost: u32,
    amount: u32,
) -> LibraryCard {
    artifact_card(name, mana_cost).with_activated_ability(
        ActivatedAbilityProfile::tap_to_gain_life_to_controller(amount)
            .with_mana_cost(ManaCost::generic(activation_cost)),
    )
}

pub fn sacrifice_life_gain_artifact_card(name: &str, mana_cost: u32, amount: u32) -> LibraryCard {
    artifact_card(name, mana_cost).with_activated_ability(
        ActivatedAbilityProfile::tap_sacrifice_source_to_gain_life_to_controller(amount),
    )
}

pub fn etb_life_gain_creature_card(
    name: &str,
    mana_cost: u32,
    power: u32,
    toughness: u32,
    amount: u32,
) -> LibraryCard {
    creature_card(name, mana_cost, power, toughness).with_triggered_ability(
        TriggeredAbilityProfile::enters_battlefield_gain_life_to_controller(amount),
    )
}

pub fn dies_life_gain_creature_card(
    name: &str,
    mana_cost: u32,
    power: u32,
    toughness: u32,
    amount: u32,
) -> LibraryCard {
    creature_card(name, mana_cost, power, toughness).with_triggered_ability(
        TriggeredAbilityProfile::dies_gain_life_to_controller(amount),
    )
}

pub fn upkeep_life_gain_artifact_card(name: &str, mana_cost: u32, amount: u32) -> LibraryCard {
    artifact_card(name, mana_cost).with_triggered_ability(
        TriggeredAbilityProfile::beginning_of_upkeep_gain_life_to_controller(amount),
    )
}

pub fn end_step_life_gain_artifact_card(name: &str, mana_cost: u32, amount: u32) -> LibraryCard {
    artifact_card(name, mana_cost).with_triggered_ability(
        TriggeredAbilityProfile::beginning_of_end_step_gain_life_to_controller(amount),
    )
}

pub fn enchantment_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(
        CardDefinitionId::new(name),
        CardType::Enchantment,
        mana_cost,
    )
}

pub fn planeswalker_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(
        CardDefinitionId::new(name),
        CardType::Planeswalker,
        mana_cost,
    )
}

pub fn loyalty_planeswalker_card(
    name: &str,
    mana_cost: u32,
    loyalty: u32,
    loyalty_change: i32,
    amount: u32,
) -> LibraryCard {
    LibraryCard::new(
        CardDefinitionId::new(name),
        CardType::Planeswalker,
        mana_cost,
    )
    .with_initial_loyalty(loyalty)
    .with_activated_ability(ActivatedAbilityProfile::loyalty_gain_life_to_controller(
        loyalty_change,
        amount,
    ))
}

pub fn vanilla_creature(name: &str) -> LibraryCard {
    LibraryCard::creature(CardDefinitionId::new(name), 0, 2, 2)
}

pub fn creature_card(name: &str, mana_cost: u32, power: u32, toughness: u32) -> LibraryCard {
    LibraryCard::creature(CardDefinitionId::new(name), mana_cost, power, toughness)
}

pub fn flash_creature_card(name: &str, mana_cost: u32, power: u32, toughness: u32) -> LibraryCard {
    LibraryCard::creature(CardDefinitionId::new(name), mana_cost, power, toughness)
        .with_casting_rule(CastingRule::OpenPriorityWindow)
}

pub fn flash_artifact_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Artifact, mana_cost)
        .with_casting_rule(CastingRule::OpenPriorityWindow)
}

pub fn flash_enchantment_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(
        CardDefinitionId::new(name),
        CardType::Enchantment,
        mana_cost,
    )
    .with_casting_rule(CastingRule::OpenPriorityWindow)
}

pub fn flash_planeswalker_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(
        CardDefinitionId::new(name),
        CardType::Planeswalker,
        mana_cost,
    )
    .with_casting_rule(CastingRule::OpenPriorityWindow)
}

pub fn own_turn_priority_artifact_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(CardDefinitionId::new(name), CardType::Artifact, mana_cost)
        .with_casting_rule(CastingRule::OpenPriorityWindowDuringOwnTurn)
}

pub fn own_turn_priority_enchantment_card(name: &str, mana_cost: u32) -> LibraryCard {
    LibraryCard::new(
        CardDefinitionId::new(name),
        CardType::Enchantment,
        mana_cost,
    )
    .with_casting_rule(CastingRule::OpenPriorityWindowDuringOwnTurn)
}

pub fn creature_card_with_keywords(
    name: &str,
    mana_cost: u32,
    power: u32,
    toughness: u32,
    flying: bool,
    reach: bool,
) -> LibraryCard {
    let mut keyword_abilities = KeywordAbilitySet::empty();
    if flying {
        keyword_abilities = keyword_abilities.with(KeywordAbility::Flying);
    }
    if reach {
        keyword_abilities = keyword_abilities.with(KeywordAbility::Reach);
    }

    LibraryCard::creature_with_keywords(
        CardDefinitionId::new(name),
        mana_cost,
        power,
        toughness,
        keyword_abilities,
    )
}

pub fn creature_card_with_keyword(
    name: &str,
    mana_cost: u32,
    power: u32,
    toughness: u32,
    keyword: KeywordAbility,
) -> LibraryCard {
    LibraryCard::creature_with_keywords(
        CardDefinitionId::new(name),
        mana_cost,
        power,
        toughness,
        KeywordAbilitySet::only(keyword),
    )
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

pub fn pass_priority(service: &TestService, game: &mut Game, player_id: &str) {
    service
        .pass_priority(game, PassPriorityCommand::new(PlayerId::new(player_id)))
        .unwrap();
}

pub fn pass_priority_to_non_active_player_in_end_of_combat(service: &TestService, game: &mut Game) {
    advance_to_player_first_main_satisfying_cleanup(service, game, "player-1");

    let attacker_id = game
        .players()
        .iter()
        .find(|player| player.id() == &PlayerId::new("player-1"))
        .unwrap_or_else(|| panic!("player-1 should exist"))
        .hand_card_by_definition(&CardDefinitionId::new("attacker"))
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
            .hand_size();

        if active_hand_size <= 7 {
            break;
        }

        let card_id = game
            .players()
            .iter()
            .find(|player| player.id() == &active_player)
            .unwrap_or_else(|| panic!("active player should exist: {active_player}"))
            .hand_card_at(0)
            .unwrap_or_else(|| panic!("active player should have a hand card: {active_player}"))
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
