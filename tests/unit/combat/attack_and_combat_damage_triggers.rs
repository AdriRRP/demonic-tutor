#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

//! Unit coverage for unit combat attack and combat damage triggers.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, advance_turn_raw,
        attacks_life_gain_haste_creature_card, cast_spell_and_resolve, close_empty_priority_window,
        combat_damage_to_player_life_gain_haste_creature_card, filled_library, player,
        setup_two_player_game,
    },
    demonictutor::{
        CardDefinitionId, DeclareAttackersCommand, DeclareBlockersCommand, PassPriorityCommand,
        PlayerId, ResolveCombatDamageCommand, TriggeredAbilityEvent,
    },
};

fn resolve_current_stack(
    service: &demonictutor::GameService<
        demonictutor::InMemoryEventStore,
        demonictutor::InMemoryEventBus,
    >,
    game: &mut demonictutor::Game,
) {
    let first_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, PassPriorityCommand::new(first_holder))
        .unwrap();

    let second_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, PassPriorityCommand::new(second_holder))
        .unwrap();
}

#[test]
fn attack_trigger_is_put_on_stack_when_creature_is_declared_as_attacker() {
    let (service, mut game) = setup_two_player_game(
        "game-attack-trigger",
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
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", creature_id.clone());
    advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);

    let outcome = service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![creature_id]),
        )
        .unwrap();

    assert_eq!(outcome.triggered_abilities_put_on_stack.len(), 1);
    assert_eq!(
        outcome.triggered_abilities_put_on_stack[0].trigger,
        TriggeredAbilityEvent::Attacks
    );
    assert_eq!(game.stack().len(), 1);
    assert_eq!(player(&game, "player-1").life(), 20);

    resolve_current_stack(&service, &mut game);

    assert_eq!(player(&game, "player-1").life(), 22);
}

#[test]
fn combat_damage_to_player_trigger_is_put_on_stack_after_damage_resolves() {
    let (service, mut game) = setup_two_player_game(
        "game-combat-damage-trigger",
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
        .unwrap()
        .id()
        .clone();
    cast_spell_and_resolve(&service, &mut game, "player-1", creature_id.clone());
    advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);

    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![creature_id]),
        )
        .unwrap();
    resolve_current_stack(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(PlayerId::new("player-2"), Vec::new()),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);

    let outcome = service
        .resolve_combat_damage(
            &mut game,
            ResolveCombatDamageCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    assert_eq!(outcome.triggered_abilities_put_on_stack.len(), 1);
    assert_eq!(
        outcome.triggered_abilities_put_on_stack[0].trigger,
        TriggeredAbilityEvent::DealsCombatDamageToPlayer
    );
    assert_eq!(player(&game, "player-1").life(), 20);
    assert_eq!(player(&game, "player-2").life(), 18);
    assert_eq!(game.stack().len(), 1);

    resolve_current_stack(&service, &mut game);

    assert_eq!(player(&game, "player-1").life(), 23);
}
