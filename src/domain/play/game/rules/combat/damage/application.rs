use crate::domain::play::{game::model::Player, ids::CardInstanceId};
use std::collections::HashMap;

pub(super) fn apply_damage_and_clear_combat_state(
    players: &mut [Player],
    damage_received: &HashMap<CardInstanceId, u32>,
) {
    for player in players.iter_mut() {
        for card in player.battlefield_mut().iter_mut() {
            if let Some(damage) = damage_received.get(card.id()) {
                card.add_damage(*damage);
            }
            card.set_attacking(false);
            card.set_blocking(false);
        }
    }
}
