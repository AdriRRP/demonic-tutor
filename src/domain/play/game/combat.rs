use super::{invariants, rules, Game, PriorityState, ResolveCombatDamageOutcome};
use crate::domain::play::{
    commands::{DeclareAttackersCommand, DeclareBlockersCommand, ResolveCombatDamageCommand},
    errors::DomainError,
    events::{AttackersDeclared, BlockersDeclared},
    phase::Phase,
};

impl Game {
    /// Declares attackers in combat.
    ///
    /// # Errors
    /// See [`rules::combat::declare_attackers`].
    pub fn declare_attackers(
        &mut self,
        cmd: DeclareAttackersCommand,
    ) -> Result<AttackersDeclared, DomainError> {
        invariants::require_game_active(self.is_over())?;
        invariants::require_no_open_priority_window(self.priority())?;
        let event = rules::combat::declare_attackers(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )?;
        self.phase = Phase::DeclareBlockers;
        self.priority = Some(PriorityState::opened(self.active_player.clone()));
        Ok(event)
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
        let event = rules::combat::declare_blockers(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            cmd,
        )?;
        self.phase = Phase::CombatDamage;
        self.priority = Some(PriorityState::opened(self.active_player.clone()));
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
        let outcome = rules::combat::resolve_combat_damage(
            &self.id,
            &mut self.players,
            &self.active_player,
            &self.phase,
            &mut self.terminal_state,
            cmd,
        )?;

        self.phase = Phase::EndOfCombat;
        self.priority = if self.is_over() {
            None
        } else {
            Some(PriorityState::opened(self.active_player.clone()))
        };

        Ok(outcome)
    }

    /// Resets all blocker states.
    pub fn reset_blockers(&mut self) {
        for player in &mut self.players {
            for card in player.battlefield_mut().iter_mut() {
                card.set_blocking(false);
            }
        }
    }
}
