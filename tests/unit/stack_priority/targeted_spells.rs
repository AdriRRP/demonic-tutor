#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority targeted spells.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, advance_to_player_first_main_satisfying_cleanup,
        advance_to_player_phase_satisfying_cleanup, advance_turn_raw, artifact_card,
        cannot_block_target_creature_instant_card,
        choose_one_target_player_gain_or_lose_life_instant_card, close_empty_priority_window,
        counter_target_spell_instant_card, creature_aura_enchantment_card, creature_card,
        creature_card_with_keyword, destroy_target_artifact_or_enchantment_instant_card,
        enchantment_card, filled_library, land_card, resolve_top_stack_with_passes,
        return_target_permanent_to_hand_instant_card, setup_two_player_game,
        stat_boost_creature_aura_enchantment_card, tap_target_creature_instant_card,
        target_player_discards_chosen_card_sorcery_card,
        targeted_attacking_creature_damage_instant_card,
        targeted_controlled_creature_damage_instant_card, targeted_damage_instant_card,
        targeted_destroy_creature_instant_card, targeted_exile_creature_instant_card,
        targeted_exile_graveyard_card_instant_card, targeted_gain_life_instant_card,
        targeted_lose_life_instant_card, targeted_opponent_damage_instant_card,
        targeted_opponents_creature_damage_instant_card, targeted_player_damage_instant_card,
        targeted_pump_creature_instant_card, untap_target_creature_instant_card,
    },
    demonictutor::{
        CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand, DiscardKind,
        DomainError, GameEndReason, GameError, KeywordAbility, LibraryCard, ModalSpellMode,
        PlayerId, ResolveCombatDamageCommand, SpellCastOutcome, SpellChoice, SpellTarget,
    },
};

fn resolve_current_stack(
    service: &demonictutor::GameService<
        demonictutor::InMemoryEventStore,
        demonictutor::InMemoryEventBus,
    >,
    game: &mut demonictutor::Game,
) -> demonictutor::PassPriorityOutcome {
    let first_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, demonictutor::PassPriorityCommand::new(first_holder))
        .unwrap();

    let second_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, demonictutor::PassPriorityCommand::new(second_holder))
        .unwrap()
}

fn pass_priority_once(
    service: &demonictutor::GameService<
        demonictutor::InMemoryEventStore,
        demonictutor::InMemoryEventBus,
    >,
    game: &mut demonictutor::Game,
) -> demonictutor::PassPriorityOutcome {
    let holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, demonictutor::PassPriorityCommand::new(holder))
        .unwrap()
}

fn hand_card_id_by_definition(
    game: &demonictutor::Game,
    player_index: usize,
    definition_id: &str,
) -> CardInstanceId {
    game.players()[player_index]
        .hand_card_by_definition(&CardDefinitionId::new(definition_id))
        .unwrap()
        .id()
        .clone()
}

#[test]
fn targeted_instant_requires_a_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-missing",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-target-missing-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::MissingSpellTarget(card_id)))
            if card_id == CardInstanceId::new("game-target-missing-player-1-0")
    ));
}

#[test]
fn targeted_instant_rejects_unknown_player_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-player-invalid",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-target-player-invalid-player-1-0"),
        )
        .with_target(SpellTarget::Player(PlayerId::new("missing-player"))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::InvalidPlayerTarget(player_id)))
            if player_id == PlayerId::new("missing-player")
    ));
}

#[test]
fn creature_aura_requires_a_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-aura-target-missing",
        filled_library(vec![creature_aura_enchantment_card("holy-strength", 0)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-aura-target-missing-player-1-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::MissingSpellTarget(card_id)))
            if card_id == CardInstanceId::new("game-aura-target-missing-player-1-0")
    ));
}

#[test]
fn creature_aura_goes_to_graveyard_if_target_is_missing_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-aura-fizzles",
        filled_library(
            vec![
                creature_card("silvercoat", 0, 2, 2),
                creature_aura_enchantment_card("holy-strength", 0),
            ],
            10,
        ),
        filled_library(
            vec![targeted_destroy_creature_instant_card("murder-lite", 0)],
            10,
        ),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-aura-fizzles-player-1-0"),
            ),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    let creature =
        game.players()[0].battlefield_card_by_definition(&CardDefinitionId::new("silvercoat"));
    assert!(creature.is_some(), "creature should be on battlefield");
    let creature_id = creature.map(|card| card.id().clone());
    let Some(creature_id) = creature_id else {
        return;
    };
    let aura = game.players()[0].hand_card_by_definition(&CardDefinitionId::new("holy-strength"));
    assert!(aura.is_some(), "aura should be in hand");
    let aura_id = aura.map(|card| card.id().clone());
    let Some(aura_id) = aura_id else {
        return;
    };

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), aura_id.clone())
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();
    let _ = pass_priority_once(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-aura-fizzles-player-2-0"),
            )
            .with_target(SpellTarget::Creature(creature_id)),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);
    let _ = resolve_current_stack(&service, &mut game);

    assert!(game.players()[0].battlefield_card(&aura_id).is_none());
    assert!(game.players()[0].graveyard_card(&aura_id).is_some());
}

#[test]
fn creature_aura_is_put_into_graveyard_when_enchanted_creature_dies() {
    let (service, mut game) = setup_two_player_game(
        "game-aura-detaches",
        filled_library(
            vec![
                creature_card("silvercoat", 0, 2, 2),
                creature_aura_enchantment_card("holy-strength", 0),
            ],
            10,
        ),
        filled_library(
            vec![targeted_destroy_creature_instant_card("murder-lite", 0)],
            10,
        ),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-aura-detaches-player-1-0"),
            ),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    let creature =
        game.players()[0].battlefield_card_by_definition(&CardDefinitionId::new("silvercoat"));
    assert!(creature.is_some(), "creature should be on battlefield");
    let creature_id = creature.map(|card| card.id().clone());
    let Some(creature_id) = creature_id else {
        return;
    };
    let aura = game.players()[0].hand_card_by_definition(&CardDefinitionId::new("holy-strength"));
    assert!(aura.is_some(), "aura should be in hand");
    let aura_id = aura.map(|card| card.id().clone());
    let Some(aura_id) = aura_id else {
        return;
    };

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), aura_id.clone())
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    assert!(game.players()[0].battlefield_card(&aura_id).is_some());
    let _ = pass_priority_once(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-aura-detaches-player-2-0"),
            )
            .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    assert!(game.players()[0].battlefield_card(&creature_id).is_none());
    assert!(game.players()[0].battlefield_card(&aura_id).is_none());
    assert!(game.players()[0].graveyard_card(&creature_id).is_some());
    assert!(game.players()[0].graveyard_card(&aura_id).is_some());
}

