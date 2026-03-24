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
                PriorityState, SpellOnStack, SpellTarget, StackObject, StackObjectKind, StackZone,
            },
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
    caster_id: &PlayerId,
    card_id: &CardInstanceId,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
) -> Result<(), DomainError> {
    let targeting = supported_spell_rules.targeting();
    match evaluate_target_legality(
        TargetLegalityContext::Cast { players, caster_id },
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

struct PreparedHandSpellCast {
    mana_cost_paid: u32,
    payload: crate::domain::play::cards::SpellPayload,
}

fn prepare_validated_hand_spell_for_cast(
    player: &mut crate::domain::play::game::Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
    mana_cost: u32,
    mana_cost_profile: crate::domain::play::cards::ManaCost,
) -> Result<PreparedHandSpellCast, DomainError> {
    let available_mana = player.mana();
    if !player.spend_mana_cost(mana_cost_profile) {
        return Err(DomainError::Game(GameError::InsufficientMana {
            player: player_id.clone(),
            required: mana_cost,
            available: available_mana,
        }));
    }

    let payload = helpers::remove_card_from_hand(player, player_id, card_id)?.into_spell_payload();

    Ok(PreparedHandSpellCast {
        mana_cost_paid: mana_cost,
        payload,
    })
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
        &player_id,
        &card_id,
        supported_spell_rules,
        target.as_ref(),
    )?;

    if matches!(card_type, CardType::Creature) && !has_creature_stats {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            format!("creature card {card_id} must have power and toughness"),
        )));
    }

    let PreparedHandSpellCast {
        mana_cost_paid,
        payload,
    } = prepare_validated_hand_spell_for_cast(
        &mut players[player_idx],
        &player_id,
        &card_id,
        mana_cost,
        mana_cost_profile,
    )?;

    let stack_object_number = stack.next_object_number();
    let stack_object_id = stack.object_id(game_id, stack_object_number);
    stack.push(StackObject::new(
        stack_object_number,
        player_id.clone(),
        StackObjectKind::Spell(SpellOnStack::new(payload, mana_cost_paid, target.clone())),
    ));

    *priority = Some(PriorityState::opened(player_id.clone()));

    Ok(CastSpellOutcome {
        spell_put_on_stack: SpellPutOnStack::new(
            game_id.clone(),
            player_id,
            card_id,
            card_type,
            mana_cost,
            stack_object_id,
            target,
        ),
    })
}
