#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, advance_to_phase_satisfying_cleanup, filled_library,
    land_card, setup_two_player_game, sorcery_card,
};
use demonictutor::{
    CardInstanceId, CardType, CastSpellCommand, PassPriorityCommand, Phase, PlayerId,
    SpellCastOutcome,
};

#[test]
fn active_player_can_cast_and_resolve_a_sorcery_in_first_main() {
    let (service, mut game) = setup_two_player_game(
        "game-first-main-sorcery",
        filled_library(vec![sorcery_card("divination", 0)], 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-first-main-sorcery-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(
        outcome.spell_put_on_stack.card_type,
        CardType::Sorcery
    ));
    assert_eq!(game.stack().len(), 1);
    assert_eq!(game.phase(), &Phase::FirstMain);

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
        SpellCastOutcome::ResolvedToGraveyard
    ));
    assert_eq!(game.stack().len(), 0);
    assert_eq!(game.players()[0].graveyard_size(), 1);
}

#[test]
fn active_player_can_cast_and_resolve_a_sorcery_in_second_main() {
    let (service, mut game) = setup_two_player_game(
        "game-second-main-sorcery",
        filled_library(vec![sorcery_card("tidings", 0)], 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_phase_satisfying_cleanup(&service, &mut game, Phase::SecondMain);

    assert_eq!(game.phase(), &Phase::SecondMain);

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-second-main-sorcery-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(
        outcome.spell_put_on_stack.card_type,
        CardType::Sorcery
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
        SpellCastOutcome::ResolvedToGraveyard
    ));
    assert_eq!(game.stack().len(), 0);
    assert_eq!(game.players()[0].graveyard_size(), 1);
}

#[test]
fn active_player_can_cast_a_sorcery_while_holding_an_empty_first_main_priority_window() {
    let (service, mut game) = setup_two_player_game(
        "game-first-main-empty-window-sorcery",
        filled_library(vec![sorcery_card("counsel", 0)], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
    assert!(game.stack().is_empty());

    let outcome = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-first-main-empty-window-sorcery-player-1-0"),
            ),
        )
        .unwrap();

    assert!(matches!(
        outcome.spell_put_on_stack.card_type,
        CardType::Sorcery
    ));
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}
