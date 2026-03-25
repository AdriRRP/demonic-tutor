//! Supports rules stack priority passing.

use {
    super::{resolution::resolve_stack_object, PassPriorityOutcome, StackPriorityContext},
    crate::domain::play::{
        commands::PassPriorityCommand,
        errors::{DomainError, GameError},
        events::PriorityPassed,
        game::{invariants, model::PriorityState, Player},
        ids::PlayerId,
    },
};

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
        pending_optional_effect,
        pending_hand_choice_effect,
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
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
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
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
            game_ended: None,
            priority_still_open: false,
        });
    }

    if let Some(stack_object) = stack.top() {
        if stack_object_requires_optional_choice(stack_object) {
            *priority = None;
            *pending_optional_effect = Some(crate::domain::play::game::PendingOptionalEffect::new(
                stack_object.controller_index(),
                stack_object.number(),
            ));
            return Ok(PassPriorityOutcome {
                priority_passed,
                triggered_abilities_put_on_stack: Vec::new(),
                stack_top_resolved: None,
                spell_cast: None,
                card_exiled: None,
                card_discarded: None,
                life_changed: None,
                creatures_died: Vec::new(),
                moved_cards: Vec::new(),
                game_ended: None,
                priority_still_open: false,
            });
        }

        if let Some(kind) = stack_object_pending_hand_choice_kind(stack_object) {
            if let crate::domain::play::game::PendingHandChoiceKind::Loot { draw_count } = kind {
                let controller_index = stack_object.controller_index();
                for _ in 0..draw_count {
                    if players[controller_index].draw_one_into_hand().is_none() {
                        let game_ended = crate::domain::play::game::rules::game_effects::end_game_for_empty_library_draw(
                            game_id,
                            players,
                            terminal_state,
                            controller_index,
                        )?;
                        *priority = None;
                        return Ok(PassPriorityOutcome {
                            priority_passed,
                            triggered_abilities_put_on_stack: Vec::new(),
                            stack_top_resolved: None,
                            spell_cast: None,
                            card_exiled: None,
                            card_discarded: None,
                            life_changed: None,
                            creatures_died: Vec::new(),
                            moved_cards: Vec::new(),
                            game_ended: Some(game_ended),
                            priority_still_open: false,
                        });
                    }
                }
            }

            *priority = None;
            *pending_hand_choice_effect =
                Some(crate::domain::play::game::PendingHandChoiceEffect::new(
                    stack_object.controller_index(),
                    stack_object.number(),
                    kind,
                ));
            return Ok(PassPriorityOutcome {
                priority_passed,
                triggered_abilities_put_on_stack: Vec::new(),
                stack_top_resolved: None,
                spell_cast: None,
                card_exiled: None,
                card_discarded: None,
                life_changed: None,
                creatures_died: Vec::new(),
                moved_cards: Vec::new(),
                game_ended: None,
                priority_still_open: false,
            });
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
        card_exiled,
        card_discarded,
        life_changed,
        creatures_died,
        moved_cards,
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
        card_exiled,
        card_discarded,
        life_changed,
        creatures_died,
        moved_cards,
        game_ended,
        priority_still_open: priority.is_some(),
    })
}
