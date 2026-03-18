use super::super::{
    super::{
        invariants,
        model::{StackObject, StackObjectKind},
        Player, TerminalState,
    },
    state_based_actions::{self, StateBasedActionsResult},
};
use super::spell_effects::{spell_effect, SpellEffect};
use crate::domain::play::{
    cards::CardType,
    errors::{DomainError, GameError},
    events::{CreatureDied, GameEnded, LifeChanged, SpellCast, SpellCastOutcome, StackTopResolved},
    game::SpellTarget,
    ids::{CardInstanceId, GameId},
};

type SpellResolutionSideEffects = (Option<LifeChanged>, Vec<CreatureDied>, Option<GameEnded>);
type ResolvedSpellOutcome = (
    StackTopResolved,
    SpellCast,
    Option<LifeChanged>,
    Vec<CreatureDied>,
    Option<GameEnded>,
);

fn apply_damage_to_creature(players: &mut [Player], target_id: &CardInstanceId, damage: u32) {
    for player in players.iter_mut() {
        if let Some(card) = player.battlefield_mut().card_mut(target_id) {
            card.add_damage(damage);
            return;
        }
    }
}

fn resolve_spell_effect_from_effect(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    effect: &SpellEffect,
    target: Option<&SpellTarget>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    match effect {
        SpellEffect::None => {
            let StateBasedActionsResult {
                creatures_died,
                game_ended,
            } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
            Ok((None, creatures_died, game_ended))
        }
        SpellEffect::DealDamageToAnyTarget { damage } => {
            let Some(target) = target else {
                return Err(DomainError::Game(GameError::InternalInvariantViolation(
                    "targeted spell resolved without target".to_string(),
                )));
            };

            let life_changed = match target {
                SpellTarget::Player(player_id) => {
                    Some(super::super::game_effects::adjust_player_life(
                        game_id,
                        players,
                        player_id,
                        -(*damage).cast_signed(),
                    )?)
                }
                SpellTarget::Creature(card_id) => {
                    apply_damage_to_creature(players, card_id, *damage);
                    None
                }
            };

            let StateBasedActionsResult {
                creatures_died,
                game_ended,
            } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
            Ok((life_changed, creatures_died, game_ended))
        }
    }
}

pub(super) fn resolve_spell_from_stack(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    stack_object: &StackObject,
) -> Result<ResolvedSpellOutcome, DomainError> {
    let stack_object_id = stack_object.id().clone();
    let controller_id = stack_object.controller_id().clone();
    let source_card_id = stack_object.source_card_id().clone();

    let StackObjectKind::Spell(spell) = stack_object.kind().clone();
    let mana_cost_paid = spell.mana_cost_paid();
    let target = spell.target().cloned();
    let card = spell.into_card();
    let effect = spell_effect(&card);
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
    let (life_changed, creatures_died, game_ended) = match effect {
        SpellEffect::None => {
            let StateBasedActionsResult {
                creatures_died,
                game_ended,
            } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
            (None, creatures_died, game_ended)
        }
        effect @ SpellEffect::DealDamageToAnyTarget { .. } => resolve_spell_effect_from_effect(
            game_id,
            players,
            terminal_state,
            &effect,
            target.as_ref(),
        )?,
    };

    Ok((
        stack_top_resolved,
        spell_cast,
        life_changed,
        creatures_died,
        game_ended,
    ))
}
