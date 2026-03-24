//! BDD coverage for world setup combat windows keyword combat.

use {
    super::super::support,
    super::super::GameplayWorld,
    demonictutor::{CastSpellCommand, LibraryCard, Phase},
};

impl GameplayWorld {
    pub fn setup_haste_attack(&mut self) {
        self.reset_game_with_libraries(
            "bdd-haste-attack",
            support::filled_library(
                vec![support::creature_card_with_keyword(
                    "attacker",
                    0,
                    2,
                    2,
                    demonictutor::KeywordAbility::Haste,
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
        let attacker_id = self.hand_card_by_definition("Alice", "attacker");
        service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id("Alice"), attacker_id.clone()),
            )
            .expect("attacker cast should succeed");
        support::resolve_top_stack_with_passes(&service, self.game_mut());
        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::DeclareAttackers,
        );

        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    pub fn setup_vigilance_attack(&mut self) {
        self.reset_game_with_libraries(
            "bdd-vigilance-attack",
            support::filled_library(
                vec![support::creature_card_with_keyword(
                    "attacker",
                    0,
                    2,
                    2,
                    demonictutor::KeywordAbility::Vigilance,
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
        let attacker_id = self.hand_card_by_definition("Alice", "attacker");
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
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::DeclareAttackers,
        );

        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    fn setup_keyword_combat(
        &mut self,
        game_id: &str,
        attacker_card: LibraryCard,
        blocker_card: LibraryCard,
    ) {
        self.reset_game_with_libraries(
            game_id,
            support::filled_library(vec![attacker_card], 10),
            support::filled_library(vec![blocker_card], 10),
        );

        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", "attacker");
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
        let blocker_id = self.hand_card_by_definition("Bob", "blocker");
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
        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::DeclareAttackers,
        );
        service
            .declare_attackers(
                self.game_mut(),
                demonictutor::DeclareAttackersCommand::new(
                    Self::player_id("Alice"),
                    vec![attacker_id.clone()],
                ),
            )
            .expect("declare attackers should succeed");

        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::DeclareBlockers,
        );

        self.tracked_attacker_id = Some(attacker_id);
        self.tracked_blocker_id = Some(blocker_id);
    }

    pub fn setup_flying_attack_and_block(&mut self) {
        self.setup_keyword_combat(
            "bdd-flying-block",
            support::creature_card_with_keywords("attacker", 0, 2, 2, true, false),
            support::creature_card_with_keywords("blocker", 0, 2, 2, true, false),
        );
        let service = support::create_service();
        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::DeclareBlockers,
        );
        support::close_empty_priority_window(&service, self.game_mut());
        self.reset_observations();
    }

    pub fn setup_flying_attack_and_reach_block(&mut self) {
        self.setup_keyword_combat(
            "bdd-reach-block",
            support::creature_card_with_keywords("attacker", 0, 2, 2, true, false),
            support::creature_card_with_keywords("blocker", 0, 2, 2, false, true),
        );
        let service = support::create_service();
        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::DeclareBlockers,
        );
        support::close_empty_priority_window(&service, self.game_mut());
        self.reset_observations();
    }

    pub fn setup_flying_attack_and_nonflying_block(&mut self) {
        self.setup_keyword_combat(
            "bdd-nonflying-block",
            support::creature_card_with_keywords("attacker", 0, 2, 2, true, false),
            support::creature_card_with_keywords("blocker", 0, 2, 2, false, false),
        );
        let service = support::create_service();
        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::DeclareBlockers,
        );
        support::close_empty_priority_window(&service, self.game_mut());
        self.reset_observations();
    }

    pub fn setup_nonflying_attack_and_block(&mut self) {
        self.setup_keyword_combat(
            "bdd-nonflying-attacker",
            support::creature_card_with_keywords("attacker", 0, 2, 2, false, false),
            support::creature_card_with_keywords("blocker", 0, 2, 2, false, false),
        );
        let service = support::create_service();
        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::DeclareBlockers,
        );
        support::close_empty_priority_window(&service, self.game_mut());
        self.reset_observations();
    }

    pub fn setup_flying_and_reach_block(&mut self) {
        self.setup_keyword_combat(
            "bdd-flying-reach-block",
            support::creature_card_with_keywords("attacker", 0, 2, 2, true, false),
            support::creature_card_with_keywords("blocker", 0, 2, 2, true, true),
        );
        let service = support::create_service();
        support::advance_to_phase_satisfying_cleanup(
            &service,
            self.game_mut(),
            Phase::DeclareBlockers,
        );
        support::close_empty_priority_window(&service, self.game_mut());
        self.reset_observations();
    }

    pub fn setup_unblocked_flying_attack(&mut self) {
        self.setup_combat(
            "bdd-unblocked-flying",
            "attacker",
            support::creature_card_with_keywords("attacker", 0, 3, 2, true, false),
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
}
