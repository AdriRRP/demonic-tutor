//! Projects the aggregate into the public gameplay read contract.

use std::collections::HashMap;

use crate::domain::play::{
    cards::{CardInstance, KeywordAbility},
    game::{Game, Player, StackObjectKind},
    ids::{CardInstanceId, PlayerId, StackObjectId},
    phase::Phase,
};

use super::{
    PublicActivatableCard, PublicBattlefieldCardView, PublicBinaryChoice, PublicBlockerOption,
    PublicCardView, PublicCastableCard, PublicChoiceCandidate, PublicChoiceRequest,
    PublicCombatStateView, PublicCommandApplication, PublicCommandResult, PublicEventLogEntry,
    PublicGameView, PublicLegalAction, PublicModalSpellChoice, PublicPermanentStateView,
    PublicPlayerView, PublicPriorityView, PublicScryChoice, PublicStackObjectView,
    PublicStackTargetView, PublicSurveilChoice,
};

#[derive(Debug, Default)]
struct PublicSurfaceState {
    legal_actions: Vec<PublicLegalAction>,
    choice_requests: Vec<PublicChoiceRequest>,
}

#[must_use]
pub fn game_view(game: &Game) -> PublicGameView {
    let active_player_id = game.active_player().clone();
    let players = game
        .players()
        .iter()
        .enumerate()
        .map(|(index, player)| player_view(player, index, &active_player_id))
        .collect();
    let stack = game
        .stack()
        .objects()
        .iter()
        .map(|object| stack_object_view(game, object))
        .collect();

    PublicGameView {
        game_id: game.id().clone(),
        active_player_id,
        phase: *game.phase(),
        turn_number: game.turn_number(),
        priority: game.priority().map(|priority| PublicPriorityView {
            current_holder: priority.current_holder().clone(),
            has_pending_pass: priority.has_pending_pass(),
        }),
        is_over: game.is_over(),
        winner_id: game.winner().cloned(),
        loser_id: game.loser().cloned(),
        end_reason: game.end_reason(),
        players,
        stack,
    }
}

#[must_use]
fn pending_action_and_request(game: &Game) -> Option<PublicSurfaceState> {
    if let Some(pending_decision) = game.pending_decision() {
        let player_id = game.players()[pending_decision.controller_index()]
            .id()
            .clone();
        let (action, request) = match pending_decision {
            crate::domain::play::game::PendingDecision::Scry { .. } => (
                PublicLegalAction::ResolvePendingScry { player_id },
                pending_scry_request(game),
            ),
            crate::domain::play::game::PendingDecision::Surveil { .. } => (
                PublicLegalAction::ResolvePendingSurveil { player_id },
                pending_surveil_request(game),
            ),
            crate::domain::play::game::PendingDecision::HandChoice { .. } => (
                PublicLegalAction::ResolvePendingHandChoice { player_id },
                pending_hand_choice_request(game),
            ),
            crate::domain::play::game::PendingDecision::OptionalEffect { .. } => (
                PublicLegalAction::ResolveOptionalEffect { player_id },
                pending_optional_effect_request(game),
            ),
        };
        return Some(PublicSurfaceState {
            legal_actions: vec![action],
            choice_requests: request.into_iter().collect(),
        });
    }

    None
}

