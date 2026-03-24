//! BDD coverage for world actions stack casting.

use {
    super::super::support,
    super::super::GameplayWorld,
    demonictutor::{CastSpellCommand, PlayerId, SpellTarget},
};

impl GameplayWorld {
    fn cast_targeted_creature_spell_with_card(
        &mut self,
        caster_alias: &str,
        card_id: demonictutor::CardInstanceId,
        target_card_id: demonictutor::CardInstanceId,
    ) {
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(caster_alias), card_id)
                    .with_target(SpellTarget::Creature(target_card_id)),
            )
            .expect("casting targeted creature spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }

    fn try_cast_targeted_creature_spell_with_card(
        &mut self,
        caster_alias: &str,
        card_id: demonictutor::CardInstanceId,
        target_card_id: demonictutor::CardInstanceId,
    ) {
        let service = support::create_service();
        let res = service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(caster_alias), card_id)
                .with_target(SpellTarget::Creature(target_card_id)),
        );
        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    fn cast_targeted_graveyard_card_spell_with_card(
        &mut self,
        caster_alias: &str,
        card_id: demonictutor::CardInstanceId,
        target_card_id: demonictutor::CardInstanceId,
    ) {
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(caster_alias), card_id)
                    .with_target(SpellTarget::GraveyardCard(target_card_id)),
            )
            .expect("casting targeted graveyard-card spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }

    pub fn cast_tracked_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(alias), card_id),
            )
            .expect("casting tracked spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }

    pub fn try_cast_tracked_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let res = service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id),
        );
        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    pub fn cast_tracked_targeted_player_spell(&mut self, caster_alias: &str, target_alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(caster_alias), card_id)
                    .with_target(SpellTarget::Player(Self::player_id(target_alias))),
            )
            .expect("casting targeted player spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }

    pub fn try_cast_tracked_targeted_player_spell(
        &mut self,
        caster_alias: &str,
        target_alias: &str,
    ) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let res = service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(caster_alias), card_id).with_target(
                SpellTarget::Player(match target_alias {
                    "Alice" | "Bob" => Self::player_id(target_alias),
                    raw_player_id => PlayerId::new(raw_player_id),
                }),
            ),
        );
        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    pub fn cast_tracked_targeted_creature_spell(&mut self, caster_alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let target_card_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked target creature should exist");
        self.cast_targeted_creature_spell_with_card(caster_alias, card_id, target_card_id);
    }

    pub fn cast_tracked_targeted_graveyard_card_spell(&mut self, caster_alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let target_card_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked target graveyard card should exist");
        self.cast_targeted_graveyard_card_spell_with_card(caster_alias, card_id, target_card_id);
    }

    pub fn try_cast_tracked_targeted_creature_spell(&mut self, caster_alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let target_card_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked target creature should exist");
        self.try_cast_targeted_creature_spell_with_card(caster_alias, card_id, target_card_id);
    }

    pub fn cast_tracked_targeted_attacker_spell(&mut self, caster_alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let target_card_id = self
            .tracked_attacker_id
            .clone()
            .expect("tracked attacker should exist");
        self.cast_targeted_creature_spell_with_card(caster_alias, card_id, target_card_id);
    }

    pub fn cast_tracked_targeted_response_spell_at_blocker(&mut self, caster_alias: &str) {
        let card_id = self
            .tracked_response_card_id
            .clone()
            .expect("tracked response card should exist");
        let target_card_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked blocker should exist");
        self.cast_targeted_creature_spell_with_card(caster_alias, card_id, target_card_id);
    }

    pub fn try_cast_tracked_targeted_response_spell_at_blocker(&mut self, caster_alias: &str) {
        let card_id = self
            .tracked_response_card_id
            .clone()
            .expect("tracked response card should exist");
        let target_card_id = self
            .tracked_blocker_id
            .clone()
            .expect("tracked blocker should exist");
        self.try_cast_targeted_creature_spell_with_card(caster_alias, card_id, target_card_id);
    }

    pub fn cast_tracked_targeted_response_spell_at_attacker(&mut self, caster_alias: &str) {
        let card_id = self
            .tracked_response_card_id
            .clone()
            .expect("tracked response card should exist");
        let target_card_id = self
            .tracked_attacker_id
            .clone()
            .expect("tracked attacker should exist");
        self.cast_targeted_creature_spell_with_card(caster_alias, card_id, target_card_id);
    }

    pub fn try_cast_tracked_targeted_response_spell_at_attacker(&mut self, caster_alias: &str) {
        let card_id = self
            .tracked_response_card_id
            .clone()
            .expect("tracked response card should exist");
        let target_card_id = self
            .tracked_attacker_id
            .clone()
            .expect("tracked attacker should exist");
        self.try_cast_targeted_creature_spell_with_card(caster_alias, card_id, target_card_id);
    }

    pub fn cast_tracked_response_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_response_card_id
            .clone()
            .expect("tracked response card should exist");
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(alias), card_id),
            )
            .expect("casting response spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }

    pub fn try_cast_tracked_response_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_response_card_id
            .clone()
            .expect("tracked response card should exist");
        let service = support::create_service();
        let res = service.cast_spell(
            self.game_mut(),
            CastSpellCommand::new(Self::player_id(alias), card_id),
        );
        if let Err(e) = res {
            self.last_error = Some(e.to_string());
        }
    }

    pub fn cast_tracked_second_response_spell(&mut self, alias: &str) {
        let card_id = self
            .tracked_second_response_card_id
            .clone()
            .expect("tracked second response card should exist");
        let service = support::create_service();
        let outcome = service
            .cast_spell(
                self.game_mut(),
                CastSpellCommand::new(Self::player_id(alias), card_id),
            )
            .expect("casting second response spell should succeed");
        self.last_spell_put_on_stack = Some(outcome.spell_put_on_stack);
    }
}
