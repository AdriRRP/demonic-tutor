//! Supports play game resource actions.

use {
    super::{invariants, rules, AdjustPlayerLifeEffectOutcome, Game},
    crate::domain::play::{
        commands::{AdjustPlayerLifeEffectCommand, PlayLandCommand, TapLandCommand},
        errors::DomainError,
        events::{LandPlayed, LandTapped, ManaAdded},
    },
};

impl Game {
    /// Plays a land from hand to battlefield.
    ///
    /// # Errors
    /// See [`rules::resource_actions::play_land`].
    pub fn play_land(&mut self, cmd: PlayLandCommand) -> Result<LandPlayed, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_priority_with_pending_stack(self.priority(), self.stack.is_empty())?;
        let active_player_index = self.active_player_index;
        let result = rules::resource_actions::play_land(
            &self.id,
            &mut self.players,
            active_player_index,
            &self.phase,
            cmd,
        );
        if let Ok(event) = &result {
            let zone_changes = [Self::zone_change_for_land_played(event)];
            self.sync_zone_changes(&zone_changes)?;
        }
        result
    }

    /// Resolves an explicit life effect from a caster onto a target player.
    ///
    /// # Errors
    /// See [`rules::resource_actions::adjust_player_life_effect`].
    pub fn adjust_player_life_effect(
        &mut self,
        cmd: AdjustPlayerLifeEffectCommand,
    ) -> Result<AdjustPlayerLifeEffectOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_empty_stack_priority_action_window(
            self.priority(),
            self.stack.is_empty(),
            self.active_player(),
        )?;
        let caster_index = super::helpers::find_player_index(&self.players, &cmd.caster_id)?;
        let result = rules::resource_actions::adjust_player_life_effect(
            &self.id,
            &mut self.players,
            &mut self.terminal_state,
            caster_index,
            cmd,
        );
        if let Ok(outcome) = &result {
            let zone_changes = outcome
                .creatures_died
                .iter()
                .map(Self::zone_change_for_creature_died)
                .collect::<Vec<_>>();
            self.sync_zone_changes(&zone_changes)?;
        }
        result
    }

    /// Taps a land to produce mana.
    ///
    /// # Errors
    /// See [`rules::resource_actions::tap_land`].
    pub fn tap_land(
        &mut self,
        cmd: TapLandCommand,
    ) -> Result<(LandTapped, ManaAdded), DomainError> {
        invariants::require_game_active(self.is_over())?;
        let active_player_index = self.active_player_index;
        let priority = self.priority.clone();
        if let Some(priority) = priority.as_ref() {
            invariants::require_priority_holder(Some(priority), &cmd.player_id)?;
        }
        let result = rules::resource_actions::tap_land(
            &self.id,
            &mut self.players,
            active_player_index,
            &self.phase,
            priority.as_ref(),
            cmd,
        );
        result
    }
}