#[test]
fn stat_boost_aura_increases_enchanted_creature_stats_while_attached() {
    let (service, mut game) = setup_two_player_game(
        "game-aura-boosts",
        filled_library(
            vec![
                creature_card("silvercoat", 0, 2, 2),
                stat_boost_creature_aura_enchantment_card("holy-strength", 0, 2, 2),
            ],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-aura-boosts-player-1-0"),
            ),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    let creature_id = game.players()[0]
        .battlefield_card_by_definition(&CardDefinitionId::new("silvercoat"))
        .map(|card| card.id().clone());
    assert!(creature_id.is_some(), "creature should be on battlefield");
    let Some(creature_id) = creature_id else {
        return;
    };
    let aura_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("holy-strength"))
        .map(|card| card.id().clone());
    assert!(aura_id.is_some(), "aura should be in hand");
    let Some(aura_id) = aura_id else {
        return;
    };

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), aura_id.clone())
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    let creature = game.players()[0].battlefield_card(&creature_id);
    assert_eq!(
        creature.and_then(demonictutor::CardInstance::creature_stats),
        Some((4, 4))
    );
    assert_eq!(
        game.players()[0]
            .battlefield_card(&aura_id)
            .and_then(|card| card.attached_to().cloned()),
        Some(creature_id)
    );
}

#[test]
fn stat_boost_aura_bonus_is_removed_when_the_aura_leaves_battlefield() {
    let (service, mut game) = setup_two_player_game(
        "game-aura-bonus-removed",
        filled_library(
            vec![
                creature_card("silvercoat", 0, 2, 2),
                stat_boost_creature_aura_enchantment_card("holy-strength", 0, 2, 2),
            ],
            10,
        ),
        filled_library(
            vec![destroy_target_artifact_or_enchantment_instant_card(
                "disenchant-lite",
                0,
            )],
            10,
        ),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-aura-bonus-removed-player-1-0"),
            ),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    let creature_id = game.players()[0]
        .battlefield_card_by_definition(&CardDefinitionId::new("silvercoat"))
        .map(|card| card.id().clone());
    assert!(creature_id.is_some(), "creature should be on battlefield");
    let Some(creature_id) = creature_id else {
        return;
    };
    let aura_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("holy-strength"))
        .map(|card| card.id().clone());
    assert!(aura_id.is_some(), "aura should be in hand");
    let Some(aura_id) = aura_id else {
        return;
    };

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), aura_id.clone())
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    assert_eq!(
        game.players()[0]
            .battlefield_card(&creature_id)
            .and_then(demonictutor::CardInstance::creature_stats),
        Some((4, 4))
    );

    let _ = pass_priority_once(&service, &mut game);
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-aura-bonus-removed-player-2-0"),
            )
            .with_target(SpellTarget::Permanent(aura_id.clone())),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    assert!(game.players()[0].battlefield_card(&aura_id).is_none());
    assert!(game.players()[0].graveyard_card(&aura_id).is_some());
    assert_eq!(
        game.players()[0]
            .battlefield_card(&creature_id)
            .and_then(demonictutor::CardInstance::creature_stats),
        Some((2, 2))
    );
}

#[test]
fn modal_choose_one_spell_requires_a_selected_mode_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-modal-choice-missing",
        filled_library(
            vec![choose_one_target_player_gain_or_lose_life_instant_card(
                "choice-life",
                0,
                3,
                2,
            )],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-modal-choice-missing-player-1-0"),
        )
        .with_target(SpellTarget::Player(PlayerId::new("player-1"))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::MissingSpellChoice(card_id)))
            if card_id == CardInstanceId::new("game-modal-choice-missing-player-1-0")
    ));
}

#[test]
fn modal_choose_one_spell_can_gain_life_to_the_target_player() {
    let (service, mut game) = setup_two_player_game(
        "game-modal-choice-gain",
        filled_library(
            vec![choose_one_target_player_gain_or_lose_life_instant_card(
                "choice-life",
                0,
                3,
                2,
            )],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-modal-choice-gain-player-1-0"),
            )
            .with_target(SpellTarget::Player(PlayerId::new("player-1")))
            .with_choice(SpellChoice::ModalMode(ModalSpellMode::TargetPlayerGainLife)),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);

    assert_eq!(game.players()[0].life(), 23);
    assert_eq!(game.players()[1].life(), 20);
}

#[test]
fn modal_choose_one_spell_can_make_the_target_player_lose_life() {
    let (service, mut game) = setup_two_player_game(
        "game-modal-choice-lose",
        filled_library(
            vec![choose_one_target_player_gain_or_lose_life_instant_card(
                "choice-life",
                0,
                3,
                2,
            )],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-modal-choice-lose-player-1-0"),
            )
            .with_target(SpellTarget::Player(PlayerId::new("player-2")))
            .with_choice(SpellChoice::ModalMode(ModalSpellMode::TargetPlayerLoseLife)),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);

    assert_eq!(game.players()[0].life(), 20);
    assert_eq!(game.players()[1].life(), 18);
}

#[test]
fn tap_target_creature_spell_taps_the_target_creature_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-tap-target-creature",
        filled_library(
            vec![
                creature_card("silvercoat", 0, 2, 2),
                tap_target_creature_instant_card("frost-breath-lite", 0),
            ],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-tap-target-creature-player-1-0"),
            ),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    let creature_id = game.players()[0]
        .battlefield_card_by_definition(&CardDefinitionId::new("silvercoat"))
        .map(|card| card.id().clone());
    assert!(creature_id.is_some(), "creature should be on battlefield");
    let Some(creature_id) = creature_id else {
        return;
    };
    let tap_spell_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("frost-breath-lite"))
        .map(|card| card.id().clone());
    assert!(tap_spell_id.is_some(), "tap spell should be in hand");
    let Some(tap_spell_id) = tap_spell_id else {
        return;
    };

    assert!(!game.players()[0]
        .battlefield_card(&creature_id)
        .is_some_and(demonictutor::CardInstance::is_tapped));

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), tap_spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);

    assert!(game.players()[0]
        .battlefield_card(&creature_id)
        .is_some_and(demonictutor::CardInstance::is_tapped));
}

