//! Supports rules stack priority resolution.

mod destination;
mod effects;
mod events;
mod extract;

use self::{
    destination::move_resolved_spell_to_its_destination,
    effects::apply_supported_spell_rules,
    events::build_resolution_events,
    extract::{
        extract_resolved_activated_ability, extract_resolved_spell_object,
        ResolvedActivatedAbility, ResolvedSpellObject,
    },
};
use crate::domain::play::{
    cards::ActivatedAbilityEffect,
    events::{CardExiled, CreatureDied, GameEnded, LifeChanged, SpellCast, StackTopResolved},
    game::{
        model::{StackObject, StackObjectKind},
        Player, TerminalState,
    },
    ids::GameId,
};

type ResolvedSpellOutcome = (
    StackTopResolved,
    Option<SpellCast>,
    Option<CardExiled>,
    Option<LifeChanged>,
    Vec<CreatureDied>,
    Option<GameEnded>,
);

fn resolve_spell_from_stack(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    terminal_state: &mut TerminalState,
    stack_object: StackObject,
) -> Result<ResolvedSpellOutcome, crate::domain::play::errors::DomainError> {
    let ResolvedSpellObject {
        source_card_id,
        controller_id,
        stack_object_id,
        payload,
        card_type,
        mana_cost_paid,
        supported_spell_rules,
        target,
    } = extract_resolved_spell_object(game_id, stack_object)?;

    let outcome =
        move_resolved_spell_to_its_destination(players, &controller_id, card_type, payload)?;

    let (stack_top_resolved, spell_cast) = build_resolution_events(
        game_id,
        &controller_id,
        &stack_object_id,
        &source_card_id,
        card_type,
        mana_cost_paid,
        outcome,
    );
    let (card_exiled, life_changed, creatures_died, game_ended) = apply_supported_spell_rules(
        game_id,
        players,
        card_locations,
        terminal_state,
        &controller_id,
        supported_spell_rules,
        target.as_ref(),
    )?;

    Ok((
        stack_top_resolved,
        Some(spell_cast),
        card_exiled,
        life_changed,
        creatures_died,
        game_ended,
    ))
}

fn resolve_activated_ability_from_stack(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    stack_object: StackObject,
) -> Result<ResolvedSpellOutcome, crate::domain::play::errors::DomainError> {
    let ResolvedActivatedAbility {
        source_card_id,
        controller_id,
        stack_object_id,
        ability,
    } = extract_resolved_activated_ability(game_id, stack_object)?;

    let stack_top_resolved = StackTopResolved::new(
        game_id.clone(),
        controller_id.clone(),
        stack_object_id,
        source_card_id,
    );

    let life_changed = match ability.effect() {
        ActivatedAbilityEffect::GainLifeToController(amount) => {
            Some(super::super::game_effects::adjust_player_life(
                game_id,
                players,
                &controller_id,
                amount.cast_signed(),
            )?)
        }
    };

    let super::super::state_based_actions::StateBasedActionsResult {
        creatures_died,
        game_ended,
    } = super::super::state_based_actions::check_state_based_actions(
        game_id,
        players,
        terminal_state,
    )?;

    Ok((
        stack_top_resolved,
        None,
        None,
        life_changed,
        creatures_died,
        game_ended,
    ))
}

pub(super) fn resolve_stack_object(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    terminal_state: &mut TerminalState,
    stack_object: StackObject,
) -> Result<ResolvedSpellOutcome, crate::domain::play::errors::DomainError> {
    match stack_object.kind() {
        StackObjectKind::Spell(_) => resolve_spell_from_stack(
            game_id,
            players,
            card_locations,
            terminal_state,
            stack_object,
        ),
        StackObjectKind::ActivatedAbility(_) => {
            resolve_activated_ability_from_stack(game_id, players, terminal_state, stack_object)
        }
    }
}
