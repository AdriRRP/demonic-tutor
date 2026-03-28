//! Unit coverage for the public replay event log query.

use std::{
    error::Error,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use demonictutor::{
    public_command_result, public_event_log, DomainEvent, EventStore, GameId, GameService,
    InMemoryEventBus, PlayLandCommand, PlayerId, PriorityPassed, PublicCommandStatus, PublicEvent,
    PublicGameCommand,
};

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, filled_library, first_hand_card_id, land_card,
    player, setup_two_player_game,
};

#[derive(Clone)]
struct CountingEventStore {
    events: Arc<[DomainEvent]>,
    reads: Arc<AtomicUsize>,
}

impl EventStore for CountingEventStore {
    fn append(
        &self,
        _aggregate_id: &str,
        _events: &[DomainEvent],
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    fn get_events(
        &self,
        _aggregate_id: &str,
    ) -> Result<Arc<[DomainEvent]>, Box<dyn Error + Send + Sync>> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        Ok(Arc::clone(&self.events))
    }
}

#[test]
fn public_event_log_assigns_stable_sequences_to_raw_events() {
    let log = public_event_log(vec![
        DomainEvent::PriorityPassed(PriorityPassed {
            game_id: GameId::new("game-public-event-log-seq"),
            player_id: PlayerId::new("player-1"),
        }),
        DomainEvent::PriorityPassed(PriorityPassed {
            game_id: GameId::new("game-public-event-log-seq"),
            player_id: PlayerId::new("player-2"),
        }),
    ]);

    assert_eq!(log.len(), 2);
    assert_eq!(log[0].sequence, 1);
    assert_eq!(log[1].sequence, 2);
}

#[test]
fn game_service_public_event_log_returns_persisted_public_timeline() {
    let (service, mut game) = setup_two_player_game(
        "game-public-event-log",
        filled_library(vec![land_card("public-log-plains")], 10),
        filled_library(Vec::new(), 10),
    );

    advance_to_player_first_main_satisfying_cleanup(&service, &mut game, "player-1");

    let land_id = first_hand_card_id(&game, "player-1");
    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::PlayLand(PlayLandCommand::new(
            PlayerId::new("player-1"),
            land_id.clone(),
        )),
    );
    let result = public_command_result(&game, application, &PlayerId::new("player-1"));
    assert!(matches!(result.status, PublicCommandStatus::Applied));

    let log_result = service.public_event_log(&GameId::new("game-public-event-log"));
    assert!(
        log_result.is_ok(),
        "public event log query should succeed: {log_result:?}"
    );
    let Ok(log) = log_result else {
        return;
    };

    assert!(
        log.len() >= 4,
        "game should have at least start, opening hands, and land play events"
    );
    assert_eq!(log[0].sequence, 1);
    assert!(!log.is_empty(), "public event log should not be empty");
    let Some(last_entry) = log.last() else {
        return;
    };
    assert_eq!(last_entry.sequence, log.len() as u64);
    assert!(matches!(&log[0].event, PublicEvent::GameStarted(_)));
    assert!(log
        .iter()
        .any(|entry| matches!(entry.event, PublicEvent::OpeningHandDealt(_))));
    assert!(matches!(
        &last_entry.event,
        PublicEvent::LandPlayed(played) if played.card_id == land_id
    ));
    assert!(player(&game, "player-1").battlefield_contains(&land_id));
}

#[test]
fn public_event_log_redacts_hidden_opening_hand_and_draw_card_ids() {
    let log = public_event_log(vec![
        DomainEvent::OpeningHandDealt(demonictutor::OpeningHandDealt::new(
            GameId::new("game-public-event-log-redaction"),
            PlayerId::new("player-1"),
            vec![
                demonictutor::CardInstanceId::new("hidden-a"),
                demonictutor::CardInstanceId::new("hidden-b"),
            ],
        )),
        DomainEvent::CardDrawn(demonictutor::CardDrawn::new(
            GameId::new("game-public-event-log-redaction"),
            PlayerId::new("player-1"),
            demonictutor::CardInstanceId::new("drawn-hidden"),
            demonictutor::DrawKind::TurnStep,
        )),
    ]);

    assert!(matches!(
        &log[0].event,
        PublicEvent::OpeningHandDealt(event)
            if event.card_count == 2 && event.player_id == PlayerId::new("player-1")
    ));
    assert!(matches!(
        &log[1].event,
        PublicEvent::CardDrawn(event)
            if event.player_id == PlayerId::new("player-1")
                && event.draw_kind == demonictutor::DrawKind::TurnStep
    ));
}

