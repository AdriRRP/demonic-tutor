//! Supports stack priority resolution effects.

use {
    super::super::super::{
        super::{helpers, model::StackZone, AggregateCardLocationIndex, Player, TerminalState},
        state_based_actions::{self, StateBasedActionsResult},
    },
    super::super::spell_effects::{
        evaluate_target_legality, SpellTargetLegality, TargetLegalityContext,
    },
    crate::domain::play::{
        cards::{SpellResolutionProfile, SupportedSpellRules},
        errors::{DomainError, GameError},
        events::{CardDiscarded, CardExiled, CreatureDied, DiscardKind, GameEnded, LifeChanged},
        game::{rules::zones, SpellTarget},
        ids::{CardInstanceId, GameId},
    },
};

type SpellResolutionSideEffects = (
    Option<CardExiled>,
    Option<CardDiscarded>,
    Option<LifeChanged>,
    Vec<CreatureDied>,
    Vec<CardInstanceId>,
    Option<GameEnded>,
);

pub(super) struct ResolutionContext<'a> {
    game_id: &'a GameId,
    players: &'a mut [Player],
    card_locations: &'a AggregateCardLocationIndex,
    terminal_state: &'a mut TerminalState,
    stack: &'a mut StackZone,
    controller_index: usize,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&'a SpellTarget>,
    choice: Option<crate::domain::play::game::model::StackSpellChoice>,
}

impl<'a> ResolutionContext<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(super) const fn new(
        game_id: &'a GameId,
        players: &'a mut [Player],
        card_locations: &'a AggregateCardLocationIndex,
        terminal_state: &'a mut TerminalState,
        stack: &'a mut StackZone,
        controller_index: usize,
        supported_spell_rules: SupportedSpellRules,
        target: Option<&'a SpellTarget>,
        choice: Option<crate::domain::play::game::model::StackSpellChoice>,
    ) -> Self {
        Self {
            game_id,
            players,
            card_locations,
            terminal_state,
            stack,
            controller_index,
            supported_spell_rules,
            target,
            choice,
        }
    }
}

struct EffectOutcomeSeed {
    card_exiled: Option<CardExiled>,
    card_discarded: Option<CardDiscarded>,
    life_changed: Option<LifeChanged>,
    creatures_died: Vec<CreatureDied>,
    moved_cards: Vec<CardInstanceId>,
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
    let handle = card_locations.location(target_id)?.handle();
    players[owner_index].move_battlefield_handle_to_graveyard(handle)?;
    Some(CreatureDied::new(
        game_id.clone(),
        owner_id,
        target_id.clone(),
    ))
}

fn return_permanent_to_owners_hand(
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardInstanceId> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Battlefield).then_some(())?;
    players[location.owner_index()].move_battlefield_handle_to_hand(location.handle())?;
    Some(target_id.clone())
}

fn destroy_noncreature_permanent(
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardInstanceId> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Battlefield).then_some(())?;
    players[location.owner_index()].move_battlefield_handle_to_graveyard(location.handle())?;
    Some(target_id.clone())
}

fn discard_chosen_hand_card(
    game_id: &GameId,
    players: &mut [Player],
    target_player_index: usize,
    choice: crate::domain::play::game::model::StackSpellChoice,
) -> Option<(CardDiscarded, CardInstanceId)> {
    let crate::domain::play::game::model::StackSpellChoice::HandCard(card_ref) = choice;
    (card_ref.owner_index() == target_player_index).then_some(())?;
    let player = players.get_mut(target_player_index)?;
    player.move_hand_handle_to_graveyard(card_ref.handle())?;
    let card_id = player.card_by_handle(card_ref.handle())?.id().clone();
    let event = CardDiscarded::new(
        game_id.clone(),
        player.id().clone(),
        card_id.clone(),
        DiscardKind::SpellEffect,
    );
    Some((event, card_id))
}

fn exile_creature_from_battlefield(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardExiled> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Battlefield).then_some(())?;
    zones::exile_card_from_battlefield_handle_by_index(
        game_id,
        players,
        location.owner_index(),
        location.handle(),
    )
    .ok()
}

fn exile_card_from_graveyard(
    game_id: &GameId,
    players: &mut [Player],
    card_locations: &AggregateCardLocationIndex,
    target_id: &CardInstanceId,
) -> Option<CardExiled> {
    let location = card_locations.location(target_id)?;
    (location.zone() == crate::domain::play::game::PlayerCardZone::Graveyard).then_some(())?;
    zones::exile_card_from_graveyard_handle_by_index(
        game_id,
        players,
        location.owner_index(),
        location.handle(),
    )
    .ok()
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
    Ok((None, None, None, creatures_died, Vec::new(), game_ended))
}

