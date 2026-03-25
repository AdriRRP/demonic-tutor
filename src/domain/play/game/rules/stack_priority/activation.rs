//! Supports rules stack priority activation.

use {
    super::{ActivateAbilityOutcome, StackPriorityContext},
    crate::domain::play::{
        cards::{ActivatedAbilitySacrificeCost, CardType, SpellTargetingProfile},
        commands::ActivateAbilityCommand,
        errors::{CardError, DomainError, GameError},
        events::ActivatedAbilityPutOnStack,
        events::CreatureDied,
        game::{
            helpers, invariants,
            model::{
                ActivatedAbilityOnStack, PriorityState, StackCardRef, StackObject, StackObjectKind,
                StackTargetRef,
            },
        },
        ids::CardInstanceId,
    },
};

struct PreparedActivationSource {
    handle: StackCardRef,
    source_card_core: u64,
    public_source_card_id: CardInstanceId,
    ability: crate::domain::play::cards::ActivatedAbilityProfile,
    card_type: CardType,
    loyalty: Option<u32>,
}

fn prepare_activation_source(
    players: &[crate::domain::play::game::Player],
    player_index: usize,
    player_id: &crate::domain::play::ids::PlayerId,
    source_card_id: &CardInstanceId,
) -> Result<PreparedActivationSource, DomainError> {
    let handle = players[player_index]
        .battlefield_handle(source_card_id)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: player_id.clone(),
                card: source_card_id.clone(),
            })
        })?;
    let card = players[player_index]
        .card_by_handle(handle)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: player_id.clone(),
                card: source_card_id.clone(),
            })
        })?;

    Ok(PreparedActivationSource {
        handle: StackCardRef::new(player_index, handle),
        source_card_core: source_card_id.core_u64(),
        public_source_card_id: source_card_id.clone(),
        ability: card.activated_ability().ok_or_else(|| {
            DomainError::Card(CardError::NoActivatedAbility(source_card_id.clone()))
        })?,
        card_type: *card.card_type(),
        loyalty: card.loyalty(),
    })
}

fn validate_loyalty_timing(
    source_card_id: &CardInstanceId,
    player_id: &crate::domain::play::ids::PlayerId,
    active_player: &crate::domain::play::ids::PlayerId,
    phase: crate::domain::play::phase::Phase,
    stack: &crate::domain::play::game::model::StackZone,
) -> Result<(), DomainError> {
    if player_id != active_player
        || !matches!(
            phase,
            crate::domain::play::phase::Phase::FirstMain
                | crate::domain::play::phase::Phase::SecondMain
        )
        || !stack.is_empty()
    {
        return Err(DomainError::Game(
            GameError::ActivatedAbilityTimingNotAllowed {
                card: source_card_id.clone(),
            },
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn pay_activation_costs(
    players: &mut [crate::domain::play::game::Player],
    game_id: &crate::domain::play::ids::GameId,
    player_index: usize,
    player_id: crate::domain::play::ids::PlayerId,
    source_card_id: CardInstanceId,
    source_handle: StackCardRef,
    source_loyalty: Option<u32>,
    ability: crate::domain::play::cards::ActivatedAbilityProfile,
) -> Result<(Vec<CreatureDied>, Vec<CardInstanceId>), DomainError> {
    let mut creatures_died = Vec::new();
    let mut moved_cards = Vec::new();
    if ability.loyalty_change().is_negative() {
        let available = source_loyalty.unwrap_or(0);
        let required = ability.loyalty_change().unsigned_abs();
        if available < required {
            return Err(DomainError::Game(GameError::InsufficientLoyalty {
                card: source_card_id,
                required,
                available,
            }));
        }
    }
    let available_mana = players[player_index].mana();
    let mana_cost = ability.mana_cost();
    if ability.mana_value() > 0 && !players[player_index].mana_pool().clone().spend(mana_cost) {
        return Err(DomainError::Game(GameError::InsufficientMana {
            player: player_id,
            required: ability.mana_value(),
            available: available_mana,
        }));
    }

    if ability.requires_tap() {
        if players[player_index]
            .card_by_handle(source_handle.handle())
            .is_some_and(crate::domain::play::cards::CardInstance::is_tapped)
        {
            return Err(DomainError::Card(CardError::AlreadyTapped {
                player: player_id,
                card: source_card_id,
            }));
        }
        let card = players[player_index]
            .card_mut_by_handle(source_handle.handle())
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: player_id.clone(),
                    card: source_card_id.clone(),
                })
            })?;
        card.tap();
    }

    if ability.mana_value() > 0 {
        let spent = players[player_index].spend_mana_cost(mana_cost);
        debug_assert!(
            spent,
            "validated activation mana cost should remain payable"
        );
    }

    if ability.loyalty_change() != 0 {
        let card = players[player_index]
            .card_mut_by_handle(source_handle.handle())
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: player_id.clone(),
                    card: source_card_id.clone(),
                })
            })?;
        let changed = card.adjust_loyalty(ability.loyalty_change());
        debug_assert!(changed, "validated loyalty change should remain payable");
        let marked = card.mark_loyalty_ability_activated();
        debug_assert!(
            marked,
            "validated loyalty activation should only happen once per turn"
        );
    }

    if matches!(
        ability.sacrifice_cost(),
        Some(ActivatedAbilitySacrificeCost::Source)
    ) {
        let source_type = players[player_index]
            .card_by_handle(source_handle.handle())
            .map(|card| *card.card_type())
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: player_id.clone(),
                    card: source_card_id.clone(),
                })
            })?;
        players[player_index]
            .move_battlefield_handle_to_graveyard(source_handle.handle())
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: player_id.clone(),
                    card: source_card_id.clone(),
                })
            })?;
        moved_cards.push(source_card_id.clone());
        if matches!(source_type, CardType::Creature) {
            creatures_died.push(CreatureDied::new(
                game_id.clone(),
                player_id,
                source_card_id,
            ));
        }
    }

    Ok((creatures_died, moved_cards))
}

