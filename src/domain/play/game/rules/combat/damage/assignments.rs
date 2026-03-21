use crate::domain::play::{game::model::Player, ids::CardInstanceId};
use std::collections::HashMap;

pub(super) fn blocker_by_attacker(player: &Player) -> HashMap<CardInstanceId, CardInstanceId> {
    player
        .battlefield()
        .cards()
        .iter()
        .filter_map(|card| {
            card.blocking_target()
                .map(|attacker_id| (attacker_id.clone(), card.id().clone()))
        })
        .collect()
}
