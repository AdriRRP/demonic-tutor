#![allow(clippy::unwrap_used)]

//! Unit coverage for unit stack priority reject own turn priority enchantment response.

use {
    crate::support::{
        advance_to_first_main_satisfying_cleanup, filled_library, instant_card,
        own_turn_priority_enchantment_card, setup_two_player_game,
    },
    demonictutor::{
        CardInstanceId, CastSpellCommand, DomainError, GameError, PassPriorityCommand, PlayerId,
    },
};

#[test]
fn non_active_player_cannot_cast_an_own_turn_priority_enchantment_as_a_response() {
    let (service, mut game) = setup_two_player_game(
        "game-respond-own-turn-enchantment",
        filled_library(vec![instant_card("lightning-bolt", 0)], 10),
        filled_library(
            vec![own_turn_priority_enchantment_card("battle-rite", 0)],
            10,
        ),
    );

    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(
                PlayerId::new("player-1"),
                CardInstanceId::new("game-respond-own-turn-enchantment-player-1-0"),
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
            CardInstanceId::new("game-respond-own-turn-enchantment-player-2-0"),
        ),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::CastingTimingNotAllowed { card, permission }))
            if card == CardInstanceId::new("game-respond-own-turn-enchantment-player-2-0")
                && permission.supports(demonictutor::CastingRule::OpenPriorityWindowDuringOwnTurn)
    ));
    assert_eq!(game.stack().len(), 1);
}
