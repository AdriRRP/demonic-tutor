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
        commands::{CastSpellCommand, SpellChoice},
        errors::{CardError, DomainError, GameError, PhaseError},
        events::SpellPutOnStack,
        game::{
            helpers, invariants,
            model::{
                PrepareHandSpellCastError, PriorityState, SpellOnStack, StackCardRef, StackObject,
                StackObjectKind, StackSpellChoice, StackTargetRef, StackZone,
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
    stack: &StackZone,
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
            stack,
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
        SpellTargetLegality::MissingPermanent(target_card_id) => Err(DomainError::Game(
            GameError::InvalidPermanentTarget(target_card_id),
        )),
        SpellTargetLegality::MissingGraveyardCard(target_card_id) => Err(DomainError::Game(
            GameError::InvalidGraveyardCardTarget(target_card_id),
        )),
        SpellTargetLegality::MissingStackSpell(target_stack_object_id) => Err(DomainError::Game(
            GameError::InvalidStackObjectTarget(target_stack_object_id),
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
    from_graveyard: bool,
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
    _stack: &StackZone,
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
        Some(SpellTarget::Permanent(card_id)) => {
            let location = card_locations.location(card_id).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "target permanent {card_id} disappeared before stack insertion"
                )))
            })?;
            Ok(Some(StackTargetRef::Permanent(StackCardRef::new(
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
        Some(SpellTarget::StackObject(stack_object_id)) => Ok(Some(StackTargetRef::StackSpell(
            stack_object_id.object_number().ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "invalid stack object target identifier {stack_object_id}"
                )))
            })?,
        ))),
    }
}

fn read_spell_metadata(
    player: &crate::domain::play::game::Player,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Result<HandSpellMetadata, DomainError> {
    let (spell_card, from_graveyard) =
        if let Ok(hand_card) = helpers::hand_card(player, player_id, card_id) {
            (hand_card, false)
        } else if let Some(graveyard_card) = player.graveyard_card(card_id) {
            let permission = graveyard_card.casting_permission_profile().ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "spell card {card_id} must define casting permission"
                )))
            })?;
            if !permission.supports(CastingRule::CastFromOwnGraveyard) {
                return Err(DomainError::Card(CardError::NotInHand {
                    player: player_id.clone(),
                    card: card_id.clone(),
                }));
            }
            (graveyard_card, true)
        } else {
            return Err(DomainError::Card(CardError::NotInHand {
                player: player_id.clone(),
                card: card_id.clone(),
            }));
        };
    let card_type = *spell_card.card_type();
    let casting_permission = if card_type.is_land() {
        None
    } else {
        Some(spell_card.casting_permission_profile().ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "spell card {} must define casting permission",
                spell_card.id()
            )))
        })?)
    };

    Ok(HandSpellMetadata {
        card_type,
        casting_permission,
        supported_spell_rules: supported_spell_rules(spell_card),
        has_creature_stats: spell_card.creature_stats().is_some(),
        mana_cost: spell_card.mana_cost(),
        mana_cost_profile: spell_card.mana_cost_profile(),
        from_graveyard,
    })
}

fn prepare_stack_spell_object(
    prepared_cast: crate::domain::play::game::model::PreparedHandSpellCast,
    card_type: CardType,
    mana_cost: u32,
    target: Option<StackTargetRef>,
    choice: Option<StackSpellChoice>,
) -> PreparedStackSpellObject {
    PreparedStackSpellObject {
        card_type,
        mana_cost,
        spell: SpellOnStack::new(prepared_cast.into_payload(), mana_cost, target, choice),
    }
}

