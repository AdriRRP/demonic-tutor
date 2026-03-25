//! Supports rules stack priority activation.

use {
    super::{ActivateAbilityOutcome, StackPriorityContext},
    crate::domain::play::{
        cards::SpellTargetingProfile,
        commands::ActivateAbilityCommand,
        errors::{CardError, DomainError, GameError},
        events::ActivatedAbilityPutOnStack,
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
    let source_card_handle = players[player_index]
        .battlefield_handle(&source_card_id)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: player_id.clone(),
                card: source_card_id.clone(),
            })
        })?;
    let (ability, already_tapped) = {
        let card = players[player_index]
            .card_by_handle(source_card_handle)
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: player_id.clone(),
                    card: source_card_id.clone(),
                })
            })?;
        (
            card.activated_ability().ok_or_else(|| {
                DomainError::Card(CardError::NoActivatedAbility(source_card_id.clone()))
            })?,
            card.is_tapped(),
        )
    };
    validate_activation_target(
        players,
        card_locations,
        stack,
        player_index,
        &source_card_id,
        ability.targeting(),
        target.as_ref(),
    )?;

    if ability.requires_tap() {
        if already_tapped {
            return Err(DomainError::Card(CardError::AlreadyTapped {
                player: player_id,
                card: source_card_id,
            }));
        }
        let card = players[player_index]
            .card_mut_by_handle(source_card_handle)
            .ok_or_else(|| {
                DomainError::Card(CardError::NotOnBattlefield {
                    player: player_id.clone(),
                    card: source_card_id.clone(),
                })
            })?;
        card.tap();
    }

    let prepared_target = prepare_ability_target(players, card_locations, target.as_ref())?;
    let stack_object_number = stack.next_object_number();
    stack.push(StackObject::new(
        stack_object_number,
        player_index,
        StackObjectKind::ActivatedAbility(ActivatedAbilityOnStack::new(
            StackCardRef::new(player_index, source_card_handle),
            source_card_id.core_u64(),
            ability,
            prepared_target,
        )),
    ));

    *priority = Some(PriorityState::opened(player_id.clone()));

    Ok(ActivateAbilityOutcome {
        activated_ability_put_on_stack: ActivatedAbilityPutOnStack::new(
            game_id.clone(),
            player_id,
            source_card_id,
            ability.effect(),
            crate::domain::play::ids::StackObjectId::for_stack_object(game_id, stack_object_number),
        ),
    })
}
