use super::support;
use super::GameplayWorld;
use demonictutor::Phase;

impl GameplayWorld {
    fn advance_to_turn_one_priority_window(&mut self, phase: Phase) {
        let service = support::create_service();
        for _ in 0..64 {
            while self.game().phase() == &Phase::EndStep && phase != Phase::EndStep {
                self.satisfy_cleanup_for_setup();
                if self.player_hand_size("Alice") <= 7 {
                    break;
                }
            }

            if self.game().phase() == &phase
                && self.game().active_player() == &Self::player_id("Alice")
                && self.game().turn_number() == 1
            {
                break;
            }

            support::advance_turn_raw(&service, self.game_mut());
        }
    }

    fn assert_priority_window(&self, phase: Phase, holder_alias: &str) {
        assert_eq!(self.game().phase(), &phase);
        assert_eq!(
            self.game()
                .priority()
                .expect("target phase should have an open priority window")
                .current_holder(),
            &Self::player_id(holder_alias)
        );
        assert!(self.game().stack().is_empty());
    }

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

    pub fn setup_non_active_priority_in_declare_blockers_with_two_instants(&mut self) {
        self.reset_game_with_libraries(
            "bdd-declare-blockers-response-two-instants",
            support::filled_library(Vec::new(), 10),
            support::filled_library(
                vec![
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                10,
            ),
        );

        let service = support::create_service();
        for _ in 0..32 {
            if self.game().phase() == &Phase::DeclareAttackers
                && self.game().active_player() == &Self::player_id("Alice")
            {
                break;
            }
            support::advance_turn_raw(&service, self.game_mut());
        }

        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(Self::player_id("Alice"), Vec::new()),
            )
            .expect("empty attacker declaration should succeed");

        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        self.pass_priority("Alice");
        self.reset_observations();
        self.assert_priority_window(Phase::DeclareBlockers, "Bob");
    }

    pub fn setup_non_active_priority_in_combat_damage_with_two_instants(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-damage-response-two-instants",
            support::filled_library(Vec::new(), 10),
            support::filled_library(
                vec![
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                10,
            ),
        );

        let service = support::create_service();
        for _ in 0..32 {
            if self.game().phase() == &Phase::DeclareBlockers
                && self.game().active_player() == &Self::player_id("Alice")
            {
                break;
            }
            support::advance_turn_raw(&service, self.game_mut());
        }

        service
            .declare_blockers(
                self.game_mut(),
                demonictutor::DeclareBlockersCommand::new(Self::player_id("Bob"), Vec::new()),
            )
            .expect("empty blocker declaration should succeed");

        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        self.pass_priority("Alice");
        self.reset_observations();
        self.assert_priority_window(Phase::CombatDamage, "Bob");
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