#[test]
fn tap_target_creature_spell_does_not_apply_if_target_is_gone_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-tap-target-creature-fizzles",
        filled_library(
            vec![
                creature_card("silvercoat", 0, 2, 2),
                tap_target_creature_instant_card("frost-breath-lite", 0),
            ],
            10,
        ),
        filled_library(
            vec![targeted_destroy_creature_instant_card("murder-lite", 0)],
            10,
        ),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-tap-target-creature-fizzles-player-1-0"),
            ),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    let creature_id = game.players()[0]
        .battlefield_card_by_definition(&CardDefinitionId::new("silvercoat"))
        .map(|card| card.id().clone());
    assert!(creature_id.is_some(), "creature should be on battlefield");
    let Some(creature_id) = creature_id else {
        return;
    };
    let tap_spell_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("frost-breath-lite"))
        .map(|card| card.id().clone());
    assert!(tap_spell_id.is_some(), "tap spell should be in hand");
    let Some(tap_spell_id) = tap_spell_id else {
        return;
    };

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), tap_spell_id.clone())
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();
    let _ = pass_priority_once(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-2"),
                CardInstanceId::new("game-tap-target-creature-fizzles-player-2-0"),
            )
            .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);
    let _ = resolve_current_stack(&service, &mut game);

    assert!(game.players()[0].battlefield_card(&creature_id).is_none());
    assert!(game.players()[0].graveyard_card(&creature_id).is_some());
    assert!(game.players()[0].graveyard_card(&tap_spell_id).is_some());
}

#[test]
fn untap_target_creature_spell_untaps_the_target_creature_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-untap-target-creature",
        filled_library(
            vec![
                creature_card("silvercoat", 0, 2, 2),
                tap_target_creature_instant_card("frost-breath-lite", 0),
                untap_target_creature_instant_card("battlefield-reprieve", 0),
            ],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-untap-target-creature-player-1-0"),
            ),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    let creature_id = game.players()[0]
        .battlefield_card_by_definition(&CardDefinitionId::new("silvercoat"))
        .map(|card| card.id().clone());
    assert!(creature_id.is_some(), "creature should be on battlefield");
    let Some(creature_id) = creature_id else {
        return;
    };
    let tap_spell_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("frost-breath-lite"))
        .map(|card| card.id().clone());
    assert!(tap_spell_id.is_some(), "tap spell should be in hand");
    let Some(tap_spell_id) = tap_spell_id else {
        return;
    };
    let untap_spell_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("battlefield-reprieve"))
        .map(|card| card.id().clone());
    assert!(untap_spell_id.is_some(), "untap spell should be in hand");
    let Some(untap_spell_id) = untap_spell_id else {
        return;
    };

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), tap_spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    assert!(game.players()[0]
        .battlefield_card(&creature_id)
        .is_some_and(demonictutor::CardInstance::is_tapped));

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), untap_spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);

    assert!(!game.players()[0]
        .battlefield_card(&creature_id)
        .is_some_and(demonictutor::CardInstance::is_tapped));
}

#[test]
fn cannot_block_target_creature_spell_prevents_blocking_for_the_rest_of_turn() {
    let (service, mut game) = setup_two_player_game(
        "game-cannot-block-target-creature",
        filled_library(
            vec![
                creature_card("attacker", 0, 2, 2),
                cannot_block_target_creature_instant_card("falter-ping", 0),
            ],
            10,
        ),
        filled_library(vec![creature_card("blocker", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let attacker_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("attacker"))
        .map(|card| card.id().clone());
    assert!(attacker_id.is_some(), "attacker should be in hand");
    let Some(attacker_id) = attacker_id else {
        return;
    };
    let spell_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("falter-ping"))
        .map(|card| card.id().clone());
    assert!(spell_id.is_some(), "spell should be in hand");
    let Some(spell_id) = spell_id else {
        return;
    };
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let blocker_id = game.players()[1]
        .hand_card_by_definition(&CardDefinitionId::new("blocker"))
        .map(|card| card.id().clone());
    assert!(blocker_id.is_some(), "blocker should be in hand");
    let Some(blocker_id) = blocker_id else {
        return;
    };
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(blocker_id.clone())),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_to_player_phase_satisfying_cleanup(
        &service,
        &mut game,
        "player-1",
        demonictutor::Phase::DeclareAttackers,
    );

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);

    let result = service.declare_blockers(
        &mut game,
        demonictutor::DeclareBlockersCommand::new(
            PlayerId::new("player-2"),
            vec![(blocker_id.clone(), attacker_id)],
        ),
    );
    assert!(matches!(
        result,
        Err(DomainError::Card(demonictutor::CardError::CannotBlock { card, .. })) if card == blocker_id
    ));
}

#[test]
fn untap_target_creature_spell_is_a_no_op_for_an_untapped_target() {
    let (service, mut game) = setup_two_player_game(
        "game-untap-target-creature-no-op",
        filled_library(
            vec![
                creature_card("silvercoat", 0, 2, 2),
                untap_target_creature_instant_card("battlefield-reprieve", 0),
            ],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-untap-target-creature-no-op-player-1-0"),
            ),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    let creature_id = game.players()[0]
        .battlefield_card_by_definition(&CardDefinitionId::new("silvercoat"))
        .map(|card| card.id().clone());
    assert!(creature_id.is_some(), "creature should be on battlefield");
    let Some(creature_id) = creature_id else {
        return;
    };
    let untap_spell_id = game.players()[0]
        .hand_card_by_definition(&CardDefinitionId::new("battlefield-reprieve"))
        .map(|card| card.id().clone());
    assert!(untap_spell_id.is_some(), "untap spell should be in hand");
    let Some(untap_spell_id) = untap_spell_id else {
        return;
    };

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), untap_spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);

    assert!(!game.players()[0]
        .battlefield_card(&creature_id)
        .is_some_and(demonictutor::CardInstance::is_tapped));
}

#[test]
fn counterspell_counters_target_spell_on_the_stack() {
    let (service, mut game) = setup_two_player_game(
        "game-counterspell-resolve",
        filled_library(
            vec![
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
            ],
            10,
        ),
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let shock_id = CardInstanceId::new("game-counterspell-resolve-player-2-0");
    let shock_outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), shock_id.clone())
                .with_target(SpellTarget::Player(PlayerId::new("player-1"))),
        )
        .unwrap();
    let _ = pass_priority_once(&service, &mut game);

    let cancel_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), cancel_id).with_target(
                SpellTarget::StackObject(shock_outcome.spell_put_on_stack.stack_object_id),
            ),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(matches!(
        resolution.spell_cast.as_ref().unwrap().outcome,
        SpellCastOutcome::ResolvedToGraveyard
    ));
    assert_eq!(game.players()[0].life(), 20);
    assert_eq!(game.stack().len(), 0);
    assert!(game.players()[1].graveyard_card(&shock_id).is_some());
}