fn resolve_target_legality_for_effect(
    players: &[Player],
    card_locations: &AggregateCardLocationIndex,
    stack: &StackZone,
    controller_index: usize,
    supported_spell_rules: SupportedSpellRules,
    target: Option<&SpellTarget>,
    missing_profile_message: &str,
) -> Result<Option<SpellTarget>, DomainError> {
    let legality = evaluate_target_legality(
        TargetLegalityContext::Resolution {
            players,
            card_locations,
            stack,
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
            | SpellTargetLegality::MissingPermanent(_)
            | SpellTargetLegality::MissingGraveyardCard(_)
            | SpellTargetLegality::MissingStackSpell(_),
            _,
        ) => Ok(None),
    }
}

fn review_state_based_actions_after_effect(
    game_id: &GameId,
    players: &mut [Player],
    terminal_state: &mut TerminalState,
    seed: EffectOutcomeSeed,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let EffectOutcomeSeed {
        card_exiled,
        card_discarded,
        life_changed,
        mut creatures_died,
        moved_cards,
    } = seed;
    let StateBasedActionsResult {
        creatures_died: sba_creatures_died,
        game_ended,
    } = state_based_actions::check_state_based_actions(game_id, players, terminal_state)?;
    creatures_died.extend(sba_creatures_died);
    Ok((
        card_exiled,
        card_discarded,
        life_changed,
        creatures_died,
        moved_cards,
        game_ended,
    ))
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
        &StackZone::empty(),
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
        SpellTarget::Player(_)
        | SpellTarget::Permanent(_)
        | SpellTarget::GraveyardCard(_)
        | SpellTarget::StackObject(_) => None,
    };

    review_state_based_actions_after_effect(
        game_id,
        players,
        terminal_state,
        EffectOutcomeSeed {
            card_exiled,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
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
        &StackZone::empty(),
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
        SpellTarget::Player(_)
        | SpellTarget::Creature(_)
        | SpellTarget::Permanent(_)
        | SpellTarget::StackObject(_) => None,
    };

    review_state_based_actions_after_effect(
        game_id,
        players,
        terminal_state,
        EffectOutcomeSeed {
            card_exiled,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

fn resolve_pump_target_creature_effect(
    context: &mut ResolutionContext<'_>,
    bonus: (u32, u32),
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
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
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
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
        context.stack,
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
        SpellTarget::Creature(_)
        | SpellTarget::Permanent(_)
        | SpellTarget::GraveyardCard(_)
        | SpellTarget::StackObject(_) => None,
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

fn resolve_damage_effect(
    context: &mut ResolutionContext<'_>,
    damage: u32,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
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
        SpellTarget::Permanent(_) | SpellTarget::GraveyardCard(_) | SpellTarget::StackObject(_) => {
            None
        }
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed,
            creatures_died: Vec::new(),
            moved_cards: Vec::new(),
        },
    )
}

fn resolve_destroy_target_creature_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
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
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died,
            moved_cards: Vec::new(),
        },
    )
}

fn resolve_return_target_permanent_to_hand_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "bounce spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let moved_cards = match target {
        SpellTarget::Permanent(card_id) => {
            return_permanent_to_owners_hand(context.players, context.card_locations, &card_id)
                .into_iter()
                .collect()
        }
        SpellTarget::Player(_)
        | SpellTarget::Creature(_)
        | SpellTarget::GraveyardCard(_)
        | SpellTarget::StackObject(_) => Vec::new(),
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

fn resolve_destroy_target_artifact_or_enchantment_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "artifact or enchantment destruction spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let moved_cards = match target {
        SpellTarget::Permanent(card_id) => {
            destroy_noncreature_permanent(context.players, context.card_locations, &card_id)
                .into_iter()
                .collect()
        }
        SpellTarget::Player(_)
        | SpellTarget::Creature(_)
        | SpellTarget::GraveyardCard(_)
        | SpellTarget::StackObject(_) => Vec::new(),
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

