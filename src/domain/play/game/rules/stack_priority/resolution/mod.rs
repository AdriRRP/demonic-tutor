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
    cards::{
        ActivatedAbilityEffect, CardType, SpellPayload, SpellResolutionProfile,
        TriggeredAbilityEffect, TriggeredAbilityEvent,
    },
    events::{
        CardDiscarded, CardExiled, CreatureDied, GameEnded, LifeChanged, SpellCast,
        SpellCastOutcome, StackTopResolved, TriggeredAbilityPutOnStack,
    },
    game::{
        model::{StackCardRef, StackObject, StackObjectKind, StackTargetRef, StackZone},
        AggregateCardLocationIndex, Player, SpellTarget, TerminalState,
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

struct SpellDestinationContext<'a> {
    players: &'a mut [Player],
    card_locations: &'a AggregateCardLocationIndex,
    stack: &'a StackZone,
    controller_index: usize,
    card_type: CardType,
    supported_spell_rules: crate::domain::play::cards::SupportedSpellRules,
    target: Option<&'a SpellTarget>,
}

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
                .get(card_ref.player_index())
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
                .get(card_ref.player_index())
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
                .get(card_ref.player_index())
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
        .get(card_ref.player_index())
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

fn move_resolved_spell_to_destination(
    game_id: &GameId,
    context: &mut SpellDestinationContext<'_>,
    payload: SpellPayload,
) -> Result<(SpellCastOutcome, Option<CardExiled>), crate::domain::play::errors::DomainError> {
    if matches!(
        (
            context.card_type,
            context.supported_spell_rules.resolution()
        ),
        (
            CardType::Enchantment,
            SpellResolutionProfile::AttachToTargetCreature
        )
    ) {
        return move_resolved_aura_to_its_destination(
            context.players,
            context.card_locations,
            context.stack,
            context.controller_index,
            context.supported_spell_rules,
            context.target,
            payload,
        )
        .map(|outcome| (outcome, None));
    }

    move_resolved_spell_to_its_destination(
        game_id,
        context.players,
        context.controller_index,
        context.card_type,
        payload,
    )
}

fn apply_attached_aura_effects(
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    controller_index: usize,
    aura_id: &crate::domain::play::ids::CardInstanceId,
) -> Result<(), crate::domain::play::errors::DomainError> {
    let Some(card) = players[controller_index].battlefield_card(aura_id) else {
        return Ok(());
    };
    let Some(target_id) = card.attached_to().cloned() else {
        return Ok(());
    };
    let attached_stat_boost = card.attached_stat_boost();
    let attached_combat_restriction = card.attached_combat_restriction();

    if attached_stat_boost.is_none() && attached_combat_restriction.is_none() {
        return Ok(());
    }

    let target_location = card_locations.location(&target_id).ok_or_else(|| {
        crate::domain::play::errors::DomainError::Game(
            crate::domain::play::errors::GameError::InternalInvariantViolation(format!(
                "missing attached aura target {target_id} during aura resolution"
            )),
        )
    })?;
    let target = players[target_location.player_index()]
        .card_mut_by_handle(target_location.handle())
        .ok_or_else(|| {
            crate::domain::play::errors::DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(format!(
                    "missing attached aura target handle {} during aura resolution",
                    target_location.handle().index()
                )),
            )
        })?;

    if let Some(attached_stat_boost) = attached_stat_boost {
        target
            .add_attached_stat_bonus(attached_stat_boost.power(), attached_stat_boost.toughness());
    }
    if attached_combat_restriction.is_some() {
        target.add_attached_cant_attack_or_block();
    }

    Ok(())
}