#[test]
fn counterspell_rejects_non_stack_targets_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-counterspell-illegal-target",
        filled_library(
            vec![
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
                counter_target_spell_instant_card("cancel-lite", 0),
            ],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let cancel_id = game.players()[0].hand_card_ids()[0].clone();
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), cancel_id)
            .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id)))
            if card_id == CardInstanceId::new("game-counterspell-illegal-target-player-1-0")
    ));
}

#[test]
fn lower_counterspell_does_nothing_if_target_spell_is_already_gone() {
    let (service, mut game) = setup_two_player_game(
        "game-counterspell-target-gone",
        filled_library(
            vec![
                counter_target_spell_instant_card("cancel-a", 0),
                counter_target_spell_instant_card("cancel-b", 0),
                counter_target_spell_instant_card("cancel-c", 0),
                counter_target_spell_instant_card("cancel-d", 0),
                counter_target_spell_instant_card("cancel-e", 0),
                counter_target_spell_instant_card("cancel-f", 0),
                counter_target_spell_instant_card("cancel-g", 0),
            ],
            10,
        ),
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let shock_id = CardInstanceId::new("game-counterspell-target-gone-player-2-0");
    let shock_outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), shock_id.clone())
                .with_target(SpellTarget::Player(PlayerId::new("player-1"))),
        )
        .unwrap();
    let _ = pass_priority_once(&service, &mut game);

    let first_cancel_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), first_cancel_id).with_target(
                SpellTarget::StackObject(shock_outcome.spell_put_on_stack.stack_object_id.clone()),
            ),
        )
        .unwrap();

    let second_cancel_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), second_cancel_id).with_target(
                SpellTarget::StackObject(shock_outcome.spell_put_on_stack.stack_object_id),
            ),
        )
        .unwrap();

    let first_resolution = resolve_current_stack(&service, &mut game);
    assert!(game.players()[1].graveyard_card(&shock_id).is_some());
    assert_eq!(game.stack().len(), 1);
    assert!(first_resolution.life_changed.is_none());

    let second_resolution = pass_priority_once(&service, &mut game);
    assert!(second_resolution.priority_still_open);
    let third_resolution = pass_priority_once(&service, &mut game);
    assert!(third_resolution.life_changed.is_none());
    assert_eq!(game.players()[0].life(), 20);
    assert_eq!(game.stack().len(), 0);
}

#[test]
fn targeted_spell_rejects_opponents_hexproof_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-hexproof-creature-target",
        filled_library(
            vec![
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
            ],
            10,
        ),
        filled_library(
            vec![creature_card_with_keyword(
                "slippery-bear",
                0,
                2,
                2,
                KeywordAbility::Hexproof,
            )],
            10,
        ),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-hexproof-creature-target-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let removal_id = hand_card_id_by_definition(&game, 0, "murder-lite");
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), removal_id.clone())
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id)))
            if card_id == removal_id
    ));
}

#[test]
fn targeted_spell_can_target_own_hexproof_creature() {
    let (service, mut game) = setup_two_player_game(
        "game-hexproof-own-target",
        filled_library(
            vec![
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                creature_card_with_keyword("slippery-bear", 0, 2, 2, KeywordAbility::Hexproof),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let creature_id = hand_card_id_by_definition(&game, 0, "slippery-bear");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    let removal_id = hand_card_id_by_definition(&game, 0, "murder-lite");
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), removal_id)
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(result.is_ok());
}

#[test]
fn destroy_target_creature_does_not_destroy_indestructible_creature() {
    let (service, mut game) = setup_two_player_game(
        "game-indestructible-destroy",
        filled_library(
            vec![
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_destroy_creature_instant_card("murder-lite", 0),
            ],
            10,
        ),
        filled_library(
            vec![creature_card_with_keyword(
                "adamant-guardian",
                0,
                2,
                2,
                KeywordAbility::Indestructible,
            )],
            10,
        ),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-indestructible-destroy-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let removal_id = hand_card_id_by_definition(&game, 0, "murder-lite");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), removal_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let outcome = resolve_current_stack(&service, &mut game);

    assert!(outcome.creatures_died.is_empty());
    assert!(game.players()[1].battlefield_card(&creature_id).is_some());
    assert!(game.players()[1].graveyard_card(&creature_id).is_none());
}

#[test]
fn bounce_spell_returns_target_permanent_to_its_owners_hand() {
    let (service, mut game) = setup_two_player_game(
        "game-bounce-permanent",
        filled_library(
            vec![
                return_target_permanent_to_hand_instant_card("unsummon-relic", 0),
                return_target_permanent_to_hand_instant_card("unsummon-relic", 0),
                return_target_permanent_to_hand_instant_card("unsummon-relic", 0),
                return_target_permanent_to_hand_instant_card("unsummon-relic", 0),
                return_target_permanent_to_hand_instant_card("unsummon-relic", 0),
                return_target_permanent_to_hand_instant_card("unsummon-relic", 0),
                return_target_permanent_to_hand_instant_card("unsummon-relic", 0),
            ],
            10,
        ),
        filled_library(vec![artifact_card("howling-mine", 0)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let artifact_id = hand_card_id_by_definition(&game, 1, "howling-mine");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), artifact_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    let _ = pass_priority_once(&service, &mut game);

    let bounce_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), bounce_id)
                .with_target(SpellTarget::Permanent(artifact_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);
    assert!(game.players()[1].hand_card(&artifact_id).is_some());
    assert!(game.players()[1].battlefield_card(&artifact_id).is_none());
}

#[test]
fn bounce_spell_rejects_non_battlefield_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-bounce-illegal-target",
        filled_library(
            vec![return_target_permanent_to_hand_instant_card(
                "unsummon-relic",
                0,
            )],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let bounce_id = hand_card_id_by_definition(&game, 0, "unsummon-relic");
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), bounce_id)
            .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(_)))
    ));
}

