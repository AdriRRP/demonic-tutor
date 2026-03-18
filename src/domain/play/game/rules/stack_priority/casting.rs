use super::{CastSpellOutcome, StackPriorityContext};
use crate::domain::play::{
    cards::CardType,
    commands::CastSpellCommand,
    errors::{CardError, DomainError, GameError, PhaseError},
    events::SpellPutOnStack,
    game::{
        invariants,
        model::{PriorityState, SpellOnStack, StackObject, StackObjectKind, StackZone},
    },
    ids::{CardInstanceId, GameId, PlayerId, StackObjectId},
    phase::Phase,
};

fn next_stack_object_id(game_id: &GameId, next_stack_object_number: &mut u32) -> StackObjectId {
    let id = StackObjectId::new(format!(
        "{}-stack-{}",
        game_id.as_str(),
        *next_stack_object_number
    ));
    *next_stack_object_number += 1;
    id
}

fn require_cast_timing(
    active_player: &PlayerId,
    phase: Phase,
    stack: &StackZone,
    priority: Option<&PriorityState>,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
    card_type: &CardType,
) -> Result<(), DomainError> {
    if let Some(priority) = priority {
        invariants::require_priority_holder(Some(priority), player_id)?;

        if card_type.is_instant() {
            return Ok(());
        }

        let active_player_in_empty_main_phase_window = stack.is_empty()
            && player_id == active_player
            && matches!(phase, Phase::FirstMain | Phase::SecondMain);
        if card_type.is_sorcery_speed_spell() && active_player_in_empty_main_phase_window {
            return Ok(());
        }

        return Err(DomainError::Game(
            GameError::OnlyInstantSpellsSupportedAsResponses(card_id.clone()),
        ));
    }

    invariants::require_active_player(active_player, player_id)?;

    if !matches!(phase, Phase::FirstMain | Phase::SecondMain) {
        return Err(DomainError::Phase(PhaseError::InvalidForPlayingCard {
            phase,
        }));
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
        next_stack_object_number,
        ..
    } = ctx;

    let CastSpellCommand { player_id, card_id } = cmd;

    let player = invariants::find_player_mut(players, &player_id)?;
    let card_type = invariants::hand_card_type(player, &player_id, &card_id)?;

    if card_type.is_land() {
        return Err(DomainError::Card(CardError::CannotCastLand(card_id)));
    }

    require_cast_timing(
        active_player,
        *phase,
        stack,
        priority.as_ref(),
        &player_id,
        &card_id,
        &card_type,
    )?;

    let hand_card = invariants::hand_card(player, &player_id, &card_id)?;
    if matches!(card_type, CardType::Creature) && hand_card.creature_stats().is_none() {
        return Err(DomainError::Game(GameError::InternalInvariantViolation(
            format!(
                "creature card {} must have power and toughness",
                hand_card.id()
            ),
        )));
    }

    let mana_cost = hand_card.mana_cost();
    if player.mana() < mana_cost {
        return Err(DomainError::Game(GameError::InsufficientMana {
            player: player_id.clone(),
            required: mana_cost,
            available: player.mana(),
        }));
    }

    let card = invariants::remove_card_from_hand(player, &player_id, &card_id)?;
    let spent = player.spend_mana(mana_cost);
    debug_assert!(spent, "mana was checked before removing the card from hand");

    let stack_object_id = next_stack_object_id(game_id, next_stack_object_number);
    stack.push(StackObject::new(
        stack_object_id.clone(),
        player_id.clone(),
        card_id.clone(),
        StackObjectKind::Spell(SpellOnStack::new(card, mana_cost)),
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
        ),
    })
}
