#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, close_empty_priority_window, filled_library,
    resolve_top_stack_with_passes, setup_two_player_game,
    targeted_blocking_creature_damage_instant_card,
};
use demonictutor::{
    CardDefinitionId, CardInstanceId, CastSpellCommand, DeclareAttackersCommand,
    DeclareBlockersCommand, LibraryCard, Phase, PlayerId, SpellTarget,
};

#[test]
fn active_player_can_cast_a_blocking_creature_spell_after_blockers() {
    let (service, mut game) = setup_two_player_game(
        "game-target-after-blockers",
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
    let attacker_id = CardInstanceId::new("game-target-after-blockers-player-1-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = CardInstanceId::new("game-target-after-blockers-player-2-0");
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

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-target-after-blockers-player-1-1"),
            )
            .with_target(SpellTarget::Creature(blocker_id.clone())),
        )
        .unwrap();

    assert_eq!(
        outcome.spell_put_on_stack.target,
        Some(SpellTarget::Creature(blocker_id))
    );
    assert_eq!(game.stack().len(), 1);
}
