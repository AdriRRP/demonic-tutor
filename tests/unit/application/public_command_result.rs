//! Tests the deterministic public command-result envelope.

#![allow(clippy::expect_used, clippy::panic)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, advance_to_player_first_main_satisfying_cleanup,
    advance_to_player_phase_satisfying_cleanup, advance_turn_raw,
    attacks_life_gain_haste_creature_card, cast_spell_and_resolve, close_empty_priority_window,
    combat_damage_to_player_life_gain_haste_creature_card, create_service, creature_card,
    etb_may_life_gain_creature_card, filled_library, first_hand_card_id, forest_card,
    loot_sorcery_card, player, player_deck, player_library, resolve_top_stack_with_passes,
    rummage_sorcery_card, sacrifice_life_gain_artifact_card, sacrifice_life_gain_creature_card,
    setup_two_player_game, surveil_sorcery_card, target_player_discards_chosen_card_sorcery_card,
};
use demonictutor::{
    public_command_result, ActivateAbilityCommand, CardDefinitionId, CastSpellCommand,
    DealOpeningHandsCommand, DeclareAttackersCommand, DeclareBlockersCommand,
    DiscardForCleanupCommand, DiscardKind, DomainEvent, GameId, PassPriorityCommand, Phase,
    PlayLandCommand, PlayerId, PublicCommandStatus, PublicGameCommand, ResolveCombatDamageCommand,
    ResolveOptionalEffectCommand, ResolvePendingHandChoiceCommand, ResolvePendingSurveilCommand,
    SpellChoice, SpellTarget, StartGameCommand, TriggeredAbilityEvent,
};