fn priority_surface_state(game: &Game, player_id: &PlayerId) -> PublicSurfaceState {
    let playable_land_ids = playable_land_ids(game, player_id);
    let mana_source_ids = tappable_mana_source_ids(game, player_id);
    let castable_cards = castable_cards(game, player_id);
    let activatable_cards = activatable_cards(game, player_id);
    let spell_target_candidates_by_card =
        spell_target_candidate_cache(game, player_id, &castable_cards);
    let ability_target_candidates_by_card =
        ability_target_candidate_cache(game, player_id, &activatable_cards);

    let mut actions = Vec::new();
    actions.push(PublicLegalAction::PassPriority {
        player_id: player_id.clone(),
    });

    if !playable_land_ids.is_empty() {
        actions.push(PublicLegalAction::PlayLand {
            player_id: player_id.clone(),
            playable_land_ids,
        });
    }

    if !mana_source_ids.is_empty() {
        actions.push(PublicLegalAction::TapManaSource {
            player_id: player_id.clone(),
            mana_source_ids,
        });
    }

    if !castable_cards.is_empty() {
        actions.push(PublicLegalAction::CastSpell {
            player_id: player_id.clone(),
            castable_cards: castable_cards.clone(),
        });
    }

    if !activatable_cards.is_empty() {
        actions.push(PublicLegalAction::ActivateAbility {
            player_id: player_id.clone(),
            activatable_cards: activatable_cards.clone(),
        });
    }

    let mut choice_requests = Vec::new();
    for castable in &castable_cards {
        if castable.requires_target {
            choice_requests.push(PublicChoiceRequest::SpellTarget {
                player_id: player_id.clone(),
                source_card_id: castable.card_id.clone(),
                candidates: spell_target_candidates_by_card
                    .get(&castable.card_id)
                    .cloned()
                    .unwrap_or_default(),
            });
        }
        if castable.requires_choice {
            if let Some(request) = spell_choice_request(game, player_id, &castable.card_id) {
                choice_requests.push(request);
            }
        }
    }

    for activatable in &activatable_cards {
        if activatable.requires_target {
            let source_card_id = activatable.card_id.clone();
            choice_requests.push(PublicChoiceRequest::AbilityTarget {
                player_id: player_id.clone(),
                source_card_id: source_card_id.clone(),
                candidates: ability_target_candidates_by_card
                    .get(&source_card_id)
                    .cloned()
                    .unwrap_or_default(),
            });
        }
    }

    PublicSurfaceState {
        legal_actions: actions,
        choice_requests,
    }
}

fn spell_target_candidate_cache(
    game: &Game,
    actor_id: &PlayerId,
    castable_cards: &[PublicCastableCard],
) -> HashMap<CardInstanceId, Vec<PublicChoiceCandidate>> {
    castable_cards
        .iter()
        .filter(|card| card.requires_target)
        .map(|card| {
            (
                card.card_id.clone(),
                spell_target_candidates(game, actor_id, &card.card_id),
            )
        })
        .collect()
}

fn ability_target_candidate_cache(
    game: &Game,
    actor_id: &PlayerId,
    activatable_cards: &[PublicActivatableCard],
) -> HashMap<CardInstanceId, Vec<PublicChoiceCandidate>> {
    activatable_cards
        .iter()
        .filter(|card| card.requires_target)
        .map(|card| {
            (
                card.card_id.clone(),
                ability_target_candidates(game, actor_id, &card.card_id),
            )
        })
        .collect()
}

fn phase_surface_state(game: &Game) -> PublicSurfaceState {
    let mut actions = Vec::new();
    let mut choice_requests = Vec::new();

    match game.phase() {
        Phase::DeclareAttackers => {
            let player_id = game.active_player().clone();
            actions.push(PublicLegalAction::DeclareAttackers {
                player_id: player_id.clone(),
                attacker_ids: attack_candidate_ids(game, &player_id),
            });
        }
        Phase::DeclareBlockers => {
            let player_id =
                defending_player_id(game).unwrap_or_else(|| game.active_player().clone());
            let attacker_ids = attacking_creature_ids(game, game.active_player());
            actions.push(PublicLegalAction::DeclareBlockers {
                player_id: player_id.clone(),
                attacker_ids,
                blocker_options: game
                    .blocker_options(&player_id)
                    .into_iter()
                    .map(|option| PublicBlockerOption {
                        blocker_id: option.blocker_id().clone(),
                        attacker_ids: option.attacker_ids().to_vec(),
                    })
                    .collect(),
            });
        }
        Phase::CombatDamage => {
            actions.push(PublicLegalAction::ResolveCombatDamage {
                player_id: game.active_player().clone(),
            });
        }
        Phase::EndStep => {
            let Some(player) = active_player(game) else {
                return PublicSurfaceState {
                    legal_actions: actions,
                    choice_requests,
                };
            };
            if player.hand_size() > 7 {
                actions.push(PublicLegalAction::DiscardForCleanup {
                    player_id: player.id().clone(),
                    card_ids: player.hand_card_ids(),
                });
                choice_requests.push(PublicChoiceRequest::CleanupDiscard {
                    player_id: player.id().clone(),
                    hand_card_ids: player.hand_card_ids(),
                });
            } else {
                actions.push(PublicLegalAction::AdvanceTurn {
                    player_id: game.active_player().clone(),
                });
            }
        }
        _ => {
            actions.push(PublicLegalAction::AdvanceTurn {
                player_id: game.active_player().clone(),
            });
        }
    }

    PublicSurfaceState {
        legal_actions: actions,
        choice_requests,
    }
}

