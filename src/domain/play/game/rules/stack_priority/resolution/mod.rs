mod destination;
mod effects;
mod events;
mod extract;

use self::{
    destination::move_resolved_spell_to_its_destination,
    effects::apply_supported_spell_rules,
    events::build_resolution_events,
    extract::{extract_resolved_spell_object, ResolvedSpellObject},
};
use crate::domain::play::{
    events::{CreatureDied, GameEnded, LifeChanged, SpellCast, StackTopResolved},
    game::{model::StackObject, Player, TerminalState},
    ids::GameId,
};

type ResolvedSpellOutcome = (
    StackTopResolved,
    SpellCast,
    Option<LifeChanged>,
    Vec<CreatureDied>,
    Option<GameEnded>,
);

pub(super) fn resolve_spell_from_stack(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    stack_object: &StackObject,
) -> Result<ResolvedSpellOutcome, crate::domain::play::errors::DomainError> {
    let ResolvedSpellObject {
        source_card_id,
        controller_id,
        stack_object_id,
        card,
        card_type,
        mana_cost_paid,
        supported_spell_rules,
        target,
    } = extract_resolved_spell_object(stack_object);

    let outcome =
        move_resolved_spell_to_its_destination(players, &controller_id, &card_type, card)?;

    let (stack_top_resolved, spell_cast) = build_resolution_events(
        game_id,
        &controller_id,
        &stack_object_id,
        &source_card_id,
        card_type,
        mana_cost_paid,
        outcome,
    );
    let (life_changed, creatures_died, game_ended) = apply_supported_spell_rules(
        game_id,
        players,
        terminal_state,
        &controller_id,
        supported_spell_rules,
        target.as_ref(),
    )?;

    Ok((
        stack_top_resolved,
        spell_cast,
        life_changed,
        creatures_died,
        game_ended,
    ))
}
