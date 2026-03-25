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
    events::{
        CardDiscarded, CardExiled, CreatureDied, GameEnded, LifeChanged, SpellCast,
        StackTopResolved,
    },
    game::{
        model::{StackCardRef, StackObject, StackObjectKind, StackTargetRef, StackZone},
        Player, SpellTarget, TerminalState,
    },
    ids::GameId,
};

type ResolvedSpellOutcome = (
    StackTopResolved,
    Option<SpellCast>,
    Option<CardExiled>,
    Option<CardDiscarded>,
    Option<LifeChanged>,
    Vec<CreatureDied>,
    Vec<crate::domain::play::ids::CardInstanceId>,
    Option<GameEnded>,
);

fn materialize_spell_target(
    game_id: &GameId,
    players: &[Player],
    target: StackTargetRef,
) -> Result<SpellTarget, crate::domain::play::errors::DomainError> {
    match target {
        StackTargetRef::Player(player_index) => Ok(SpellTarget::Player(
            players
                .get(player_index)
                .ok_or_else(|| {
                    crate::domain::play::errors::DomainError::Game(
                        crate::domain::play::errors::GameError::InternalInvariantViolation(
                            format!("missing player at stack target index {player_index}"),
                        ),
                    )
                })?
                .id()
                .clone(),
        )),
        StackTargetRef::Creature(card_ref) => Ok(SpellTarget::Creature(
            players
                .get(card_ref.owner_index())
                .and_then(|player| player.card_by_handle(card_ref.handle()))
                .ok_or_else(|| {
                    crate::domain::play::errors::DomainError::Game(
                        crate::domain::play::errors::GameError::InternalInvariantViolation(
                            "missing creature stack target handle".to_string(),
                        ),
                    )
                })?
                .id()
                .clone(),
        )),
        StackTargetRef::Permanent(card_ref) => Ok(SpellTarget::Permanent(
            players
                .get(card_ref.owner_index())
                .and_then(|player| player.card_by_handle(card_ref.handle()))
                .ok_or_else(|| {
                    crate::domain::play::errors::DomainError::Game(
                        crate::domain::play::errors::GameError::InternalInvariantViolation(
                            "missing permanent stack target handle".to_string(),
                        ),
                    )
                })?
                .id()
                .clone(),
        )),
        StackTargetRef::GraveyardCard(card_ref) => Ok(SpellTarget::GraveyardCard(
            players
                .get(card_ref.owner_index())
                .and_then(|player| player.card_by_handle(card_ref.handle()))
                .ok_or_else(|| {
                    crate::domain::play::errors::DomainError::Game(
                        crate::domain::play::errors::GameError::InternalInvariantViolation(
                            "missing graveyard stack target handle".to_string(),
                        ),
                    )
                })?
                .id()
                .clone(),
        )),
        StackTargetRef::StackSpell(object_number) => Ok(SpellTarget::StackObject(
            crate::domain::play::ids::StackObjectId::for_stack_object(game_id, object_number),
        )),
    }
}

fn materialize_stack_card_id(
    players: &[Player],
    card_ref: StackCardRef,
    missing_message: &str,
) -> Result<crate::domain::play::ids::CardInstanceId, crate::domain::play::errors::DomainError> {
    Ok(players
        .get(card_ref.owner_index())
        .and_then(|player| player.card_by_handle(card_ref.handle()))
        .ok_or_else(|| {
            crate::domain::play::errors::DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(
                    missing_message.to_string(),
                ),
            )
        })?
        .id()
        .clone())
}

fn resolve_spell_from_stack(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    terminal_state: &mut TerminalState,
    stack: &mut StackZone,
    stack_object: StackObject,
) -> Result<ResolvedSpellOutcome, crate::domain::play::errors::DomainError> {
    let ResolvedSpellObject {
        stack_object_number,
        controller_index,
        payload,
        mana_cost_paid,
        target,
        choice,
    } = extract_resolved_spell_object(stack_object)?;
    let target = target
        .map(|target_ref| materialize_spell_target(game_id, players, target_ref))
        .transpose()?;
    let card_type = *payload.card_type();
    let supported_spell_rules = payload.supported_spell_rules();
    let source_card_id = payload.id().clone();

    let outcome =
        move_resolved_spell_to_its_destination(players, controller_index, card_type, payload)?;
    let controller_id = players[controller_index].id().clone();

    let (stack_top_resolved, spell_cast) = build_resolution_events(
        game_id,
        &controller_id,
        stack_object_number,
        &source_card_id,
        card_type,
        mana_cost_paid,
        outcome,
    );
    let (card_exiled, card_discarded, life_changed, creatures_died, moved_cards, game_ended) =
        apply_supported_spell_rules(self::effects::ResolutionContext::new(
            game_id,
            players,
            card_locations,
            terminal_state,
            stack,
            controller_index,
            supported_spell_rules,
            target.as_ref(),
            choice,
        ))?;

    Ok((
        stack_top_resolved,
        Some(spell_cast),
        card_exiled,
        card_discarded,
        life_changed,
        creatures_died,
        moved_cards,
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
        stack_object_number,
        source_card_ref,
        controller_index,
        ability,
    } = extract_resolved_activated_ability(stack_object)?;
    let controller_id = players[controller_index].id().clone();
    let source_card_id = materialize_stack_card_id(
        players,
        source_card_ref,
        "missing activated ability source handle",
    )?;

    let stack_top_resolved = StackTopResolved::new(
        game_id.clone(),
        controller_id,
        crate::domain::play::ids::StackObjectId::for_stack_object(game_id, stack_object_number),
        source_card_id,
    );

    let life_changed = match ability.effect() {
        ActivatedAbilityEffect::GainLifeToController(amount) => {
            Some(super::super::game_effects::adjust_player_life_by_index(
                game_id,
                players,
                controller_index,
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
        None,
        life_changed,
        creatures_died,
        Vec::new(),
        game_ended,
    ))
}

pub(super) fn resolve_stack_object(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    terminal_state: &mut TerminalState,
    stack: &mut StackZone,
    stack_object: StackObject,
) -> Result<ResolvedSpellOutcome, crate::domain::play::errors::DomainError> {
    match stack_object.kind() {
        StackObjectKind::Spell(_) => resolve_spell_from_stack(
            game_id,
            players,
            card_locations,
            terminal_state,
            stack,
            stack_object,
        ),
        StackObjectKind::ActivatedAbility(_) => {
            resolve_activated_ability_from_stack(game_id, players, terminal_state, stack_object)
        }
    }
}
