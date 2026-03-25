//! Supports combat damage application.

use crate::domain::play::{
    game::{helpers, model::Player, AggregateCardLocationIndex},
};

use super::CreatureDamageAssignment;

pub(super) fn apply_damage(
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    damage_received: &[CreatureDamageAssignment],
) {
    for assignment in damage_received {
        if let Some(card) =
            helpers::battlefield_card_mut(players, card_locations, &assignment.target)
        {
            if assignment.source_has_deathtouch {
                card.add_deathtouch_damage(assignment.damage);
            } else {
                card.add_damage(assignment.damage);
            }
        }
    }
}

pub(super) fn clear_combat_state(players: &mut [Player]) {
    for player in players.iter_mut() {
        player.for_each_battlefield_card_mut(|card| {
            card.set_attacking(false);
            card.set_blocking(false);
        });
    }
}
