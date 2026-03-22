#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, advance_to_phase_satisfying_cleanup, filled_library,
    planeswalker_card, setup_two_player_game,
};
use demonictutor::{
    CardInstanceId, CardType, CastSpellCommand, PassPriorityCommand, Phase, PlayerId,
    SpellCastOutcome,
};

#[test]
fn active_player_can_cast_and_resolve_a_planeswalker_in_first_main() {
    let (service, mut game) = setup_two_player_game(
        "game-first-main-planeswalker",
        filled_library(vec![planeswalker_card("ajani", 0)], 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-first-main-planeswalker-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(
        outcome.spell_put_on_stack.card_type,
        CardType::Planeswalker
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
    assert_eq!(game.players()[0].battlefield_size(), 1);
    assert_eq!(game.players()[0].graveyard_size(), 0);
}

#[test]
fn active_player_can_cast_and_resolve_a_planeswalker_in_second_main() {
    let (service, mut game) = setup_two_player_game(
        "game-second-main-planeswalker",
        filled_library(vec![planeswalker_card("garruk", 0)], 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_phase_satisfying_cleanup(&service, &mut game, Phase::SecondMain);

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-second-main-planeswalker-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(
        outcome.spell_put_on_stack.card_type,
        CardType::Planeswalker
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
    assert_eq!(game.players()[0].battlefield_size(), 1);
    assert_eq!(game.players()[0].graveyard_size(), 0);
}
