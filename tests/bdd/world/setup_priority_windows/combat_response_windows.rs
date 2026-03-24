//! BDD coverage for world setup priority windows combat response windows.

use {super::super::support, super::super::GameplayWorld, demonictutor::Phase};

impl GameplayWorld {
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
}
