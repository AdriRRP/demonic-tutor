use super::{
    spell_effects::{accepts_target, supported_spell_rules},
    CastSpellOutcome, StackPriorityContext,
};
use crate::domain::play::{
    cards::{CardType, CastingPermissionProfile, SupportedSpellRules},
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

        if casting_permission.allows_open_priority_window_cast() {
            return Ok(());
        }

        let active_player_in_empty_main_phase_window = stack.is_empty()
            && player_id == active_player
            && matches!(phase, Phase::FirstMain | Phase::SecondMain);
        if casting_permission.allows_active_player_empty_main_phase_cast()
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
    card_id: &CardInstanceId,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
) -> Result<(), DomainError> {
    let targeting = supported_spell_rules.targeting();

    if targeting.requires_target() {
        let Some(target) = target else {
            return Err(DomainError::Game(GameError::MissingSpellTarget(
                card_id.clone(),
            )));
        };

        if !accepts_target(targeting, target) {
            return Err(DomainError::Game(GameError::SpellDoesNotUseTargets(
                card_id.clone(),
            )));
        }

        match target {
            SpellTarget::Player(player_id) => {
                helpers::find_player_index(players, player_id)?;
            }
            SpellTarget::Creature(card_id) => {
                let found = players.iter().any(|player| {
                    player
                        .battlefield()
                        .cards()
                        .iter()
                        .any(|card| card.id() == card_id)
                });
                if !found {
                    return Err(DomainError::Game(GameError::InvalidCreatureTarget(
                        card_id.clone(),
                    )));
                }
            }
        }

        return Ok(());
    }

    if target.is_some() {
        return Err(DomainError::Game(GameError::SpellDoesNotUseTargets(
            card_id.clone(),
        )));
    }

    Ok(())
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
    let hand_card = players[player_idx]
        .hand()
        .cards()
        .iter()
        .find(|card| card.id() == &card_id)
        .cloned()
        .ok_or_else(|| {
            DomainError::Card(CardError::NotInHand {
                player: player_id.clone(),
                card: card_id.clone(),
            })
        })?;
    let card_type = hand_card.card_type().clone();
    if card_type.is_land() {
        return Err(DomainError::Card(CardError::CannotCastLand(card_id)));
    }

    let casting_permission = hand_card.casting_permission_profile().ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(format!(
            "spell card {} must define casting permission",
            hand_card.id()
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

    let supported_spell_rules = supported_spell_rules(&hand_card);
    validate_spell_target(players, &card_id, supported_spell_rules, target.as_ref())?;

    if matches!(card_type, CardType::Creature) && hand_card.creature_stats().is_none() {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            format!(
                "creature card {} must have power and toughness",
                hand_card.id()
            ),
        )));
    }

    let mana_cost = hand_card.mana_cost();
    let player = &mut players[player_idx];
    if player.mana() < mana_cost {
        return Err(DomainError::Game(GameError::InsufficientMana {
            player: player_id.clone(),
            required: mana_cost,
            available: player.mana(),
        }));
    }

    let card = helpers::remove_card_from_hand(player, &player_id, &card_id)?;
    let spent = player.spend_mana(mana_cost);
    debug_assert!(spent, "mana was checked before removing the card from hand");

    let stack_object_id = stack.next_id(game_id);
    stack.push(StackObject::new(
        stack_object_id.clone(),
        player_id.clone(),
        StackObjectKind::Spell(SpellOnStack::new(card, mana_cost, target.clone())),
    ));

    *priority = Some(PriorityState::new(player_id.clone()));

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
