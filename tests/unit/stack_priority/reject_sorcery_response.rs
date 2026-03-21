#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, filled_library, setup_two_player_game, sorcery_card,
};
use demonictutor::{
    CardInstanceId, CastSpellCommand, DomainError, GameError, PassPriorityCommand, PlayerId,
};

#[test]
fn opponent_cannot_cast_a_sorcery_as_a_response_after_the_caster_passes() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-sorcery",
        filled_library(vec![sorcery_card("nights-whisper", 0)], 10),
        filled_library(vec![sorcery_card("sign-in-blood", 0)], 10),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-respond-sorcery-player-1-0"),
            ),
        )
        .unwrap();

    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();

    let result = service.cast_spell(
        &mut game,
        CastSpellCommand::new(
            PlayerId::new("player-2"),
            CardInstanceId::new("game-respond-sorcery-player-2-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::CastingTimingNotAllowed { card, permission }))
            if card == CardInstanceId::new("game-respond-sorcery-player-2-0")
                && permission
                    == demonictutor::CastingPermissionProfile::ActivePlayerEmptyMainPhaseWindow
    ));
    assert_eq!(game.stack().len(), 1);
}
