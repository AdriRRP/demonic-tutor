use super::super::support;
use super::super::GameplayWorld;
use demonictutor::{
    AdjustPlayerLifeEffectCommand, CardDefinitionId, CastSpellCommand, DiscardForCleanupCommand,
    LibraryCard, Phase,
};

impl GameplayWorld {
    pub(super) fn setup_combat(
        &mut self,
        game_id: &str,
        attacker_definition: &str,
        attacker_card: LibraryCard,
        blocker_definition: Option<&str>,
        blocker_card: Option<LibraryCard>,
    ) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(vec![attacker_card], 10),
            support::filled_library(blocker_card.into_iter().collect(), 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", attacker_definition);
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

        if let Some(blocker_definition) = blocker_definition {
            let blocker_id = self.hand_card_by_definition("Bob", blocker_definition);
            service
                .cast_spell(
                    self.game_mut(),
                    CastSpellCommand::new(Self::player_id("Bob"), blocker_id.clone()),
                )
                .expect("blocker cast should succeed");
            support::resolve_top_stack_with_passes(&service, self.game_mut());
            self.tracked_blocker_id = Some(blocker_id);
        }

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

        self.tracked_attacker_id = Some(attacker_id.clone());

        if let Some(blocker_id) = self.tracked_blocker_id.clone() {
            self.blocker_assignments = vec![(blocker_id, attacker_id)];
            let assignments = self.blocker_assignments.clone();
            service
                .declare_blockers(
                    self.game_mut(),
                    demonictutor::DeclareBlockersCommand::new(Self::player_id("Bob"), assignments),
                )
                .expect("declare blockers should succeed");
            support::close_empty_priority_window(&service, self.game_mut());
        } else {
            self.blocker_assignments.clear();
        }

        self.reset_observations();
    }

    pub fn setup_blocked_damage_marking(&mut self) {
        self.setup_combat(
            "bdd-blocked-combat",
            "bdd-attacker-marking",
            LibraryCard::creature(CardDefinitionId::new("bdd-attacker-marking"), 0, 2, 4),
            Some("bdd-blocker-marking"),
            Some(LibraryCard::creature(
                CardDefinitionId::new("bdd-blocker-marking"),
                0,
                3,
                4,
            )),
        );
    }

    pub fn setup_multiple_blockers_not_supported(&mut self) {
        self.reset_game_with_libraries(
            "bdd-single-blocker",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-single-blocker"),
                    0,
                    3,
                    3,
                )],
                10,
            ),
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-blocker-left"), 0, 2, 2),
                    LibraryCard::creature(CardDefinitionId::new("bdd-blocker-right"), 0, 2, 2),
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
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-single-blocker");
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
        let left_blocker_id = self.hand_card_by_definition("Bob", "bdd-blocker-left");
        let right_blocker_id = self.hand_card_by_definition("Bob", "bdd-blocker-right");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Bob"), left_blocker_id.clone()),
            )
            .expect("left blocker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Bob"), right_blocker_id.clone()),
            )
            .expect("right blocker cast should succeed");
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
        self.tracked_attacker_id = Some(attacker_id.clone());
        self.tracked_blocker_id = Some(left_blocker_id.clone());
        self.blocker_assignments = vec![
            (left_blocker_id, attacker_id.clone()),
            (right_blocker_id, attacker_id),
        ];
        self.reset_observations();
    }

    pub fn setup_unblocked_combat(&mut self) {
        self.setup_combat(
            "bdd-unblocked-combat",
            "bdd-attacker-unblocked",
            LibraryCard::creature(CardDefinitionId::new("bdd-attacker-unblocked"), 0, 3, 3),
            None,
            None,
        );

        let service = support::create_service();
        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::CombatDamage,
        );
        support::close_empty_priority_window(&service, self.game_mut());
        self.reset_observations();
    }

    pub fn setup_unblocked_combat_with_defender_life(&mut self, life: u32) {
        self.setup_unblocked_combat();
        let current_life = self.player_life("Bob");
        let delta = life.cast_signed() - current_life.cast_signed();
        let service = support::create_service();
        let outcome = service
            .adjust_player_life_effect(
                self.game_mut(),
                AdjustPlayerLifeEffectCommand::new(
                    Self::player_id("Alice"),
                    Self::player_id("Bob"),
                    delta,
                ),
            )
            .expect("BDD combat life setup should succeed");
        assert!(outcome.game_ended.is_none());
        self.reset_observations();
    }

    pub fn setup_lethal_damage_combat(&mut self) {
        self.setup_combat(
            "bdd-lethal-combat",
            "bdd-doomed-attacker",
            LibraryCard::creature(CardDefinitionId::new("bdd-doomed-attacker"), 0, 2, 2),
            Some("bdd-lethal-blocker"),
            Some(LibraryCard::creature(
                CardDefinitionId::new("bdd-lethal-blocker"),
                0,
                3,
                3,
            )),
        );
        self.tracked_card_id = self.tracked_attacker_id.clone();
    }

    pub fn setup_nonlethal_damage_combat(&mut self) {
        self.setup_combat(
            "bdd-nonlethal-combat",
            "bdd-sturdy-attacker",
            LibraryCard::creature(CardDefinitionId::new("bdd-sturdy-attacker"), 0, 2, 4),
            Some("bdd-sturdy-blocker"),
            Some(LibraryCard::creature(
                CardDefinitionId::new("bdd-sturdy-blocker"),
                0,
                2,
                4,
            )),
        );
        self.tracked_card_id = self.tracked_attacker_id.clone();
    }

    pub fn setup_end_step_with_surviving_damage(&mut self) {
        self.setup_combat(
            "bdd-cleanup-damage",
            "bdd-survivor",
            LibraryCard::creature(CardDefinitionId::new("bdd-survivor"), 0, 3, 3),
            Some("bdd-trader"),
            Some(LibraryCard::creature(
                CardDefinitionId::new("bdd-trader"),
                0,
                2,
                2,
            )),
        );
        self.resolve_combat_damage("Alice");
        let service = support::create_service();
        support::advance_n_satisfying_cleanup(&service, self.game_mut(), 2);
        while self.player_hand_size("Alice") > 7 {
            let card_id = self
                .player("Alice")
                .hand_card_at(0)
                .expect("Alice hand card should exist")
                .id()
                .clone();
            service
                .discard_for_cleanup(
                    self.game_mut(),
                    DiscardForCleanupCommand::new(Self::player_id("Alice"), card_id),
                )
                .expect("BDD cleanup discard setup should succeed");
        }
        self.tracked_card_id = self.tracked_attacker_id.clone();
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::EndStep);
    }
}