fn validate_spell_choice(
    players: &[crate::domain::play::game::Player],
    supported_spell_rules: SupportedSpellRules,
    card_id: &CardInstanceId,
    target: Option<&SpellTarget>,
    choice: Option<&SpellChoice>,
) -> Result<(), DomainError> {
    if supported_spell_rules.requires_explicit_hand_card_choice() {
        let Some(SpellChoice::HandCard(chosen_card_id)) = choice else {
            return Err(DomainError::Game(GameError::MissingSpellChoice(
                card_id.clone(),
            )));
        };

        let Some(SpellTarget::Player(target_player_id)) = target else {
            return Err(DomainError::Game(GameError::InternalInvariantViolation(
                "discard spell choice requires a player target".to_string(),
            )));
        };

        let target_player_index = helpers::find_player_index(players, target_player_id)?;
        let target_player = &players[target_player_index];
        if target_player.hand_card(chosen_card_id).is_none() {
            return Err(DomainError::Game(GameError::InvalidHandCardChoice(
                chosen_card_id.clone(),
            )));
        }

        return Ok(());
    }

    if supported_spell_rules.requires_explicit_modal_choice() {
        let Some(SpellChoice::ModalMode(_)) = choice else {
            return Err(DomainError::Game(GameError::MissingSpellChoice(
                card_id.clone(),
            )));
        };
    }

    Ok(())
}

fn prepare_stack_choice(
    players: &[crate::domain::play::game::Player],
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
    choice: Option<&SpellChoice>,
) -> Result<Option<StackSpellChoice>, DomainError> {
    if supported_spell_rules.requires_explicit_hand_card_choice() {
        let Some(SpellChoice::HandCard(chosen_card_id)) = choice else {
            return Err(DomainError::Game(GameError::InternalInvariantViolation(
                "missing hand-card spell choice during stack insertion".to_string(),
            )));
        };

        let Some(SpellTarget::Player(target_player_id)) = target else {
            return Err(DomainError::Game(GameError::InternalInvariantViolation(
                "discard spell choice requires a player target during stack insertion".to_string(),
            )));
        };

        let target_player_index = helpers::find_player_index(players, target_player_id)?;
        let handle = players[target_player_index]
            .resolve_public_card_handle(chosen_card_id)
            .ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "chosen discard card {chosen_card_id} disappeared before stack insertion"
                )))
            })?;

        return Ok(Some(StackSpellChoice::HandCard(StackCardRef::new(
            target_player_index,
            handle,
        ))));
    }

    if supported_spell_rules.requires_explicit_modal_choice() {
        let Some(SpellChoice::ModalMode(mode)) = choice else {
            return Err(DomainError::Game(GameError::InternalInvariantViolation(
                format!(
                "missing modal spell choice during stack insertion for {supported_spell_rules:?}"
            ),
            )));
        };
        return Ok(Some(StackSpellChoice::ModalMode(*mode)));
    }

    Ok(None)
}

/// Puts a spell card from hand onto the stack and opens a priority window.
///
/// # Errors
/// Returns an error if the player is not allowed to cast now, if the card is
/// not a spell card in that player's hand, if mana is insufficient, or if the
/// current priority holder does not match the command issuer.
#[allow(clippy::too_many_lines)]
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
        choice,
    } = cmd;

    let player_idx = helpers::find_player_index(players, &player_id)?;
    let HandSpellMetadata {
        card_type,
        casting_permission,
        supported_spell_rules,
        has_creature_stats,
        mana_cost,
        mana_cost_profile,
        from_graveyard,
    } = read_spell_metadata(&players[player_idx], &player_id, &card_id)?;
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
        stack,
        player_idx,
        &card_id,
        supported_spell_rules,
        target.as_ref(),
    )?;
    validate_spell_choice(
        players,
        supported_spell_rules,
        &card_id,
        target.as_ref(),
        choice.as_ref(),
    )?;

    if matches!(card_type, CardType::Creature) && !has_creature_stats {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            format!("creature card {card_id} must have power and toughness"),
        )));
    }

    let prepared_cast = match if from_graveyard {
        players[player_idx].prepare_graveyard_spell_cast(&card_id, mana_cost, mana_cost_profile)
    } else {
        players[player_idx].prepare_hand_spell_cast(&card_id, mana_cost, mana_cost_profile)
    } {
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
    let prepared_stack_target =
        prepare_stack_target(players, card_locations, stack, target.as_ref())?;
    let prepared_stack_choice = prepare_stack_choice(
        players,
        supported_spell_rules,
        target.as_ref(),
        choice.as_ref(),
    )?;
    let prepared_stack_spell = prepare_stack_spell_object(
        prepared_cast,
        card_type,
        mana_cost,
        prepared_stack_target,
        prepared_stack_choice,
    );

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
