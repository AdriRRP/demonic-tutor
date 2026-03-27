//! Tests the public legal-action surface.

#![allow(clippy::expect_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, advance_to_player_phase_satisfying_cleanup,
    advance_turn_raw, close_empty_priority_window, create_service, creature_card,
    creature_card_with_keyword, filled_library, forest_card, instant_card,
    pacifism_creature_aura_enchantment_card, player, player_deck, player_library,
    resolve_top_stack_with_passes, setup_two_player_game,
};
use demonictutor::{
    legal_actions, CardDefinitionId, CastSpellCommand, DealOpeningHandsCommand,
    DeclareAttackersCommand, DrawCardsEffectCommand, Game, GameId, KeywordAbility, Phase, PlayerId,
    PublicLegalAction, SpellTarget, StartGameCommand,
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

#[test]
fn legal_actions_do_not_offer_pacified_creature_as_blocker_option() {
    let (service, mut game) = setup_two_player_game(
        "game-legal-actions-pacifism-block",
        filled_library(vec![creature_card("attacker", 0, 2, 2)], 10),
        filled_library(
            vec![
                creature_card("blocker", 0, 2, 2),
                pacifism_creature_aura_enchantment_card("pacifism-lite", 0),
            ],
            10,
        ),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("attacker"))
        .expect("attacker should be in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id.clone()),
        )
        .expect("attacker should be castable");
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = player(&game, "player-2")
        .hand_card_by_definition(&CardDefinitionId::new("blocker"))
        .expect("blocker should be in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .expect("blocker should be castable");
    resolve_top_stack_with_passes(&service, &mut game);

    let aura_id = player(&game, "player-2")
        .hand_card_by_definition(&CardDefinitionId::new("pacifism-lite"))
        .expect("aura should be in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), aura_id)
                .with_target(SpellTarget::Creature(blocker_id.clone())),
        )
        .expect("aura should be castable");
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![attacker_id]),
        )
        .expect("attacker should be declared");
    close_empty_priority_window(&service, &mut game);

    let actions = legal_actions(&game);

    assert!(actions.iter().any(|action| matches!(
        action,
        PublicLegalAction::DeclareBlockers { player_id, blocker_options, .. }
            if player_id.as_str() == "player-2"
                && blocker_options.iter().all(|option| option.blocker_id != blocker_id)
    )));
}

#[test]
fn legal_actions_do_not_offer_single_blocker_option_against_menace() {
    let (service, mut game) = setup_two_player_game(
        "game-legal-actions-menace-block",
        filled_library(
            vec![creature_card_with_keyword(
                "menacer",
                0,
                2,
                2,
                KeywordAbility::Menace,
            )],
            10,
        ),
        filled_library(vec![creature_card("lone-blocker", 0, 2, 2)], 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    let attacker_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("menacer"))
        .expect("menace attacker should be in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), attacker_id),
        )
        .expect("menace attacker should be castable");
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-2");
    let blocker_id = player(&game, "player-2")
        .hand_card_by_definition(&CardDefinitionId::new("lone-blocker"))
        .expect("lone blocker should be in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-2"), blocker_id.clone()),
        )
        .expect("lone blocker should be castable");
    resolve_top_stack_with_passes(&service, &mut game);

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");
    advance_turn_raw(&service, &mut game);
    close_empty_priority_window(&service, &mut game);
    advance_turn_raw(&service, &mut game);

    let menacing_attacker_id = player(&game, "player-1")
        .battlefield_card_by_definition(&CardDefinitionId::new("menacer"))
        .expect("menace attacker should be on battlefield")
        .id()
        .clone();
    service
        .declare_attackers(
            &mut game,
            DeclareAttackersCommand::new(PlayerId::new("player-1"), vec![menacing_attacker_id]),
        )
        .expect("menace attacker should be declared");
    close_empty_priority_window(&service, &mut game);

    let actions = legal_actions(&game);

    assert!(actions.iter().any(|action| matches!(
        action,
        PublicLegalAction::DeclareBlockers { player_id, blocker_options, .. }
            if player_id.as_str() == "player-2"
                && blocker_options.iter().all(|option| option.blocker_id != blocker_id)
    )));
}
