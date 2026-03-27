//! Covers bounded scry resolution.

#![allow(clippy::expect_used)]

use crate::support::{
    advance_to_first_main_satisfying_cleanup, create_service, forest_card, player, player_deck,
    player_library, scry_sorcery_card,
};
use demonictutor::{
    CardDefinitionId, CastSpellCommand, DealOpeningHandsCommand, DrawCardsEffectCommand, GameId,
    PassPriorityCommand, PlayerId, ResolvePendingScryCommand, StartGameCommand,
};

fn setup_game() -> (crate::support::TestService, demonictutor::Game) {
    let service = create_service();
    let libraries = vec![
        player_library(
            "p1",
            vec![
                scry_sorcery_card("p1-scry", 0, 1),
                forest_card("p1-hand-a"),
                forest_card("p1-hand-b"),
                forest_card("p1-hand-c"),
                forest_card("p1-hand-d"),
                forest_card("p1-hand-e"),
                forest_card("p1-hand-f"),
                forest_card("p1-top-card"),
                forest_card("p1-next-card"),
                forest_card("p1-pad"),
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
        .start_game(StartGameCommand::new(GameId::new("game-scry"), decks))
        .expect("game should start");
    service
        .deal_opening_hands(&mut game, &DealOpeningHandsCommand::new(libraries))
        .expect("opening hands should be dealt");
    advance_to_first_main_satisfying_cleanup(&service, &mut game);

    (service, game)
}

fn cast_and_pass_to_pending_scry(
    service: &crate::support::TestService,
    game: &mut demonictutor::Game,
) {
    let scry_id = player(game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-scry"))
        .expect("scry should be in hand")
        .id()
        .clone();
    service
        .cast_spell(game, CastSpellCommand::new(PlayerId::new("p1"), scry_id))
        .expect("scry should cast");
    service
        .pass_priority(game, PassPriorityCommand::new(PlayerId::new("p1")))
        .expect("active player should pass");
    service
        .pass_priority(game, PassPriorityCommand::new(PlayerId::new("p2")))
        .expect("opponent should pass");
}

#[test]
fn scry_spell_opens_pending_scry_request() {
    let (service, mut game) = setup_game();

    cast_and_pass_to_pending_scry(&service, &mut game);

    assert!(matches!(
        game.pending_decision(),
        Some(demonictutor::PendingDecision::Scry { .. })
    ));
    assert!(game.priority().is_none());
    assert_eq!(player(&game, "p1").library_size(), 2);
    assert_eq!(
        player(&game, "p1")
            .top_library_card_id()
            .and_then(|card_id| player(&game, "p1").library_card(&card_id))
            .map(|card| card.definition_id().clone()),
        Some(CardDefinitionId::new("p1-next-card"))
    );
}

#[test]
fn scry_keep_on_top_preserves_next_draw() {
    let (service, mut game) = setup_game();

    cast_and_pass_to_pending_scry(&service, &mut game);
    service
        .resolve_pending_scry(
            &mut game,
            ResolvePendingScryCommand::keep_on_top(PlayerId::new("p1")),
        )
        .expect("keep-on-top should resolve");
    service
        .draw_cards_effect(
            &mut game,
            &DrawCardsEffectCommand::new(PlayerId::new("p1"), PlayerId::new("p1"), 1),
        )
        .expect("draw effect should resolve");

    assert!(player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-top-card"))
        .is_some());
}

#[test]
fn scry_move_to_bottom_changes_next_draw() {
    let (service, mut game) = setup_game();

    cast_and_pass_to_pending_scry(&service, &mut game);
    service
        .resolve_pending_scry(
            &mut game,
            ResolvePendingScryCommand::move_to_bottom(PlayerId::new("p1")),
        )
        .expect("move-to-bottom should resolve");
    service
        .draw_cards_effect(
            &mut game,
            &DrawCardsEffectCommand::new(PlayerId::new("p1"), PlayerId::new("p1"), 1),
        )
        .expect("draw effect should resolve");

    assert!(player(&game, "p1")
        .hand_card_by_definition(&CardDefinitionId::new("p1-pad"))
        .is_some());
}
