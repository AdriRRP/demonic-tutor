#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, creature_card, filled_library, land_card,
    resolve_top_stack_with_passes, setup_two_player_game,
    targeted_blocking_creature_damage_instant_card,
};
use demonictutor::{
    CardInstanceId, CastSpellCommand, DomainError, GameError, PlayerId, SpellTarget,
};

#[test]
fn targeted_blocking_creature_spell_rejects_a_non_blocking_creature_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-blocking-nonblocker",
        filled_library(
            vec![
                land_card("alice-setup-land"),
                targeted_blocking_creature_damage_instant_card("hold-the-line", 0, 2),
            ],
            10,
        ),
        filled_library(vec![creature_card("bob-bear", 0, 2, 2)], 10),
    );
    let spell_id = CardInstanceId::new("game-target-blocking-nonblocker-player-1-1");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let creature_id = CardInstanceId::new("game-target-blocking-nonblocker-player-2-0");
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), creature_id.clone()),
        )
        .unwrap();
    resolve_top_stack_with_passes(&service, &mut game);
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
