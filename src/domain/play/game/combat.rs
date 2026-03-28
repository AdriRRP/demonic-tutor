//! Supports play game combat.

use {
    super::{invariants, rules, Game, PriorityState, ResolveCombatDamageOutcome},
    crate::domain::play::{
        commands::{DeclareAttackersCommand, DeclareBlockersCommand, ResolveCombatDamageCommand},
        errors::DomainError,
        events::BlockersDeclared,
        phase::Phase,
    },
};

impl Game {
    /// Declares attackers in combat.
    ///
    /// # Errors
    /// See [`rules::combat::declare_attackers`].
    pub fn declare_attackers(
        &mut self,
        cmd: DeclareAttackersCommand,
    ) -> Result<rules::combat::DeclareAttackersOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        let active_player = self.active_player().clone();
        let active_player_index = self.active_player_index;
        let outcome = rules::combat::declare_attackers(
            &self.id,
            &mut self.players,
            active_player_index,
            &self.phase,
            &mut self.stack,
            cmd,
        )?;
        self.phase = Phase::DeclareBlockers;
        self.priority = Some(PriorityState::opened(active_player));
        Ok(outcome)
    }

    /// Declares blockers in combat.
    ///
    /// # Errors
    /// See [`rules::combat::declare_blockers`].
    pub fn declare_blockers(
        &mut self,
        cmd: DeclareBlockersCommand,
    ) -> Result<BlockersDeclared, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        let active_player = self.active_player().clone();
        let active_player_index = self.active_player_index;
        let event = rules::combat::declare_blockers(
            &self.id,
            &mut self.players,
            active_player_index,
            &self.phase,
            cmd,
        )?;
        self.phase = Phase::CombatDamage;
        self.priority = Some(PriorityState::opened(active_player));
        Ok(event)
    }

    /// Resolves combat damage.
    ///
    /// # Errors
    /// See [`rules::combat::resolve_combat_damage`].
    pub fn resolve_combat_damage(
        &mut self,
        cmd: ResolveCombatDamageCommand,
    ) -> Result<ResolveCombatDamageOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        let active_player = self.active_player().clone();
        let active_player_index = self.active_player_index;
        let outcome = rules::combat::resolve_combat_damage(
            &self.id,
            &mut self.players,
            &self.card_locations,
            &mut self.stack,
            active_player_index,
            &self.phase,
            &mut self.terminal_state,
            cmd,
        )?;

        self.phase = Phase::EndOfCombat;
        self.priority = if self.is_over() {
            None
        } else {
            Some(PriorityState::opened(active_player))
        };

        self.sync_zone_changes(&outcome.zone_changes)?;
        Ok(outcome)
    }
}
