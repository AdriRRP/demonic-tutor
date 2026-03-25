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
        extract_resolved_triggered_ability, ResolvedActivatedAbility, ResolvedSpellObject,
        ResolvedTriggeredAbility,
    },
};
use crate::domain::play::{
    cards::{ActivatedAbilityEffect, TriggeredAbilityEffect, TriggeredAbilityEvent},
    events::{
        CardDiscarded, CardExiled, CreatureDied, GameEnded, LifeChanged, SpellCast,
        SpellCastOutcome, StackTopResolved, TriggeredAbilityPutOnStack,
    },
    game::{
        model::{StackCardRef, StackObject, StackObjectKind, StackTargetRef, StackZone},
        Player, SpellTarget, TerminalState,
    },
    ids::GameId,
};

type ResolvedSpellOutcome = (
    StackTopResolved,
    Vec<TriggeredAbilityPutOnStack>,
    Option<SpellCast>,
    Option<CardExiled>,
    Option<CardDiscarded>,
    Option<LifeChanged>,
    Vec<CreatureDied>,
    Vec<crate::domain::play::ids::CardInstanceId>,
    Option<GameEnded>,
);

fn enqueue_dies_triggers(
    game_id: &GameId,
    players: &[Player],
    stack: &mut StackZone,
    creatures_died: &[CreatureDied],
) -> Result<Vec<TriggeredAbilityPutOnStack>, crate::domain::play::errors::DomainError> {
    let mut events = Vec::new();

    for creature_died in creatures_died {
        let owner_index = crate::domain::play::game::helpers::find_player_index(
            players,
            &creature_died.player_id,
        )?;
        let Some(handle) = players[owner_index].resolve_public_card_handle(&creature_died.card_id)
        else {
            continue;
        };
        events.extend(super::triggers::enqueue_trigger_for_card_handle(
            game_id,
            players,
            owner_index,
            handle,
            TriggeredAbilityEvent::Dies,
            stack,
        )?);
    }

    Ok(events)
}

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
    let mut triggered_abilities_put_on_stack = Vec::new();

    if matches!(outcome, SpellCastOutcome::EnteredBattlefield) {
        let entered_handle = players[controller_index]
            .battlefield_handle(&source_card_id)
            .ok_or_else(|| {
                crate::domain::play::errors::DomainError::Game(
                    crate::domain::play::errors::GameError::InternalInvariantViolation(
                        "resolved permanent should exist on battlefield before ETB triggers"
                            .to_string(),
                    ),
                )
            })?;
        triggered_abilities_put_on_stack.extend(super::triggers::enqueue_trigger_for_card_handle(
            game_id,
            players,
            controller_index,
            entered_handle,
            TriggeredAbilityEvent::EntersBattlefield,
            stack,
        )?);
    }

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
    for card_id in &moved_cards {
        let Some((owner_index, handle)) =
            players
                .iter()
                .enumerate()
                .find_map(|(owner_index, player)| {
                    let handle = player.resolve_public_card_handle(card_id)?;
                    (player.card_zone(card_id)
                        == Some(crate::domain::play::game::PlayerCardZone::Battlefield))
                    .then_some((owner_index, handle))
                })
        else {
            continue;
        };
        triggered_abilities_put_on_stack.extend(super::triggers::enqueue_trigger_for_card_handle(
            game_id,
            players,
            owner_index,
            handle,
            TriggeredAbilityEvent::EntersBattlefield,
            stack,
        )?);
    }
    triggered_abilities_put_on_stack.extend(enqueue_dies_triggers(
        game_id,
        players,
        stack,
        &creatures_died,
    )?);

    Ok((
        stack_top_resolved,
        triggered_abilities_put_on_stack,
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
        target,
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

    let materialized_target = match target {
        Some(target_ref) => Some(materialize_spell_target(game_id, players, target_ref)?),
        None => None,
    };
    let life_changed = match (ability.effect(), materialized_target.as_ref()) {
        (ActivatedAbilityEffect::GainLifeToController(amount), _) => {
            Some(super::super::game_effects::adjust_player_life_by_index(
                game_id,
                players,
                controller_index,
                amount.cast_signed(),
            )?)
        }
        (ActivatedAbilityEffect::PutPlusOnePlusOneCounterOnSource, _) => {
            if let Some(card) =
                players[controller_index].card_mut_by_handle(source_card_ref.handle())
            {
                card.add_plus_one_plus_one_counters(1);
            }
            None
        }
        (
            ActivatedAbilityEffect::GainLifeToTargetPlayer(amount),
            Some(SpellTarget::Player(player_id)),
        ) => Some(super::super::game_effects::adjust_player_life(
            game_id,
            players,
            player_id,
            amount.cast_signed(),
        )?),
        (ActivatedAbilityEffect::GainLifeToTargetPlayer(_), _) => None,
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
        Vec::new(),
        None,
        None,
        None,
        life_changed,
        creatures_died,
        Vec::new(),
        game_ended,
    ))
}

fn resolve_triggered_ability_from_stack(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    stack_object: StackObject,
) -> Result<ResolvedSpellOutcome, crate::domain::play::errors::DomainError> {
    let ResolvedTriggeredAbility {
        stack_object_number,
        source_card_ref,
        controller_index,
        ability,
    } = extract_resolved_triggered_ability(stack_object)?;
    let controller_id = players[controller_index].id().clone();
    let source_card_id = materialize_stack_card_id(
        players,
        source_card_ref,
        "missing triggered ability source handle",
    )?;

    let stack_top_resolved = StackTopResolved::new(
        game_id.clone(),
        controller_id,
        crate::domain::play::ids::StackObjectId::for_stack_object(game_id, stack_object_number),
        source_card_id,
    );

    let life_changed = match ability.effect() {
        TriggeredAbilityEffect::GainLifeToController(amount)
        | TriggeredAbilityEffect::MayGainLifeToController(amount) => {
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
        Vec::new(),
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
        StackObjectKind::TriggeredAbility(_) => {
            resolve_triggered_ability_from_stack(game_id, players, terminal_state, stack_object)
        }
    }
}
