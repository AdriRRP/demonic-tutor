//! BDD coverage for world setup combat windows beginning of combat.

use {super::super::support, super::super::GameplayWorld, demonictutor::Phase};

impl GameplayWorld {
    pub fn setup_priority_when_entering_combat(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-priority",
            support::filled_library(Vec::new(), 10),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
    }

    pub fn setup_priority_when_entering_combat_with_instant(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-instant",
            support::filled_library(vec![support::instant_card("bdd-window-instant", 0)], 10),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
    }

    pub fn setup_priority_when_entering_combat_with_flash_artifact(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-flash-artifact",
            support::filled_library(
                vec![support::flash_artifact_card("bdd-window-flash-artifact", 0)],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-flash-artifact"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
    }

    pub fn setup_priority_when_entering_combat_with_flash_enchantment(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-flash-enchantment",
            support::filled_library(
                vec![support::flash_enchantment_card(
                    "bdd-window-flash-enchantment",
                    0,
                )],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-flash-enchantment"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
    }

    pub fn setup_priority_when_entering_combat_with_flash_planeswalker(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-flash-planeswalker",
            support::filled_library(
                vec![support::flash_planeswalker_card(
                    "bdd-window-flash-planeswalker",
                    0,
                )],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-flash-planeswalker"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
    }

    pub fn setup_priority_when_entering_combat_with_own_turn_artifact(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-own-turn-artifact",
            support::filled_library(
                vec![support::own_turn_priority_artifact_card(
                    "bdd-window-own-turn-artifact",
                    0,
                )],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-own-turn-artifact"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
    }

    pub fn setup_priority_when_entering_combat_with_own_turn_enchantment(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-own-turn-enchantment",
            support::filled_library(
                vec![support::own_turn_priority_enchantment_card(
                    "bdd-window-own-turn-enchantment",
                    0,
                )],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.tracked_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-own-turn-enchantment"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
    }

    pub fn setup_priority_when_entering_combat_with_two_instants(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-two-instants",
            support::filled_library(
                vec![
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                10,
            ),
            support::filled_library(Vec::new(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant-a"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-instant-b"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_non_active_priority_when_entering_combat_with_instant(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-response",
            support::filled_library(Vec::new(), 10),
            support::filled_library(vec![support::instant_card("bdd-window-instant", 0)], 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant"));
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.pass_priority("Alice");
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Bob")
        );
    }

    pub fn setup_non_active_priority_when_entering_combat_with_two_instants(&mut self) {
        self.reset_game_with_libraries(
            "bdd-beginning-combat-response-two-instants",
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
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());

        self.pass_priority("Alice");
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::BeginningOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat should open priority")
                .current_holder(),
            &Self::player_id("Bob")
        );
    }
}
