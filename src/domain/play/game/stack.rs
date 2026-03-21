use super::{invariants, rules, CastSpellOutcome, Game, PassPriorityOutcome, StackPriorityContext};
use crate::domain::play::{
    commands::{CastSpellCommand, PassPriorityCommand},
    errors::DomainError,
};

impl Game {
    /// Casts a spell.
    ///
    /// # Errors
    /// See [`rules::stack_priority::cast_spell`].
    pub fn cast_spell(&mut self, cmd: CastSpellCommand) -> Result<CastSpellOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        rules::stack_priority::cast_spell(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                active_player: &self.active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        )
    }

    /// Passes priority in an open priority window.
    ///
    /// # Errors
    /// See [`rules::stack_priority::pass_priority`].
    pub fn pass_priority(
        &mut self,
        cmd: PassPriorityCommand,
    ) -> Result<PassPriorityOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        rules::stack_priority::pass_priority(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                active_player: &self.active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        )
    }
}