#[test]
fn lower_bounce_spell_does_nothing_if_target_permanent_is_already_gone() {
    let (service, mut game) = setup_two_player_game(
        "game-bounce-target-gone",
        filled_library(
            vec![
                return_target_permanent_to_hand_instant_card("unsummon-a", 0),
                return_target_permanent_to_hand_instant_card("unsummon-b", 0),
                return_target_permanent_to_hand_instant_card("unsummon-c", 0),
                return_target_permanent_to_hand_instant_card("unsummon-d", 0),
                return_target_permanent_to_hand_instant_card("unsummon-e", 0),
                return_target_permanent_to_hand_instant_card("unsummon-f", 0),
                return_target_permanent_to_hand_instant_card("unsummon-g", 0),
            ],
            10,
        ),
        filled_library(vec![artifact_card("howling-mine", 0)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let artifact_id = hand_card_id_by_definition(&game, 1, "howling-mine");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), artifact_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    let _ = pass_priority_once(&service, &mut game);

    let first_bounce_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), first_bounce_id)
                .with_target(SpellTarget::Permanent(artifact_id.clone())),
        )
        .unwrap();

    let second_bounce_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), second_bounce_id)
                .with_target(SpellTarget::Permanent(artifact_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);
    let _ = pass_priority_once(&service, &mut game);
    let resolution = pass_priority_once(&service, &mut game);

    assert!(resolution.life_changed.is_none());
    assert!(game.players()[1].hand_card(&artifact_id).is_some());
    assert!(game.players()[1].battlefield_card(&artifact_id).is_none());
}

#[test]
fn destroy_artifact_or_enchantment_spell_destroys_target_artifact() {
    let (service, mut game) = setup_two_player_game(
        "game-destroy-artifact",
        filled_library(
            vec![
                destroy_target_artifact_or_enchantment_instant_card("shatter-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-lite", 0),
            ],
            10,
        ),
        filled_library(vec![artifact_card("howling-mine", 0)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let artifact_id = hand_card_id_by_definition(&game, 1, "howling-mine");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), artifact_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    let _ = pass_priority_once(&service, &mut game);

    let removal_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), removal_id)
                .with_target(SpellTarget::Permanent(artifact_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);
    assert!(game.players()[1].graveyard_card(&artifact_id).is_some());
    assert!(game.players()[1].battlefield_card(&artifact_id).is_none());
}

#[test]
fn destroy_artifact_or_enchantment_spell_destroys_target_enchantment() {
    let (service, mut game) = setup_two_player_game(
        "game-destroy-enchantment",
        filled_library(
            vec![
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
            ],
            10,
        ),
        filled_library(vec![enchantment_card("battle-rite", 0)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let enchantment_id = hand_card_id_by_definition(&game, 1, "battle-rite");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), enchantment_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    let _ = pass_priority_once(&service, &mut game);

    let removal_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), removal_id)
                .with_target(SpellTarget::Permanent(enchantment_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);
    assert!(game.players()[1].graveyard_card(&enchantment_id).is_some());
    assert!(game.players()[1]
        .battlefield_card(&enchantment_id)
        .is_none());
}

#[test]
fn destroy_artifact_or_enchantment_spell_rejects_creature_target() {
    let (service, mut game) = setup_two_player_game(
        "game-destroy-artifact-illegal-target",
        filled_library(
            vec![
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
                destroy_target_artifact_or_enchantment_instant_card("disenchant-lite", 0),
            ],
            10,
        ),
        filled_library(vec![creature_card("bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let creature_id = hand_card_id_by_definition(&game, 1, "bear");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    let _ = pass_priority_once(&service, &mut game);

    let removal_id = game.players()[0].hand_card_ids()[0].clone();
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), removal_id)
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(_)))
    ));
}

#[test]
fn lower_artifact_removal_does_nothing_if_target_is_already_gone() {
    let (service, mut game) = setup_two_player_game(
        "game-destroy-artifact-target-gone",
        filled_library(
            vec![
                destroy_target_artifact_or_enchantment_instant_card("shatter-a", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-b", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-c", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-d", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-e", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-f", 0),
                destroy_target_artifact_or_enchantment_instant_card("shatter-g", 0),
            ],
            10,
        ),
        filled_library(vec![artifact_card("howling-mine", 0)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");

    let artifact_id = hand_card_id_by_definition(&game, 1, "howling-mine");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), artifact_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    let _ = pass_priority_once(&service, &mut game);

    let first_removal_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), first_removal_id)
                .with_target(SpellTarget::Permanent(artifact_id.clone())),
        )
        .unwrap();

    let second_removal_id = game.players()[0].hand_card_ids()[0].clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), second_removal_id)
                .with_target(SpellTarget::Permanent(artifact_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);
    let _ = pass_priority_once(&service, &mut game);
    let resolution = pass_priority_once(&service, &mut game);

    assert!(resolution.life_changed.is_none());
    assert!(game.players()[1].graveyard_card(&artifact_id).is_some());
    assert!(game.players()[1].battlefield_card(&artifact_id).is_none());
}

#[test]
fn targeted_instant_deals_damage_to_target_player_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-player-resolve",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
        )
        .unwrap();

    assert_eq!(
        outcome.spell_put_on_stack.target,
        Some(SpellTarget::Player(PlayerId::new("player-2")))
    );

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(
        resolution.life_changed.as_ref().unwrap().player_id,
        PlayerId::new("player-2")
    );
    assert_eq!(resolution.life_changed.as_ref().unwrap().from_life, 20);
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 18);
    assert!(matches!(
        resolution.spell_cast.as_ref().unwrap().outcome,
        SpellCastOutcome::ResolvedToGraveyard
    ));
    assert_eq!(game.players()[1].life(), 18);
}

#[test]
fn targeted_player_damage_can_end_the_game() {
    let (service, mut game) = setup_two_player_game(
        "game-target-player-lethal",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .adjust_player_life_effect(
            &mut game,
            demonictutor::AdjustPlayerLifeEffectCommand::new(
                PlayerId::new("player-1"),
                PlayerId::new("player-2"),
                -18,
            ),
        )
        .unwrap();

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 0);
    assert_eq!(
        resolution.game_ended.as_ref().unwrap().reason,
        GameEndReason::ZeroLife
    );
}

#[test]
fn targeted_gain_life_spell_increases_the_targets_life_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-player-gain-life",
        filled_library(
            vec![targeted_gain_life_instant_card("healing-light", 0, 3)],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let spell_id = hand_card_id_by_definition(&game, 0, "healing-light");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(
        resolution.life_changed.as_ref().unwrap().player_id,
        PlayerId::new("player-2")
    );
    assert_eq!(resolution.life_changed.as_ref().unwrap().from_life, 20);
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 23);
    assert_eq!(game.players()[1].life(), 23);
}

#[test]
fn targeted_lose_life_spell_reduces_the_targets_life_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-player-lose-life",
        filled_library(
            vec![targeted_lose_life_instant_card("soul-drain", 0, 3)],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let spell_id = hand_card_id_by_definition(&game, 0, "soul-drain");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(
        resolution.life_changed.as_ref().unwrap().player_id,
        PlayerId::new("player-2")
    );
    assert_eq!(resolution.life_changed.as_ref().unwrap().from_life, 20);
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 17);
    assert_eq!(game.players()[1].life(), 17);
}