fn public_surface_state(game: &Game) -> PublicSurfaceState {
    if game.is_over() {
        return PublicSurfaceState::default();
    }

    if let Some(state) = pending_action_and_request(game) {
        return state;
    }

    if let Some(priority) = game.priority() {
        return priority_surface_state(game, priority.current_holder());
    }

    phase_surface_state(game)
}

#[must_use]
pub fn legal_actions(game: &Game) -> Vec<PublicLegalAction> {
    public_surface_state(game).legal_actions
}

#[must_use]
pub fn choice_requests(game: &Game) -> Vec<PublicChoiceRequest> {
    public_surface_state(game).choice_requests
}

#[must_use]
pub fn public_command_result(
    game: &Game,
    application: PublicCommandApplication,
) -> PublicCommandResult {
    let surface = public_surface_state(game);

    PublicCommandResult {
        status: application.status,
        emitted_events: application.emitted_events,
        game: game_view(game),
        legal_actions: surface.legal_actions,
        choice_requests: surface.choice_requests,
    }
}

#[must_use]
pub fn public_event_log(
    events: &[crate::domain::play::events::DomainEvent],
) -> Vec<PublicEventLogEntry> {
    events
        .iter()
        .cloned()
        .zip(1_u64..)
        .map(|(event, sequence)| PublicEventLogEntry { sequence, event })
        .collect()
}

fn player_view(player: &Player, _index: usize, active_player_id: &PlayerId) -> PublicPlayerView {
    PublicPlayerView {
        player_id: player.id().clone(),
        is_active: player.id() == active_player_id,
        life: player.life(),
        mana_total: player.mana(),
        hand_count: player.hand_size(),
        library_count: player.library_size(),
        battlefield: player
            .battlefield_cards()
            .map(battlefield_card_view)
            .collect(),
        graveyard: player
            .graveyard()
            .iter()
            .filter_map(|handle| player.card_by_handle(*handle))
            .map(card_view)
            .collect(),
        exile: player
            .exile()
            .iter()
            .filter_map(|handle| player.card_by_handle(*handle))
            .map(card_view)
            .collect(),
    }
}

fn card_view(card: &CardInstance) -> PublicCardView {
    PublicCardView {
        card_id: card.id().clone(),
        definition_id: card.definition_id().clone(),
        card_type: *card.card_type(),
    }
}

fn battlefield_card_view(card: &CardInstance) -> PublicBattlefieldCardView {
    PublicBattlefieldCardView {
        card_id: card.id().clone(),
        definition_id: card.definition_id().clone(),
        card_type: *card.card_type(),
        permanent_state: PublicPermanentStateView {
            tapped: card.is_tapped(),
            token: card.is_token(),
        },
        attached_to: card.attached_to().cloned(),
        power: card.power(),
        toughness: card.toughness(),
        loyalty: card.loyalty(),
        combat_state: PublicCombatStateView {
            summoning_sickness: card.has_summoning_sickness(),
            attacking: card.is_attacking(),
            blocking: card.is_blocking(),
        },
        keywords: keyword_list(card),
    }
}

