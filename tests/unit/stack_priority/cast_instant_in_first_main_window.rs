#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority cast instant in first main window.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, filled_library, instant_card,
        setup_two_player_game,
    },
    demonictutor::{
        CardInstanceId, CastSpellCommand, PassPriorityCommand, PlayerId, SpellCastOutcome,
    },
};

#[test]
fn active_player_can_cast_and_resolve_an_instant_in_first_main() {
    let (service, mut game) = setup_two_player_game(
        "game-first-main-instant",
        filled_library(vec![instant_card("giant-growth", 0)], 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    let card_id = CardInstanceId::new("game-first-main-instant-player-1-0");
    let cast = service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), card_id.clone()),
        )
        .unwrap();

    assert_eq!(cast.spell_put_on_stack.card_id, card_id);
    assert_eq!(game.stack().len(), 1);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );

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
    assert_eq!(spell_cast.card_id, card_id);
    assert!(matches!(
        spell_cast.outcome,
        SpellCastOutcome::ResolvedToGraveyard
    ));
    assert_eq!(game.stack().len(), 0);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
}
