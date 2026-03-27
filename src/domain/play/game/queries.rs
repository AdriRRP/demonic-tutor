//! Supports read-only legality queries over the game aggregate.

use super::{helpers, invariants, rules, Game};
use crate::domain::play::ids::{CardInstanceId, PlayerId};

impl Game {
    #[must_use]
    pub fn can_play_land(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err()
            || invariants::require_no_priority_with_pending_stack(
                self.priority(),
                self.stack.is_empty(),
            )
            .is_err()
        {
            return false;
        }

        let Ok(player_index) = helpers::find_player_index(&self.players, player_id) else {
            return false;
        };

        rules::resource_actions::is_playable_land_candidate(
            &self.players,
            self.active_player_index,
            self.phase,
            player_index,
            player_id,
            card_id,
        )
    }

    #[must_use]
    pub fn can_tap_mana_source(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err() {
            return false;
        }

        if let Some(priority) = self.priority() {
            if invariants::require_priority_holder(Some(priority), player_id).is_err() {
                return false;
            }
        }

        rules::resource_actions::is_tappable_mana_source_candidate(
            &self.players,
            self.active_player_index,
            self.phase,
            self.priority(),
            player_id,
            card_id,
        )
    }

    #[must_use]
    pub fn castable_card(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err() {
            return false;
        }

        rules::stack_priority::is_castable_candidate(
            &self.players,
            &self.card_locations,
            self.active_player(),
            self.phase,
            &self.stack,
            self.priority(),
            player_id,
            card_id,
        )
    }

    #[must_use]
    pub fn activatable_card(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err() {
            return false;
        }

        rules::stack_priority::is_activatable_candidate(
            &self.players,
            &self.card_locations,
            self.active_player(),
            self.phase,
            &self.stack,
            self.priority(),
            player_id,
            card_id,
        )
    }

    #[must_use]
    pub fn can_attack_with(&self, player_id: &PlayerId, card_id: &CardInstanceId) -> bool {
        if invariants::require_game_active(self.is_over()).is_err()
            || invariants::require_no_open_priority_window(self.priority()).is_err()
        {
            return false;
        }

        let Ok(player_index) = helpers::find_player_index(&self.players, player_id) else {
            return false;
        };
        if player_index != self.active_player_index {
            return false;
        }

        rules::combat::can_attack_with_candidate(
            &self.players,
            self.active_player_index,
            self.phase,
            player_id,
            card_id,
        )
    }
}