fn keyword_list(card: &CardInstance) -> Vec<KeywordAbility> {
    const ORDER: [KeywordAbility; 13] = [
        KeywordAbility::Flying,
        KeywordAbility::Reach,
        KeywordAbility::Haste,
        KeywordAbility::Vigilance,
        KeywordAbility::Trample,
        KeywordAbility::FirstStrike,
        KeywordAbility::Deathtouch,
        KeywordAbility::DoubleStrike,
        KeywordAbility::Lifelink,
        KeywordAbility::Menace,
        KeywordAbility::Hexproof,
        KeywordAbility::Indestructible,
        KeywordAbility::Defender,
    ];

    let Some(keywords) = card.keyword_abilities() else {
        return Vec::new();
    };

    ORDER
        .into_iter()
        .filter(|ability| keywords.contains(*ability))
        .collect()
}

fn stack_object_view(
    game: &Game,
    object: &crate::domain::play::game::StackObject,
) -> PublicStackObjectView {
    let controller_id = game.players()[object.controller_index()].id().clone();
    match object.kind() {
        StackObjectKind::Spell(spell) => PublicStackObjectView::Spell {
            number: object.number(),
            controller_id,
            source_card_id: spell.source_card_id().clone(),
            card_type: *spell.card_type(),
            target: spell
                .target()
                .and_then(|target| stack_target_view(game, *target)),
            requires_choice: spell.choice().is_some(),
        },
        StackObjectKind::ActivatedAbility(ability) => PublicStackObjectView::ActivatedAbility {
            number: object.number(),
            controller_id,
            source_card_id: ability.source_card_id(),
            target: ability
                .target()
                .and_then(|target| stack_target_view(game, *target)),
        },
        StackObjectKind::TriggeredAbility(ability) => PublicStackObjectView::TriggeredAbility {
            number: object.number(),
            controller_id,
            source_card_id: ability.source_card_id(),
        },
    }
}

fn stack_target_view(
    game: &Game,
    target: crate::domain::play::game::model::StackTargetRef,
) -> Option<PublicStackTargetView> {
    match target {
        crate::domain::play::game::model::StackTargetRef::Player(index) => Some(
            PublicStackTargetView::Player(game.players()[index].id().clone()),
        ),
        crate::domain::play::game::model::StackTargetRef::Creature(card_ref)
        | crate::domain::play::game::model::StackTargetRef::Permanent(card_ref)
        | crate::domain::play::game::model::StackTargetRef::GraveyardCard(card_ref) => {
            let card = game.players()[card_ref.player_index()].card_by_handle(card_ref.handle())?;
            Some(PublicStackTargetView::Card(card.id().clone()))
        }
        crate::domain::play::game::model::StackTargetRef::StackSpell(number) => Some(
            PublicStackTargetView::StackSpell(StackObjectId::for_stack_object(game.id(), number)),
        ),
    }
}

fn playable_land_ids(game: &Game, player_id: &PlayerId) -> Vec<CardInstanceId> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    player
        .hand_card_ids()
        .into_iter()
        .filter(|card_id| game.can_play_land(player_id, card_id))
        .collect()
}

fn tappable_mana_source_ids(game: &Game, player_id: &PlayerId) -> Vec<CardInstanceId> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    player
        .battlefield_card_ids()
        .filter(|card_id| game.can_tap_mana_source(player_id, card_id))
        .cloned()
        .collect()
}

fn castable_cards(game: &Game, player_id: &PlayerId) -> Vec<PublicCastableCard> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    let mut candidates = player.hand_card_ids();
    candidates.extend(
        player
            .graveyard()
            .iter()
            .filter_map(|handle| player.card_by_handle(*handle))
            .map(|card| card.id().clone()),
    );

    candidates
        .into_iter()
        .filter_map(|card_id| castable_card(game, player_id, &card_id))
        .collect()
}

