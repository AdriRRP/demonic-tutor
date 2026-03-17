use super::super::{
    super::{
        invariants,
        model::{StackObject, StackObjectKind},
        Player, TerminalState,
    },
    state_based_actions::{self, StateBasedActionsResult},
};
use crate::domain::play::{
    cards::CardType,
    errors::{DomainError, GameError},
    events::{CreatureDied, GameEnded, SpellCast, SpellCastOutcome, StackTopResolved},
    ids::GameId,
};

pub(super) fn resolve_spell_from_stack(
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
