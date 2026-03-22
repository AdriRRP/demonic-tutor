use crate::domain::play::{game::model::Player, ids::CardInstanceId};

pub(super) fn apply_damage_and_clear_combat_state(
    players: &mut [Player],
    damage_received: &[(CardInstanceId, u32)],
) {
    for player in players.iter_mut() {
        for card in player.battlefield_mut().iter_mut() {
            if let Some((_, damage)) = damage_received
                .iter()
                .find(|(card_id, _)| card_id == card.id())
            {
                card.add_damage(*damage);
            }
            card.set_attacking(false);
            card.set_blocking(false);
        }
    }
}
