#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority sorcery speed spells require active player priority.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, artifact_card, creature_card, enchantment_card,
        filled_library, planeswalker_card, setup_two_player_game, sorcery_card,
    },
    demonictutor::{
        CardInstanceId, CastSpellCommand, DomainError, GameError, PassPriorityCommand, PlayerId,
    },
};

#[test]
fn non_active_player_cannot_cast_sorcery_speed_spells_in_an_empty_main_phase_window() {
    let player_two_library = filled_library(
        vec![
            creature_card("grizzly-bears", 0, 2, 2),
            sorcery_card("divination", 0),
            artifact_card("howling-mine", 0),
            enchantment_card("glorious-anthem", 0),
            planeswalker_card("jace", 0),
        ],
        10,
    );

    let (service, mut game) = setup_two_player_game(
        "game-sorcery-speed-priority",
        filled_library(Vec::new(), 10),
        player_two_library,
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    for suffix in 0..5 {
        let card_id = CardInstanceId::new(format!("game-sorcery-speed-priority-player-2-{suffix}"));
        let result = service.cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), card_id.clone()),
        );

        assert!(matches!(
            result,
            Err(DomainError::Game(GameError::CastingTimingNotAllowed { card, permission }))
                if card == card_id
                    && permission
                        == demonictutor::CastingPermissionProfile::active_player_empty_main_phase_window()
        ));
    }

    assert!(game.stack().is_empty());
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-2")
    );
}
