//! Supports rules stack priority casting.

use {
    super::{
        spell_effects::{
            evaluate_target_legality, supported_spell_rules, SpellTargetLegality,
            TargetLegalityContext,
        },
        CastSpellOutcome, StackPriorityContext,
    },
    crate::domain::play::{
        cards::{CardType, CastingPermissionProfile, CastingRule, SupportedSpellRules},
        commands::CastSpellCommand,
        errors::{CardError, DomainError, GameError, PhaseError},
        events::SpellPutOnStack,
        game::{
            helpers, invariants,
            model::{
                PrepareHandSpellCastError, PriorityState, SpellOnStack, StackCardRef, StackObject,
                StackObjectKind, StackTargetRef, StackZone,
            },
            SpellTarget,
        },
        ids::{CardInstanceId, PlayerId},
        phase::Phase,
    },
};

fn require_cast_timing(
    active_player: &PlayerId,
    phase: Phase,
    stack: &StackZone,
    priority: Option<&PriorityState>,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
    casting_permission: CastingPermissionProfile,
) -> Result<(), DomainError> {
    if let Some(priority) = priority {
        invariants::require_priority_holder(Some(priority), player_id)?;

        if casting_permission.supports(CastingRule::OpenPriorityWindow) {
            return Ok(());
        }

        if casting_permission.supports(CastingRule::OpenPriorityWindowDuringOwnTurn)
            && player_id == active_player
        {
            return Ok(());
        }

        let active_player_in_empty_main_phase_window = stack.is_empty()
            && player_id == active_player
            && matches!(phase, Phase::FirstMain | Phase::SecondMain);
        if casting_permission.supports(CastingRule::ActivePlayerEmptyMainPhaseWindow)
            && active_player_in_empty_main_phase_window
        {
            return Ok(());
        }

        return Err(DomainError::Game(GameError::CastingTimingNotAllowed {
            card: card_id.clone(),
            permission: casting_permission,
        }));
    }

    invariants::require_active_player(active_player, player_id)?;

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(PhaseError::InvalidForPlayingCard {
            phase,
        }));
    }

    Ok(())
}

fn validate_spell_target(
    players: &[crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    caster_index: usize,
    card_id: &CardInstanceId,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
) -> Result<(), DomainError> {
    let targeting = supported_spell_rules.targeting();
    match evaluate_target_legality(
        TargetLegalityContext::Cast {
            players,
            card_locations,
            actor_index: caster_index,
        },
        targeting,
        target,
    ) {
        SpellTargetLegality::NoTargetRequired | SpellTargetLegality::Legal => Ok(()),
        SpellTargetLegality::MissingRequiredTarget => Err(DomainError::Game(
            GameError::MissingSpellTarget(card_id.clone()),
        )),
        SpellTargetLegality::IllegalTargetKind | SpellTargetLegality::IllegalTargetRule => Err(
            DomainError::Game(GameError::IllegalSpellTarget(card_id.clone())),
        ),
        SpellTargetLegality::MissingPlayer(player_id) => {
            Err(DomainError::Game(GameError::InvalidPlayerTarget(player_id)))
        }
        SpellTargetLegality::MissingCreature(target_card_id) => Err(DomainError::Game(
            GameError::InvalidCreatureTarget(target_card_id),
        )),
        SpellTargetLegality::MissingGraveyardCard(target_card_id) => Err(DomainError::Game(
            GameError::InvalidGraveyardCardTarget(target_card_id),
        )),
    }
}

struct HandSpellMetadata {
    card_type: CardType,
    casting_permission: Option<CastingPermissionProfile>,
    supported_spell_rules: SupportedSpellRules,
    has_creature_stats: bool,
    mana_cost: u32,
    mana_cost_profile: crate::domain::play::cards::ManaCost,
}

struct PreparedStackSpellObject {
    card_type: CardType,
    mana_cost: u32,
    spell: SpellOnStack,
}

impl PreparedStackSpellObject {
    fn into_stack_object(self, number: u32, controller_index: usize) -> StackObject {
        StackObject::new(number, controller_index, StackObjectKind::Spell(self.spell))
    }
}

fn prepare_stack_target(
    players: &[crate::domain::play::game::Player],
    card_locations: &crate::domain::play::game::AggregateCardLocationIndex,
    target: Option<&SpellTarget>,
) -> Result<Option<StackTargetRef>, DomainError> {
    match target {
        None => Ok(None),
        Some(SpellTarget::Player(player_id)) => Ok(Some(StackTargetRef::Player(
            helpers::find_player_index(players, player_id)?,
        ))),
        Some(SpellTarget::Creature(card_id)) => {
            let location = card_locations.location(card_id).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "target creature {card_id} disappeared before stack insertion"
                )))
            })?;
            Ok(Some(StackTargetRef::Creature(StackCardRef::new(
                location.owner_index(),
                location.handle(),
            ))))
        }
        Some(SpellTarget::GraveyardCard(card_id)) => {
            let location = card_locations.location(card_id).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "target graveyard card {card_id} disappeared before stack insertion"
                )))
            })?;
            Ok(Some(StackTargetRef::GraveyardCard(StackCardRef::new(
                location.owner_index(),
                location.handle(),
            ))))
        }
    }
}

