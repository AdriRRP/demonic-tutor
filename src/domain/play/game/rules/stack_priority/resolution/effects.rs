//! Supports stack priority resolution effects.

use {
    super::super::super::{
        super::{helpers, AggregateCardLocationIndex, Player, TerminalState},
        state_based_actions::{self, StateBasedActionsResult},
    },
    super::super::spell_effects::{
        evaluate_target_legality, SpellTargetLegality, TargetLegalityContext,
    },
    crate::domain::play::{
        cards::{SpellResolutionProfile, SupportedSpellRules},
        errors::{DomainError, GameError},
        events::{CardExiled, CreatureDied, GameEnded, LifeChanged},
        game::{rules::zones, SpellTarget},
        ids::{CardInstanceId, GameId, PlayerId},
    },
};

type SpellResolutionSideEffects = (
    Option<CardExiled>,
    Option<LifeChanged>,
    Vec<CreatureDied>,
    Option<GameEnded>,
);

struct ResolutionContext<'a> {
    game_id: &'a GameId,
    players: &'a mut [Player],
    card_locations: &'a AggregateCardLocationIndex,
    terminal_state: &'a mut TerminalState,
    controller_index: usize,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&'a SpellTarget>,
}

fn apply_damage_to_creature(
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    target_id: &CardInstanceId,
    damage: u32,
) {
    if let Some(card) = helpers::battlefield_card_mut(players, card_locations, target_id) {
        card.add_damage(damage);
    }
}

fn apply_temporary_pump_to_creature(
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    target_id: &CardInstanceId,
    power: u32,
    toughness: u32,
) {
    if let Some(card) = helpers::battlefield_card_mut(players, card_locations, target_id) {
        card.apply_temporary_stat_bonus(power, toughness);
    }
}

fn destroy_creature(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CreatureDied> {
    let target = helpers::battlefield_card_location(players, card_locations, target_id)?;
    let owner_index = target.owner_index();
    let owner_id = players[owner_index].id().clone();
    players[owner_index].move_battlefield_card_to_graveyard(target_id)?;
    Some(CreatureDied::new(
        game_id.clone(),
        owner_id,
        target_id.clone(),
    ))
}

fn exile_creature_from_battlefield(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardExiled> {
    let owner_index =
        helpers::battlefield_card_location(players, card_locations, target_id)?.owner_index();
    zones::exile_card_from_battlefield_by_index(game_id, players, owner_index, target_id).ok()
}

fn exile_card_from_graveyard(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardExiled> {
    let owner_index =
        helpers::graveyard_card_location(players, card_locations, target_id)?.owner_index();
    zones::exile_card_from_graveyard_by_index(game_id, players, owner_index, target_id).ok()
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
    Ok((None, None, creatures_died, game_ended))
}

fn resolve_target_legality_for_effect(
    players: &[Player],
    card_locations: &AggregateCardLocationIndex,
    controller_index: usize,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
    missing_profile_message: &str,
) -> Result<Option<SpellTarget>, DomainError> {
    let legality = evaluate_target_legality(
        TargetLegalityContext::Resolution {
            players,
            card_locations,
            actor_index: controller_index,
        },
        supported_spell_rules.targeting(),
        target,
    );

    match (legality, target) {
        (SpellTargetLegality::Legal, Some(target)) => Ok(Some(target.clone())),
        (SpellTargetLegality::Legal, None) => {
            Err(DomainError::Game(GameError::InternalInvariantViolation(
                "legal targeted spell resolution requires an attached target".to_string(),
            )))
        }
        (SpellTargetLegality::MissingRequiredTarget, _) => {
            Err(DomainError::Game(GameError::InternalInvariantViolation(
                "targeted spell resolved without target".to_string(),
            )))
        }
        (SpellTargetLegality::NoTargetRequired, _) => Err(DomainError::Game(
            GameError::InternalInvariantViolation(missing_profile_message.to_string()),
        )),
        (
            SpellTargetLegality::IllegalTargetKind
            | SpellTargetLegality::IllegalTargetRule
            | SpellTargetLegality::MissingPlayer(_)
            | SpellTargetLegality::MissingCreature(_)
            | SpellTargetLegality::MissingGraveyardCard(_),
            _,
        ) => Ok(None),
    }
}

fn review_state_based_actions_after_effect(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    card_exiled: Option<CardExiled>,
    life_changed: Option<LifeChanged>,
    mut creatures_died: Vec<CreatureDied>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let StateBasedActionsResult {
        creatures_died: sba_creatures_died,
        game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
    creatures_died.extend(sba_creatures_died);
    Ok((card_exiled, life_changed, creatures_died, game_ended))
}

fn resolve_exile_target_creature_effect(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    card_locations: &AggregateCardLocationIndex,
    controller_index: usize,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        players,
        card_locations,
        controller_index,
        supported_spell_rules,
        target,
        "exile spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(game_id, players, terminal_state);
    };

    let card_exiled = match target {
        SpellTarget::Creature(card_id) => {
            exile_creature_from_battlefield(game_id, players, card_locations, &card_id)
        }
        SpellTarget::Player(_) | SpellTarget::GraveyardCard(_) => None,
    };

    review_state_based_actions_after_effect(
        game_id,
        players,
        terminal_state,
        card_exiled,
        None,
        Vec::new(),
    )
}

fn resolve_exile_target_graveyard_card_effect(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    card_locations: &AggregateCardLocationIndex,
    controller_index: usize,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        players,
        card_locations,
        controller_index,
        supported_spell_rules,
        target,
        "graveyard exile spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(game_id, players, terminal_state);
    };

    let card_exiled = match target {
        SpellTarget::GraveyardCard(card_id) => {
            exile_card_from_graveyard(game_id, players, card_locations, &card_id)
        }
        SpellTarget::Player(_) | SpellTarget::Creature(_) => None,
    };

    review_state_based_actions_after_effect(
        game_id,
        players,
        terminal_state,
        card_exiled,
        None,
        Vec::new(),
    )
}

fn resolve_pump_target_creature_effect(
    context: &mut ResolutionContext<'_>,
    bonus: (u32, u32),
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "pump spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    if let SpellTarget::Creature(card_id) = target {
        apply_temporary_pump_to_creature(
            context.players,
            context.card_locations,
            &card_id,
            bonus.0,
            bonus.1,
        );
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        None,
        None,
        Vec::new(),
    )
}

fn resolve_targeted_player_life_effect(
    context: &mut ResolutionContext<'_>,
    life_delta: i32,
    missing_profile_message: &str,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        missing_profile_message,
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let life_changed = match target {
        SpellTarget::Player(player_id) => {
            let player_index = helpers::find_player_index(context.players, &player_id)?;
            Some(
                super::super::super::game_effects::adjust_player_life_by_index(
                    context.game_id,
                    context.players,
                    player_index,
                    life_delta,
                )?,
            )
        }
        SpellTarget::Creature(_) | SpellTarget::GraveyardCard(_) => None,
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        None,
        life_changed,
        Vec::new(),
    )
}

fn resolve_damage_effect(
    context: &mut ResolutionContext<'_>,
    damage: u32,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "damage spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let life_changed = match target {
        SpellTarget::Player(player_id) => {
            let player_index = helpers::find_player_index(context.players, &player_id)?;
            Some(
                super::super::super::game_effects::adjust_player_life_by_index(
                    context.game_id,
                    context.players,
                    player_index,
                    -(damage).cast_signed(),
                )?,
            )
        }
        SpellTarget::Creature(card_id) => {
            apply_damage_to_creature(context.players, context.card_locations, &card_id, damage);
            None
        }
        SpellTarget::GraveyardCard(_) => None,
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        None,
        life_changed,
        Vec::new(),
    )
}

