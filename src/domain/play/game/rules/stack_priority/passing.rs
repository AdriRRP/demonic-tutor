//! Supports rules stack priority passing.

use {
    super::{
        deferred_resolution::{remove_pending_spell, resolve_pending_spell_to_default_destination},
        hand_choice_effect::draw_cards_for_pending_effect,
        resolution::resolve_stack_object,
        PassPriorityOutcome, StackPriorityContext,
    },
    crate::domain::play::{
        commands::PassPriorityCommand,
        errors::{DomainError, GameError},
        events::{CardDrawn, CardMovedZone, PriorityPassed, SpellCast, ZoneType},
        game::{invariants, model::PriorityState, PendingDecision, Player},
        ids::PlayerId,
    },
};

fn zone_change_for_spell_cast(event: &SpellCast) -> CardMovedZone {
    let destination_zone = match event.outcome {
        crate::domain::play::events::SpellCastOutcome::EnteredBattlefield => ZoneType::Battlefield,
        crate::domain::play::events::SpellCastOutcome::ResolvedToGraveyard => ZoneType::Graveyard,
        crate::domain::play::events::SpellCastOutcome::ResolvedToExile => ZoneType::Exile,
    };
    CardMovedZone::new(
        event.game_id.clone(),
        event.player_id.clone(),
        event.card_id.clone(),
        ZoneType::Stack,
        destination_zone,
    )
}

fn zone_changes_for_drawn_cards(card_drawn: &[CardDrawn]) -> Vec<CardMovedZone> {
    card_drawn
        .iter()
        .map(|event| {
            CardMovedZone::new(
                event.game_id.clone(),
                event.player_id.clone(),
                event.card_id.clone(),
                ZoneType::Library,
                ZoneType::Hand,
            )
        })
        .collect()
}

fn other_player_id(players: &[Player], player_id: &PlayerId) -> Result<PlayerId, DomainError> {
    players
        .iter()
        .find(|player| player.id() != player_id)
        .map(|player| player.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "two-player game should have an opposing player".to_string(),
            ))
        })
}

const fn stack_object_requires_optional_choice(
    stack_object: &crate::domain::play::game::StackObject,
) -> bool {
    match stack_object.kind() {
        crate::domain::play::game::StackObjectKind::TriggeredAbility(ability) => {
            ability.ability().requires_optional_choice()
        }
        _ => false,
    }
}

const fn stack_object_pending_hand_choice_kind(
    stack_object: &crate::domain::play::game::StackObject,
) -> Option<crate::domain::play::game::PendingHandChoiceKind> {
    let crate::domain::play::game::StackObjectKind::Spell(spell) = stack_object.kind() else {
        return None;
    };

    match spell.supported_spell_rules().resolution() {
        crate::domain::play::cards::SpellResolutionProfile::LootDrawThenDiscard { draw_count } => {
            Some(crate::domain::play::game::PendingHandChoiceKind::Loot { draw_count })
        }
        crate::domain::play::cards::SpellResolutionProfile::RummageDiscardThenDraw {
            draw_count,
        } => Some(crate::domain::play::game::PendingHandChoiceKind::Rummage { draw_count }),
        _ => None,
    }
}

const fn stack_object_pending_scry_amount(
    stack_object: &crate::domain::play::game::StackObject,
) -> Option<u32> {
    let crate::domain::play::game::StackObjectKind::Spell(spell) = stack_object.kind() else {
        return None;
    };

    match spell.supported_spell_rules().resolution() {
        crate::domain::play::cards::SpellResolutionProfile::Scry { amount } => Some(amount),
        _ => None,
    }
}

const fn stack_object_pending_surveil_amount(
    stack_object: &crate::domain::play::game::StackObject,
) -> Option<u32> {
    let crate::domain::play::game::StackObjectKind::Spell(spell) = stack_object.kind() else {
        return None;
    };

    match spell.supported_spell_rules().resolution() {
        crate::domain::play::cards::SpellResolutionProfile::Surveil { amount } => Some(amount),
        _ => None,
    }
}