#[test]
fn targeted_lose_life_spell_can_end_the_game() {
    let (service, mut game) = setup_two_player_game(
        "game-target-player-lose-life-lethal",
        filled_library(
            vec![targeted_lose_life_instant_card("soul-drain", 0, 3)],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service
        .adjust_player_life_effect(
            &mut game,
            demonictutor::AdjustPlayerLifeEffectCommand::new(
                PlayerId::new("player-1"),
                PlayerId::new("player-2"),
                -18,
            ),
        )
        .unwrap();

    let spell_id = hand_card_id_by_definition(&game, 0, "soul-drain");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 0);
    assert_eq!(
        resolution.game_ended.as_ref().unwrap().reason,
        GameEndReason::ZeroLife
    );
}

#[test]
fn targeted_instant_rejects_invalid_creature_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-creature-invalid",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-1"),
            CardInstanceId::new("game-target-creature-invalid-player-1-0"),
        )
        .with_target(SpellTarget::Creature(CardInstanceId::new(
            "missing-creature",
        ))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::InvalidCreatureTarget(card_id)))
            if card_id == CardInstanceId::new("missing-creature")
    ));
}

#[test]
fn targeted_player_spell_rejects_a_creature_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-illegal-kind",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_player_damage_instant_card("bolt", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );
    let bolt_id = hand_card_id_by_definition(&game, 0, "bolt");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-illegal-kind-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), bolt_id.clone())
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(
        matches!(
            result,
            Err(DomainError::Game(GameError::IllegalSpellTarget(ref card_id)))
                if card_id == &bolt_id
        ),
        "unexpected result: {result:?}"
    );
}

#[test]
fn targeted_attacking_creature_spell_rejects_a_player_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-attacking-illegal-kind",
        filled_library(
            vec![targeted_attacking_creature_damage_instant_card(
                "marked-for-battle",
                0,
                2,
            )],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let spell_id = hand_card_id_by_definition(&game, 0, "marked-for-battle");
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), spell_id.clone())
            .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spell_id
    ));
}

#[test]
fn targeted_attacking_creature_spell_rejects_a_non_attacking_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-attacking-nonattacker",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_attacking_creature_damage_instant_card("marked-for-battle", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );
    let spell_id = hand_card_id_by_definition(&game, 0, "marked-for-battle");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-attacking-nonattacker-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), spell_id.clone())
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spell_id
    ));
}

#[test]
fn targeted_attacking_creature_spell_can_destroy_an_attacker_after_attackers() {
    let (service, mut game) = setup_two_player_game(
        "game-target-attacking-lethal",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker"), 0, 2, 2),
                targeted_attacking_creature_damage_instant_card("marked-for-battle", 0, 2),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-target-attacking-lethal-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    crate::support::advance_turn_raw(&service, &mut game);
    crate::support::close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();

    let spell_id = hand_card_id_by_definition(&game, 0, "marked-for-battle");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(attacker_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(resolution.creatures_died.len(), 1);
    assert_eq!(resolution.creatures_died[0].card_id, attacker_id);
    assert!(game.players()[0]
        .battlefield_cards()
        .all(|card| card.id() != &resolution.creatures_died[0].card_id));
}

#[test]
fn targeted_attacking_creature_spell_marks_nonlethal_damage_and_leaves_the_attacker_in_combat() {
    let (service, mut game) = setup_two_player_game(
        "game-target-attacking-nonlethal",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker"), 0, 3, 3),
                targeted_attacking_creature_damage_instant_card("marked-for-battle", 0, 1),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-target-attacking-nonlethal-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    crate::support::advance_turn_raw(&service, &mut game);
    crate::support::close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();

    let spell_id = hand_card_id_by_definition(&game, 0, "marked-for-battle");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(attacker_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.creatures_died.is_empty());

    let attacker = game.players()[0]
        .battlefield_cards()
        .find(|card| card.id() == &attacker_id)
        .unwrap();
    assert_eq!(attacker.damage(), 1);
    assert!(attacker.is_attacking());
}

#[test]
fn targeted_instant_deals_damage_to_target_creature_and_state_based_actions_destroy_it() {
    let (service, mut game) = setup_two_player_game(
        "game-target-creature-resolve",
        filled_library(
            vec![
                land_card("mountain"),
                targeted_damage_instant_card("shock", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-creature-resolve-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.life_changed.is_none());
    assert_eq!(resolution.creatures_died.len(), 1);
    assert_eq!(resolution.creatures_died[0].card_id, creature_id);
    assert!(game.players()[1]
        .battlefield_cards()
        .all(|card| card.definition_id() != &CardDefinitionId::new("bob-bear")));
    assert_eq!(game.players()[1].graveyard_size(), 1);
}

#[test]
fn targeted_opponent_spell_rejects_the_caster_as_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponent-only",
        filled_library(
            vec![targeted_opponent_damage_instant_card("lava-spike", 0, 2)],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let spike_id = hand_card_id_by_definition(&game, 0, "lava-spike");
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), spike_id.clone())
            .with_target(SpellTarget::Player(PlayerId::new("player-1"))),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spike_id
    ));
}

#[test]
fn targeted_opponent_spell_can_target_the_opponent_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponent-resolve",
        filled_library(
            vec![targeted_opponent_damage_instant_card("lava-spike", 0, 2)],
            10,
        ),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let spike_id = hand_card_id_by_definition(&game, 0, "lava-spike");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spike_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2"))),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(
        resolution.life_changed.as_ref().unwrap().player_id,
        PlayerId::new("player-2")
    );
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 18);
}

#[test]
fn targeted_any_player_spell_can_target_the_caster_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-any-player-self",
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-1"))),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(
        resolution.life_changed.as_ref().unwrap().player_id,
        PlayerId::new("player-1")
    );
    assert_eq!(resolution.life_changed.as_ref().unwrap().to_life, 18);
}

#[test]
fn targeted_controlled_creature_spell_rejects_an_opponents_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-controlled-creature-invalid",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_controlled_creature_damage_instant_card("reckless-surge", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );
    let spell_id = hand_card_id_by_definition(&game, 0, "reckless-surge");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-controlled-creature-invalid-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), spell_id.clone())
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spell_id
    ));
}

#[test]
fn targeted_opponents_creature_spell_can_target_an_opponents_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponents-creature-cast",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_opponents_creature_damage_instant_card("hostile-bolt", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );
    let spell_id = hand_card_id_by_definition(&game, 0, "hostile-bolt");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-opponents-creature-cast-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    assert_eq!(
        outcome.spell_put_on_stack.target,
        Some(SpellTarget::Creature(creature_id))
    );
}

