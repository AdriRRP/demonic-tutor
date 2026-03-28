//! BDD coverage for world setup golden matchups.

use {
    super::super::support,
    super::super::GameplayWorld,
    demonictutor::{CastSpellCommand, KeywordAbility, Phase},
};

impl GameplayWorld {
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
