//! BDD coverage for world setup golden matchups.

use {
    super::super::support,
    super::super::GameplayWorld,
    demonictutor::{CastSpellCommand, KeywordAbility, Phase},
};

impl GameplayWorld {
    pub fn setup_black_red_sacrifice_outlet_in_first_main(&mut self) {
        self.reset_game_with_libraries(
            "bdd-golden-br-sac-outlet",
            support::filled_library(
                vec![support::sacrifice_life_gain_artifact_card(
                    "bdd-blood-shard",
                    0,
                    2,
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
        let artifact_id = self.hand_card_by_definition("Alice", "bdd-blood-shard");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), artifact_id.clone()),
            )
            .expect("sacrifice outlet cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        self.tracked_card_id = Some(artifact_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::FirstMain);
    }

    pub fn setup_black_red_discard_pressure_in_first_main(&mut self) {
        self.reset_game_with_libraries(
            "bdd-golden-br-discard",
            support::filled_library(
                vec![support::creature_card("bdd-ashen-raider", 0, 2, 2)],
                10,
            ),
            support::filled_library(
                vec![support::target_player_discards_chosen_card_sorcery_card(
                    "bdd-coerce-lite",
                    0,
                )],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );

        self.tracked_card_id = Some(self.hand_card_by_definition("Bob", "bdd-coerce-lite"));
        self.tracked_blocker_id = Some(
            self.player("Alice")
                .hand_card_at(0)
                .expect("alice should have a hand card to discard")
                .id()
                .clone(),
        );
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::FirstMain);
    }

    pub fn setup_black_red_removal_and_recursion(&mut self) {
        self.reset_game_with_libraries(
            "bdd-golden-br-removal-recursion",
            support::filled_library(
                vec![
                    support::creature_card("bdd-cinder-ghoul", 0, 2, 2),
                    support::return_target_creature_card_from_graveyard_to_hand_sorcery_card(
                        "bdd-raise-dead-lite",
                        0,
                    ),
                ],
                10,
            ),
            support::filled_library(
                vec![support::targeted_destroy_creature_instant_card(
                    "bdd-cull", 0,
                )],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let creature_id = self.hand_card_by_definition("Alice", "bdd-cinder-ghoul");
        let recursion_id = self.hand_card_by_definition("Alice", "bdd-raise-dead-lite");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), creature_id.clone()),
            )
            .expect("creature cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );

        self.tracked_card_id = Some(self.hand_card_by_definition("Bob", "bdd-cull"));
        self.tracked_response_card_id = Some(recursion_id);
        self.tracked_blocker_id = Some(creature_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::FirstMain);
    }

    pub fn setup_white_blue_tempo_bounce_after_attackers(&mut self) {
        self.reset_game_with_libraries(
            "bdd-golden-wu-tempo-bounce",
            support::filled_library(
                vec![
                    support::creature_card_with_keyword(
                        "bdd-attacker-priority",
                        0,
                        2,
                        2,
                        KeywordAbility::Flying,
                    ),
                    support::return_target_permanent_to_hand_instant_card("bdd-boomerang-lite", 0),
                ],
                10,
            ),
            support::filled_library(
                vec![support::creature_card_with_keyword(
                    "bdd-blocker-priority",
                    0,
                    2,
                    2,
                    KeywordAbility::Flying,
                )],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-priority");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), attacker_id.clone()),
            )
            .expect("attacker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );
        let blocker_id = self.hand_card_by_definition("Bob", "bdd-blocker-priority");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Bob"), blocker_id.clone()),
            )
            .expect("blocker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::advance_turn_raw(&service, self.game_mut());
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());
        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(
                    Self::player_id("Alice"),
                    vec![attacker_id.clone()],
                ),
            )
            .expect("declare attackers should succeed");

        self.tracked_attacker_id = Some(attacker_id);
        self.tracked_blocker_id = Some(blocker_id);
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-boomerang-lite"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::DeclareBlockers);
    }

    pub fn setup_white_blue_tempo_pump_after_blockers(&mut self) {
        self.reset_game_with_libraries(
            "bdd-golden-wu-tempo-pump",
            support::filled_library(
                vec![
                    support::creature_card_with_keyword(
                        "bdd-attacker-priority",
                        0,
                        2,
                        2,
                        KeywordAbility::Flying,
                    ),
                    support::targeted_pump_creature_instant_card("bdd-giant-growth-lite", 0, 2, 2),
                ],
                10,
            ),
            support::filled_library(
                vec![support::creature_card_with_keyword(
                    "bdd-blocker-priority",
                    0,
                    2,
                    2,
                    KeywordAbility::Flying,
                )],
                10,
            ),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-priority");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), attacker_id.clone()),
            )
            .expect("attacker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-2",
        );
        let blocker_id = self.hand_card_by_definition("Bob", "bdd-blocker-priority");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Bob"), blocker_id.clone()),
            )
            .expect("blocker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());

        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::advance_turn_raw(&service, self.game_mut());
        support::close_empty_priority_window(&service, self.game_mut());
        support::advance_turn_raw(&service, self.game_mut());
        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(
                    Self::player_id("Alice"),
                    vec![attacker_id.clone()],
                ),
            )
            .expect("declare attackers should succeed");
        support::close_empty_priority_window(&service, self.game_mut());
        service
            .declare_blockers(
                self.game_mut(),
                demonictutor::DeclareBlockersCommand::new(
                    Self::player_id("Bob"),
                    vec![(blocker_id.clone(), attacker_id.clone())],
                ),
            )
            .expect("declare blockers should succeed");

        self.tracked_attacker_id = Some(attacker_id);
        self.tracked_blocker_id = Some(blocker_id);
        self.blocker_assignments = vec![(
            self.tracked_blocker_id
                .clone()
                .expect("tracked blocker should exist"),
            self.tracked_attacker_id
                .clone()
                .expect("tracked attacker should exist"),
        )];
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-giant-growth-lite"));
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::CombatDamage);
    }
}
