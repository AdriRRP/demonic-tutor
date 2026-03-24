//! BDD coverage for world setup priority windows response windows.

use {super::super::support, super::super::GameplayWorld, demonictutor::Phase};

impl GameplayWorld {
    pub fn setup_non_active_priority_window_with_instant(&mut self, game_id: &str, phase: Phase) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(Vec::new(), 10),
            support::filled_library(vec![support::instant_card("bdd-window-instant", 0)], 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant"));
        self.pass_priority("Alice");
        self.reset_observations();
        self.assert_priority_window(phase, "Bob");
    }

    pub fn setup_non_active_priority_window_with_artifact(&mut self, game_id: &str, phase: Phase) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(Vec::new(), 10),
            support::filled_library(vec![support::artifact_card("bdd-window-artifact", 0)], 10),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-artifact"));
        self.pass_priority("Alice");
        self.reset_observations();
        self.assert_priority_window(phase, "Bob");
    }

    pub fn setup_non_active_priority_window_with_two_instants(
        &mut self,
        game_id: &str,
        phase: Phase,
    ) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(Vec::new(), 10),
            support::filled_library(
                vec![
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                10,
            ),
        );
        self.advance_to_turn_one_priority_window(phase);
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        self.pass_priority("Alice");
        self.reset_observations();
        self.assert_priority_window(phase, "Bob");
    }
}
