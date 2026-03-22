#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_phase_satisfying_cleanup, creature_card, filled_library, setup_two_player_game,
};
use demonictutor::{
    CardInstanceId, CardType, CastSpellCommand, PassPriorityCommand, Phase, PlayerId,
    SpellCastOutcome,
};

#[test]
fn active_player_can_cast_and_resolve_a_creature_in_second_main() {
    let (service, mut game) = setup_two_player_game(
        "game-second-main-creature",
        filled_library(vec![creature_card("grizzly-bears", 0, 2, 2)], 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_phase_satisfying_cleanup(&service, &mut game, Phase::SecondMain);

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-second-main-creature-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(
        outcome.spell_put_on_stack.card_type,
        CardType::Creature
    ));
    assert_eq!(game.stack().len(), 1);

    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    let resolution = service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let spell_cast = resolution.spell_cast.unwrap();
    assert!(matches!(
        spell_cast.outcome,
        SpellCastOutcome::EnteredBattlefield
    ));
    assert_eq!(game.players()[0].battlefield().cards().len(), 1);
    assert_eq!(game.players()[0].graveyard_size(), 0);
}
