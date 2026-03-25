#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority dies trigger foundation.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, dies_life_gain_creature_card, filled_library,
        player, setup_two_player_game, targeted_destroy_creature_instant_card,
        targeted_exile_creature_instant_card,
    },
    demonictutor::{
        CardDefinitionId, CastSpellCommand, PassPriorityCommand, PlayerId, SpellTarget,
        TriggeredAbilityEvent,
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
        .pass_priority(game, PassPriorityCommand::new(first_holder))
        .unwrap();

    let second_holder = game.priority().unwrap().current_holder().clone();
    service
        .pass_priority(game, PassPriorityCommand::new(second_holder))
        .unwrap()
}

#[test]
fn supported_creature_death_enqueues_a_dies_trigger() {
    let (service, mut game) = setup_two_player_game(
        "game-dies-trigger",
        filled_library(
            vec![
                targeted_destroy_creature_instant_card("murder-lite", 0),
                dies_life_gain_creature_card("soul-wisp", 0, 2, 2, 2),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("soul-wisp"))
        .unwrap()
        .id()
        .clone();
    let removal_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("murder-lite"))
        .unwrap()
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    resolve_current_stack(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), removal_id)
                .with_target(SpellTarget::Creature(creature_id.clone())),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);

    assert_eq!(resolution.creatures_died.len(), 1);
    assert_eq!(resolution.creatures_died[0].card_id, creature_id);
    assert_eq!(resolution.triggered_abilities_put_on_stack.len(), 1);
    assert_eq!(
        resolution.triggered_abilities_put_on_stack[0].trigger,
        TriggeredAbilityEvent::Dies
    );
    assert_eq!(player(&game, "player-1").life(), 20);
    assert_eq!(game.stack().len(), 1);
}

#[test]
fn exiling_a_creature_does_not_enqueue_a_dies_trigger() {
    let (service, mut game) = setup_two_player_game(
        "game-dies-trigger-exile",
        filled_library(
            vec![
                targeted_exile_creature_instant_card("swords-lite", 0),
                dies_life_gain_creature_card("soul-wisp", 0, 2, 2, 2),
            ],
            10,
        ),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("soul-wisp"))
        .unwrap()
        .id()
        .clone();
    let exile_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("swords-lite"))
        .unwrap()
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id.clone()),
        )
        .unwrap();
    resolve_current_stack(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), exile_id)
                .with_target(SpellTarget::Creature(creature_id)),
        )
        .unwrap();

    let resolution = resolve_current_stack(&service, &mut game);

    assert!(resolution.creatures_died.is_empty());
    assert!(resolution.triggered_abilities_put_on_stack.is_empty());
    assert_eq!(game.stack().len(), 0);
}
