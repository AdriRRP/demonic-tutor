use super::super::support;
use super::super::GameplayWorld;
use demonictutor::{DeclareBlockersCommand, ExileCardCommand, ResolveCombatDamageCommand};

impl GameplayWorld {
    pub fn resolve_combat_damage(&mut self, alias: &str) {
        let service = support::create_service();
        let outcome = service
            .resolve_combat_damage(
                self.game_mut(),
                ResolveCombatDamageCommand::new(Self::player_id(alias)),
            )
            .expect("resolving combat damage should succeed");

        self.last_combat_damage = Some(outcome.combat_damage_resolved);
        self.last_life_changed = outcome.life_changed;
        self.last_creature_died = outcome.creatures_died;
        self.last_game_ended = outcome.game_ended;
    }

    pub fn try_declare_multiple_blockers_on_one_attacker(&mut self, alias: &str) {
        let attacker_id = self
            .tracked_attacker_id
            .clone()
            .expect("tracked attacker should exist");
        let blocker_1_id = self.player(alias).battlefield().cards()[0].id().clone();
        let blocker_2_id = self.player(alias).battlefield().cards()[1].id().clone();

        let service = support::create_service();
        let res = service.declare_blockers(
            self.game_mut(),
            DeclareBlockersCommand::new(
                Self::player_id(alias),
                vec![
                    (blocker_1_id, attacker_id.clone()),
                    (blocker_2_id, attacker_id),
                ],
            ),
        );

        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    pub fn declare_blocker_against(
        &mut self,
        blocker_alias: &str,
        blocker_id: &demonictutor::CardInstanceId,
        attacker_id: &demonictutor::CardInstanceId,
    ) {
        let service = support::create_service();
        service
            .declare_blockers(
                self.game_mut(),
                DeclareBlockersCommand::new(
                    Self::player_id(blocker_alias),
                    vec![(blocker_id.clone(), attacker_id.clone())],
                ),
            )
            .expect("blocking should succeed");
    }

    pub fn try_declare_blocker_against(
        &mut self,
        blocker_alias: &str,
        blocker_id: &demonictutor::CardInstanceId,
        attacker_id: &demonictutor::CardInstanceId,
    ) {
        let service = support::create_service();
        let res = service.declare_blockers(
            self.game_mut(),
            DeclareBlockersCommand::new(
                Self::player_id(blocker_alias),
                vec![(blocker_id.clone(), attacker_id.clone())],
            ),
        );
        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    pub fn exile_tracked_card(&mut self, alias: &str, from_battlefield: bool) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let event = service
            .exile_card(
                self.game_mut(),
                &ExileCardCommand::new(Self::player_id(alias), card_id, from_battlefield),
            )
            .expect("exiling tracked card should succeed");
        self.last_card_exiled = Some(event);
    }
}