fn game_in_first_main() -> (crate::support::TestService, demonictutor::Game) {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                forest_card("p1-forest-a"),
                forest_card("p1-forest-b"),
                forest_card("p1-forest-c"),
                forest_card("p1-forest-d"),
                forest_card("p1-forest-e"),
                forest_card("p1-forest-f"),
                forest_card("p1-forest-g"),
                forest_card("p1-forest-h"),
                forest_card("p1-forest-i"),
                forest_card("p1-forest-j"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-forest-a"),
                forest_card("p2-forest-b"),
                forest_card("p2-forest-c"),
                forest_card("p2-forest-d"),
                forest_card("p2-forest-e"),
                forest_card("p2-forest-f"),
                forest_card("p2-forest-g"),
                forest_card("p2-forest-h"),
                forest_card("p2-forest-i"),
                forest_card("p2-forest-j"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-public-command"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");
    (service, game)
}

fn loot_game_in_pending_choice() -> (
    crate::support::TestService,
    demonictutor::Game,
    demonictutor::CardInstanceId,
) {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                loot_sorcery_card("p1-loot", 0, 2),
                forest_card("p1-hand-a"),
                forest_card("p1-hand-b"),
                forest_card("p1-hand-c"),
                forest_card("p1-hand-d"),
                forest_card("p1-hand-e"),
                forest_card("p1-draw-a"),
                forest_card("p1-pad-a"),
                forest_card("p1-pad-b"),
                forest_card("p1-pad-c"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-a"),
                forest_card("p2-b"),
                forest_card("p2-c"),
                forest_card("p2-d"),
                forest_card("p2-e"),
                forest_card("p2-f"),
                forest_card("p2-g"),
                forest_card("p2-h"),
                forest_card("p2-i"),
                forest_card("p2-j"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-public-loot"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");

    let loot_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-loot"))
        .expect("loot spell should be in hand")
        .id()
        .clone();

    service.execute_public_command(
        &mut game,
        PublicGameCommand::CastSpell(CastSpellCommand::new(PlayerId::new("p1"), loot_id)),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p2"))),
    );

    let discard_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-draw-a"))
        .expect("drawn card should be in hand")
        .id()
        .clone();

    (service, game, discard_id)
}

fn terminal_loot_game_on_stack() -> (
    crate::support::TestService,
    demonictutor::Game,
    demonictutor::CardInstanceId,
) {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                loot_sorcery_card("p1-loot", 0, 2),
                forest_card("p1-hand-a"),
                forest_card("p1-hand-b"),
                forest_card("p1-hand-c"),
                forest_card("p1-hand-d"),
                forest_card("p1-hand-e"),
                forest_card("p1-hand-f"),
                forest_card("p1-draw-step"),
                forest_card("p1-loot-draw"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-a"),
                forest_card("p2-b"),
                forest_card("p2-c"),
                forest_card("p2-d"),
                forest_card("p2-e"),
                forest_card("p2-f"),
                forest_card("p2-g"),
                forest_card("p2-h"),
                forest_card("p2-i"),
                forest_card("p2-j"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-public-terminal-loot"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");

    let loot_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-loot"))
        .expect("loot spell should be in hand")
        .id()
        .clone();

    service.execute_public_command(
        &mut game,
        PublicGameCommand::CastSpell(CastSpellCommand::new(PlayerId::new("p1"), loot_id.clone())),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p1"))),
    );

    (service, game, loot_id)
}

#[test]
fn declare_attackers_emits_triggered_ability_put_on_stack_for_supported_attack_triggers() {
    let (service, mut game) = setup_two_player_game(
        "game-public-attack-trigger",
        filled_library(
            vec![attacks_life_gain_haste_creature_card(
                "battle-adept",
                0,
                2,
                2,
                2,
            )],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("battle-adept"))
        .expect("creature should be in hand")
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", creature_id.clone());
    advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);

    let result = service.execute_public_command(
        &mut game,
        PublicGameCommand::DeclareAttackers(DeclareAttackersCommand::new(
            PlayerId::new("player-1"),
            vec![creature_id],
        )),
    );

    assert!(matches!(result.status, PublicCommandStatus::Applied));
    assert!(matches!(
        result.emitted_events.as_slice(),
        [
            DomainEvent::AttackersDeclared(_),
            DomainEvent::TriggeredAbilityPutOnStack(event),
        ] if event.trigger == TriggeredAbilityEvent::Attacks
    ));
}

#[test]
fn resolve_combat_damage_emits_effects_before_close_and_supported_damage_triggers() {
    let (service, mut game) = setup_two_player_game(
        "game-public-combat-damage-trigger",
        filled_library(
            vec![combat_damage_to_player_life_gain_haste_creature_card(
                "graveknell-raider",
                0,
                2,
                2,
                3,
            )],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("graveknell-raider"))
        .expect("creature should be in hand")
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", creature_id.clone());
    advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);
    service.execute_public_command(
        &mut game,
        PublicGameCommand::DeclareAttackers(DeclareAttackersCommand::new(
            PlayerId::new("player-1"),
            vec![creature_id],
        )),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-2"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-2"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::DeclareBlockers(DeclareBlockersCommand::new(
            PlayerId::new("player-2"),
            Vec::new(),
        )),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-2"))),
    );

    let result = service.execute_public_command(
        &mut game,
        PublicGameCommand::ResolveCombatDamage(ResolveCombatDamageCommand::new(PlayerId::new(
            "player-1",
        ))),
    );

    assert!(matches!(result.status, PublicCommandStatus::Applied));
    assert!(matches!(
        result.emitted_events.as_slice(),
        [
            DomainEvent::LifeChanged(_),
            DomainEvent::CombatDamageResolved(_),
            DomainEvent::TriggeredAbilityPutOnStack(event),
        ] if event.trigger == TriggeredAbilityEvent::DealsCombatDamageToPlayer
    ));
}

#[test]
fn resolve_combat_damage_surfaces_zone_moves_for_creatures_that_die_in_combat() {
    let (service, mut game) = setup_two_player_game(
        "game-public-combat-zone-moves",
        filled_library(vec![creature_card("attacker", 0, 3, 3)], 10),
        filled_library(vec![creature_card("blocker", 0, 2, 2)], 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let attacker_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("attacker"))
        .expect("attacker should be in hand")
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", attacker_id.clone());

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = player(&game, "player-2")
        .hand_card_by_definition(&CardDefinitionId::new("blocker"))
        .expect("blocker should be in hand")
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-2", blocker_id.clone());

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .expect("attackers should be declared");
    close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(
                PlayerId::new("player-2"),
                vec![(blocker_id.clone(), attacker_id)],
            ),
        )
        .expect("blockers should be declared");
    close_empty_priority_window(&service, &mut game);

    let result = service.execute_public_command(
        &mut game,
        PublicGameCommand::ResolveCombatDamage(ResolveCombatDamageCommand::new(PlayerId::new(
            "player-1",
        ))),
    );

    assert!(matches!(result.status, PublicCommandStatus::Applied));
    assert!(matches!(
        result.emitted_events.as_slice(),
        [
            DomainEvent::CreatureDied(creature_died),
            DomainEvent::CardMovedZone(zone_change),
            DomainEvent::CombatDamageResolved(_),
        ] if creature_died.card_id == blocker_id
            && zone_change.card_id == blocker_id
            && zone_change.origin_zone.as_str() == "battlefield"
            && zone_change.destination_zone.as_str() == "graveyard"
    ));
}

fn rummage_game_in_pending_choice() -> (
    crate::support::TestService,
    demonictutor::Game,
    demonictutor::CardInstanceId,
) {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                rummage_sorcery_card("p1-rummage", 0, 1),
                forest_card("p1-hand-a"),
                forest_card("p1-hand-b"),
                forest_card("p1-hand-c"),
                forest_card("p1-hand-d"),
                forest_card("p1-hand-e"),
                forest_card("p1-hand-f"),
                forest_card("p1-draw-a"),
                forest_card("p1-pad-a"),
                forest_card("p1-pad-b"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-a"),
                forest_card("p2-b"),
                forest_card("p2-c"),
                forest_card("p2-d"),
                forest_card("p2-e"),
                forest_card("p2-f"),
                forest_card("p2-g"),
                forest_card("p2-h"),
                forest_card("p2-i"),
                forest_card("p2-j"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-public-rummage"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");

    let rummage_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-rummage"))
        .expect("rummage spell should be in hand")
        .id()
        .clone();
    let discard_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-hand-a"))
        .expect("discard choice should be in hand")
        .id()
        .clone();

    service.execute_public_command(
        &mut game,
        PublicGameCommand::CastSpell(CastSpellCommand::new(PlayerId::new("p1"), rummage_id)),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p2"))),
    );

    (service, game, discard_id)
}

fn optional_effect_game_in_pending_choice() -> (crate::support::TestService, demonictutor::Game) {
    let (service, mut game) = setup_two_player_game(
        "game-public-optional-effect",
        filled_library(
            vec![etb_may_life_gain_creature_card("kindly-cleric", 0, 1, 1, 2)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    service.execute_public_command(
        &mut game,
        PublicGameCommand::CastSpell(CastSpellCommand::new(
            PlayerId::new("player-1"),
            demonictutor::CardInstanceId::new("game-public-optional-effect-player-1-0"),
        )),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-2"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-2"))),
    );

    (service, game)
}

fn surveil_game_in_pending_choice() -> (
    crate::support::TestService,
    demonictutor::Game,
    demonictutor::CardInstanceId,
) {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                surveil_sorcery_card("p1-surveil", 0, 1),
                forest_card("p1-hand-a"),
                forest_card("p1-hand-b"),
                forest_card("p1-hand-c"),
                forest_card("p1-hand-d"),
                forest_card("p1-hand-e"),
                forest_card("p1-hand-f"),
                forest_card("p1-top-card"),
                forest_card("p1-next-card"),
                forest_card("p1-pad"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-a"),
                forest_card("p2-b"),
                forest_card("p2-c"),
                forest_card("p2-d"),
                forest_card("p2-e"),
                forest_card("p2-f"),
                forest_card("p2-g"),
                forest_card("p2-h"),
                forest_card("p2-i"),
                forest_card("p2-j"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-public-surveil"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let surveil_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-surveil"))
        .expect("surveil spell should be in hand")
        .id()
        .clone();

    service.execute_public_command(
        &mut game,
        PublicGameCommand::CastSpell(CastSpellCommand::new(PlayerId::new("p1"), surveil_id)),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p2"))),
    );

    let looked_at_card_id = player(&game, "p1")
        .top_library_card_id()
        .expect("pending surveil should expose one looked-at card");

    (service, game, looked_at_card_id)
}

#[test]
fn execute_public_command_returns_applied_status_events_and_next_snapshot() {
    let (service, mut game) = game_in_first_main();
    let land_id = first_hand_card_id(&game, "p1");

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::PlayLand(PlayLandCommand::new(PlayerId::new("p1"), land_id.clone())),
    );
    let result = public_command_result(&game, application);

    assert!(matches!(result.status, PublicCommandStatus::Applied));
    assert!(!result.emitted_events.is_empty());
    let p1 = result
        .game
        .players
        .iter()
        .find(|player| player.player_id.as_str() == "p1")
        .expect("p1 should exist");
    assert!(p1.battlefield.iter().any(|card| card.card_id == land_id));
}

#[test]
fn execute_public_command_returns_rejected_status_and_preserves_follow_up_contract() {
    let (service, mut game) = game_in_first_main();
    let land_id = first_hand_card_id(&game, "p1");

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::PlayLand(PlayLandCommand::new(PlayerId::new("p2"), land_id)),
    );
    let result = public_command_result(&game, application);

    match result.status {
        PublicCommandStatus::Rejected(rejection) => {
            assert!(!rejection.message.is_empty());
        }
        PublicCommandStatus::Applied => panic!("command should have been rejected"),
    }
    assert!(result.emitted_events.is_empty());
    assert!(!result.legal_actions.is_empty());
}

#[test]
fn execute_public_command_preserves_loot_effect_event_order() {
    let (service, mut game, discard_id) = loot_game_in_pending_choice();

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::ResolvePendingHandChoice(ResolvePendingHandChoiceCommand::new(
            PlayerId::new("p1"),
            discard_id,
        )),
    );

    assert!(matches!(
        application.emitted_events.as_slice(),
        [
            DomainEvent::CardDiscarded(_),
            DomainEvent::CardMovedZone(_),
            DomainEvent::StackTopResolved(_),
            DomainEvent::SpellCast(_),
        ]
    ));
}

#[test]
fn execute_public_command_preserves_rummage_effect_event_order() {
    let (service, mut game, discard_id) = rummage_game_in_pending_choice();

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::ResolvePendingHandChoice(ResolvePendingHandChoiceCommand::new(
            PlayerId::new("p1"),
            discard_id,
        )),
    );

    assert!(matches!(
        application.emitted_events.as_slice(),
        [
            DomainEvent::CardDiscarded(_),
            DomainEvent::CardMovedZone(_),
            DomainEvent::CardDrawn(_),
            DomainEvent::StackTopResolved(_),
            DomainEvent::SpellCast(_),
        ]
    ));
}