fn move_resolved_aura_to_its_destination(
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    stack: &StackZone,
    controller_index: usize,
    supported_spell_rules: crate::domain::play::cards::SupportedSpellRules,
    target: Option<&SpellTarget>,
    payload: SpellPayload,
) -> Result<SpellCastOutcome, crate::domain::play::errors::DomainError> {
    let legality = super::spell_effects::evaluate_target_legality(
        super::spell_effects::TargetLegalityContext::Resolution {
            players,
            card_locations,
            stack,
            actor_index: controller_index,
        },
        supported_spell_rules.targeting(),
        target,
    );

    match (legality, target) {
        (
            super::spell_effects::SpellTargetLegality::Legal,
            Some(SpellTarget::Creature(target_id)),
        ) => {
            let mut permanent = payload.into_card_instance();
            permanent.attach_to(target_id.clone());
            let aura_id = permanent.id().clone();
            players[controller_index]
                .receive_battlefield_card(permanent)
                .ok_or_else(|| {
                    crate::domain::play::errors::DomainError::Game(
                        crate::domain::play::errors::GameError::InternalInvariantViolation(
                            "failed to place resolved aura on the battlefield".to_string(),
                        ),
                    )
                })?;
            apply_attached_aura_effects(players, card_locations, controller_index, &aura_id)?;
            Ok(SpellCastOutcome::EnteredBattlefield)
        }
        (super::spell_effects::SpellTargetLegality::Legal, Some(_)) => {
            Err(crate::domain::play::errors::DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(
                    "aura spell resolved with a non-creature target".to_string(),
                ),
            ))
        }
        (
            super::spell_effects::SpellTargetLegality::IllegalTargetKind
            | super::spell_effects::SpellTargetLegality::IllegalTargetRule
            | super::spell_effects::SpellTargetLegality::MissingPlayer(_)
            | super::spell_effects::SpellTargetLegality::MissingCreature(_)
            | super::spell_effects::SpellTargetLegality::MissingPermanent(_)
            | super::spell_effects::SpellTargetLegality::MissingGraveyardCard(_)
            | super::spell_effects::SpellTargetLegality::MissingStackSpell(_),
            _,
        ) => {
            players[controller_index].receive_graveyard_card(payload.into_card_instance());
            Ok(SpellCastOutcome::ResolvedToGraveyard)
        }
        (super::spell_effects::SpellTargetLegality::MissingRequiredTarget, _) => {
            Err(crate::domain::play::errors::DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(
                    "aura spell resolved without a required target".to_string(),
                ),
            ))
        }
        (super::spell_effects::SpellTargetLegality::NoTargetRequired, _) => {
            Err(crate::domain::play::errors::DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(
                    "aura spell resolved without a targeting profile".to_string(),
                ),
            ))
        }
        (super::spell_effects::SpellTargetLegality::Legal, None) => {
            Err(crate::domain::play::errors::DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(
                    "aura spell resolved without a materialized target".to_string(),
                ),
            ))
        }
    }
}

fn enqueue_spell_etb_triggers(
    game_id: &GameId,
    players: &[Player],
    stack: &mut StackZone,
    controller_index: usize,
    outcome: &SpellCastOutcome,
    source_card_id: &crate::domain::play::ids::CardInstanceId,
) -> Result<Vec<TriggeredAbilityPutOnStack>, crate::domain::play::errors::DomainError> {
    if !matches!(outcome, SpellCastOutcome::EnteredBattlefield) {
        return Ok(Vec::new());
    }

    let entered_handle = players[controller_index]
        .battlefield_handle(source_card_id)
        .ok_or_else(|| {
            crate::domain::play::errors::DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(
                    "resolved permanent should exist on battlefield before ETB triggers"
                        .to_string(),
                ),
            )
        })?;

    super::triggers::enqueue_trigger_for_card_handle(
        game_id,
        players,
        controller_index,
        entered_handle,
        TriggeredAbilityEvent::EntersBattlefield,
        stack,
    )
}

fn enqueue_entered_battlefield_triggers_for_moved_cards(
    game_id: &GameId,
    players: &[Player],
    stack: &mut StackZone,
    moved_cards: &[crate::domain::play::ids::CardInstanceId],
) -> Result<Vec<TriggeredAbilityPutOnStack>, crate::domain::play::errors::DomainError> {
    let mut events = Vec::new();

    for card_id in moved_cards {
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
        events.extend(super::triggers::enqueue_trigger_for_card_handle(
            game_id,
            players,
            owner_index,
            handle,
            TriggeredAbilityEvent::EntersBattlefield,
            stack,
        )?);
    }

    Ok(events)
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

    let (outcome, resolved_spell_card_exiled) = move_resolved_spell_to_destination(
        game_id,
        &mut SpellDestinationContext {
            players,
            card_locations,
            stack,
            controller_index,
            card_type,
            supported_spell_rules,
            target: target.as_ref(),
        },
        payload,
    )?;
    let controller_id = players[controller_index].id().clone();
    let mut triggered_abilities_put_on_stack = enqueue_spell_etb_triggers(
        game_id,
        players,
        stack,
        controller_index,
        &outcome,
        &source_card_id,
    )?;

    let (stack_top_resolved, spell_cast) = build_resolution_events(
        game_id,
        &controller_id,
        stack_object_number,
        &source_card_id,
        card_type,
        mana_cost_paid,
        outcome,
    );
    let (effect_card_exiled, card_discarded, life_changed, creatures_died, moved_cards, game_ended) =
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
    let card_exiled = effect_card_exiled.or(resolved_spell_card_exiled);
    triggered_abilities_put_on_stack.extend(enqueue_entered_battlefield_triggers_for_moved_cards(
        game_id,
        players,
        stack,
        &moved_cards,
    )?);
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
    card_locations: &AggregateCardLocationIndex,
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
        Some(card_locations),
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
    card_locations: &AggregateCardLocationIndex,
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
        Some(card_locations),
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
        StackObjectKind::ActivatedAbility(_) => resolve_activated_ability_from_stack(
            game_id,
            players,
            card_locations,
            terminal_state,
            stack_object,
        ),
        StackObjectKind::TriggeredAbility(_) => resolve_triggered_ability_from_stack(
            game_id,
            players,
            card_locations,
            terminal_state,
            stack_object,
        ),
    }
}
