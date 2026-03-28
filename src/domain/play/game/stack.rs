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
        events::{
            CardDiscarded, CardDrawn, CardExiled, CardMovedZone, CreatureDied, SpellCast,
            SpellCastOutcome, ZoneType,
        },
    },
};

impl Game {
    fn sync_zone_changes(&mut self, zone_changes: &[CardMovedZone]) -> Result<(), DomainError> {
        for zone_change in zone_changes {
            self.sync_card_location_from_zone_change(zone_change)?;
        }

        Ok(())
    }

    fn append_spell_resolution_zone_change(
        zone_changes: &mut Vec<CardMovedZone>,
        spell_cast: &SpellCast,
    ) {
        let destination_zone = match spell_cast.outcome {
            SpellCastOutcome::EnteredBattlefield => ZoneType::Battlefield,
            SpellCastOutcome::ResolvedToGraveyard => ZoneType::Graveyard,
            SpellCastOutcome::ResolvedToExile => ZoneType::Exile,
        };
        zone_changes.push(CardMovedZone::new(
            spell_cast.game_id.clone(),
            spell_cast.player_id.clone(),
            spell_cast.card_id.clone(),
            ZoneType::Stack,
            destination_zone,
        ));
    }

    fn append_drawn_card_zone_changes(
        zone_changes: &mut Vec<CardMovedZone>,
        card_drawn: &[CardDrawn],
    ) {
        zone_changes.extend(card_drawn.iter().map(|event| {
            CardMovedZone::new(
                event.game_id.clone(),
                event.player_id.clone(),
                event.card_id.clone(),
                ZoneType::Library,
                ZoneType::Hand,
            )
        }));
    }

    fn append_discarded_card_zone_change(
        zone_changes: &mut Vec<CardMovedZone>,
        card_discarded: Option<&CardDiscarded>,
    ) {
        let Some(event) = card_discarded else {
            return;
        };
        zone_changes.push(CardMovedZone::new(
            event.game_id.clone(),
            event.player_id.clone(),
            event.card_id.clone(),
            ZoneType::Hand,
            ZoneType::Graveyard,
        ));
    }

    fn append_exiled_card_zone_change(
        zone_changes: &mut Vec<CardMovedZone>,
        card_exiled: Option<&CardExiled>,
    ) {
        let Some(event) = card_exiled else {
            return;
        };
        zone_changes.push(CardMovedZone::new(
            event.game_id.clone(),
            event.zone_owner_id.clone(),
            event.card_id.clone(),
            event.origin_zone.clone(),
            ZoneType::Exile,
        ));
    }

    fn append_creature_died_zone_changes(
        zone_changes: &mut Vec<CardMovedZone>,
        creatures_died: &[CreatureDied],
    ) {
        zone_changes.extend(creatures_died.iter().map(|event| {
            CardMovedZone::new(
                event.game_id.clone(),
                event.player_id.clone(),
                event.card_id.clone(),
                ZoneType::Battlefield,
                ZoneType::Graveyard,
            )
        }));
    }

    fn canonical_zone_changes_for_activate_ability(
        outcome: &ActivateAbilityOutcome,
    ) -> Vec<CardMovedZone> {
        let mut zone_changes = outcome.zone_changes.clone();
        Self::append_creature_died_zone_changes(&mut zone_changes, &outcome.creatures_died);
        zone_changes
    }

    fn canonical_zone_changes_for_pass_priority(
        outcome: &PassPriorityOutcome,
    ) -> Vec<CardMovedZone> {
        let mut zone_changes = outcome.zone_changes.clone();
        if let Some(spell_cast) = &outcome.spell_cast {
            Self::append_spell_resolution_zone_change(&mut zone_changes, spell_cast);
        }
        Self::append_drawn_card_zone_changes(&mut zone_changes, &outcome.card_drawn);
        Self::append_exiled_card_zone_change(&mut zone_changes, outcome.card_exiled.as_ref());
        Self::append_discarded_card_zone_change(&mut zone_changes, outcome.card_discarded.as_ref());
        Self::append_creature_died_zone_changes(&mut zone_changes, &outcome.creatures_died);
        zone_changes
    }

    fn canonical_zone_changes_for_resolve_optional_effect(
        outcome: &ResolveOptionalEffectOutcome,
    ) -> Vec<CardMovedZone> {
        let mut zone_changes = outcome.zone_changes.clone();
        if let Some(spell_cast) = &outcome.spell_cast {
            Self::append_spell_resolution_zone_change(&mut zone_changes, spell_cast);
        }
        Self::append_exiled_card_zone_change(&mut zone_changes, outcome.card_exiled.as_ref());
        Self::append_discarded_card_zone_change(&mut zone_changes, outcome.card_discarded.as_ref());
        Self::append_creature_died_zone_changes(&mut zone_changes, &outcome.creatures_died);
        zone_changes
    }

    fn canonical_zone_changes_for_resolve_pending_hand_choice(
        outcome: &ResolvePendingHandChoiceOutcome,
    ) -> Vec<CardMovedZone> {
        let mut zone_changes = outcome.zone_changes.clone();
        if let Some(spell_cast) = &outcome.spell_cast {
            Self::append_spell_resolution_zone_change(&mut zone_changes, spell_cast);
        }
        Self::append_drawn_card_zone_changes(&mut zone_changes, &outcome.card_drawn);
        Self::append_discarded_card_zone_change(&mut zone_changes, outcome.card_discarded.as_ref());
        zone_changes
    }

    fn canonical_zone_changes_for_resolve_pending_scry(
        outcome: &ResolvePendingScryOutcome,
    ) -> Vec<CardMovedZone> {
        let mut zone_changes = outcome.zone_changes.clone();
        if let Some(spell_cast) = &outcome.spell_cast {
            Self::append_spell_resolution_zone_change(&mut zone_changes, spell_cast);
        }
        zone_changes
    }

    fn canonical_zone_changes_for_resolve_pending_surveil(
        outcome: &ResolvePendingSurveilOutcome,
    ) -> Vec<CardMovedZone> {
        let mut zone_changes = outcome.zone_changes.clone();
        if let Some(spell_cast) = &outcome.spell_cast {
            Self::append_spell_resolution_zone_change(&mut zone_changes, spell_cast);
        }
        zone_changes
    }

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
            let zone_changes = Self::canonical_zone_changes_for_activate_ability(outcome);
            self.sync_zone_changes(&zone_changes)?;
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
            let zone_changes = Self::canonical_zone_changes_for_pass_priority(outcome);
            self.sync_zone_changes(&zone_changes)?;
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
            let zone_changes = Self::canonical_zone_changes_for_resolve_optional_effect(outcome);
            self.sync_zone_changes(&zone_changes)?;
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
            let zone_changes =
                Self::canonical_zone_changes_for_resolve_pending_hand_choice(outcome);
            self.sync_zone_changes(&zone_changes)?;
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
            let zone_changes = Self::canonical_zone_changes_for_resolve_pending_scry(outcome);
            self.sync_zone_changes(&zone_changes)?;
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
            let zone_changes = Self::canonical_zone_changes_for_resolve_pending_surveil(outcome);
            self.sync_zone_changes(&zone_changes)?;
        }
        result
    }
}
