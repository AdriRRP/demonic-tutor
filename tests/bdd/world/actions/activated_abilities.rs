use super::super::support;
use super::super::GameplayWorld;
use demonictutor::ActivateAbilityCommand;

impl GameplayWorld {
    pub fn activate_tracked_ability(&mut self, alias: &str) {
        let card_id = self
            .tracked_card_id
            .clone()
            .expect("tracked card should exist");
        let service = support::create_service();
        let outcome = service
            .activate_ability(
                self.game_mut(),
                ActivateAbilityCommand::new(Self::player_id(alias), card_id),
            )
            .expect("activating tracked ability should succeed");
        self.last_activated_ability_put_on_stack = Some(outcome.activated_ability_put_on_stack);
    }
}
