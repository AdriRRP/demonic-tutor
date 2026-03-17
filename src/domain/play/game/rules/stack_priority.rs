use super::{
    super::{
        invariants,
        model::{PriorityState, SpellOnStack, StackObject, StackObjectKind, StackZone},
        Player, TerminalState,
    },
    state_based_actions::{self, StateBasedActionsResult},
};
use crate::domain::play::{
    cards::CardType,
    commands::{CastSpellCommand, PassPriorityCommand},
    errors::{CardError, DomainError, GameError, PhaseError},
    events::{
        CreatureDied, GameEnded, PriorityPassed, SpellCast, SpellCastOutcome, SpellPutOnStack,
        StackTopResolved,
    },
    ids::{GameId, PlayerId, StackObjectId},
    phase::Phase,
};

pub struct StackPriorityContext<'a> {
    pub game_id: &'a GameId,
    pub players: &'a mut [Player],
    pub active_player: &'a PlayerId,
    pub phase: &'a Phase,
    pub stack: &'a mut StackZone,
    pub priority: &'a mut Option<PriorityState>,
    pub next_stack_object_number: &'a mut u32,
    pub terminal_state: &'a mut TerminalState,
}

#[derive(Debug, Clone)]
pub struct CastSpellOutcome {
    pub spell_put_on_stack: SpellPutOnStack,
}

#[derive(Debug, Clone)]
pub struct PassPriorityOutcome {
    pub priority_passed: PriorityPassed,
    pub stack_top_resolved: Option<StackTopResolved>,
    pub spell_cast: Option<SpellCast>,
    pub creatures_died: Vec<CreatureDied>,
    pub game_ended: Option<GameEnded>,
    pub priority_still_open: bool,
}

fn other_player_id(players: &[Player], player_id: &PlayerId) -> Result<PlayerId, DomainError> {
    players
        .iter()
        .find(|player| player.id() != player_id)
        .map(|player| player.id().clone())
        .ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(
                "two-player game should have an opposing player".to_string(),
            ))
        })
}

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
    card_id: &crate::domain::play::ids::CardInstanceId,
    card_type: &CardType,
) -> Result<(), DomainError> {
    if let Some(priority) = priority {
        invariants::require_priority_holder(Some(priority), player_id)?;

        let stack_is_empty = stack.is_empty();
        let active_player_in_main =
            player_id == active_player && matches!(phase, Phase::FirstMain | Phase::SecondMain);
        if stack_is_empty && active_player_in_main {
            return Ok(());
        }

        if card_type.is_instant() {
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

fn resolve_spell_from_stack(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    stack_object: &StackObject,
) -> Result<
    (
        StackTopResolved,
        SpellCast,
        Vec<CreatureDied>,
        Option<GameEnded>,
    ),
    DomainError,
> {
    let stack_object_id = stack_object.id().clone();
    let controller_id = stack_object.controller_id().clone();
    let source_card_id = stack_object.source_card_id().clone();

    let StackObjectKind::Spell(spell) = stack_object.kind().clone();
    let mana_cost_paid = spell.mana_cost_paid();
    let card = spell.into_card();
    let card_type = card.card_type().clone();

    let player = invariants::find_player_mut(players, &controller_id)?;
    let outcome = match card_type {
        CardType::Creature
        | CardType::Enchantment
        | CardType::Artifact
        | CardType::Planeswalker => {
            player.battlefield_mut().add(card);
            SpellCastOutcome::EnteredBattlefield
        }
        CardType::Instant | CardType::Sorcery => {
            player.graveyard_mut().add(card);
            SpellCastOutcome::ResolvedToGraveyard
        }
        CardType::Land => {
            return Err(DomainError::Game(GameError::InternalInvariantViolation(
                "land cards cannot resolve from the stack as spells".to_string(),
            )));
        }
    };

    let spell_cast = SpellCast::new(
        game_id.clone(),
        controller_id.clone(),
        source_card_id.clone(),
        card_type,
        mana_cost_paid,
        outcome,
    );
    let stack_top_resolved = StackTopResolved::new(
        game_id.clone(),
        controller_id,
        stack_object_id,
        source_card_id,
    );
    let StateBasedActionsResult {
        creatures_died,
        game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;

    Ok((stack_top_resolved, spell_cast, creatures_died, game_ended))
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

/// Passes priority in the current priority window, and may resolve the top
/// object on the stack when both players pass consecutively.
///
/// # Errors
/// Returns an error if there is no open priority window, if the caller does
/// not currently hold priority, or if resolving the top stack object fails.
pub fn pass_priority(
    ctx: StackPriorityContext<'_>,
    cmd: PassPriorityCommand,
) -> Result<PassPriorityOutcome, DomainError> {
    let StackPriorityContext {
        game_id,
        players,
        active_player,
        stack,
        priority,
        terminal_state,
        ..
    } = ctx;

    let PassPriorityCommand { player_id } = cmd;

    invariants::require_priority_holder(priority.as_ref(), &player_id)?;
    let priority_passed = PriorityPassed::new(game_id.clone(), player_id.clone());
    let passes_in_row = priority
        .as_ref()
        .map(PriorityState::passes_in_row)
        .ok_or(DomainError::Game(GameError::NoPriorityWindow))?;

    if passes_in_row == 0 {
        let next_holder = other_player_id(players, &player_id)?;
        *priority = Some(PriorityState::new_with_passes(next_holder, 1));
        return Ok(PassPriorityOutcome {
            priority_passed,
            stack_top_resolved: None,
            spell_cast: None,
            creatures_died: Vec::new(),
            game_ended: None,
            priority_still_open: true,
        });
    }

    if stack.is_empty() {
        *priority = None;
        return Ok(PassPriorityOutcome {
            priority_passed,
            stack_top_resolved: None,
            spell_cast: None,
            creatures_died: Vec::new(),
            game_ended: None,
            priority_still_open: false,
        });
    }

    let stack_object = stack.pop().ok_or_else(|| {
        DomainError::Game(GameError::InternalInvariantViolation(
            "priority resolution expected a stack object".to_string(),
        ))
    })?;
    let (stack_top_resolved, spell_cast, creatures_died, game_ended) =
        resolve_spell_from_stack(game_id, players, terminal_state, &stack_object)?;

    if terminal_state.is_over() {
        *priority = None;
    } else {
        *priority = Some(PriorityState::new(active_player.clone()));
    }

    Ok(PassPriorityOutcome {
        priority_passed,
        stack_top_resolved: Some(stack_top_resolved),
        spell_cast: Some(spell_cast),
        creatures_died,
        game_ended,
        priority_still_open: priority.is_some(),
    })
}
