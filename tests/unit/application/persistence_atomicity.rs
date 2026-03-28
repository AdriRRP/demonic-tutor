//! Unit coverage for application service atomicity on persistence failure.

#![allow(clippy::expect_used)]

use std::{error::Error, io, sync::Arc};

use demonictutor::{EventStore, GameService, InMemoryEventBus, PlayLandCommand, PlayerId};

use crate::support::{
    advance_to_player_first_main_satisfying_cleanup, filled_library, first_hand_card_id,
    forest_card, player, setup_two_player_game,
};

struct FailingEventStore;

impl EventStore for FailingEventStore {
    fn append(
        &self,
        _aggregate_id: &str,
        _events: &[demonictutor::DomainEvent],
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Err(Box::new(io::Error::other("simulated append failure")))
    }

    fn get_events(
        &self,
        _aggregate_id: &str,
    ) -> Result<Arc<[demonictutor::DomainEvent]>, Box<dyn Error + Send + Sync>> {
        Ok(Arc::from(Vec::<demonictutor::DomainEvent>::new()))
    }
}

#[test]
fn play_land_does_not_mutate_the_in_memory_game_when_persistence_fails() {
    let (setup_service, mut game) = setup_two_player_game(
        "game-persistence-atomicity",
        filled_library(vec![forest_card("p1-forest-a")], 10),
        filled_library(vec![forest_card("p2-forest-a")], 10),
    );
    advance_to_player_first_main_satisfying_cleanup(&setup_service, &mut game, "player-1");

    let land_id = first_hand_card_id(&game, "player-1");
    let failing_service = GameService::new(FailingEventStore, InMemoryEventBus::new());

    let result = failing_service.play_land(
        &mut game,
        PlayLandCommand::new(PlayerId::new("player-1"), land_id.clone()),
    );

    assert!(
        result.is_err(),
        "persistence failure should reject the command"
    );
    assert!(
        player(&game, "player-1").hand_contains(&land_id),
        "the land should remain in hand when the append fails"
    );
    assert!(
        !player(&game, "player-1").battlefield_contains(&land_id),
        "the battlefield should remain unchanged when the append fails"
    );
}
