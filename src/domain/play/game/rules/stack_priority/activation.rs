//! Supports rules stack priority activation.

use {
    super::{ActivateAbilityOutcome, StackPriorityContext},
    crate::domain::play::{
        commands::ActivateAbilityCommand,
        errors::{CardError, DomainError},
        events::ActivatedAbilityPutOnStack,
        game::{
            helpers, invariants,
            model::{ActivatedAbilityOnStack, PriorityState, StackObject, StackObjectKind},
        },
    },
};

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
        stack,
        priority,
        ..
    } = ctx;

    let ActivateAbilityCommand {
        player_id,
        source_card_id,
    } = cmd;

    invariants::require_priority_holder(priority.as_ref(), &player_id)?;
    let player_index = helpers::find_player_index(players, &player_id)?;
    let player = &mut players[player_index];
    let card = player
        .battlefield_card_mut(&source_card_id)
        .ok_or_else(|| {
            DomainError::Card(CardError::NotOnBattlefield {
                player: player_id.clone(),
                card: source_card_id.clone(),
            })
        })?;
    let ability = card
        .activated_ability()
        .ok_or_else(|| DomainError::Card(CardError::NoActivatedAbility(source_card_id.clone())))?;

    if ability.requires_tap() {
        if card.is_tapped() {
            return Err(DomainError::Card(CardError::AlreadyTapped {
                player: player_id.clone(),
                card: source_card_id.clone(),
            }));
        }
        card.tap();
    }

    let stack_object_number = stack.next_object_number();
    stack.push(StackObject::new(
        stack_object_number,
        player_id.clone(),
        StackObjectKind::ActivatedAbility(ActivatedAbilityOnStack::new(
            source_card_id.clone(),
            ability,
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
