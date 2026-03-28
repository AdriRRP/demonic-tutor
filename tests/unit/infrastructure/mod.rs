#![allow(clippy::unwrap_used)]
#![allow(clippy::significant_drop_tightening)]

//! Unit coverage for unit infrastructure.

use demonictutor::{
    CardDiscarded, CardDrawn, CardInstanceId, CardType, CreatureDied, DiscardKind, DomainEvent,
    DrawKind, EventBus, EventStore, GameEndReason, GameEnded, GameId, GameLogProjection,
    GameStarted, InMemoryEventBus, InMemoryEventStore, LandPlayed, MulliganTaken, OpeningHandDealt,
    PlayerId, SpellCast, SpellCastOutcome, TurnProgressed,
};

#[test]
fn event_bus_publishes_to_handler() {
    let mut bus = InMemoryEventBus::new();

    let received = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let received_clone = std::sync::Arc::clone(&received);

    bus.subscribe(std::sync::Arc::new(move |event: &DomainEvent| {
        received_clone.lock().unwrap().push(event.clone());
    }));

    let event = DomainEvent::GameStarted(GameStarted::new(
        GameId::new("game-1"),
        vec![PlayerId::new("player-1"), PlayerId::new("player-2")],
    ));

    bus.publish(&event);

    let locked = received.lock().unwrap();
    assert_eq!(locked.len(), 1);
}

#[test]
fn event_bus_multiple_handlers() {
    let mut bus = InMemoryEventBus::new();

    let handler1_calls = std::sync::Arc::new(std::sync::Mutex::new(0));
    let handler2_calls = std::sync::Arc::new(std::sync::Mutex::new(0));

    let calls1 = std::sync::Arc::clone(&handler1_calls);
    let calls2 = std::sync::Arc::clone(&handler2_calls);

    bus.subscribe(std::sync::Arc::new(move |_: &DomainEvent| {
        *calls1.lock().unwrap() += 1;
    }));

    bus.subscribe(std::sync::Arc::new(move |_: &DomainEvent| {
        *calls2.lock().unwrap() += 1;
    }));

    let event = DomainEvent::GameStarted(GameStarted::new(
        GameId::new("game-1"),
        vec![PlayerId::new("player-1")],
    ));

    bus.publish(&event);

    assert_eq!(*handler1_calls.lock().unwrap(), 1);
    assert_eq!(*handler2_calls.lock().unwrap(), 1);
}

#[test]
fn event_store_append_and_retrieve() {
    let store = InMemoryEventStore::new();

    let event1 = DomainEvent::GameStarted(GameStarted::new(
        GameId::new("game-1"),
        vec![PlayerId::new("player-1"), PlayerId::new("player-2")],
    ));

    let event2 = DomainEvent::OpeningHandDealt(OpeningHandDealt::new(
        GameId::new("game-1"),
        PlayerId::new("player-1"),
        vec![],
    ));

    store
        .append("game-1", std::slice::from_ref(&event1))
        .unwrap();
    store
        .append("game-1", std::slice::from_ref(&event2))
        .unwrap();

    let stored_events = store.get_events("game-1").unwrap();

    assert_eq!(stored_events.len(), 2);
}

#[test]
fn event_store_empty_for_unknown_aggregate() {
    let store = InMemoryEventStore::new();

    let stored_events = store.get_events("nonexistent").unwrap();

    assert!(stored_events.is_empty());
}

#[test]
fn projection_logs_events() {
    let projection = GameLogProjection::new();

    let event = DomainEvent::GameStarted(GameStarted::new(
        GameId::new("game-1"),
        vec![PlayerId::new("player-1"), PlayerId::new("player-2")],
    ));

    projection.handle(&event);

    let logs = projection.logs().unwrap();
    assert_eq!(logs.len(), 1);
    assert!(logs[0].contains("game-1"));
    assert!(logs[0].contains("player-1"));
}

#[test]
fn projection_logs_reuse_cached_snapshot_across_reads() {
    let projection = GameLogProjection::new();

    projection.handle(&DomainEvent::GameStarted(GameStarted::new(
        GameId::new("game-1"),
        vec![PlayerId::new("player-1"), PlayerId::new("player-2")],
    )));

    let first = projection.logs().unwrap();
    let second = projection.logs().unwrap();

    assert!(std::sync::Arc::ptr_eq(&first, &second));
}