/// Passes priority in the current priority window, and may resolve the top
/// object on the stack when both players pass consecutively.
///
/// # Errors
/// Returns an error if there is no open priority window, if the caller does
/// not currently hold priority, or if resolving the top stack object fails.
#[allow(clippy::too_many_lines)]
pub fn pass_priority(
    ctx: StackPriorityContext<'_>,
    cmd: PassPriorityCommand,
) -> Result<PassPriorityOutcome, DomainError> {
    let StackPriorityContext {
        game_id,
        players,
        card_locations,
        active_player,
        stack,
        priority,
        pending_decision,
        terminal_state,
        ..
    } = ctx;

    let PassPriorityCommand { player_id } = cmd;

    invariants::require_priority_holder(priority.as_ref(), &player_id)?;
    let priority_passed = PriorityPassed::new(game_id.clone(), player_id.clone());
    let has_pending_pass = priority
        .as_ref()
        .map(PriorityState::has_pending_pass)
        .ok_or(DomainError::Game(GameError::NoPriorityWindow))?;

    if !has_pending_pass {
        let next_holder = other_player_id(players, &player_id)?;
        *priority = Some(PriorityState::after_first_pass(next_holder));
        return Ok(PassPriorityOutcome {
            priority_passed,
            triggered_abilities_put_on_stack: Vec::new(),
            stack_top_resolved: None,
            spell_cast: None,
            card_drawn: Vec::new(),
            card_discarded: None,
            zone_changes: Vec::new(),
            life_changed: None,
            creatures_died: Vec::new(),
            game_ended: None,
            priority_still_open: true,
        });
    }

    if stack.is_empty() {
        *priority = None;
        return Ok(PassPriorityOutcome {
            priority_passed,
            triggered_abilities_put_on_stack: Vec::new(),
            stack_top_resolved: None,
            spell_cast: None,
            card_drawn: Vec::new(),
            card_discarded: None,
            zone_changes: Vec::new(),
            life_changed: None,
            creatures_died: Vec::new(),
            game_ended: None,
            priority_still_open: false,
        });
    }

    if let Some(stack_object) = stack.top() {
        if stack_object_requires_optional_choice(stack_object) {
            *priority = None;
            *pending_decision = Some(PendingDecision::optional_effect(
                stack_object.controller_index(),
                stack_object.number(),
            ));
            return Ok(PassPriorityOutcome {
                priority_passed,
                triggered_abilities_put_on_stack: Vec::new(),
                stack_top_resolved: None,
                spell_cast: None,
                card_drawn: Vec::new(),
                card_discarded: None,
                zone_changes: Vec::new(),
                life_changed: None,
                creatures_died: Vec::new(),
                game_ended: None,
                priority_still_open: false,
            });
        }

        if let Some(kind) = stack_object_pending_hand_choice_kind(stack_object) {
            let pending_card_drawn =
                if let crate::domain::play::game::PendingHandChoiceKind::Loot { draw_count } = kind
                {
                    let controller_index = stack_object.controller_index();
                    let stack_object_number = stack_object.number();
                    let (card_drawn, game_ended) = draw_cards_for_pending_effect(
                        game_id,
                        players,
                        terminal_state,
                        controller_index,
                        draw_count,
                    )?;

                    if let Some(game_ended) = game_ended {
                        let pending_spell = remove_pending_spell(
                            players,
                            stack,
                            controller_index,
                            stack_object_number,
                            "loot spell should still be on the stack while opening its pending hand choice",
                            "pending loot resolution requires a spell stack object",
                        )?;
                        let (stack_top_resolved, spell_cast, _moved_cards) =
                            resolve_pending_spell_to_default_destination(
                                game_id,
                                players,
                                controller_index,
                                pending_spell,
                            )?;
                        let mut zone_changes = zone_changes_for_drawn_cards(&card_drawn);
                        zone_changes.push(zone_change_for_spell_cast(&spell_cast));

                        *priority = None;
                        return Ok(PassPriorityOutcome {
                            priority_passed,
                            triggered_abilities_put_on_stack: Vec::new(),
                            stack_top_resolved: Some(stack_top_resolved),
                            spell_cast: Some(spell_cast),
                            card_drawn,
                            card_discarded: None,
                            zone_changes,
                            life_changed: None,
                            creatures_died: Vec::new(),
                            game_ended: Some(game_ended),
                            priority_still_open: false,
                        });
                    }

                    card_drawn
                } else {
                    Vec::new()
                };

            *priority = None;
            let zone_changes = zone_changes_for_drawn_cards(&pending_card_drawn);
            *pending_decision = Some(PendingDecision::hand_choice(
                stack_object.controller_index(),
                stack_object.number(),
                kind,
            ));
            return Ok(PassPriorityOutcome {
                priority_passed,
                triggered_abilities_put_on_stack: Vec::new(),
                stack_top_resolved: None,
                spell_cast: None,
                card_drawn: pending_card_drawn,
                card_discarded: None,
                zone_changes,
                life_changed: None,
                creatures_died: Vec::new(),
                game_ended: None,
                priority_still_open: false,
            });
        }

        if let Some(amount) = stack_object_pending_scry_amount(stack_object) {
            let controller_index = stack_object.controller_index();
            if amount == 1 && players[controller_index].library_size() != 0 {
                *priority = None;
                *pending_decision = Some(PendingDecision::scry(
                    controller_index,
                    stack_object.number(),
                    amount,
                ));
                return Ok(PassPriorityOutcome {
                    priority_passed,
                    triggered_abilities_put_on_stack: Vec::new(),
                    stack_top_resolved: None,
                    spell_cast: None,
                    card_drawn: Vec::new(),
                    card_discarded: None,
                    zone_changes: Vec::new(),
                    life_changed: None,
                    creatures_died: Vec::new(),
                    game_ended: None,
                    priority_still_open: false,
                });
            }
        }

        if let Some(amount) = stack_object_pending_surveil_amount(stack_object) {
            let controller_index = stack_object.controller_index();
            if amount == 1 && players[controller_index].library_size() != 0 {
                *priority = None;
                *pending_decision = Some(PendingDecision::surveil(
                    controller_index,
                    stack_object.number(),
                    amount,
                ));
                return Ok(PassPriorityOutcome {
                    priority_passed,
                    triggered_abilities_put_on_stack: Vec::new(),
                    stack_top_resolved: None,
                    spell_cast: None,
                    card_drawn: Vec::new(),
                    card_discarded: None,
                    zone_changes: Vec::new(),
                    life_changed: None,
                    creatures_died: Vec::new(),
                    game_ended: None,
                    priority_still_open: false,
                });
            }
        }
    }

    let stack_object = stack.pop().ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "priority resolution expected a stack object".to_string(),
        ))
    })?;
    let (
        stack_top_resolved,
        triggered_abilities_put_on_stack,
        spell_cast,
        card_discarded,
        zone_changes,
        life_changed,
        creatures_died,
        _moved_cards,
        game_ended,
    ) = resolve_stack_object(
        game_id,
        players,
        card_locations,
        terminal_state,
        stack,
        stack_object,
    )?;

    if terminal_state.is_over() {
        *priority = None;
    } else {
        *priority = Some(PriorityState::opened(active_player.clone()));
    }

    Ok(PassPriorityOutcome {
        priority_passed,
        triggered_abilities_put_on_stack,
        stack_top_resolved: Some(stack_top_resolved),
        spell_cast,
        card_drawn: Vec::new(),
        card_discarded,
        zone_changes,
        life_changed,
        creatures_died,
        game_ended,
        priority_still_open: priority.is_some(),
    })
}
