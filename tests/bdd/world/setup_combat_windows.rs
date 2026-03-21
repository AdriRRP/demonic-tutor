use super::support;
use super::GameplayWorld;
use demonictutor::{
    AdjustPlayerLifeEffectCommand, CardDefinitionId, CastSpellCommand, DiscardForCleanupCommand,
    LibraryCard, Phase, ResolveCombatDamageCommand,
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

    pub fn setup_priority_after_attackers_declared(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-priority"),
                    0,
                    2,
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
        self.reset_observations();
    }

    pub fn setup_priority_after_attackers_declared_with_instant(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers-instant",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-priority"), 0, 2, 2),
                    support::instant_card("bdd-window-instant", 0),
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
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-priority");
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant"));
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
        self.reset_observations();
    }

    pub fn setup_priority_after_attackers_declared_with_two_instants(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers-two-instants",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-priority"), 0, 2, 2),
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
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-priority");
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant-a"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-instant-b"));
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
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::DeclareBlockers);
        assert_eq!(
            self.game()
                .priority()
                .expect("attackers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_non_active_priority_after_attackers_declared_with_instant(&mut self) {
        let mut bob_cards = support::filled_library(Vec::new(), 7);
        bob_cards.push(support::instant_card("bdd-window-instant", 0));
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers-response",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-priority"),
                    0,
                    2,
                    2,
                )],
                10,
            ),
            bob_cards,
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

        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_attackers_declared_with_two_instants(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-priority-attackers-response-two-instants",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-priority"),
                    0,
                    2,
                    2,
                )],
                10,
            ),
            support::filled_library(
                vec![
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                7,
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

        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
    }

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

    pub fn setup_priority_after_blockers_declared(&mut self) {
        self.reset_game_with_libraries(
            "bdd-combat-priority-blockers",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-priority"),
                    0,
                    2,
                    2,
                )],
                10,
            ),
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-blocker-priority"),
                    0,
                    2,
                    2,
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
                .expect("blocker should exist"),
            self.tracked_attacker_id
                .clone()
                .expect("attacker should exist"),
        )];
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_instant(&mut self) {
        /* moved setup kept in dedicated combat harness module */
        self.reset_game_with_libraries(
            "bdd-combat-priority-blockers-instant",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-priority"), 0, 2, 2),
                    support::instant_card("bdd-window-instant", 0),
                ],
                10,
            ),
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-blocker-priority"),
                    0,
                    2,
                    2,
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
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant"));
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
                .expect("blocker should exist"),
            self.tracked_attacker_id
                .clone()
                .expect("attacker should exist"),
        )];
        self.reset_observations();
    }

    pub fn setup_priority_after_blockers_declared_with_two_instants(&mut self) {
        /* kept same behavior in dedicated combat harness module */
        self.reset_game_with_libraries(
            "bdd-combat-priority-blockers-two-instants",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-priority"), 0, 2, 2),
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                10,
            ),
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-blocker-priority"),
                    0,
                    2,
                    2,
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
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant-a"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-instant-b"));
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
                .expect("blocker should exist"),
            self.tracked_attacker_id
                .clone()
                .expect("attacker should exist"),
        )];
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::CombatDamage);
        assert_eq!(
            self.game()
                .priority()
                .expect("blockers declaration should open priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_non_active_priority_after_blockers_declared_with_instant(&mut self) {
        /* dedicated combat harness module */
        let mut bob_cards = support::filled_library(
            vec![LibraryCard::creature(
                CardDefinitionId::new("bdd-blocker-priority"),
                0,
                2,
                2,
            )],
            7,
        );
        bob_cards.push(support::instant_card("bdd-window-instant", 0));
        self.reset_game_with_libraries(
            "bdd-combat-priority-blockers-response",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-priority"),
                    0,
                    2,
                    2,
                )],
                10,
            ),
            bob_cards,
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
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.tracked_blocker_id = Some(blocker_id);
        self.blocker_assignments = vec![(
            self.tracked_blocker_id
                .clone()
                .expect("blocker should exist"),
            self.tracked_attacker_id
                .clone()
                .expect("attacker should exist"),
        )];
        self.reset_observations();
    }

    pub fn setup_non_active_priority_after_blockers_declared_with_two_instants(&mut self) {
        /* dedicated combat harness module */
        self.reset_game_with_libraries(
            "bdd-combat-priority-blockers-response-two-instants",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-priority"),
                    0,
                    2,
                    2,
                )],
                10,
            ),
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-blocker-priority"), 0, 2, 2),
                    support::instant_card("bdd-window-instant-a", 0),
                    support::instant_card("bdd-window-instant-b", 0),
                ],
                7,
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
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.tracked_blocker_id = Some(blocker_id);
        self.blocker_assignments = vec![(
            self.tracked_blocker_id
                .clone()
                .expect("blocker should exist"),
            self.tracked_attacker_id
                .clone()
                .expect("attacker should exist"),
        )];
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
        /* moved combat setup family */
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

    pub fn setup_priority_after_combat_damage_with_instant(&mut self) {
        /* dedicated combat harness module */
        self.reset_game_with_libraries(
            "bdd-post-combat-damage-instant",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-unblocked"), 0, 3, 3),
                    support::instant_card("bdd-window-instant", 0),
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
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-unblocked");
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant"));
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
        support::advance_turn_raw(&service, self.game_mut());
        service
            .resolve_combat_damage(
                self.game_mut(),
                ResolveCombatDamageCommand::new(Self::player_id("Alice")),
            )
            .expect("combat damage should resolve");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::EndOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat damage should reopen priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
    }

    pub fn setup_priority_after_combat_damage_with_two_instants(&mut self) {
        /* dedicated combat harness module */
        self.reset_game_with_libraries(
            "bdd-post-combat-damage-two-instants",
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-attacker-unblocked"), 0, 3, 3),
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
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-unblocked");
        self.tracked_card_id = Some(self.hand_card_by_definition("Alice", "bdd-window-instant-a"));
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Alice", "bdd-window-instant-b"));
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
        support::advance_turn_raw(&service, self.game_mut());
        service
            .resolve_combat_damage(
                self.game_mut(),
                ResolveCombatDamageCommand::new(Self::player_id("Alice")),
            )
            .expect("combat damage should resolve");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::EndOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat damage should reopen priority")
                .current_holder(),
            &Self::player_id("Alice")
        );
        assert!(self.game().stack().is_empty());
    }

    pub fn setup_non_active_priority_after_combat_damage_with_instant(&mut self) {
        /* dedicated combat harness module */
        let mut bob_cards = support::filled_library(Vec::new(), 7);
        bob_cards.push(support::instant_card("bdd-window-instant", 0));
        self.reset_game_with_libraries(
            "bdd-post-combat-damage-response",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-unblocked"),
                    0,
                    3,
                    3,
                )],
                10,
            ),
            bob_cards,
        );
        let service = support::create_service();
        support::advance_to_player_first_main_satisfying_cleanup(
            &service,
            self.game_mut(),
            "player-1",
        );
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-unblocked");
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
        support::advance_turn_raw(&service, self.game_mut());
        service
            .resolve_combat_damage(
                self.game_mut(),
                ResolveCombatDamageCommand::new(Self::player_id("Alice")),
            )
            .expect("combat damage should resolve");
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::EndOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat damage should reopen priority")
                .current_holder(),
            &Self::player_id("Bob")
        );
    }

    pub fn setup_non_active_priority_in_end_of_combat_with_two_instants(&mut self) {
        /* dedicated combat harness module */
        self.reset_game_with_libraries(
            "bdd-post-combat-damage-response-two-instants",
            support::filled_library(
                vec![LibraryCard::creature(
                    CardDefinitionId::new("bdd-attacker-unblocked"),
                    0,
                    3,
                    3,
                )],
                10,
            ),
            support::filled_library(
                vec![
                    LibraryCard::creature(CardDefinitionId::new("bdd-bob-buffer"), 0, 2, 2),
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
        let attacker_id = self.hand_card_by_definition("Alice", "bdd-attacker-unblocked");
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
        support::advance_turn_raw(&service, self.game_mut());
        service
            .resolve_combat_damage(
                self.game_mut(),
                ResolveCombatDamageCommand::new(Self::player_id("Alice")),
            )
            .expect("combat damage should resolve");
        self.tracked_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-a"));
        self.tracked_second_response_card_id =
            Some(self.hand_card_by_definition("Bob", "bdd-window-instant-b"));
        self.pass_priority("Alice");
        self.tracked_attacker_id = Some(attacker_id);
        self.reset_observations();
        assert_eq!(self.game().phase(), &Phase::EndOfCombat);
        assert_eq!(
            self.game()
                .priority()
                .expect("combat damage should reopen priority")
                .current_holder(),
            &Self::player_id("Bob")
        );
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
            let card_id = self.player("Alice").hand().cards()[0].id().clone();
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
