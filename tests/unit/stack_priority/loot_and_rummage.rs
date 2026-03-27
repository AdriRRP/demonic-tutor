//! Covers loot and rummage spell resolution.

#![allow(clippy::expect_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, create_service, forest_card, loot_sorcery_card,
    player, player_deck, player_library, rummage_sorcery_card,
};
use demonictutor::{
    CardDefinitionId, CardInstanceId, CastSpellCommand, DealOpeningHandsCommand, DomainError,
    GameError, GameId, PassPriorityCommand, ResolvePendingHandChoiceCommand, StartGameCommand,
};

fn setup_game() -> (crate::support::TestService, demonictutor::Game) {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                loot_sorcery_card("p1-loot", 0, 1),
                rummage_sorcery_card("p1-rummage", 0, 1),
                forest_card("p1-hand-a"),
                forest_card("p1-hand-b"),
                forest_card("p1-hand-c"),
                forest_card("p1-hand-d"),
                forest_card("p1-hand-e"),
                forest_card("p1-draw-a"),
                forest_card("p1-draw-b"),
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
            GameId::new("game-loot-rummage"),
            decks,
        ))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    (service, game)
}

fn pass_both_players(service: &crate::support::TestService, game: &mut demonictutor::Game) {
    service
        .pass_priority(
            game,
            PassPriorityCommand::new(demonictutor::PlayerId::new("p1")),
        )
        .expect("active player should pass");
    service
        .pass_priority(
            game,
            PassPriorityCommand::new(demonictutor::PlayerId::new("p2")),
        )
        .expect("opponent should pass");
}

#[test]
fn loot_spell_draws_then_prompts_for_discard() {
    let (service, mut game) = setup_game();
    let loot_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-loot"))
        .expect("loot spell should be in hand")
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(demonictutor::PlayerId::new("p1"), loot_id),
        )
        .expect("loot should cast");
    pass_both_players(&service, &mut game);

    assert!(game.priority().is_none());
    assert!(matches!(
        game.pending_decision(),
        Some(demonictutor::PendingDecision::HandChoice { .. })
    ));
    assert!(player(&game, "p1").hand_size() >= 7);
    assert!(player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-draw-a"))
        .is_some());
}

#[test]
fn loot_spell_discards_the_selected_card_after_the_draw() {
    let (service, mut game) = setup_game();
    let loot_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-loot"))
        .expect("loot spell should be in hand")
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(demonictutor::PlayerId::new("p1"), loot_id.clone()),
        )
        .expect("loot should cast");
    pass_both_players(&service, &mut game);

    let discard_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-draw-a"))
        .expect("drawn card should be in hand")
        .id()
        .clone();
    service
        .resolve_pending_hand_choice(
            &mut game,
            ResolvePendingHandChoiceCommand::new(
                demonictutor::PlayerId::new("p1"),
                discard_id.clone(),
            ),
        )
        .expect("pending loot choice should resolve");

    assert!(game.pending_decision().is_none());
    assert!(game.stack().is_empty());
    assert!(game.priority().is_some());
    assert_eq!(
        player(&game, "p1").card_zone(&discard_id),
        Some(demonictutor::domain::play::game::PlayerCardZone::Graveyard)
    );
    assert_eq!(
        player(&game, "p1").card_zone(&loot_id),
        Some(demonictutor::domain::play::game::PlayerCardZone::Graveyard)
    );
}

#[test]
fn rummage_spell_discards_selected_card_before_drawing() {
    let (service, mut game) = setup_game();
    let rummage_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-rummage"))
        .expect("rummage spell should be in hand")
        .id()
        .clone();
    let discard_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-hand-a"))
        .expect("known hand card should be in hand")
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(demonictutor::PlayerId::new("p1"), rummage_id),
        )
        .expect("rummage should cast");
    pass_both_players(&service, &mut game);
    service
        .resolve_pending_hand_choice(
            &mut game,
            ResolvePendingHandChoiceCommand::new(
                demonictutor::PlayerId::new("p1"),
                discard_id.clone(),
            ),
        )
        .expect("pending rummage choice should resolve");

    assert_eq!(
        player(&game, "p1").card_zone(&discard_id),
        Some(demonictutor::domain::play::game::PlayerCardZone::Graveyard)
    );
    assert!(player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-draw-a"))
        .is_some());
}

#[test]
fn only_controller_can_answer_pending_hand_choice() {
    let (service, mut game) = setup_game();
    let rummage_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-rummage"))
        .expect("rummage spell should be in hand")
        .id()
        .clone();
    let chosen_card_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-hand-a"))
        .expect("known hand card should be in hand")
        .id()
        .clone();

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(demonictutor::PlayerId::new("p1"), rummage_id),
        )
        .expect("rummage should cast");
    pass_both_players(&service, &mut game);

    let result = service.resolve_pending_hand_choice(
        &mut game,
        ResolvePendingHandChoiceCommand::new(demonictutor::PlayerId::new("p2"), chosen_card_id),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(
            GameError::NotPendingHandChoiceController { .. }
        ))
    ));
}

#[test]
fn pending_hand_choice_rejects_card_not_in_hand() {
    let (service, mut game) = setup_game();
    let rummage_id = player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-rummage"))
        .expect("rummage spell should be in hand")
        .id()
        .clone();
    let invalid_card_id = CardInstanceId::new("missing-card");

    service
        .cast_spell(
            &mut game,
            CastSpellCommand::new(demonictutor::PlayerId::new("p1"), rummage_id),
        )
        .expect("rummage should cast");
    pass_both_players(&service, &mut game);

    let result = service.resolve_pending_hand_choice(
        &mut game,
        ResolvePendingHandChoiceCommand::new(demonictutor::PlayerId::new("p1"), invalid_card_id),
    );

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::InvalidHandCardChoice(_)))
    ));
}
