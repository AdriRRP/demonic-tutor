//! Unit coverage for application session-loop helpers.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::support::{
    create_service, filled_library, forest_card, instant_card, land_card, player,
    setup_two_player_game,
};
use demonictutor::{
    public_command_result, ActivatedAbilityProfile, CardDefinitionId, DomainError, GameEndReason,
    GameError, GameId, LibraryCard, PlayerId, PublicCommandStatus, PublicEvent, PublicGameCommand,
    PublicPlayableSubsetVersion, PublicRematchCommand, PublicSeededGameSetup,
    PublicSeededPlayerSetup, TriggeredAbilityProfile,
};

fn seeded_setup(game_id: &str, seed: u64) -> PublicSeededGameSetup {
    PublicSeededGameSetup::new(
        GameId::new(game_id),
        vec![
            PublicSeededPlayerSetup::new(
                PlayerId::new("player-1"),
                demonictutor::DeckId::new("deck-1"),
                vec![
                    forest_card("p1-forest-a"),
                    forest_card("p1-forest-b"),
                    instant_card("p1-spell-a", 0),
                    instant_card("p1-spell-b", 0),
                    instant_card("p1-spell-c", 0),
                    instant_card("p1-spell-d", 0),
                    instant_card("p1-spell-e", 0),
                    instant_card("p1-spell-f", 0),
                    instant_card("p1-spell-g", 0),
                    instant_card("p1-spell-h", 0),
                ],
            ),
            PublicSeededPlayerSetup::new(
                PlayerId::new("player-2"),
                demonictutor::DeckId::new("deck-2"),
                vec![
                    land_card("p2-land-a"),
                    land_card("p2-land-b"),
                    instant_card("p2-spell-a", 0),
                    instant_card("p2-spell-b", 0),
                    instant_card("p2-spell-c", 0),
                    instant_card("p2-spell-d", 0),
                    instant_card("p2-spell-e", 0),
                    instant_card("p2-spell-f", 0),
                    instant_card("p2-spell-g", 0),
                    instant_card("p2-spell-h", 0),
                ],
            ),
        ],
        seed,
    )
}

fn invalid_seeded_setup(game_id: &str, seed: u64) -> PublicSeededGameSetup {
    let unsupported_card =
        LibraryCard::creature(CardDefinitionId::new("illegal-creature"), 1, 2, 2)
            .with_activated_ability(ActivatedAbilityProfile::tap_to_gain_life_to_controller(1))
            .with_triggered_ability(TriggeredAbilityProfile::attacks_gain_life_to_controller(1));

    PublicSeededGameSetup::new(
        GameId::new(game_id),
        vec![
            PublicSeededPlayerSetup::new(
                PlayerId::new("player-1"),
                demonictutor::DeckId::new("deck-1"),
                vec![
                    unsupported_card,
                    forest_card("p1-forest-a"),
                    forest_card("p1-forest-b"),
                    instant_card("p1-spell-a", 0),
                    instant_card("p1-spell-b", 0),
                    instant_card("p1-spell-c", 0),
                    instant_card("p1-spell-d", 0),
                    instant_card("p1-spell-e", 0),
                    instant_card("p1-spell-f", 0),
                    instant_card("p1-spell-g", 0),
                ],
            ),
            PublicSeededPlayerSetup::new(
                PlayerId::new("player-2"),
                demonictutor::DeckId::new("deck-2"),
                vec![
                    land_card("p2-land-a"),
                    land_card("p2-land-b"),
                    instant_card("p2-spell-a", 0),
                    instant_card("p2-spell-b", 0),
                    instant_card("p2-spell-c", 0),
                    instant_card("p2-spell-d", 0),
                    instant_card("p2-spell-e", 0),
                    instant_card("p2-spell-f", 0),
                    instant_card("p2-spell-g", 0),
                    instant_card("p2-spell-h", 0),
                ],
            ),
        ],
        seed,
    )
}

fn hand_definitions(game: &demonictutor::Game, player_id: &str) -> Vec<CardDefinitionId> {
    player(game, player_id)
        .hand_card_ids()
        .into_iter()
        .map(|card_id| {
            player(game, player_id)
                .hand_card(&card_id)
                .expect("hand card should exist")
                .definition_id()
                .clone()
        })
        .collect()
}

