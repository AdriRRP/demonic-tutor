use super::{invariants, rules, AdjustPlayerLifeEffectOutcome, Game};
use crate::domain::play::{
    commands::{AdjustPlayerLifeEffectCommand, PlayLandCommand, TapLandCommand},
    errors::DomainError,
    events::{LandPlayed, LandTapped, ManaAdded},
};

impl Game {
    /// Plays a land from hand to battlefield.
    ///
    /// # Errors
    /// See [`rules::resource_actions::play_land`].
    pub fn play_land(&mut self, cmd: PlayLandCommand) -> Result<LandPlayed, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_priority_with_pending_stack(self.priority(), self.stack.is_empty())?;
        rules::resource_actions::play_land(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )
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
            &self.active_player,
        )?;
        rules::resource_actions::adjust_player_life_effect(
            &self.id,
            &mut self.players,
            &mut self.terminal_state,
            cmd,
        )
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
        let priority = self.priority.clone();
        invariants::require_empty_stack_priority_action_window(
            priority.as_ref(),
            self.stack.is_empty(),
            &cmd.player_id,
        )?;
        rules::resource_actions::tap_land(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            priority.as_ref(),
            cmd,
        )
    }
}
