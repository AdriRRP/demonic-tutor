#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority nonlethal opponents blocking target damage.

use {
    crate::support::{
        advance_to_player_first_main_satisfying_cleanup, close_empty_priority_window,
        filled_library, resolve_top_stack_with_passes, setup_two_player_game,
        targeted_opponents_blocking_creature_damage_instant_card,
    },
    demonictutor::{
        CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand,
        DeclareBlockersCommand, LibraryCard, Phase, PlayerId, SpellTarget,
    },
};

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
fn opponents_blocking_creature_spell_marks_nonlethal_damage_and_leaves_the_blocker_in_combat() {
    let (service, mut game) = setup_two_player_game(
        "game-target-opponents-blocking-nonlethal",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker"), 0, 2, 2),
                targeted_opponents_blocking_creature_damage_instant_card("punish-shield", 0, 1),
            ],
            10,
        ),
        filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("blocker"),
                0,
                2,
                3,
            )],
            10,
        ),
    );

    let spell_id = hand_card_id_by_definition(&game, 0, "punish-shield");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-target-opponents-blocking-nonlethal-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = CardInstanceId::new("game-target-opponents-blocking-nonlethal-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    crate::support::advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    crate::support::advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id.clone()]),
        )
        .unwrap();
    close_empty_priority_window(&service, &mut game);
    service
        .declare_blockers(
            &mut game,
            DeclareBlockersCommand::new(
                PlayerId::new("player-2"),
                vec![(blocker_id.clone(), attacker_id)],
            ),
        )
        .unwrap();

    assert_eq!(game.phase(), &Phase::CombatDamage);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), spell_id)
                .with_target(SpellTarget::Creature(blocker_id.clone())),
        )
        .unwrap();

    resolve_top_stack_with_passes(&service, &mut game);

    let blocker = game.players()[1]
        .battlefield_cards()
        .find(|card| card.id() == &blocker_id)
        .unwrap();
    assert_eq!(blocker.damage(), 1);
    assert!(blocker.is_blocking());
}