#[test]
fn execute_public_command_surfaces_card_discarded_from_pass_priority_resolution() {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                target_player_discards_chosen_card_sorcery_card("p1-discard", 0),
                forest_card("p1-a"),
                forest_card("p1-b"),
                forest_card("p1-c"),
                forest_card("p1-d"),
                forest_card("p1-e"),
                forest_card("p1-f"),
                forest_card("p1-g"),
                forest_card("p1-h"),
                forest_card("p1-i"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-keep"),
                forest_card("p2-a"),
                forest_card("p2-b"),
                forest_card("p2-c"),
                forest_card("p2-d"),
                forest_card("p2-e"),
                forest_card("p2-f"),
                forest_card("p2-g"),
                forest_card("p2-h"),
                forest_card("p2-i"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];
    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-public-pass-priority-discard"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");

    let discard_spell_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-discard"))
        .expect("discard spell should be in hand")
        .id()
        .clone();
    let chosen_id = first_hand_card_id(&game, "p2");

    service.execute_public_command(
        &mut game,
        PublicGameCommand::CastSpell(
            CastSpellCommand::new(PlayerId::new("p1"), discard_spell_id)
                .with_target(SpellTarget::Player(PlayerId::new("p2")))
                .with_choice(SpellChoice::HandCard(chosen_id.clone())),
        ),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p1"))),
    );
    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p2"))),
    );

    assert!(matches!(
        application.emitted_events.as_slice(),
        [
            DomainEvent::PriorityPassed(_),
            DomainEvent::CardDiscarded(discarded),
            DomainEvent::CardMovedZone(moved),
            DomainEvent::StackTopResolved(_),
            DomainEvent::SpellCast(_),
        ] if discarded.card_id == chosen_id
            && discarded.discard_kind == DiscardKind::SpellEffect
            && moved.card_id == chosen_id
            && moved.origin_zone.as_str() == "hand"
            && moved.destination_zone.as_str() == "graveyard"
    ));
}

