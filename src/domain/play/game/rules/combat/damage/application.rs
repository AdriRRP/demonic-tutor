use crate::domain::play::{
    game::{helpers, model::Player},
    ids::CardInstanceId,
};

pub(super) fn apply_damage_and_clear_combat_state(
    players: &mut [Player],
    damage_received: &[(CardInstanceId, u32)],
) {
    for player in players.iter_mut() {
        player.for_each_battlefield_card_mut(|card| {
            card.set_attacking(false);
            card.set_blocking(false);
        });
    }

    for (card_id, damage) in damage_received {
        if let Some(card) = helpers::battlefield_card_mut(players, card_id) {
            card.add_damage(*damage);
        }
    }
}
