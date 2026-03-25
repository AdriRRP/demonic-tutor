//! Tests the public choice-request surface.

#![allow(clippy::expect_used)]

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup,
    choose_one_target_player_gain_or_lose_life_instant_card, close_empty_priority_window,
    create_service, etb_may_life_gain_creature_card, forest_card, loot_sorcery_card, player,
    player_deck, player_library, target_player_discards_chosen_card_sorcery_card,
    targeted_opponent_damage_instant_card,
};
use demonictutor::{
    choice_requests, CardDefinitionId, CardInstanceId, CastSpellCommand, DealOpeningHandsCommand,
    DrawCardsEffectCommand, Game, GameId, PassPriorityCommand, Phase, PlayerId, PublicBinaryChoice,
    PublicChoiceRequest, PublicGameCommand, PublicModalSpellChoice, StartGameCommand,
};

fn first_main_game_with_choice_cards() -> Game {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                targeted_opponent_damage_instant_card("p1-bolt-opponent", 0, 2),
                choose_one_target_player_gain_or_lose_life_instant_card(
                    "p1-choose-one-life",
                    0,
                    3,
                    2,
                ),
                target_player_discards_chosen_card_sorcery_card("p1-discard-choice", 0),
                forest_card("p1-forest-a"),
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
            GameId::new("game-choice-requests"),
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
fn choice_requests_surface_target_candidates_for_supported_targeted_spells() {
    let game = first_main_game_with_choice_cards();
    let bolt_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-bolt-opponent"))
        .expect("bolt should be in hand")
        .id()
        .clone();

    let requests = choice_requests(&game);

    assert!(requests.iter().any(|request| matches!(
        request,
        PublicChoiceRequest::SpellTarget { player_id, source_card_id, candidates }
            if player_id.as_str() == "p1"
                && *source_card_id == bolt_id
                && candidates.iter().any(|candidate| matches!(
                    candidate,
                    demonictutor::PublicChoiceCandidate::Player(target_player)
                        if target_player.as_str() == "p2"
                ))
    )));
}

#[test]
fn choice_requests_surface_modal_modes_for_supported_choose_one_spells() {
    let game = first_main_game_with_choice_cards();
    let modal_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-choose-one-life"))
        .expect("modal spell should be in hand")
        .id()
        .clone();

    let requests = choice_requests(&game);

    assert!(requests.iter().any(|request| matches!(
        request,
        PublicChoiceRequest::SpellModalChoice { player_id, source_card_id, modes }
            if player_id.as_str() == "p1"
                && *source_card_id == modal_id
                && modes == &vec![
                    PublicModalSpellChoice::TargetPlayerGainLife,
                    PublicModalSpellChoice::TargetPlayerLoseLife,
                ]
    )));
}

#[test]
fn choice_requests_surface_explicit_hand_choice_for_discard_spells() {
    let game = first_main_game_with_choice_cards();
    let discard_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-discard-choice"))
        .expect("discard spell should be in hand")
        .id()
        .clone();

    let requests = choice_requests(&game);

    assert!(requests.iter().any(|request| matches!(
        request,
        PublicChoiceRequest::SpellChoice { player_id, source_card_id, hand_card_ids }
            if player_id.as_str() == "p1"
                && *source_card_id == discard_id
                && !hand_card_ids.is_empty()
    )));
}

#[test]
fn choice_requests_surface_cleanup_discard_as_pending_choice() {
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
            GameId::new("game-cleanup-choice"),
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
    crate::support::advance_to_player_phase_satisfying_cleanup(
        &service,
        &mut game,
        "p1",
        Phase::EndStep,
    );
    close_empty_priority_window(&service, &mut game);

    let requests = choice_requests(&game);

    assert!(requests.iter().any(|request| matches!(
        request,
        PublicChoiceRequest::CleanupDiscard { player_id, hand_card_ids }
            if player_id.as_str() == "p1" && !hand_card_ids.is_empty()
    )));
}

#[test]
fn choice_requests_surface_pending_optional_effect_decisions() {
    let (service, mut game) = crate::support::setup_two_player_game(
        "game-optional-choice-request",
        crate::support::filled_library(
            vec![etb_may_life_gain_creature_card("kindly-cleric", 0, 1, 1, 2)],
            10,
        ),
        crate::support::filled_library(Vec::new(), 10),
    );
    crate::support::advance_to_first_main_satisfying_cleanup(&service, &mut game);
    let creature_id = player(&game, "player-1")
        .hand_card_by_definition(&CardDefinitionId::new("kindly-cleric"))
        .expect("creature should be in hand")
        .id()
        .clone();
    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(PlayerId::new("player-1"), creature_id),
        )
        .expect("creature should be castable");
    for _ in 0..2 {
        let first_holder = game
            .priority()
            .expect("priority should be open before first pass")
            .current_holder()
            .clone();
        service
            .pass_priority(&mut game, PassPriorityCommand::new(first_holder))
            .expect("first pass should succeed");
        let second_holder = game
            .priority()
            .expect("priority should pass to the other player")
            .current_holder()
            .clone();
        service
            .pass_priority(&mut game, PassPriorityCommand::new(second_holder))
            .expect("second pass should progress toward optional decision");
    }

    let requests = choice_requests(&game);

    assert!(requests.iter().any(|request| matches!(
        request,
        PublicChoiceRequest::OptionalEffectDecision { player_id, source_card_id, options }
            if player_id.as_str() == "player-1"
                && *source_card_id == CardInstanceId::new("game-optional-choice-request-player-1-0")
                && options == &vec![PublicBinaryChoice::Yes, PublicBinaryChoice::No]
    )));
}

#[test]
fn choice_requests_surface_pending_hand_choice_for_loot_resolution() {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                loot_sorcery_card("p1-loot", 0, 1),
                forest_card("p1-a"),
                forest_card("p1-b"),
                forest_card("p1-c"),
                forest_card("p1-d"),
                forest_card("p1-e"),
                forest_card("p1-f"),
                forest_card("p1-draw"),
                forest_card("p1-pad-a"),
                forest_card("p1-pad-b"),
            ],
        ),
        player_library(
            "p2",
            vec![
                forest_card("p2-a"),
                forest_card("p2-b"),
                forest_card("p2-c"),
                forest_card("p2-d"),
                forest_card("p2-e"),
                forest_card("p2-f"),
                forest_card("p2-g"),
                forest_card("p2-h"),
                forest_card("p2-i"),
                forest_card("p2-j"),
            ],
        ),
    ];
    let decks = vec![player_deck("p1", "d1"), player_deck("p2", "d2")];

    let (mut game, _) = service
        .start_game(StartGameCommand::new(
            GameId::new("game-pending-hand-choice"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "p1");

    let loot_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-loot"))
        .expect("loot spell should be in hand")
        .id()
        .clone();

    service.execute_public_command(
        &mut game,
        PublicGameCommand::CastSpell(CastSpellCommand::new(PlayerId::new("p1"), loot_id.clone())),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p1"))),
    );
    service.execute_public_command(
        &mut game,
        PublicGameCommand::PassPriority(PassPriorityCommand::new(PlayerId::new("p2"))),
    );

    let requests = choice_requests(&game);

    assert!(requests.iter().any(|request| matches!(
        request,
        PublicChoiceRequest::PendingHandChoice { player_id, source_card_id, hand_card_ids }
            if player_id.as_str() == "p1"
                && *source_card_id == loot_id
                && !hand_card_ids.is_empty()
    )));
}