fn castable_card(
    game: &Game,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Option<PublicCastableCard> {
    let player = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)?;
    let card = player
        .hand_card(card_id)
        .or_else(|| player.graveyard_card(card_id))?;
    if game.castable_card(player_id, card_id) {
        let rules = card.supported_spell_rules();
        Some(PublicCastableCard {
            card_id: card.id().clone(),
            definition_id: card.definition_id().clone(),
            card_type: *card.card_type(),
            requires_target: rules.targeting().requires_target(),
            requires_choice: rules.requires_choice(),
        })
    } else {
        None
    }
}

fn spell_choice_request(
    game: &Game,
    player_id: &PlayerId,
    source_card_id: &CardInstanceId,
) -> Option<PublicChoiceRequest> {
    let player = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)?;
    let card = player
        .hand_card(source_card_id)
        .or_else(|| player.graveyard_card(source_card_id))?;
    let rules = card.supported_spell_rules();

    if rules.requires_explicit_hand_card_choice() {
        return Some(PublicChoiceRequest::SpellChoice {
            player_id: player_id.clone(),
            source_card_id: source_card_id.clone(),
            hand_card_ids: opponent_hand_choice_candidates(game, player_id),
        });
    }

    if rules.requires_explicit_secondary_creature_choice() {
        return Some(PublicChoiceRequest::SpellSecondaryCreatureChoice {
            player_id: player_id.clone(),
            source_card_id: source_card_id.clone(),
            creature_ids: spell_target_candidates(game, player_id, source_card_id)
                .into_iter()
                .filter_map(|candidate| match candidate {
                    PublicChoiceCandidate::Card(card_id) => Some(card_id),
                    PublicChoiceCandidate::Player(_) | PublicChoiceCandidate::StackSpell(_) => None,
                })
                .collect(),
            allows_skipping: true,
        });
    }

    if rules.requires_explicit_modal_choice() {
        return Some(PublicChoiceRequest::SpellModalChoice {
            player_id: player_id.clone(),
            source_card_id: source_card_id.clone(),
            modes: vec![
                PublicModalSpellChoice::TargetPlayerGainLife,
                PublicModalSpellChoice::TargetPlayerLoseLife,
            ],
        });
    }

    None
}

fn pending_optional_effect_request(game: &Game) -> Option<PublicChoiceRequest> {
    let crate::domain::play::game::PendingDecision::OptionalEffect {
        controller_index,
        stack_object_number,
    } = game.pending_decision()?
    else {
        return None;
    };
    let stack_object = game.stack().object(*stack_object_number)?;

    Some(PublicChoiceRequest::OptionalEffectDecision {
        player_id: game.players()[*controller_index].id().clone(),
        source_card_id: stack_object.source_card_id(),
        options: vec![PublicBinaryChoice::Yes, PublicBinaryChoice::No],
    })
}

fn pending_hand_choice_request(game: &Game) -> Option<PublicChoiceRequest> {
    let crate::domain::play::game::PendingDecision::HandChoice {
        controller_index,
        stack_object_number,
        ..
    } = game.pending_decision()?
    else {
        return None;
    };
    let stack_object = game.stack().object(*stack_object_number)?;

    Some(PublicChoiceRequest::PendingHandChoice {
        player_id: game.players()[*controller_index].id().clone(),
        source_card_id: stack_object.source_card_id(),
        hand_card_ids: game.players()[*controller_index].hand_card_ids(),
    })
}

fn pending_scry_request(game: &Game) -> Option<PublicChoiceRequest> {
    let crate::domain::play::game::PendingDecision::Scry {
        controller_index,
        stack_object_number,
        ..
    } = game.pending_decision()?
    else {
        return None;
    };
    let stack_object = game.stack().object(*stack_object_number)?;
    let top_card_id = game.players()[*controller_index].top_library_card_id()?;

    Some(PublicChoiceRequest::PendingScry {
        player_id: game.players()[*controller_index].id().clone(),
        source_card_id: stack_object.source_card_id(),
        looked_at_card_ids: vec![top_card_id],
        options: vec![PublicScryChoice::KeepOnTop, PublicScryChoice::MoveToBottom],
    })
}

