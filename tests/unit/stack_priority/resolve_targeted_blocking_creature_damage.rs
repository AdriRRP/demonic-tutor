#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, close_empty_priority_window, filled_library,
    resolve_top_stack_with_passes, setup_two_player_game,
    targeted_blocking_creature_damage_instant_card,
};
use demonictutor::{
    CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand,
    DeclareBlockersCommand, LibraryCard, PlayerId, SpellTarget,
};

#[test]
fn targeted_blocking_creature_spell_can_destroy_a_blocker_after_blockers() {
    let (service, mut game) = setup_two_player_game(
        "game-target-blocking-lethal",
        filled_library(
            vec![
                LibraryCard::creature(CardDefinitionId::new("attacker"), 0, 2, 2),
                targeted_blocking_creature_damage_instant_card("hold-the-line", 0, 2),
            ],
            10,
        ),
        filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("blocker"),
                0,
                2,
                2,
            )],
            10,
        ),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = CardInstanceId::new("game-target-blocking-lethal-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = CardInstanceId::new("game-target-blocking-lethal-player-2-0");
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

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-target-blocking-lethal-player-1-1"),
            )
            .with_target(SpellTarget::Creature(blocker_id.clone())),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    assert!(game.players()[1]
        .battlefield()
        .cards()
        .iter()
        .all(|card| card.id() != &blocker_id));
    assert!(game.players()[1].graveyard_contains(&blocker_id));
}
