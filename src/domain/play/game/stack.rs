//! Supports play game stack.

use {
    super::{
        invariants, rules, ActivateAbilityOutcome, CastSpellOutcome, Game, PassPriorityOutcome,
        ResolveOptionalEffectOutcome, ResolvePendingHandChoiceOutcome, ResolvePendingScryOutcome,
        ResolvePendingSurveilOutcome, StackPriorityContext,
    },
    crate::domain::play::{
        commands::{
            ActivateAbilityCommand, CastSpellCommand, PassPriorityCommand,
            ResolveOptionalEffectCommand, ResolvePendingHandChoiceCommand,
            ResolvePendingScryCommand, ResolvePendingSurveilCommand,
        },
        errors::DomainError,
    },
};

impl Game {
    /// Activates a supported non-mana ability from the battlefield.
    ///
    /// # Errors
    /// See [`rules::stack_priority::activate_ability`].
    pub fn activate_ability(
        &mut self,
        cmd: ActivateAbilityCommand,
    ) -> Result<ActivateAbilityOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        let active_player = self.active_player().clone();
        let result = rules::stack_priority::activate_ability(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                card_locations: &self.card_locations,
                active_player: &active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                pending_decision: &mut self.pending_decision,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        );
        if let Ok(outcome) = &result {
            self.sync_zone_changes(&outcome.zone_changes)?;
        }
        result
    }

    /// Casts a spell.
    ///
    /// # Errors
    /// See [`rules::stack_priority::cast_spell`].
    pub fn cast_spell(&mut self, cmd: CastSpellCommand) -> Result<CastSpellOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        let active_player = self.active_player().clone();
        let result = rules::stack_priority::cast_spell(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                card_locations: &self.card_locations,
                active_player: &active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                pending_decision: &mut self.pending_decision,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        );
        if let Ok(outcome) = &result {
            self.card_locations
                .remove(&outcome.spell_put_on_stack.card_id);
        }
        result
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
        let active_player = self.active_player().clone();
        let result = rules::stack_priority::pass_priority(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                card_locations: &self.card_locations,
                active_player: &active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                pending_decision: &mut self.pending_decision,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        );
        if let Ok(outcome) = &result {
            self.sync_zone_changes(&outcome.zone_changes)?;
        }
        result
    }

    /// Resolves a pending optional effect choice.
    ///
    /// # Errors
    /// See [`rules::stack_priority::resolve_optional_effect`].
    pub fn resolve_optional_effect(
        &mut self,
        cmd: ResolveOptionalEffectCommand,
    ) -> Result<ResolveOptionalEffectOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        let active_player = self.active_player().clone();
        let result = rules::stack_priority::resolve_optional_effect(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                card_locations: &self.card_locations,
                active_player: &active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                pending_decision: &mut self.pending_decision,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        );
        if let Ok(outcome) = &result {
            self.sync_zone_changes(&outcome.zone_changes)?;
        }
        result
    }

    /// Resolves a pending hand-choice effect.
    ///
    /// # Errors
    /// See [`rules::stack_priority::resolve_pending_hand_choice`].
    pub fn resolve_pending_hand_choice(
        &mut self,
        cmd: ResolvePendingHandChoiceCommand,
    ) -> Result<ResolvePendingHandChoiceOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        let active_player = self.active_player().clone();
        let result = rules::stack_priority::resolve_pending_hand_choice(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                card_locations: &self.card_locations,
                active_player: &active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                pending_decision: &mut self.pending_decision,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        );
        if let Ok(outcome) = &result {
            self.sync_zone_changes(&outcome.zone_changes)?;
        }
        result
    }

    /// Resolves a pending scry decision.
    ///
    /// # Errors
    /// See [`rules::stack_priority::resolve_pending_scry`].
    pub fn resolve_pending_scry(
        &mut self,
        cmd: ResolvePendingScryCommand,
    ) -> Result<ResolvePendingScryOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        let active_player = self.active_player().clone();
        let result = rules::stack_priority::resolve_pending_scry(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                card_locations: &self.card_locations,
                active_player: &active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                pending_decision: &mut self.pending_decision,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        );
        if let Ok(outcome) = &result {
            self.sync_zone_changes(&outcome.zone_changes)?;
        }
        result
    }

    /// Resolves a pending surveil decision.
    ///
    /// # Errors
    /// See [`rules::stack_priority::resolve_pending_surveil`].
    pub fn resolve_pending_surveil(
        &mut self,
        cmd: ResolvePendingSurveilCommand,
    ) -> Result<ResolvePendingSurveilOutcome, DomainError> {
        invariants::require_game_active(self.is_over())?;
        let active_player = self.active_player().clone();
        let result = rules::stack_priority::resolve_pending_surveil(
            StackPriorityContext {
                game_id: &self.id,
                players: &mut self.players,
                card_locations: &self.card_locations,
                active_player: &active_player,
                phase: &self.phase,
                stack: &mut self.stack,
                priority: &mut self.priority,
                pending_decision: &mut self.pending_decision,
                terminal_state: &mut self.terminal_state,
            },
            cmd,
        );
        if let Ok(outcome) = &result {
            self.sync_zone_changes(&outcome.zone_changes)?;
        }
        result
    }
}
