#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority target blocking creature spell foundation.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, filled_library, setup_two_player_game,
        targeted_blocking_creature_damage_instant_card,
    },
    demonictutor::{
        CardInstanceId, CastSpellCommand, DomainError, GameError, PlayerId, SpellTarget,
    },
};

#[test]
fn targeted_blocking_creature_spell_rejects_a_player_target_when_cast() {
    let (service, mut game) = setup_two_player_game(
        "game-target-blocking-illegal-kind",
        filled_library(
            vec![targeted_blocking_creature_damage_instant_card(
                "hold-the-line",
                0,
                2,
            )],
            10,
        ),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let spell_id = CardInstanceId::new("game-target-blocking-illegal-kind-player-1-0");
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
