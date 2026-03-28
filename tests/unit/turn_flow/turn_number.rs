#![allow(clippy::unwrap_used)]

//! Unit coverage for unit turn flow turn number.

use {
    crate::support::{
        advance_to_player_phase_satisfying_cleanup, filled_library, land_card,
        setup_two_player_game,
    },
    demonictutor::{
        AdvanceTurnCommand, AdvanceTurnOutcome, DomainEvent, GameLogProjection, GameService,
        InMemoryEventBus, InMemoryEventStore, Phase,
    },
    std::sync::Arc,
};

#[test]
fn game_starts_with_turn_number_1() {
    let service = crate::support::create_service();
    let game = crate::support::start_two_player_game(&service, "game-1");

    assert_eq!(game.turn_number(), 1);
}

#[test]
fn advance_turn_increments_turn_number() {
    let (service, mut game) = setup_two_player_game(
        "game-1",
        filled_library(vec![land_card("forest")], 10),
        filled_library(vec![land_card("mountain")], 10),
    );

    assert_eq!(game.turn_number(), 1);

    advance_to_player_phase_satisfying_cleanup(&service, &mut game, "player-2", Phase::Untap);

    assert_eq!(game.turn_number(), 2);
}

#[test]
fn advance_turn_emits_turn_progressed_event() {
    let projection = Arc::new(GameLogProjection::new());
    let projection_clone = Arc::clone(&projection);

    let mut bus = InMemoryEventBus::new();
    bus.subscribe(Arc::new(move |event: &DomainEvent| {
        projection_clone.handle(event);
    }));

    let service = GameService::new(InMemoryEventStore::new(), bus);
    let mut game = crate::support::start_two_player_game(&service, "game-1");

    let outcome = service
        .advance_turn(&mut game, AdvanceTurnCommand::new())
        .unwrap();
    assert!(matches!(outcome, AdvanceTurnOutcome::Progressed { .. }));

    let logs = projection.logs().unwrap();
    let turn_log = logs.iter().find(|l| l.contains("Turn progressed"));
    assert!(turn_log.is_some());
}
