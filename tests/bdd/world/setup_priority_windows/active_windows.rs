use super::super::support;
use super::super::GameplayWorld;
use demonictutor::Phase;

impl GameplayWorld {
    pub fn setup_active_priority_window_with_instant(&mut self, game_id: &str, phase: Phase) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(vec![support::instant_card("bdd-window-instant", 0)], 10),
            support::filled_library(Vec::new(), 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant"));
        self.reset_observations();
        self.assert_priority_window(phase, "Alice");
    }

    pub fn setup_active_priority_window_with_sorcery(&mut self, game_id: &str, phase: Phase) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(vec![support::sorcery_card("bdd-window-sorcery", 0)], 10),
            support::filled_library(Vec::new(), 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-sorcery"));
        self.reset_observations();
        self.assert_priority_window(phase, "Alice");
    }

    pub fn setup_active_priority_window_with_creature(&mut self, game_id: &str, phase: Phase) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(
                vec![support::creature_card("bdd-window-creature", 0, 2, 2)],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-creature"));
        self.reset_observations();
        self.assert_priority_window(phase, "Alice");
    }

    pub fn setup_active_priority_window_with_artifact(&mut self, game_id: &str, phase: Phase) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(vec![support::artifact_card("bdd-window-artifact", 0)], 10),
            support::filled_library(Vec::new(), 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-artifact"));
        self.reset_observations();
        self.assert_priority_window(phase, "Alice");
    }

    pub fn setup_active_priority_window_with_own_turn_artifact(
        &mut self,
        game_id: &str,
        phase: Phase,
    ) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(
                vec![support::own_turn_priority_artifact_card(
                    "bdd-window-own-turn-artifact",
                    0,
                )],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-own-turn-artifact"));
        self.reset_observations();
        self.assert_priority_window(phase, "Alice");
    }

    pub fn setup_active_priority_window_with_enchantment(&mut self, game_id: &str, phase: Phase) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(
                vec![support::enchantment_card("bdd-window-enchantment", 0)],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-enchantment"));
        self.reset_observations();
        self.assert_priority_window(phase, "Alice");
    }

    pub fn setup_active_priority_window_with_planeswalker(&mut self, game_id: &str, phase: Phase) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(
                vec![support::planeswalker_card("bdd-window-planeswalker", 0)],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-planeswalker"));
        self.reset_observations();
        self.assert_priority_window(phase, "Alice");
    }

    pub fn setup_active_priority_window_with_two_instants(&mut self, game_id: &str, phase: Phase) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(
                vec![
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant-a"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-instant-b"));
        self.reset_observations();
        self.assert_priority_window(phase, "Alice");
    }
}