#[test]
fn projection_logs_multiple_events() {
    let projection = GameLogProjection::new();

    projection.handle(&DomainEvent::GameStarted(GameStarted::new(
        GameId::new("game-1"),
        vec![PlayerId::new("player-1"), PlayerId::new("player-2")],
    )));

    projection.handle(&DomainEvent::OpeningHandDealt(OpeningHandDealt::new(
        GameId::new("game-1"),
        PlayerId::new("player-1"),
        vec![],
    )));

    projection.handle(&DomainEvent::LandPlayed(LandPlayed::new(
        GameId::new("game-1"),
        PlayerId::new("player-1"),
        CardInstanceId::new("card-1"),
    )));

    projection.handle(&DomainEvent::TurnProgressed(TurnProgressed::new(
        GameId::new("game-1"),
        PlayerId::new("player-2"),
        1,
        2,
        demonictutor::Phase::EndStep,
        demonictutor::Phase::Untap,
    )));

    projection.handle(&DomainEvent::CardDrawn(CardDrawn::new(
        GameId::new("game-1"),
        PlayerId::new("player-2"),
        CardInstanceId::new("card-2"),
        DrawKind::TurnStep,
    )));

    projection.handle(&DomainEvent::MulliganTaken(MulliganTaken::new(
        GameId::new("game-1"),
        PlayerId::new("player-1"),
    )));

    projection.handle(&DomainEvent::SpellCast(SpellCast::new(
        GameId::new("game-1"),
        PlayerId::new("player-1"),
        CardInstanceId::new("card-3"),
        CardType::Creature,
        3,
        SpellCastOutcome::EnteredBattlefield,
    )));

    let logs = projection.logs().unwrap();
    assert_eq!(logs.len(), 7);
    assert!(logs[0].starts_with("Game"));
    assert!(logs[1].starts_with("Player"));
    assert!(logs[2].starts_with("Player"));
    assert!(logs[3].starts_with("Turn progressed"));
    assert!(logs[4].starts_with("Player"));
    assert!(logs[5].starts_with("Player"));
    assert!(logs[6].contains("Creature"));
    assert!(logs[6].contains("3 mana"));
}

#[test]
fn projection_logs_creature_died_events() {
    let projection = GameLogProjection::new();

    projection.handle(&DomainEvent::CreatureDied(CreatureDied::new(
        GameId::new("game-1"),
        PlayerId::new("player-2"),
        CardInstanceId::new("card-7"),
    )));

    let logs = projection.logs().unwrap();
    assert_eq!(logs.len(), 1);
    assert!(logs[0].contains("card-7"));
    assert!(logs[0].contains("player-2"));
    assert!(logs[0].contains("died"));
}

#[test]
fn projection_logs_card_discarded_events() {
    let projection = GameLogProjection::new();

    projection.handle(&DomainEvent::CardDiscarded(CardDiscarded::new(
        GameId::new("game-1"),
        PlayerId::new("player-1"),
        CardInstanceId::new("card-9"),
        DiscardKind::CleanupHandSize,
    )));

    let logs = projection.logs().unwrap();
    assert_eq!(logs.len(), 1);
    assert!(logs[0].contains("player-1"));
    assert!(logs[0].contains("card-9"));
    assert!(logs[0].contains("discarded"));
    assert!(logs[0].contains("CleanupHandSize"));
}

#[test]
fn projection_logs_game_ended_events() {
    let projection = GameLogProjection::new();

    projection.handle(&DomainEvent::GameEnded(GameEnded::new(
        GameId::new("game-1"),
        PlayerId::new("player-2"),
        PlayerId::new("player-1"),
        GameEndReason::EmptyLibraryDraw,
    )));

    let logs = projection.logs().unwrap();
    assert_eq!(logs.len(), 1);
    assert!(logs[0].contains("player-1"));
    assert!(logs[0].contains("player-2"));
    assert!(logs[0].contains("EmptyLibraryDraw"));
}

#[test]
fn projection_logs_drawn_game_end_events() {
    let projection = GameLogProjection::new();

    projection.handle(&DomainEvent::GameEnded(GameEnded::draw(
        GameId::new("game-1"),
        GameEndReason::SimultaneousZeroLife,
    )));

    let logs = projection.logs().unwrap();
    assert_eq!(logs.len(), 1);
    assert!(logs[0].contains("draw"));
    assert!(logs[0].contains("SimultaneousZeroLife"));
}

#[test]
fn integration_event_store_and_bus() {
    let bus = InMemoryEventBus::new();
    let store = InMemoryEventStore::new();

    let event = DomainEvent::GameStarted(GameStarted::new(
        GameId::new("game-1"),
        vec![PlayerId::new("player-1"), PlayerId::new("player-2")],
    ));

    store
        .append("game-1", std::slice::from_ref(&event))
        .unwrap();
    bus.publish(&event);

    let stored_events = store.get_events("game-1").unwrap();
    assert_eq!(stored_events.len(), 1);
}