fn resolve_counter_target_spell_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "counter spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let mut moved_cards = Vec::new();
    if let SpellTarget::StackObject(stack_object_id) = target {
        let object_number = stack_object_id.object_number().ok_or_else(|| {
            DomainError::Game(GameError::InternalInvariantViolation(format!(
                "counter target lost stack object number for {stack_object_id}"
            )))
        })?;

        if let Some(countered_object) = context.stack.remove_by_number(object_number) {
            let countered_controller_index = countered_object.controller_index();
            let crate::domain::play::game::model::StackObjectKind::Spell(countered_spell) =
                countered_object.into_kind()
            else {
                return Err(DomainError::Game(GameError::InternalInvariantViolation(
                    "counter target must remove a spell stack object".to_string(),
                )));
            };

            let payload = countered_spell.into_payload();
            let countered_card_id = payload.id().clone();
            let player = context.players.get_mut(countered_controller_index).ok_or_else(|| {
                DomainError::Game(GameError::InternalInvariantViolation(format!(
                    "missing countered spell controller at player index {countered_controller_index}"
                )))
            })?;
            player.receive_graveyard_card(payload.into_card_instance());
            moved_cards.push(countered_card_id);
        }
    }

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded: None,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

fn resolve_target_player_discards_chosen_card_effect(
    context: &mut ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    let Some(target) = resolve_target_legality_for_effect(
        context.players,
        context.card_locations,
        context.stack,
        context.controller_index,
        context.supported_spell_rules,
        context.target,
        "discard spell resolved without a targeting profile",
    )?
    else {
        return review_state_based_actions(
            context.game_id,
            context.players,
            context.terminal_state,
        );
    };

    let (card_discarded, moved_cards) = match (target, context.choice) {
        (SpellTarget::Player(player_id), Some(choice)) => {
            let target_player_index = helpers::find_player_index(context.players, &player_id)?;
            match discard_chosen_hand_card(
                context.game_id,
                context.players,
                target_player_index,
                choice,
            ) {
                Some((event, card_id)) => (Some(event), vec![card_id]),
                None => (None, Vec::new()),
            }
        }
        _ => (None, Vec::new()),
    };

    review_state_based_actions_after_effect(
        context.game_id,
        context.players,
        context.terminal_state,
        EffectOutcomeSeed {
            card_exiled: None,
            card_discarded,
            life_changed: None,
            creatures_died: Vec::new(),
            moved_cards,
        },
    )
}

pub(super) fn apply_supported_spell_rules(
    mut context: ResolutionContext<'_>,
) -> Result<SpellResolutionSideEffects, DomainError> {
    match context.supported_spell_rules.resolution() {
        SpellResolutionProfile::None => {
            review_state_based_actions(context.game_id, context.players, context.terminal_state)
        }
        SpellResolutionProfile::DealDamage { damage } => {
            resolve_damage_effect(&mut context, damage)
        }
        SpellResolutionProfile::GainLife { amount } => resolve_targeted_player_life_effect(
            &mut context,
            amount.cast_signed(),
            "gain-life spell resolved without a targeting profile",
        ),
        SpellResolutionProfile::LoseLife { amount } => resolve_targeted_player_life_effect(
            &mut context,
            -(amount).cast_signed(),
            "lose-life spell resolved without a targeting profile",
        ),
        SpellResolutionProfile::CounterTargetSpell => {
            resolve_counter_target_spell_effect(&mut context)
        }
        SpellResolutionProfile::ReturnTargetPermanentToHand => {
            resolve_return_target_permanent_to_hand_effect(&mut context)
        }
        SpellResolutionProfile::DestroyTargetArtifactOrEnchantment => {
            resolve_destroy_target_artifact_or_enchantment_effect(&mut context)
        }
        SpellResolutionProfile::TargetPlayerDiscardsChosenCard => {
            resolve_target_player_discards_chosen_card_effect(&mut context)
        }
        SpellResolutionProfile::DestroyTargetCreature => {
            resolve_destroy_target_creature_effect(&mut context)
        }
        SpellResolutionProfile::ExileTargetCreature => resolve_exile_target_creature_effect(
            context.game_id,
            context.players,
            context.terminal_state,
            context.card_locations,
            context.controller_index,
            context.supported_spell_rules,
            context.target,
        ),
        SpellResolutionProfile::ExileTargetCardFromGraveyard => {
            resolve_exile_target_graveyard_card_effect(
                context.game_id,
                context.players,
                context.terminal_state,
                context.card_locations,
                context.controller_index,
                context.supported_spell_rules,
                context.target,
            )
        }
        SpellResolutionProfile::PumpTargetCreatureUntilEndOfTurn { power, toughness } => {
            resolve_pump_target_creature_effect(&mut context, (power, toughness))
        }
    }
}
