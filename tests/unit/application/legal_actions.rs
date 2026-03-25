//! Tests the public legal-action surface.

#![allow(clippy::expect_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, advance_to_player_phase_satisfying_cleanup,
    close_empty_priority_window, create_service, creature_card, forest_card, instant_card, player,
    player_deck, player_library,
};
use demonictutor::{
    legal_actions, DealOpeningHandsCommand, DrawCardsEffectCommand, Game, GameId, Phase, PlayerId,
    PublicLegalAction, StartGameCommand,
};

fn game_with_first_main_priority() -> Game {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                forest_card("p1-forest-a"),
                instant_card("p1-free-spell", 0),
                creature_card("p1-bear", 2, 2, 2),
                forest_card("p1-forest-b"),
                forest_card("p1-forest-c"),
                forest_card("p1-forest-d"),
                forest_card("p1-forest-e"),
                forest_card("p1-forest-f"),
                forest_card("p1-forest-g"),
                forest_card("p1-forest-h"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-forest-a"),
                forest_card("p2-forest-b"),
                forest_card("p2-forest-c"),
                forest_card("p2-forest-d"),
                forest_card("p2-forest-e"),
                forest_card("p2-forest-f"),
                forest_card("p2-forest-g"),
                forest_card("p2-forest-h"),
                forest_card("p2-forest-i"),
                forest_card("p2-forest-j"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-legal-actions"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");
    game
}

#[test]
fn legal_actions_surface_priority_holder_options_in_first_main() {
    let game = game_with_first_main_priority();

    let actions = legal_actions(&game);

    assert!(actions.iter().any(|action| matches!(
        action,
        PublicLegalAction::PassPriority { player_id } if player_id.as_str() == "p1"
    )));
    assert!(actions.iter().any(|action| matches!(
        action,
        PublicLegalAction::PlayLand { player_id, playable_land_ids } if player_id.as_str() == "p1" && !playable_land_ids.is_empty()
    )));
    assert!(actions.iter().any(|action| matches!(
        action,
        PublicLegalAction::CastSpell { player_id, castable_cards } if player_id.as_str() == "p1"
            && castable_cards.iter().any(|card| card.definition_id.as_str() == "p1-free-spell")
    )));
}

#[test]
fn legal_actions_surface_cleanup_discard_when_hand_is_too_large() {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                forest_card("p1-forest-a"),
                forest_card("p1-forest-b"),
                forest_card("p1-forest-c"),
                forest_card("p1-forest-d"),
                forest_card("p1-forest-e"),
                forest_card("p1-forest-f"),
                forest_card("p1-forest-g"),
                forest_card("p1-forest-h"),
                forest_card("p1-forest-i"),
                forest_card("p1-forest-j"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-forest-a"),
                forest_card("p2-forest-b"),
                forest_card("p2-forest-c"),
                forest_card("p2-forest-d"),
                forest_card("p2-forest-e"),
                forest_card("p2-forest-f"),
                forest_card("p2-forest-g"),
                forest_card("p2-forest-h"),
                forest_card("p2-forest-i"),
                forest_card("p2-forest-j"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-cleanup-actions"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");
    service
        .draw_cards_effect(
            &mut game,
            &DrawCardsEffectCommand::new(PlayerId::new("p1"), PlayerId::new("p1"), 1),
        )
        .expect("explicit draw should be legal in first main");
    advance_to_player_phase_satisfying_cleanup(&service, &mut game, "p1", Phase::EndStep);
    close_empty_priority_window(&service, &mut game);

    let actions = legal_actions(&game);
    let expected_hand_size = player(&game, "p1").hand_size();
    assert!(actions.iter().any(|action| matches!(
        action,
        PublicLegalAction::DiscardForCleanup { player_id, card_ids }
            if player_id.as_str() == "p1" && card_ids.len() == expected_hand_size
    )));
}