fn resolve_destroy_target_creature_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "destroy spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let mut creatures_died = Vec::new();
    if let SpellTarget::Creature(card_id) = target {
        if let Some(creature_died) = destroy_creature(
            context.game_id,
            context.players,
            context.card_locations,
            &card_id,
        ) {
            creatures_died.push(creature_died);
        }
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        None,
        None,
        creatures_died,
    )
}

pub(super) fn apply_supported_spell_rules(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    terminal_state: &mut TerminalState,
    controller_id: &PlayerId,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let controller_index = helpers::find_player_index(players, controller_id)?;
    match supported_spell_rules.resolution() {
        SpellResolutionProfile::None => {
            review_state_based_actions(game_id, players, terminal_state)
        }
        SpellResolutionProfile::DealDamage { damage } => {
            let mut context = ResolutionContext {
                game_id,
                players,
                card_locations,
                terminal_state,
                controller_index,
                supported_spell_rules,
                target,
            };
            resolve_damage_effect(&mut context, damage)
        }
        SpellResolutionProfile::GainLife { amount } => {
            let mut context = ResolutionContext {
                game_id,
                players,
                card_locations,
                terminal_state,
                controller_index,
                supported_spell_rules,
                target,
            };
            resolve_targeted_player_life_effect(
                &mut context,
                amount.cast_signed(),
                "gain-life spell resolved without a targeting profile",
            )
        }
        SpellResolutionProfile::LoseLife { amount } => {
            let mut context = ResolutionContext {
                game_id,
                players,
                card_locations,
                terminal_state,
                controller_index,
                supported_spell_rules,
                target,
            };
            resolve_targeted_player_life_effect(
                &mut context,
                -(amount).cast_signed(),
                "lose-life spell resolved without a targeting profile",
            )
        }
        SpellResolutionProfile::DestroyTargetCreature => {
            let mut context = ResolutionContext {
                game_id,
                players,
                card_locations,
                terminal_state,
                controller_index,
                supported_spell_rules,
                target,
            };
            resolve_destroy_target_creature_effect(&mut context)
        }
        SpellResolutionProfile::ExileTargetCreature => resolve_exile_target_creature_effect(
            game_id,
            players,
            terminal_state,
            card_locations,
            controller_index,
            supported_spell_rules,
            target,
        ),
        SpellResolutionProfile::ExileTargetCardFromGraveyard => {
            resolve_exile_target_graveyard_card_effect(
                game_id,
                players,
                terminal_state,
                card_locations,
                controller_index,
                supported_spell_rules,
                target,
            )
        }
        SpellResolutionProfile::PumpTargetCreatureUntilEndOfTurn { power, toughness } => {
            let mut context = ResolutionContext {
                game_id,
                players,
                card_locations,
                terminal_state,
                controller_index,
                supported_spell_rules,
                target,
            };
            resolve_pump_target_creature_effect(&mut context, (power, toughness))
        }
    }
}
