use super::super::{
    super::{
        helpers,
        model::{StackObject, StackObjectKind},
        Player, TerminalState,
    },
    state_based_actions::{self, StateBasedActionsResult},
};
use super::spell_effects::supported_spell_rules;
use crate::domain::play::{
    cards::{CardInstance, CardType, SpellResolutionProfile, SupportedSpellRules},
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

struct ResolvedSpellObject {
    source_card_id: CardInstanceId,
    controller_id: crate::domain::play::ids::PlayerId,
    stack_object_id: crate::domain::play::ids::StackObjectId,
    card: CardInstance,
    card_type: CardType,
    mana_cost_paid: u32,
    supported_spell_rules: SupportedSpellRules,
    target: Option<SpellTarget>,
}

fn apply_damage_to_creature(players: &mut [Player], target_id: &CardInstanceId, damage: u32) {
    for player in players.iter_mut() {
        if let Some(card) = player.battlefield_mut().card_mut(target_id) {
            card.add_damage(damage);
            return;
        }
    }
}

fn review_state_based_actions(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let StateBasedActionsResult {
        creatures_died,
        game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
    Ok((None, creatures_died, game_ended))
}

fn apply_supported_spell_rules(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    match supported_spell_rules.resolution() {
        SpellResolutionProfile::None => {
            review_state_based_actions(game_id, players, terminal_state)
        }
        SpellResolutionProfile::DealDamage { damage } => {
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
                        -(damage).cast_signed(),
                    )?)
                }
                SpellTarget::Creature(card_id) => {
                    apply_damage_to_creature(players, card_id, damage);
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

fn move_resolved_spell_to_its_destination(
    players: &mut [Player],
    controller_id: &crate::domain::play::ids::PlayerId,
    card_type: &CardType,
    card: CardInstance,
) -> Result<SpellCastOutcome, DomainError> {
    let player = helpers::find_player_mut(players, controller_id)?;

    match card_type {
        &CardType::Creature
        | &CardType::Enchantment
        | &CardType::Artifact
        | &CardType::Planeswalker => {
            player.battlefield_mut().add(card);
            Ok(SpellCastOutcome::EnteredBattlefield)
        }
        &CardType::Instant | &CardType::Sorcery => {
            player.graveyard_mut().add(card);
            Ok(SpellCastOutcome::ResolvedToGraveyard)
        }
        &CardType::Land => Err(DomainError::Game(GameError::InternalInvariantViolation(
            "land cards cannot resolve from the stack as spells".to_string(),
        ))),
    }
}

fn extract_resolved_spell_object(stack_object: &StackObject) -> ResolvedSpellObject {
    let stack_object_id = stack_object.id().clone();
    let controller_id = stack_object.controller_id().clone();
    let source_card_id = stack_object.source_card_id().clone();

    let StackObjectKind::Spell(spell) = stack_object.kind().clone();
    let mana_cost_paid = spell.mana_cost_paid();
    let target = spell.target().cloned();
    let card = spell.into_card();
    let supported_spell_rules = supported_spell_rules(&card);
    let card_type = card.card_type().clone();

    ResolvedSpellObject {
        source_card_id,
        controller_id,
        stack_object_id,
        card,
        card_type,
        mana_cost_paid,
        supported_spell_rules,
        target,
    }
}

pub(super) fn resolve_spell_from_stack(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    stack_object: &StackObject,
) -> Result<ResolvedSpellOutcome, DomainError> {
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
    let (life_changed, creatures_died, game_ended) = apply_supported_spell_rules(
        game_id,
        players,
        terminal_state,
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