#[test]
fn targeted_opponents_creature_spell_rejects_a_controlled_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponents-creature-invalid",
        filled_library(
            vec![
                creature_card("alice-bear", 0, 2, 2),
                targeted_opponents_creature_damage_instant_card("hostile-bolt", 0, 2),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let creature_id = hand_card_id_by_definition(&game, 0, "alice-bear");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "hostile-bolt");

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), spell_id.clone())
            .with_target(SpellTarget::Creature(creature_id)),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::IllegalSpellTarget(card_id))) if card_id == spell_id
    ));
}

#[test]
fn targeted_opponents_creature_spell_can_target_the_opponents_creature_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponents-creature-resolve",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_opponents_creature_damage_instant_card("hostile-bolt", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-opponents-creature-resolve-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "hostile-bolt");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.life_changed.is_none());
    assert_eq!(resolution.creatures_died.len(), 1);
    assert_eq!(resolution.creatures_died[0].card_id, creature_id);
    assert!(game.players()[1]
        .battlefield_cards()
        .all(|card| card.definition_id() != &CardDefinitionId::new("bob-bear")));
    assert_eq!(game.players()[1].graveyard_size(), 1);
}

#[test]
fn destroy_target_creature_spell_can_destroy_a_creature_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-destroy-target-creature-resolve",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_destroy_creature_instant_card("murder-lite", 0),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-destroy-target-creature-resolve-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "murder-lite");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.life_changed.is_none());
    assert_eq!(resolution.creatures_died.len(), 1);
    assert_eq!(resolution.creatures_died[0].card_id, creature_id);
    assert!(game.players()[1]
        .battlefield_cards()
        .all(|card| card.definition_id() != &CardDefinitionId::new("bob-bear")));
    assert_eq!(game.players()[1].graveyard_size(), 1);
}

#[test]
fn destroy_target_creature_spell_does_not_apply_if_the_target_is_gone_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-destroy-target-creature-gone",
        filled_library(
            vec![
                land_card("mountain"),
                targeted_destroy_creature_instant_card("murder-lite", 0),
                targeted_damage_instant_card("shock", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-destroy-target-creature-gone-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let destroy_id = hand_card_id_by_definition(&game, 0, "murder-lite");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), destroy_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let first_resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(first_resolution.creatures_died.len(), 1);
    assert_eq!(first_resolution.creatures_died[0].card_id, creature_id);

    let second_resolution = resolve_current_stack(&service, &mut game);
    assert!(second_resolution.life_changed.is_none());
    assert!(second_resolution.creatures_died.is_empty());
    assert_eq!(game.players()[1].graveyard_size(), 1);
}

#[test]
fn exile_target_creature_spell_can_exile_a_creature_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-exile-target-creature-resolve",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_exile_creature_instant_card("banish-lite", 0),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-exile-target-creature-resolve-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "banish-lite");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.life_changed.is_none());
    assert!(resolution.creatures_died.is_empty());
    assert!(resolution.zone_changes.iter().any(|event| {
        event.card_id == creature_id
            && event.origin_zone.as_str() == "battlefield"
            && event.destination_zone.as_str() == "exile"
    }));
    assert!(game.players()[1]
        .battlefield_cards()
        .all(|card| card.definition_id() != &CardDefinitionId::new("bob-bear")));
    assert_eq!(game.players()[1].graveyard_size(), 0);
    assert_eq!(game.players()[1].exile_size(), 1);
}

#[test]
fn exile_target_creature_spell_does_not_apply_if_the_target_is_gone_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-exile-target-creature-gone",
        filled_library(
            vec![
                land_card("mountain"),
                targeted_exile_creature_instant_card("banish-lite", 0),
                targeted_damage_instant_card("shock", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-exile-target-creature-gone-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let exile_id = hand_card_id_by_definition(&game, 0, "banish-lite");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), exile_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let shock_id = hand_card_id_by_definition(&game, 0, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let first_resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(first_resolution.creatures_died.len(), 1);
    assert_eq!(first_resolution.creatures_died[0].card_id, creature_id);

    let second_resolution = resolve_current_stack(&service, &mut game);
    assert!(second_resolution
        .zone_changes
        .iter()
        .all(|event| event.destination_zone.as_str() != "exile"));
    assert!(second_resolution.life_changed.is_none());
    assert!(second_resolution.creatures_died.is_empty());
    assert_eq!(game.players()[1].graveyard_size(), 1);
    assert_eq!(game.players()[1].exile_size(), 0);
}

#[test]
fn exile_target_graveyard_card_spell_can_exile_a_card_from_a_graveyard_when_it_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-exile-graveyard-card-resolve",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_exile_graveyard_card_instant_card("crypt-banish", 0),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-wisp", 0, 0, 0)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let graveyard_card_id = CardInstanceId::new("game-exile-graveyard-card-resolve-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), graveyard_card_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    assert!(game.players()[1].graveyard_contains(&graveyard_card_id));

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "crypt-banish");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::GraveyardCard(graveyard_card_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution.zone_changes.iter().any(|event| {
        event.card_id == graveyard_card_id
            && event.origin_zone.as_str() == "graveyard"
            && event.destination_zone.as_str() == "exile"
    }));
    assert!(game.players()[1].exile_contains(&graveyard_card_id));
    assert!(!game.players()[1].graveyard_contains(&graveyard_card_id));
}

#[test]
fn exile_target_graveyard_card_spell_rejects_a_missing_graveyard_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-exile-graveyard-card-invalid",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_exile_graveyard_card_instant_card("crypt-banish", 0),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "crypt-banish");

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), spell_id).with_target(
            SpellTarget::GraveyardCard(CardInstanceId::new("missing-graveyard-card")),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::InvalidGraveyardCardTarget(card_id)))
            if card_id == CardInstanceId::new("missing-graveyard-card")
    ));
}

