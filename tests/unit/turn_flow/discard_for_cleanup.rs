#![allow(clippy::unwrap_used)]

use crate::support::{
    advance_n_raw, advance_to_player_first_main_satisfying_cleanup, creature_library,
    setup_two_player_game,
};
use demonictutor::{
    AdvanceTurnCommand, DiscardForCleanupCommand, DiscardKind, DomainError, GameError, Phase,
    PhaseError, PlayerId,
};

fn setup_game_with_eight_cards_in_hand() -> (crate::support::TestService, demonictutor::Game) {
    let (service, mut game) =
        setup_two_player_game("game-1", creature_library(20), creature_library(20));

    advance_n_raw(&service, &mut game, 7);

    assert_eq!(game.phase(), &Phase::EndStep);
    assert_eq!(game.players()[0].hand().cards().len(), 8);

    (service, game)
}

#[test]
fn discard_for_cleanup_moves_card_from_hand_to_graveyard_during_end_step_cleanup() {
    let (service, mut game) = setup_game_with_eight_cards_in_hand();
    let card_id = game.players()[0].hand().cards()[0].id().clone();

    let event = service
        .discard_for_cleanup(
            &mut game,
            DiscardForCleanupCommand::new(PlayerId::new("player-1"), card_id.clone()),
        )
        .unwrap();

    assert_eq!(event.card_id, card_id);
    assert_eq!(event.discard_kind, DiscardKind::CleanupHandSize);
    assert_eq!(game.players()[0].hand().cards().len(), 7);
    assert_eq!(game.players()[0].graveyard().cards().len(), 1);
    assert_eq!(game.players()[0].graveyard().cards()[0].id(), &card_id);
}

#[test]
fn discard_for_cleanup_fails_outside_end_step() {
    let (service, mut game) =
        setup_two_player_game("game-1", creature_library(20), creature_library(20));
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let card_id = game.players()[0].hand().cards()[0].id().clone();

    let error = service
        .discard_for_cleanup(
            &mut game,
            DiscardForCleanupCommand::new(PlayerId::new("player-1"), card_id),
        )
        .unwrap_err();

    assert_eq!(
        error,
        DomainError::Phase(PhaseError::InvalidForDiscard {
            phase: Phase::FirstMain
        })
    );
}

#[test]
fn discard_for_cleanup_fails_when_not_active_players_turn() {
    let (service, mut game) = setup_game_with_eight_cards_in_hand();
    let card_id = game.players()[0].hand().cards()[0].id().clone();

    let error = service
        .discard_for_cleanup(
            &mut game,
            DiscardForCleanupCommand::new(PlayerId::new("player-2"), card_id),
        )
        .unwrap_err();

    assert_eq!(
        error,
        DomainError::Game(GameError::NotYourTurn {
            current: PlayerId::new("player-1"),
            requested: PlayerId::new("player-2"),
        })
    );
}

#[test]
fn discard_for_cleanup_fails_when_cleanup_discard_is_not_required() {
    let (service, mut game) = setup_game_with_eight_cards_in_hand();
    let first_card_id = game.players()[0].hand().cards()[0].id().clone();
    service
        .discard_for_cleanup(
            &mut game,
            DiscardForCleanupCommand::new(PlayerId::new("player-1"), first_card_id),
        )
        .unwrap();

    let second_card_id = game.players()[0].hand().cards()[0].id().clone();
    let error = service
        .discard_for_cleanup(
            &mut game,
            DiscardForCleanupCommand::new(PlayerId::new("player-1"), second_card_id),
        )
        .unwrap_err();

    assert_eq!(
        error,
        DomainError::Game(GameError::DiscardNotRequired {
            player: PlayerId::new("player-1"),
            hand_size: 7,
            max_hand_size: 7,
        })
    );
}

#[test]
fn advance_turn_fails_from_end_step_while_cleanup_discard_is_still_required() {
    let (service, mut game) = setup_game_with_eight_cards_in_hand();

    let error = service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap_err();

    assert_eq!(
        error,
        DomainError::Game(GameError::HandSizeLimitExceeded {
            player: PlayerId::new("player-1"),
            hand_size: 8,
            max_hand_size: 7,
        })
    );
    assert_eq!(game.phase(), &Phase::EndStep);
}

#[test]
fn advance_turn_succeeds_after_discarding_down_to_maximum_hand_size() {
    let (service, mut game) = setup_game_with_eight_cards_in_hand();
    let card_id = game.players()[0].hand().cards()[0].id().clone();

    service
        .discard_for_cleanup(
            &mut game,
            DiscardForCleanupCommand::new(PlayerId::new("player-1"), card_id),
        )
        .unwrap();

    service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();

    assert_eq!(game.phase(), &Phase::Untap);
    assert_eq!(game.active_player(), &PlayerId::new("player-2"));
}