fn read_hand_spell_metadata(
    player: &crate::domain::play::game::Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<HandSpellMetadata, DomainError> {
    let hand_card = helpers::hand_card(player, player_id, card_id)?;
    let card_type = *hand_card.card_type();
    let casting_permission = if card_type.is_land() {
        None
    } else {
        Some(hand_card.casting_permission_profile().ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "spell card {} must define casting permission",
                hand_card.id()
            )))
        })?)
    };

    Ok(HandSpellMetadata {
        card_type,
        casting_permission,
        supported_spell_rules: supported_spell_rules(hand_card),
        has_creature_stats: hand_card.creature_stats().is_some(),
        mana_cost: hand_card.mana_cost(),
        mana_cost_profile: hand_card.mana_cost_profile(),
    })
}

fn prepare_stack_spell_object(
    prepared_cast: crate::domain::play::game::model::PreparedHandSpellCast,
    card_type: CardType,
    mana_cost: u32,
    target: Option<StackTargetRef>,
) -> PreparedStackSpellObject {
    PreparedStackSpellObject {
        card_type,
        mana_cost,
        spell: SpellOnStack::new(prepared_cast.into_payload(), mana_cost, target),
    }
}

/// Puts a spell card from hand onto the stack and opens a priority window.
///
/// # Errors
/// Returns an error if the player is not allowed to cast now, if the card is
/// not a spell card in that player's hand, if mana is insufficient, or if the
/// current priority holder does not match the command issuer.
pub fn cast_spell(
    ctx: StackPriorityContext<'_>,
    cmd: CastSpellCommand,
) -> Result<CastSpellOutcome, DomainError> {
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

    let CastSpellCommand {
        player_id,
        card_id,
        target,
    } = cmd;

    let player_idx = helpers::find_player_index(players, &player_id)?;
    let HandSpellMetadata {
        card_type,
        casting_permission,
        supported_spell_rules,
        has_creature_stats,
        mana_cost,
        mana_cost_profile,
    } = read_hand_spell_metadata(&players[player_idx], &player_id, &card_id)?;
    if card_type.is_land() {
        return Err(DomainError::Card(CardError::CannotCastLand(card_id)));
    }
    let casting_permission = casting_permission.ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "spell card {card_id} must define casting permission"
        )))
    })?;

    require_cast_timing(
        active_player,
        *phase,
        stack,
        priority.as_ref(),
        &player_id,
        &card_id,
        casting_permission,
    )?;

    validate_spell_target(
        players,
        card_locations,
        player_idx,
        &card_id,
        supported_spell_rules,
        target.as_ref(),
    )?;

    if matches!(card_type, CardType::Creature) && !has_creature_stats {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            format!("creature card {card_id} must have power and toughness"),
        )));
    }

    let prepared_cast =
        match players[player_idx].prepare_hand_spell_cast(&card_id, mana_cost, mana_cost_profile) {
            Ok(prepared) => prepared,
            Err(PrepareHandSpellCastError::MissingCard) => {
                return Err(DomainError::Card(CardError::NotInHand {
                    player: player_id,
                    card: card_id,
                }))
            }
            Err(PrepareHandSpellCastError::InsufficientMana { available }) => {
                return Err(DomainError::Game(GameError::InsufficientMana {
                    player: player_id,
                    required: mana_cost,
                    available,
                }))
            }
        };
    let prepared_stack_target = prepare_stack_target(players, card_locations, target.as_ref())?;
    let prepared_stack_spell =
        prepare_stack_spell_object(prepared_cast, card_type, mana_cost, prepared_stack_target);

    let spell_card_type = prepared_stack_spell.card_type;
    let spell_mana_cost = prepared_stack_spell.mana_cost;
    let stack_object_number = stack.next_object_number();
    stack.push(prepared_stack_spell.into_stack_object(stack_object_number, player_idx));

    *priority = Some(PriorityState::opened(player_id.clone()));

    Ok(CastSpellOutcome {
        spell_put_on_stack: SpellPutOnStack::new(
            game_id.clone(),
            player_id,
            card_id,
            spell_card_type,
            spell_mana_cost,
            crate::domain::play::ids::StackObjectId::for_stack_object(game_id, stack_object_number),
            target,
        ),
    })
}