#[test]
fn seeded_public_game_setup_is_deterministic_for_the_same_seed() {
    let service = create_service();
    let setup = seeded_setup("game-seeded-setup-a", 42);

    let (game_a, result_a) = service
        .start_seeded_public_game(setup.clone())
        .expect("seeded setup should succeed");
    let (game_b, result_b) = service
        .start_seeded_public_game(setup.with_game_id(GameId::new("game-seeded-setup-b")))
        .expect("same seeded setup should succeed again");

    assert!(matches!(
        result_a.emitted_events.first(),
        Some(PublicEvent::GameStarted(_))
    ));
    assert_eq!(
        result_a
            .emitted_events
            .iter()
            .filter(|event| matches!(event, PublicEvent::OpeningHandDealt(_)))
            .count(),
        2
    );
    assert_eq!(
        hand_definitions(&game_a, "player-1"),
        hand_definitions(&game_b, "player-1")
    );
    assert_eq!(
        hand_definitions(&game_a, "player-2"),
        hand_definitions(&game_b, "player-2")
    );
    assert!(result_a.legal_actions.iter().any(|action| matches!(
        action,
        demonictutor::PublicLegalAction::Concede { player_id }
            if player_id == &PlayerId::new("player-1")
    )));
    assert!(result_b.legal_actions.iter().any(|action| matches!(
        action,
        demonictutor::PublicLegalAction::Concede { player_id }
            if player_id == &PlayerId::new("player-2")
    )));
}

#[test]
fn seeded_public_rematch_reuses_setup_with_a_new_game_id() {
    let service = create_service();
    let setup = seeded_setup("game-seeded-rematch-original", 7);
    let (original_game, _) = service
        .start_seeded_public_game(setup.clone())
        .expect("seeded setup should succeed");

    let (rematch_game, rematch_result) = service
        .rematch_seeded_public_game(PublicRematchCommand::new(
            GameId::new("game-seeded-rematch-new"),
            setup,
        ))
        .expect("seeded rematch should succeed");

    assert_ne!(original_game.id(), rematch_game.id());
    assert_eq!(
        hand_definitions(&original_game, "player-1"),
        hand_definitions(&rematch_game, "player-1")
    );
    assert_eq!(
        hand_definitions(&original_game, "player-2"),
        hand_definitions(&rematch_game, "player-2")
    );
    assert_eq!(
        rematch_result.game.game_id,
        GameId::new("game-seeded-rematch-new")
    );
    assert_eq!(
        rematch_result.game.playable_subset_version,
        PublicPlayableSubsetVersion::V1
    );
}

#[test]
fn seeded_public_game_setup_does_not_persist_partial_start_when_opening_hands_fail() {
    let service = create_service();
    let setup = invalid_seeded_setup("game-seeded-invalid-setup", 9);

    let result = service.start_seeded_public_game(setup);

    assert!(matches!(
        result,
        Err(DomainError::Game(GameError::UnsupportedCuratedCardProfile {
            player,
            definition,
        })) if player == PlayerId::new("player-1")
            && definition == CardDefinitionId::new("illegal-creature")
    ));

    let log = service
        .public_event_log(&GameId::new("game-seeded-invalid-setup"))
        .expect("failed seeded setup should not break event-log reads");

    assert!(
        log.is_empty(),
        "failed seeded setup should not persist a partial lifecycle stream"
    );
}

#[test]
fn concede_public_command_ends_the_game_with_conceded_reason() {
    let (service, mut game) = setup_two_player_game(
        "game-public-concede",
        filled_library(vec![forest_card("p1-forest-a")], 10),
        filled_library(vec![forest_card("p2-forest-a")], 10),
    );

    let application = service.execute_public_command(
        &mut game,
        PublicGameCommand::Concede(demonictutor::ConcedeCommand::new(PlayerId::new("player-1"))),
    );
    let result = public_command_result(&game, application);

    assert!(matches!(result.status, PublicCommandStatus::Applied));
    assert!(matches!(
        result.emitted_events.as_slice(),
        [PublicEvent::GameEnded(ended)] if ended.reason == GameEndReason::Conceded
            && ended.loser_id == PlayerId::new("player-1")
            && ended.winner_id == PlayerId::new("player-2")
    ));
    assert!(result.game.is_over);
    assert_eq!(result.game.end_reason, Some(GameEndReason::Conceded));
    assert!(result.legal_actions.is_empty());
    assert!(result.choice_requests.is_empty());
}