#[test]
fn exile_target_graveyard_card_spell_does_not_apply_if_the_target_is_gone_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-exile-graveyard-card-gone",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_exile_graveyard_card_instant_card("crypt-banish", 0),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-wisp", 0, 0, 0)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let graveyard_card_id = CardInstanceId::new("game-exile-graveyard-card-gone-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), graveyard_card_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    assert!(game.players()[1].graveyard_contains(&graveyard_card_id));

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let spell_id = hand_card_id_by_definition(&game, 0, "crypt-banish");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::GraveyardCard(graveyard_card_id.clone())),
        )
        .unwrap();

    service
        .exile_card(
            &mut game,
            &demonictutor::ExileCardCommand::new(
                PlayerId::new("player-2"),
                graveyard_card_id.clone(),
                false,
            ),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);
    assert!(resolution
        .zone_changes
        .iter()
        .all(|event| event.destination_zone.as_str() != "exile"));
    assert!(game.players()[1].exile_contains(&graveyard_card_id));
    assert!(!game.players()[1].graveyard_contains(&graveyard_card_id));
}

#[test]
fn pump_target_creature_spell_applies_temporary_stats_until_end_of_turn() {
    let (service, mut game) = setup_two_player_game(
        "game-pump-target-creature-expire",
        filled_library(
            vec![
                creature_card("alice-bear", 0, 2, 2),
                targeted_pump_creature_instant_card("giant-growth-lite", 0, 2, 2),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let creature_id = CardInstanceId::new("game-pump-target-creature-expire-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let spell_id = hand_card_id_by_definition(&game, 0, "giant-growth-lite");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let _ = resolve_current_stack(&service, &mut game);
    assert_eq!(
        game.players()[0]
            .battlefield_card(&creature_id)
            .unwrap()
            .creature_stats(),
        Some((4, 4))
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    assert_eq!(
        game.players()[0]
            .battlefield_card(&creature_id)
            .unwrap()
            .creature_stats(),
        Some((2, 2))
    );
}

#[test]
fn pump_target_creature_spell_changes_combat_damage_this_turn() {
    let (service, mut game) = setup_two_player_game(
        "game-pump-target-creature-combat",
        filled_library(
            vec![
                creature_card("alice-bear", 0, 2, 2),
                targeted_pump_creature_instant_card("giant-growth-lite", 0, 2, 2),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let creature_id = CardInstanceId::new("game-pump-target-creature-combat-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let spell_id = hand_card_id_by_definition(&game, 0, "giant-growth-lite");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);

    advance_turn_raw(&service, &mut game);
    advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![creature_id]),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);
    service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(game.players()[1].life(), 16);
}

#[test]
fn targeted_instant_does_not_apply_if_its_only_creature_target_is_gone_on_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-target-creature-gone",
        filled_library(
            vec![
                land_card("mountain"),
                targeted_damage_instant_card("shock-a", 0, 2),
                targeted_damage_instant_card("shock-b", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-creature-gone-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    let _ = resolve_current_stack(&service, &mut game);
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let first_shock_id = hand_card_id_by_definition(&game, 0, "shock-a");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), first_shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let second_shock_id = hand_card_id_by_definition(&game, 0, "shock-b");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), second_shock_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let first_resolution = resolve_current_stack(&service, &mut game);
    assert_eq!(first_resolution.creatures_died.len(), 1);
    assert_eq!(first_resolution.creatures_died[0].card_id, creature_id);

    let second_resolution = resolve_current_stack(&service, &mut game);
    assert!(second_resolution.life_changed.is_none());
    assert!(second_resolution.creatures_died.is_empty());
    assert!(game.players()[1].battlefield_is_empty());
    assert_eq!(game.players()[1].graveyard_size(), 1);
}

#[test]
fn discard_spell_forces_target_player_to_discard_the_chosen_card() {
    let (service, mut game) = setup_two_player_game(
        "game-discard-chosen-card",
        filled_library(
            vec![target_player_discards_chosen_card_sorcery_card(
                "coercion-lite",
                0,
            )],
            10,
        ),
        filled_library(
            vec![creature_card("bob-bear", 0, 2, 2), land_card("bob-land")],
            10,
        ),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let discard_id = hand_card_id_by_definition(&game, 0, "coercion-lite");
    let chosen_id = hand_card_id_by_definition(&game, 1, "bob-bear");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), discard_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2")))
                .with_choice(SpellChoice::HandCard(chosen_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);

    assert!(game.players()[1].hand_card(&chosen_id).is_none());
    assert!(game.players()[1].graveyard_card(&chosen_id).is_some());
    assert!(resolution.card_discarded.is_some());
    if let Some(discarded) = resolution.card_discarded.as_ref() {
        assert_eq!(discarded.player_id, PlayerId::new("player-2"));
        assert_eq!(discarded.card_id, chosen_id);
        assert_eq!(discarded.discard_kind, DiscardKind::SpellEffect);
    }
}

#[test]
fn discard_spell_rejects_a_chosen_card_not_in_the_target_players_hand() {
    let (service, mut game) = setup_two_player_game(
        "game-discard-invalid-choice",
        filled_library(
            vec![target_player_discards_chosen_card_sorcery_card(
                "coercion-lite",
                0,
            )],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let discard_id = hand_card_id_by_definition(&game, 0, "coercion-lite");
    let illegal_choice = hand_card_id_by_definition(&game, 0, "coercion-lite");
    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(PlayerId::new("player-1"), discard_id)
            .with_target(SpellTarget::Player(PlayerId::new("player-2")))
            .with_choice(SpellChoice::HandCard(illegal_choice.clone())),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::InvalidHandCardChoice(card_id)))
            if card_id == illegal_choice
    ));
}

#[test]
fn discard_spell_does_nothing_if_the_chosen_card_leaves_hand_before_resolution() {
    let (service, mut game) = setup_two_player_game(
        "game-discard-choice-gone",
        filled_library(
            vec![target_player_discards_chosen_card_sorcery_card(
                "coercion-lite",
                0,
            )],
            10,
        ),
        filled_library(vec![targeted_damage_instant_card("shock", 0, 2)], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let discard_id = hand_card_id_by_definition(&game, 0, "coercion-lite");
    let chosen_id = hand_card_id_by_definition(&game, 1, "shock");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), discard_id)
                .with_target(SpellTarget::Player(PlayerId::new("player-2")))
                .with_choice(SpellChoice::HandCard(chosen_id.clone())),
        )
        .unwrap();
    let _ = pass_priority_once(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), chosen_id.clone())
                .with_target(SpellTarget::Player(PlayerId::new("player-1"))),
        )
        .unwrap();

    let first_resolution = resolve_current_stack(&service, &mut game);
    assert!(first_resolution.life_changed.is_some());

    let second_resolution = resolve_current_stack(&service, &mut game);
    assert!(second_resolution.card_discarded.is_none());
    assert!(game.players()[1].hand_card(&chosen_id).is_none());
    assert!(game.players()[1].graveyard_card(&chosen_id).is_some());
    assert_eq!(game.players()[1].graveyard_size(), 1);
}