#[test]
fn execute_public_command_preserves_terminal_loot_draw_before_resolution_events() {
    let (service, mut game, loot_id) = terminal_loot_game_on_stack();

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p2"))),
    );

    assert!(matches!(
        application.emitted_events.as_slice(),
        [
            DomainEvent::PriorityPassed(_),
            DomainEvent::CardDrawn(_),
            DomainEvent::StackTopResolved(_),
            DomainEvent::SpellCast(_),
            DomainEvent::GameEnded(_),
        ]
    ));
    assert!(game.is_over());
    assert!(player(&game, "p1").graveyard_card(&loot_id).is_some());
}

#[test]
fn execute_public_command_preserves_optional_effect_event_order() {
    let (service, mut game) = optional_effect_game_in_pending_choice();

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::ResolveOptionalEffect(ResolveOptionalEffectCommand::accept(
            PlayerId::new("player-1"),
        )),
    );

    assert!(matches!(
        application.emitted_events.as_slice(),
        [
            DomainEvent::LifeChanged(_),
            DomainEvent::StackTopResolved(_)
        ]
    ));
}

#[test]
fn execute_public_command_surfaces_surveil_graveyard_move_before_resolution_close() {
    let (service, mut game, looked_at_card_id) = surveil_game_in_pending_choice();

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::ResolvePendingSurveil(ResolvePendingSurveilCommand::move_to_graveyard(
            PlayerId::new("p1"),
        )),
    );

    assert!(matches!(
        application.emitted_events.as_slice(),
        [
            DomainEvent::CardMovedZone(moved),
            DomainEvent::StackTopResolved(_),
            DomainEvent::SpellCast(_),
        ] if moved.card_id == looked_at_card_id
            && moved.zone_owner_id == PlayerId::new("p1")
            && moved.origin_zone.as_str() == "library"
            && moved.destination_zone.as_str() == "graveyard"
    ));
}