fn prepare_ability_target(
    players: &[crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target: Option<&crate::domain::play::game::SpellTarget>,
) -> Result<Option<StackTargetRef>, DomainError> {
    match target {
        None => Ok(None),
        Some(crate::domain::play::game::SpellTarget::Player(player_id)) => Ok(Some(
            StackTargetRef::Player(helpers::find_player_index(players, player_id)?),
        )),
        Some(crate::domain::play::game::SpellTarget::Creature(card_id)) => {
            let location = card_locations.location(card_id).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "ability target creature {card_id} disappeared before stack insertion"
                )))
            })?;
            Ok(Some(StackTargetRef::Creature(StackCardRef::new(
                location.owner_index(),
                location.handle(),
            ))))
        }
        Some(crate::domain::play::game::SpellTarget::Permanent(card_id)) => {
            let location = card_locations.location(card_id).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "ability target permanent {card_id} disappeared before stack insertion"
                )))
            })?;
            Ok(Some(StackTargetRef::Permanent(StackCardRef::new(
                location.owner_index(),
                location.handle(),
            ))))
        }
        Some(crate::domain::play::game::SpellTarget::GraveyardCard(card_id)) => {
            let location = card_locations.location(card_id).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "ability target graveyard card {card_id} disappeared before stack insertion"
                )))
            })?;
            Ok(Some(StackTargetRef::GraveyardCard(StackCardRef::new(
                location.owner_index(),
                location.handle(),
            ))))
        }
        Some(crate::domain::play::game::SpellTarget::StackObject(stack_object_id)) => Ok(Some(
            StackTargetRef::StackSpell(stack_object_id.object_number().ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "invalid stack object target identifier {stack_object_id}"
                )))
            })?),
        )),
    }
}

