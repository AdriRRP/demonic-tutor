//! Unit coverage for the public replay event log query.

use demonictutor::{
    public_command_result, public_event_log, DomainEvent, GameId, PlayLandCommand, PlayerId,
    PriorityPassed, PublicCommandStatus, PublicEvent, PublicGameCommand,
};

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, filled_library, first_hand_card_id, land_card,
    player, setup_two_player_game,
};

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
    let result = public_command_result(&game, application);
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