#[test]
fn game_service_public_event_log_reuses_cached_public_projection_across_reads() {
    let reads = Arc::new(AtomicUsize::new(0));
    let store = CountingEventStore {
        events: Arc::from(vec![DomainEvent::PriorityPassed(PriorityPassed {
            game_id: GameId::new("game-public-event-log-cache"),
            player_id: PlayerId::new("player-1"),
        })]),
        reads: Arc::clone(&reads),
    };
    let service = GameService::new(store, InMemoryEventBus::new());

    let first = service.public_event_log(&GameId::new("game-public-event-log-cache"));
    assert!(first.is_ok(), "first replay read should succeed");
    let first = first.unwrap_or_default();

    let second = service.public_event_log(&GameId::new("game-public-event-log-cache"));
    assert!(second.is_ok(), "second replay read should succeed");
    let second = second.unwrap_or_default();

    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 1);
    assert_eq!(first[0].sequence, second[0].sequence);
    assert_eq!(reads.load(Ordering::SeqCst), 1);
}

#[test]
fn game_service_public_event_log_evicts_oldest_cached_timelines() {
    let reads = Arc::new(AtomicUsize::new(0));
    let store = CountingEventStore {
        events: Arc::from(vec![DomainEvent::PriorityPassed(PriorityPassed {
            game_id: GameId::new("game-public-event-log-eviction"),
            player_id: PlayerId::new("player-1"),
        })]),
        reads: Arc::clone(&reads),
    };
    let service = GameService::new(store, InMemoryEventBus::new());

    for index in 0..64 {
        let result = service.public_event_log(&GameId::new(format!("cached-game-{index}")));
        assert!(result.is_ok(), "cache warmup should succeed: {result:?}");
    }
    assert_eq!(reads.load(Ordering::SeqCst), 64);

    let next = service.public_event_log(&GameId::new("cached-game-64"));
    assert!(next.is_ok(), "capacity-overflow read should succeed");
    assert_eq!(reads.load(Ordering::SeqCst), 65);

    let reread_evicted = service.public_event_log(&GameId::new("cached-game-0"));
    assert!(
        reread_evicted.is_ok(),
        "reading the oldest entry after eviction should succeed"
    );
    assert_eq!(reads.load(Ordering::SeqCst), 66);
}

#[test]
fn game_service_public_event_log_promotes_recently_read_timeline_before_eviction() {
    let reads = Arc::new(AtomicUsize::new(0));
    let store = CountingEventStore {
        events: Arc::from(vec![DomainEvent::PriorityPassed(PriorityPassed {
            game_id: GameId::new("game-public-event-log-promotion"),
            player_id: PlayerId::new("player-1"),
        })]),
        reads: Arc::clone(&reads),
    };
    let service = GameService::new(store, InMemoryEventBus::new());

    for index in 0..64 {
        let result = service.public_event_log(&GameId::new(format!("promoted-game-{index}")));
        assert!(result.is_ok(), "cache warmup should succeed: {result:?}");
    }
    assert_eq!(reads.load(Ordering::SeqCst), 64);

    let reread_oldest = service.public_event_log(&GameId::new("promoted-game-0"));
    assert!(reread_oldest.is_ok(), "rereading oldest cached timeline should succeed");
    assert_eq!(reads.load(Ordering::SeqCst), 64);

    let overflow = service.public_event_log(&GameId::new("promoted-game-64"));
    assert!(overflow.is_ok(), "capacity-overflow read should succeed");
    assert_eq!(reads.load(Ordering::SeqCst), 65);

    let reread_promoted = service.public_event_log(&GameId::new("promoted-game-0"));
    assert!(
        reread_promoted.is_ok(),
        "recently reread timeline should stay cached after overflow"
    );
    assert_eq!(reads.load(Ordering::SeqCst), 65);

    let reread_next_oldest = service.public_event_log(&GameId::new("promoted-game-1"));
    assert!(
        reread_next_oldest.is_ok(),
        "the true oldest untouched timeline should be evicted first"
    );
    assert_eq!(reads.load(Ordering::SeqCst), 66);
}

#[test]
fn game_service_public_event_log_evicts_large_cached_timelines_by_footprint() {
    let reads = Arc::new(AtomicUsize::new(0));
    let store = CountingEventStore {
        events: Arc::from(
            (0..5_000)
                .map(|index| {
                    DomainEvent::PriorityPassed(PriorityPassed {
                        game_id: GameId::new(format!("game-public-event-log-bytes-{index}")),
                        player_id: PlayerId::new("player-1"),
                    })
                })
                .collect::<Vec<_>>(),
        ),
        reads: Arc::clone(&reads),
    };
    let service = GameService::new(store, InMemoryEventBus::new());

    let first = service.public_event_log(&GameId::new("cached-large-game-0"));
    assert!(first.is_ok(), "first large replay read should succeed");
    assert_eq!(reads.load(Ordering::SeqCst), 1);

    let second = service.public_event_log(&GameId::new("cached-large-game-1"));
    assert!(second.is_ok(), "second large replay read should succeed");
    assert_eq!(reads.load(Ordering::SeqCst), 2);

    let reread_first = service.public_event_log(&GameId::new("cached-large-game-0"));
    assert!(
        reread_first.is_ok(),
        "rereading an oversized cached timeline should succeed after eviction"
    );
    assert_eq!(reads.load(Ordering::SeqCst), 3);
}