fn validate_activation_target(
    players: &[crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    stack: &crate::domain::play::game::model::StackZone,
    controller_index: usize,
    source_card_id: &CardInstanceId,
    targeting: SpellTargetingProfile,
    target: Option<&crate::domain::play::game::SpellTarget>,
) -> Result<(), DomainError> {
    use crate::domain::play::game::rules::stack_priority::spell_effects::{
        evaluate_target_legality, SpellTargetLegality, TargetLegalityContext,
    };

    match evaluate_target_legality(
        TargetLegalityContext::Cast {
            players,
            card_locations,
            stack,
            actor_index: controller_index,
        },
        targeting,
        target,
    ) {
        SpellTargetLegality::NoTargetRequired | SpellTargetLegality::Legal => Ok(()),
        SpellTargetLegality::MissingRequiredTarget => Err(DomainError::Game(
            GameError::MissingSpellTarget(source_card_id.clone()),
        )),
        SpellTargetLegality::IllegalTargetKind | SpellTargetLegality::IllegalTargetRule => Err(
            DomainError::Game(GameError::IllegalSpellTarget(source_card_id.clone())),
        ),
        SpellTargetLegality::MissingPlayer(player_id) => {
            Err(DomainError::Game(GameError::InvalidPlayerTarget(player_id)))
        }
        SpellTargetLegality::MissingCreature(card_id) => {
            Err(DomainError::Game(GameError::InvalidCreatureTarget(card_id)))
        }
        SpellTargetLegality::MissingPermanent(card_id) => Err(DomainError::Game(
            GameError::InvalidPermanentTarget(card_id),
        )),
        SpellTargetLegality::MissingGraveyardCard(card_id) => Err(DomainError::Game(
            GameError::InvalidGraveyardCardTarget(card_id),
        )),
        SpellTargetLegality::MissingStackSpell(object_id) => Err(DomainError::Game(
            GameError::InvalidStackObjectTarget(object_id),
        )),
    }
}

/// Puts a supported non-mana activated ability from the battlefield onto the stack.
///
/// # Errors
/// Returns an error if no priority window is open, if the caller does not
/// hold priority, if the source permanent is not on the battlefield under that
/// player's control, if the permanent has no supported activated ability, or
/// if a tap-cost activation tries to use an already tapped permanent.
pub fn activate_ability(
    ctx: StackPriorityContext<'_>,
    cmd: ActivateAbilityCommand,
) -> Result<ActivateAbilityOutcome, DomainError> {
    let StackPriorityContext {
        game_id,
        players,
        card_locations,
        active_player,
        phase,
        stack,
        priority,
        ..
    } = ctx;

    let ActivateAbilityCommand {
        player_id,
        source_card_id,
        target,
    } = cmd;

    invariants::require_priority_holder(priority.as_ref(), &player_id)?;
    let player_index = helpers::find_player_index(players, &player_id)?;
    let prepared = prepare_activation_source(players, player_index, &player_id, &source_card_id)?;
    validate_activation_target(
        players,
        card_locations,
        stack,
        player_index,
        &source_card_id,
        prepared.ability.targeting(),
        target.as_ref(),
    )?;
    if prepared.ability.loyalty_change() != 0 {
        if prepared.card_type != CardType::Planeswalker {
            return Err(DomainError::Game(
                GameError::ActivatedAbilityTimingNotAllowed {
                    card: source_card_id,
                },
            ));
        }
        let _ = prepared.loyalty;
        validate_loyalty_timing(&source_card_id, &player_id, active_player, *phase, stack)?;
        if players[player_index]
            .card_by_handle(prepared.handle.handle())
            .is_some_and(
                crate::domain::play::cards::CardInstance::loyalty_ability_activated_this_turn,
            )
        {
            return Err(DomainError::Game(
                GameError::ActivatedAbilityTimingNotAllowed {
                    card: source_card_id,
                },
            ));
        }
    }
    let prepared_target = prepare_ability_target(players, card_locations, target.as_ref())?;
    let (creatures_died, moved_cards) = pay_activation_costs(
        players,
        game_id,
        player_index,
        player_id.clone(),
        source_card_id,
        prepared.handle,
        prepared.loyalty,
        prepared.ability,
    )?;
    let stack_object_number = stack.next_object_number();
    stack.push(StackObject::new(
        stack_object_number,
        player_index,
        StackObjectKind::ActivatedAbility(ActivatedAbilityOnStack::new(
            prepared.handle,
            prepared.source_card_core,
            prepared.ability,
            prepared_target,
        )),
    ));

    *priority = Some(PriorityState::opened(player_id.clone()));

    Ok(ActivateAbilityOutcome {
        activated_ability_put_on_stack: ActivatedAbilityPutOnStack::new(
            game_id.clone(),
            player_id,
            prepared.public_source_card_id,
            prepared.ability.effect(),
            crate::domain::play::ids::StackObjectId::for_stack_object(game_id, stack_object_number),
        ),
        creatures_died,
        moved_cards,
    })
}