#[test]
fn execute_public_command_surfaces_zone_move_for_sacrifice_activation_cost() {
    let (service, mut game) = setup_two_player_game(
        "game-public-activate-sacrifice",
        filled_library(
            vec![sacrifice_life_gain_artifact_card("star-shard", 0, 2)],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("star-shard"))
        .expect("sacrifice artifact should be in hand")
        .id()
        .clone();

    service.execute_public_command(
        &mut game,
        PublicGameCommand::CastSpell(CastSpellCommand::new(
            PlayerId::new("player-1"),
            artifact_id.clone(),
        )),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("player-2"))),
    );

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::ActivateAbility(ActivateAbilityCommand::new(
            PlayerId::new("player-1"),
            artifact_id.clone(),
        )),
    );

    assert!(matches!(
        application.emitted_events.as_slice(),
        [
            DomainEvent::CardMovedZone(moved),
            DomainEvent::ActivatedAbilityPutOnStack(_),
        ] if moved.card_id == artifact_id
            && moved.zone_owner_id == PlayerId::new("player-1")
            && moved.origin_zone.as_str() == "battlefield"
            && moved.destination_zone.as_str() == "graveyard"
    ));
}

#[test]
fn execute_public_command_orders_creature_sacrifice_death_before_zone_change() {
    let (service, mut game) = setup_two_player_game(
        "game-public-creature-sacrifice-order",
        filled_library(
            vec![sacrifice_life_gain_creature_card(
                "grim-offering",
                0,
                1,
                1,
                2,
            )],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("grim-offering"))
        .expect("creature should be in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .expect("creature should cast");
    resolve_top_stack_with_passes(&service, &mut game);

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::ActivateAbility(ActivateAbilityCommand::new(
            PlayerId::new("player-1"),
            creature_id.clone(),
        )),
    );

    assert!(matches!(
        application.emitted_events.as_slice(),
        [
            DomainEvent::CreatureDied(died),
            DomainEvent::CardMovedZone(moved),
            DomainEvent::ActivatedAbilityPutOnStack(_),
        ] if died.card_id == creature_id
            && moved.card_id == creature_id
            && moved.zone_owner_id == PlayerId::new("player-1")
            && moved.origin_zone.as_str() == "battlefield"
            && moved.destination_zone.as_str() == "graveyard"
    ));
}

#[test]
fn execute_public_command_surfaces_cleanup_discard_zone_change() {
    let (service, mut game) = setup_two_player_game(
        "game-public-cleanup-discard",
        filled_library(vec![creature_card("c1", 0, 2, 2)], 20),
        filled_library(Vec::new(), 20),
    );
    advance_to_player_phase_satisfying_cleanup(&service, &mut game, "player-1", Phase::EndStep);

    let discarded_id = player(&game, "player-1")
        .hand_card_at(0)
        .expect("cleanup discard should have one hand card to discard")
        .id()
        .clone();

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::DiscardForCleanup(DiscardForCleanupCommand::new(
            PlayerId::new("player-1"),
            discarded_id.clone(),
        )),
    );

    assert!(matches!(
        application.emitted_events.as_slice(),
        [
            DomainEvent::CardDiscarded(discarded),
            DomainEvent::CardMovedZone(moved),
        ] if discarded.card_id == discarded_id
            && discarded.discard_kind == DiscardKind::CleanupHandSize
            && moved.card_id == discarded_id
            && moved.origin_zone.as_str() == "hand"
            && moved.destination_zone.as_str() == "graveyard"
    ));
}