fn pending_surveil_request(game: &Game) -> Option<PublicChoiceRequest> {
    let crate::domain::play::game::PendingDecision::Surveil {
        controller_index,
        stack_object_number,
        ..
    } = game.pending_decision()?
    else {
        return None;
    };
    let stack_object = game.stack().object(*stack_object_number)?;
    let top_card_id = game.players()[*controller_index].top_library_card_id()?;

    Some(PublicChoiceRequest::PendingSurveil {
        player_id: game.players()[*controller_index].id().clone(),
        source_card_id: stack_object.source_card_id(),
        looked_at_card_ids: vec![top_card_id],
        options: vec![
            PublicSurveilChoice::KeepOnTop,
            PublicSurveilChoice::MoveToGraveyard,
        ],
    })
}

fn activatable_cards(game: &Game, player_id: &PlayerId) -> Vec<PublicActivatableCard> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    player
        .battlefield_card_ids()
        .filter_map(|card_id| activatable_card(game, player_id, card_id))
        .collect()
}

fn activatable_card(
    game: &Game,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Option<PublicActivatableCard> {
    let player = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)?;
    let card = player.battlefield_card(card_id)?;
    let ability = card.activated_ability()?;
    if game.activatable_card(player_id, card_id) {
        Some(PublicActivatableCard {
            card_id: card.id().clone(),
            definition_id: card.definition_id().clone(),
            requires_target: ability.targeting().requires_target(),
        })
    } else {
        None
    }
}

fn attack_candidate_ids(game: &Game, player_id: &PlayerId) -> Vec<CardInstanceId> {
    let Some(player) = game
        .players()
        .iter()
        .find(|player| player.id() == player_id)
    else {
        return Vec::new();
    };

    player
        .battlefield_card_ids()
        .filter(|card_id| game.can_attack_with(player_id, card_id))
        .cloned()
        .collect()
}

fn attacking_creature_ids(game: &Game, player_id: &PlayerId) -> Vec<CardInstanceId> {
    game.players()
        .iter()
        .find(|player| player.id() == player_id)
        .map(|player| {
            player
                .battlefield_cards()
                .filter(|card| card.is_attacking())
                .map(|card| card.id().clone())
                .collect()
        })
        .unwrap_or_default()
}

fn spell_target_candidates(
    game: &Game,
    actor_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Vec<PublicChoiceCandidate> {
    game.spell_target_candidates(actor_id, card_id)
        .into_iter()
        .map(public_choice_candidate)
        .collect()
}

fn ability_target_candidates(
    game: &Game,
    actor_id: &PlayerId,
    source_card_id: &CardInstanceId,
) -> Vec<PublicChoiceCandidate> {
    game.ability_target_candidates(actor_id, source_card_id)
        .into_iter()
        .map(public_choice_candidate)
        .collect()
}

fn opponent_hand_choice_candidates(game: &Game, actor_id: &PlayerId) -> Vec<CardInstanceId> {
    game.players()
        .iter()
        .find(|player| player.id() != actor_id)
        .map(Player::hand_card_ids)
        .unwrap_or_default()
}

fn active_player(game: &Game) -> Option<&Player> {
    game.players()
        .iter()
        .find(|player| player.id() == game.active_player())
}

fn defending_player_id(game: &Game) -> Option<PlayerId> {
    game.players()
        .iter()
        .find(|player| player.id() != game.active_player())
        .map(|player| player.id().clone())
}

fn public_choice_candidate(
    target: crate::domain::play::game::SpellTarget,
) -> PublicChoiceCandidate {
    match target {
        crate::domain::play::game::SpellTarget::Player(player_id) => {
            PublicChoiceCandidate::Player(player_id)
        }
        crate::domain::play::game::SpellTarget::Creature(card_id)
        | crate::domain::play::game::SpellTarget::Permanent(card_id)
        | crate::domain::play::game::SpellTarget::GraveyardCard(card_id) => {
            PublicChoiceCandidate::Card(card_id)
        }
        crate::domain::play::game::SpellTarget::StackObject(stack_object_id) => {
            PublicChoiceCandidate::StackSpell(stack_object_id)
        }
    }
}
