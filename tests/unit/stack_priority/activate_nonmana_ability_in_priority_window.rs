#![allow(clippy::unwrap_used)]
#![allow(clippy::panic)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, filled_library, instant_card,
    life_gain_artifact_card, player, setup_two_player_game,
};
use demonictutor::{
    ActivateAbilityCommand, CardError, CardInstanceId, CastSpellCommand, DomainError,
    PassPriorityCommand, PlayerId,
};

#[test]
fn activated_ability_uses_main_phase_priority_window_and_the_stack() {
    let (service, mut game) = setup_two_player_game(
        "game-activate-ability-main",
        filled_library(vec![life_gain_artifact_card("ivory-cup-lite", 0, 1)], 10),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("ivory-cup-lite"))
        .unwrap()
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), artifact_id.clone()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let outcome = service
        .activate_ability(
            &mut game,
            ActivateAbilityCommand::new(PlayerId::new("player-1"), artifact_id.clone()),
        )
        .unwrap();

    assert_eq!(
        outcome.activated_ability_put_on_stack.source_card_id,
        artifact_id
    );
    assert_eq!(game.stack().len(), 1);
    assert_eq!(
        game.priority().unwrap().current_holder(),
        &PlayerId::new("player-1")
    );
    assert!(player(&game, "player-1")
        .battlefield_card(&artifact_id)
        .unwrap()
        .is_tapped());
}

#[test]
fn activated_ability_can_be_used_as_a_response_when_you_hold_priority() {
    let (service, mut game) = setup_two_player_game(
        "game-activate-ability-response",
        filled_library(vec![life_gain_artifact_card("ivory-cup-lite", 0, 1)], 10),
        filled_library(vec![instant_card("shock", 0)], 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("ivory-cup-lite"))
        .unwrap()
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), artifact_id.clone()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let shock_id = player(&game, "player-2")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("shock"))
        .unwrap()
        .id()
        .clone();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), shock_id),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let outcome = service
        .activate_ability(
            &mut game,
            ActivateAbilityCommand::new(PlayerId::new("player-1"), artifact_id),
        )
        .unwrap();

    assert_eq!(game.stack().len(), 2);
    assert_eq!(
        outcome.activated_ability_put_on_stack.player_id,
        PlayerId::new("player-1")
    );
}

#[test]
fn activated_ability_requires_the_current_priority_holder() {
    let (service, mut game) = setup_two_player_game(
        "game-activate-ability-priority",
        filled_library(vec![life_gain_artifact_card("ivory-cup-lite", 0, 1)], 10),
        filled_library(vec![instant_card("shock", 0)], 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("ivory-cup-lite"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), artifact_id.clone()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    let shock_id = player(&game, "player-2")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("shock"))
        .unwrap()
        .id()
        .clone();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), shock_id),
        )
        .unwrap();

    let err = service
        .activate_ability(
            &mut game,
            ActivateAbilityCommand::new(PlayerId::new("player-1"), artifact_id),
        )
        .unwrap_err();

    assert!(matches!(
        err,
        DomainError::Game(demonictutor::GameError::NotPriorityHolder { .. })
    ));
}

#[test]
fn activated_ability_resolution_changes_life() {
    let (service, mut game) = setup_two_player_game(
        "game-activate-ability-resolve",
        filled_library(vec![life_gain_artifact_card("ivory-cup-lite", 0, 1)], 10),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let artifact_id = player(&game, "player-1")
        .hand_card_by_definition(&demonictutor::CardDefinitionId::new("ivory-cup-lite"))
        .unwrap()
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), artifact_id.clone()),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();
    service
        .activate_ability(
            &mut game,
            ActivateAbilityCommand::new(PlayerId::new("player-1"), artifact_id),
        )
        .unwrap();

    let resolve_first = service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-1")),
        )
        .unwrap();
    let resolve_second = service
        .pass_priority(
            &mut game,
            PassPriorityCommand::new(PlayerId::new("player-2")),
        )
        .unwrap();

    assert!(resolve_first.stack_top_resolved.is_none());
    assert_eq!(resolve_second.life_changed.unwrap().to_life, 21);
    assert!(resolve_second.spell_cast.is_none());
    assert_eq!(player(&game, "player-1").life(), 21);
}

#[test]
fn cards_without_supported_activated_ability_are_rejected() {
    let (service, mut game) = setup_two_player_game(
        "game-activate-ability-missing",
        filled_library(vec![life_gain_artifact_card("ivory-cup-lite", 0, 1)], 10),
        filled_library(Vec::new(), 10),
    );
    advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let nonexistent = CardInstanceId::new("game-activate-ability-missing-player-1-99");

    let err = service
        .activate_ability(
            &mut game,
            ActivateAbilityCommand::new(PlayerId::new("player-1"), nonexistent),
        )
        .unwrap_err();

    assert!(matches!(
        err,
        DomainError::Card(CardError::NotOnBattlefield { .. })
    ));
}
