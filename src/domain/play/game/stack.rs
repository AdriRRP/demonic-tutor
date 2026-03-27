//! Supports play game stack.

use {
    super::{
        invariants, rules, ActivateAbilityOutcome, CastSpellOutcome, Game, PassPriorityOutcome,
        ResolveOptionalEffectOutcome, ResolvePendingHandChoiceOutcome, ResolvePendingScryOutcome,
        StackPriorityContext,
    },
    crate::domain::play::{
        commands::{
            ActivateAbilityCommand, CastSpellCommand, PassPriorityCommand,
            ResolveOptionalEffectCommand, ResolvePendingHandChoiceCommand,
            ResolvePendingScryCommand,
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
            for moved_card in &outcome.moved_cards {
                let _ = self
                    .card_locations
                    .set_zone(moved_card, super::model::PlayerCardZone::Graveyard);
            }
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
            if let Some(spell_cast) = &outcome.spell_cast {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &spell_cast.player_id)?;
                self.sync_card_location_from_player(owner_index, &spell_cast.card_id);
            }
            if let Some(card_exiled) = &outcome.card_exiled {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &card_exiled.player_id)?;
                self.sync_card_location_from_player(owner_index, &card_exiled.card_id);
            }
            if let Some(card_discarded) = &outcome.card_discarded {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &card_discarded.player_id)?;
                self.sync_card_location_from_player(owner_index, &card_discarded.card_id);
            }
            for creature_died in &outcome.creatures_died {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &creature_died.player_id)?;
                self.sync_card_location_from_player(owner_index, &creature_died.card_id);
            }
            for card_id in &outcome.moved_cards {
                self.sync_card_location_from_any_player(card_id);
            }
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
            if let Some(card_exiled) = &outcome.card_exiled {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &card_exiled.player_id)?;
                self.sync_card_location_from_player(owner_index, &card_exiled.card_id);
            }
            if let Some(card_discarded) = &outcome.card_discarded {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &card_discarded.player_id)?;
                self.sync_card_location_from_player(owner_index, &card_discarded.card_id);
            }
            for creature_died in &outcome.creatures_died {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &creature_died.player_id)?;
                self.sync_card_location_from_player(owner_index, &creature_died.card_id);
            }
            for card_id in &outcome.moved_cards {
                self.sync_card_location_from_any_player(card_id);
            }
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
            if let Some(spell_cast) = &outcome.spell_cast {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &spell_cast.player_id)?;
                self.sync_card_location_from_player(owner_index, &spell_cast.card_id);
            }
            if let Some(card_discarded) = &outcome.card_discarded {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &card_discarded.player_id)?;
                self.sync_card_location_from_player(owner_index, &card_discarded.card_id);
            }
            for card_id in &outcome.moved_cards {
                self.sync_card_location_from_any_player(card_id);
            }
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
            if let Some(spell_cast) = &outcome.spell_cast {
                let owner_index =
                    super::helpers::find_player_index(&self.players, &spell_cast.player_id)?;
                self.sync_card_location_from_player(owner_index, &spell_cast.card_id);
            }
        }
        result
    }
}
