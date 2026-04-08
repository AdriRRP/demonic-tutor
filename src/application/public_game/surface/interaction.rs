//! Projects the aggregate into the public gameplay read contract.

use std::collections::HashMap;

use crate::domain::play::{
    cards::CardInstance,
    game::{Game, Player},
    ids::{CardInstanceId, PlayerId},
    phase::Phase,
};

use super::super::{
    PublicActivatableCard, PublicBinaryChoice, PublicBlockerOption, PublicCastableCard,
    PublicChoiceCandidate, PublicChoiceRequest, PublicCommandApplication, PublicCommandResult,
    PublicLegalAction, PublicModalSpellChoice, PublicPendingDecisionKind, PublicScryChoice,
    PublicSurveilChoice,
};
use super::{game_view::game_view, players::SurfacePlayers};

#[derive(Debug, Default)]
pub(in crate::application::public_game) struct PublicSurfaceState {
    pub(super) legal_actions: Vec<PublicLegalAction>,
    pub(super) choice_requests: Vec<PublicChoiceRequest>,
}

impl PublicSurfaceState {
    fn with_choice_requests(
        legal_actions: Vec<PublicLegalAction>,
        mut choice_requests: Vec<PublicChoiceRequest>,
    ) -> Self {
        sort_choice_requests(&mut choice_requests);
        Self {
            legal_actions,
            choice_requests,
        }
    }

    pub(in crate::application::public_game) fn into_parts(
        self,
    ) -> (Vec<PublicLegalAction>, Vec<PublicChoiceRequest>) {
        (self.legal_actions, self.choice_requests)
    }
}

#[must_use]
fn pending_action_and_request(game: &Game) -> Option<PublicSurfaceState> {
    if let Some(pending_decision) = game.pending_decision() {
        let player_id = game
            .players()
            .get(pending_decision.controller_index())?
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
        return request
            .map(|request| PublicSurfaceState::with_choice_requests(vec![action], vec![request]));
    }

    None
}

fn unavailable_pending_decision_surface(
    viewer_id: &PlayerId,
    pending_decision: &crate::domain::play::game::PendingDecision,
) -> PublicSurfaceState {
    let player_id = viewer_id.clone();
    let (action, decision) = match pending_decision {
        crate::domain::play::game::PendingDecision::Scry { .. } => (
            PublicLegalAction::ResolvePendingScry {
                player_id: player_id.clone(),
            },
            PublicPendingDecisionKind::Scry,
        ),
        crate::domain::play::game::PendingDecision::Surveil { .. } => (
            PublicLegalAction::ResolvePendingSurveil {
                player_id: player_id.clone(),
            },
            PublicPendingDecisionKind::Surveil,
        ),
        crate::domain::play::game::PendingDecision::HandChoice { .. } => (
            PublicLegalAction::ResolvePendingHandChoice {
                player_id: player_id.clone(),
            },
            PublicPendingDecisionKind::HandChoice,
        ),
        crate::domain::play::game::PendingDecision::OptionalEffect { .. } => (
            PublicLegalAction::ResolveOptionalEffect {
                player_id: player_id.clone(),
            },
            PublicPendingDecisionKind::OptionalEffect,
        ),
    };

    PublicSurfaceState::with_choice_requests(
        vec![action],
        vec![PublicChoiceRequest::PendingDecisionUnavailable {
            player_id,
            decision,
        }],
    )
}

fn unavailable_priority_surface(viewer_id: &PlayerId) -> PublicSurfaceState {
    PublicSurfaceState::with_choice_requests(
        Vec::new(),
        vec![PublicChoiceRequest::PriorityUnavailable {
            player_id: viewer_id.clone(),
        }],
    )
}

fn unavailable_phase_surface(viewer_id: &PlayerId, phase: Phase) -> PublicSurfaceState {
    PublicSurfaceState::with_choice_requests(
        Vec::new(),
        vec![PublicChoiceRequest::PhaseUnavailable {
            player_id: viewer_id.clone(),
            phase,
        }],
    )
}

fn priority_surface_state(
    game: &Game,
    player: &Player,
    opponent: Option<&Player>,
) -> PublicSurfaceState {
    let player_id = player.id();

    let playable_land_ids = playable_land_ids(game, player);
    let mana_source_ids = tappable_mana_source_ids(game, player);
    let castable_cards = castable_cards(game, player);
    let activatable_cards = activatable_cards(game, player);
    let mut spell_target_candidates_by_card =
        spell_target_candidate_cache(game, player_id, &castable_cards);
    let mut ability_target_candidates_by_card =
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

    let mut choice_requests = Vec::new();
    for castable in &castable_cards {
        let cached_target_candidates = castable.requires_target.then(|| {
            take_spell_target_candidates(
                game,
                player_id,
                &mut spell_target_candidates_by_card,
                &castable.card_id,
            )
        });

        if castable.requires_choice {
            if let Some(request) = spell_choice_request(
                player,
                opponent,
                &castable.card_id,
                cached_target_candidates.as_deref(),
            ) {
                choice_requests.push(request);
            }
        }

        if castable.requires_target {
            choice_requests.push(PublicChoiceRequest::SpellTarget {
                player_id: player_id.clone(),
                source_card_id: castable.card_id.clone(),
                candidates: cached_target_candidates.unwrap_or_default(),
            });
        }
    }

    for activatable in &activatable_cards {
        if activatable.requires_target {
            let source_card_id = activatable.card_id.clone();
            choice_requests.push(PublicChoiceRequest::AbilityTarget {
                player_id: player_id.clone(),
                source_card_id: source_card_id.clone(),
                candidates: take_ability_target_candidates(
                    game,
                    player_id,
                    &mut ability_target_candidates_by_card,
                    &source_card_id,
                ),
            });
        }
    }

    if !castable_cards.is_empty() {
        actions.push(PublicLegalAction::CastSpell {
            player_id: player_id.clone(),
            castable_cards,
        });
    }

    if !activatable_cards.is_empty() {
        actions.push(PublicLegalAction::ActivateAbility {
            player_id: player_id.clone(),
            activatable_cards,
        });
    }

    PublicSurfaceState::with_choice_requests(actions, choice_requests)
}

fn take_spell_target_candidates(
    game: &Game,
    actor_id: &PlayerId,
    cache: &mut HashMap<CardInstanceId, Vec<PublicChoiceCandidate>>,
    source_card_id: &CardInstanceId,
) -> Vec<PublicChoiceCandidate> {
    cache
        .remove(source_card_id)
        .unwrap_or_else(|| spell_target_candidates(game, actor_id, source_card_id))
}

fn take_ability_target_candidates(
    game: &Game,
    actor_id: &PlayerId,
    cache: &mut HashMap<CardInstanceId, Vec<PublicChoiceCandidate>>,
    source_card_id: &CardInstanceId,
) -> Vec<PublicChoiceCandidate> {
    cache
        .remove(source_card_id)
        .unwrap_or_else(|| ability_target_candidates(game, actor_id, source_card_id))
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

fn phase_surface_state(
    game: &Game,
    viewer_id: &PlayerId,
    players: SurfacePlayers<'_>,
) -> PublicSurfaceState {
    let mut actions = Vec::new();
    let mut choice_requests = Vec::new();
    let Some(active_player) = players.active_player else {
        return unavailable_phase_surface(viewer_id, *game.phase());
    };

    match game.phase() {
        Phase::Setup => {
            return PublicSurfaceState::default();
        }
        Phase::DeclareAttackers => {
            if active_player.id() != viewer_id {
                return PublicSurfaceState::default();
            }
            actions.push(PublicLegalAction::DeclareAttackers {
                player_id: active_player.id().clone(),
                attacker_ids: attack_candidate_ids(game, active_player),
            });
        }
        Phase::DeclareBlockers => {
            let Some(defending_player) = players.defending_player else {
                return unavailable_phase_surface(viewer_id, *game.phase());
            };
            if defending_player.id() != viewer_id {
                return PublicSurfaceState::default();
            }
            let attacker_ids = attacking_creature_ids(active_player);
            actions.push(PublicLegalAction::DeclareBlockers {
                player_id: defending_player.id().clone(),
                attacker_ids,
                blocker_options: game
                    .blocker_options(defending_player.id())
                    .into_iter()
                    .map(|option| PublicBlockerOption {
                        blocker_id: option.blocker_id().clone(),
                        attacker_ids: option.attacker_ids().to_vec(),
                    })
                    .collect(),
            });
        }
        Phase::CombatDamage => {
            if active_player.id() != viewer_id {
                return PublicSurfaceState::default();
            }
            actions.push(PublicLegalAction::ResolveCombatDamage {
                player_id: active_player.id().clone(),
            });
        }
        Phase::EndStep => {
            if active_player.id() != viewer_id {
                return PublicSurfaceState::default();
            }
            if active_player.hand_size() > 7 {
                let hand_card_ids = active_player.hand_card_ids();
                actions.push(PublicLegalAction::DiscardForCleanup {
                    player_id: active_player.id().clone(),
                    card_ids: hand_card_ids.clone(),
                });
                choice_requests.push(PublicChoiceRequest::CleanupDiscard {
                    player_id: active_player.id().clone(),
                    hand_card_ids,
                });
            } else {
                actions.push(PublicLegalAction::AdvanceTurn {
                    player_id: active_player.id().clone(),
                });
            }
        }
        _ => {
            if active_player.id() != viewer_id {
                return PublicSurfaceState::default();
            }
            actions.push(PublicLegalAction::AdvanceTurn {
                player_id: active_player.id().clone(),
            });
        }
    }

    PublicSurfaceState::with_choice_requests(actions, choice_requests)
}

pub(in crate::application::public_game) fn public_surface_state(
    game: &Game,
    viewer_id: &PlayerId,
) -> PublicSurfaceState {
    if game.is_over() {
        return PublicSurfaceState::default();
    }

    let players = SurfacePlayers::resolve(game, viewer_id);
    let mut state = game.pending_decision().map_or_else(
        || {
            game.priority().map_or_else(
                || phase_surface_state(game, viewer_id, players),
                |priority| {
                    let current_holder = priority.current_holder();
                    if current_holder == viewer_id {
                        players.viewer_player.map_or_else(
                            || unavailable_priority_surface(viewer_id),
                            |player| priority_surface_state(game, player, players.viewer_opponent),
                        )
                    } else {
                        PublicSurfaceState::default()
                    }
                },
            )
        },
        |pending_decision| {
            let Some(controller_id) = game
                .players()
                .get(pending_decision.controller_index())
                .map(crate::domain::play::game::Player::id)
            else {
                return unavailable_pending_decision_surface(viewer_id, pending_decision);
            };

            if controller_id == viewer_id {
                pending_action_and_request(game).unwrap_or_else(|| {
                    unavailable_pending_decision_surface(viewer_id, pending_decision)
                })
            } else {
                PublicSurfaceState::default()
            }
        },
    );
    append_concede_action(game, viewer_id, &mut state.legal_actions);
    state
}

#[must_use]
pub fn legal_actions(game: &Game, viewer_id: &PlayerId) -> Vec<PublicLegalAction> {
    public_surface_state(game, viewer_id).legal_actions
}

#[must_use]
pub fn choice_requests(game: &Game, viewer_id: &PlayerId) -> Vec<PublicChoiceRequest> {
    public_surface_state(game, viewer_id).choice_requests
}

#[must_use]
pub fn public_command_result(
    game: &Game,
    application: PublicCommandApplication,
    viewer_id: &PlayerId,
) -> PublicCommandResult {
    let surface = public_surface_state(game, viewer_id);

    PublicCommandResult {
        status: application.status,
        emitted_events: application.emitted_events,
        game: game_view(game),
        legal_actions: surface.legal_actions,
        choice_requests: surface.choice_requests,
    }
}

fn playable_land_ids(game: &Game, player: &Player) -> Vec<CardInstanceId> {
    player
        .hand_card_ids()
        .into_iter()
        .filter(|card_id| game.can_play_land(player.id(), card_id))
        .collect()
}

fn tappable_mana_source_ids(game: &Game, player: &Player) -> Vec<CardInstanceId> {
    player
        .battlefield_card_ids()
        .filter(|card_id| game.can_tap_mana_source(player.id(), card_id))
        .cloned()
        .collect()
}

fn castable_cards(game: &Game, player: &Player) -> Vec<PublicCastableCard> {
    player
        .hand_cards()
        .chain(player.graveyard_cards())
        .filter_map(|card| castable_card(game, player.id(), card))
        .collect()
}

fn castable_card(
    game: &Game,
    player_id: &PlayerId,
    card: &CardInstance,
) -> Option<PublicCastableCard> {
    if game.castable_card(player_id, card.id()) {
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

pub(super) fn spell_choice_request(
    player: &Player,
    opponent: Option<&Player>,
    source_card_id: &CardInstanceId,
    cached_target_candidates: Option<&[PublicChoiceCandidate]>,
) -> Option<PublicChoiceRequest> {
    let card = player
        .hand_card(source_card_id)
        .or_else(|| player.graveyard_card(source_card_id))?;
    let rules = card.supported_spell_rules();

    if rules.requires_explicit_hand_card_choice() {
        return Some(opponent.map_or_else(
            || PublicChoiceRequest::SpellChoiceInvariantViolation {
                player_id: player.id().clone(),
                source_card_id: source_card_id.clone(),
            },
            |opponent| PublicChoiceRequest::SpellChoice {
                player_id: player.id().clone(),
                source_card_id: source_card_id.clone(),
                hand_card_ids: opponent.hand_card_ids(),
            },
        ));
    }

    if rules.requires_explicit_secondary_creature_choice() {
        let Some(cached_target_candidates) = cached_target_candidates else {
            return Some(
                PublicChoiceRequest::SpellSecondaryCreatureChoiceUnavailable {
                    player_id: player.id().clone(),
                    source_card_id: source_card_id.clone(),
                },
            );
        };
        return Some(PublicChoiceRequest::SpellSecondaryCreatureChoice {
            player_id: player.id().clone(),
            source_card_id: source_card_id.clone(),
            creature_ids: cached_target_candidates
                .iter()
                .filter_map(|candidate| match candidate {
                    PublicChoiceCandidate::Card(card_id) => Some(card_id.clone()),
                    PublicChoiceCandidate::Player(_) | PublicChoiceCandidate::StackSpell(_) => None,
                })
                .collect(),
            allows_skipping: true,
        });
    }

    if rules.requires_explicit_modal_choice() {
        return Some(PublicChoiceRequest::SpellModalChoice {
            player_id: player.id().clone(),
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
    let player = game.players().get(*controller_index)?;

    Some(PublicChoiceRequest::OptionalEffectDecision {
        player_id: player.id().clone(),
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
    let player = game.players().get(*controller_index)?;

    Some(PublicChoiceRequest::PendingHandChoice {
        player_id: player.id().clone(),
        source_card_id: stack_object.source_card_id(),
        hand_card_ids: player.hand_card_ids(),
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
    let player = game.players().get(*controller_index)?;
    let top_card_id = player.top_library_card_id()?;

    Some(PublicChoiceRequest::PendingScry {
        player_id: player.id().clone(),
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
    let player = game.players().get(*controller_index)?;
    let top_card_id = player.top_library_card_id()?;

    Some(PublicChoiceRequest::PendingSurveil {
        player_id: player.id().clone(),
        source_card_id: stack_object.source_card_id(),
        looked_at_card_ids: vec![top_card_id],
        options: vec![
            PublicSurveilChoice::KeepOnTop,
            PublicSurveilChoice::MoveToGraveyard,
        ],
    })
}

fn activatable_cards(game: &Game, player: &Player) -> Vec<PublicActivatableCard> {
    player
        .battlefield_cards()
        .filter_map(|card| activatable_card(game, player.id(), card))
        .collect()
}

fn activatable_card(
    game: &Game,
    player_id: &PlayerId,
    card: &CardInstance,
) -> Option<PublicActivatableCard> {
    let ability = card.activated_ability()?;
    if game.activatable_card(player_id, card.id()) {
        Some(PublicActivatableCard {
            card_id: card.id().clone(),
            definition_id: card.definition_id().clone(),
            requires_target: ability.targeting().requires_target(),
        })
    } else {
        None
    }
}

fn attack_candidate_ids(game: &Game, player: &Player) -> Vec<CardInstanceId> {
    player
        .battlefield_card_ids()
        .filter(|card_id| game.can_attack_with(player.id(), card_id))
        .cloned()
        .collect()
}

fn attacking_creature_ids(player: &Player) -> Vec<CardInstanceId> {
    player
        .battlefield_cards()
        .filter(|card| card.is_attacking())
        .map(|card| card.id().clone())
        .collect()
}

fn spell_target_candidates(
    game: &Game,
    actor_id: &PlayerId,
    card_id: &CardInstanceId,
) -> Vec<PublicChoiceCandidate> {
    let mut candidates = game
        .spell_target_candidates(actor_id, card_id)
        .into_iter()
        .map(public_choice_candidate)
        .collect::<Vec<_>>();
    sort_choice_candidates(&mut candidates);
    candidates
}

fn ability_target_candidates(
    game: &Game,
    actor_id: &PlayerId,
    source_card_id: &CardInstanceId,
) -> Vec<PublicChoiceCandidate> {
    let mut candidates = game
        .ability_target_candidates(actor_id, source_card_id)
        .into_iter()
        .map(public_choice_candidate)
        .collect::<Vec<_>>();
    sort_choice_candidates(&mut candidates);
    candidates
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

fn append_concede_action(
    game: &Game,
    viewer_id: &PlayerId,
    legal_actions: &mut Vec<PublicLegalAction>,
) {
    if game.players().iter().any(|player| player.id() == viewer_id) {
        legal_actions.push(PublicLegalAction::Concede {
            player_id: viewer_id.clone(),
        });
    }
}

fn sort_choice_requests(choice_requests: &mut [PublicChoiceRequest]) {
    choice_requests
        .sort_by(|left, right| choice_request_sort_key(left).cmp(&choice_request_sort_key(right)));
}

fn choice_request_sort_key(request: &PublicChoiceRequest) -> (u8, &str, &str) {
    match request {
        PublicChoiceRequest::PriorityUnavailable { player_id } => (0, player_id.as_str(), ""),
        PublicChoiceRequest::PhaseUnavailable { player_id, phase } => {
            (1, player_id.as_str(), phase_sort_key(*phase))
        }
        PublicChoiceRequest::PendingDecisionUnavailable {
            player_id,
            decision,
        } => (
            2 + match decision {
                PublicPendingDecisionKind::Scry => 1,
                PublicPendingDecisionKind::Surveil => 2,
                PublicPendingDecisionKind::HandChoice => 3,
                PublicPendingDecisionKind::OptionalEffect => 4,
            },
            player_id.as_str(),
            "",
        ),
        PublicChoiceRequest::PendingScry {
            player_id,
            source_card_id,
            ..
        } => (6, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::PendingSurveil {
            player_id,
            source_card_id,
            ..
        } => (7, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::PendingHandChoice {
            player_id,
            source_card_id,
            ..
        } => (8, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::OptionalEffectDecision {
            player_id,
            source_card_id,
            ..
        } => (9, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellTarget {
            player_id,
            source_card_id,
            ..
        } => (10, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellChoiceInvariantViolation {
            player_id,
            source_card_id,
        } => (11, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellChoice {
            player_id,
            source_card_id,
            ..
        } => (12, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellSecondaryCreatureChoiceUnavailable {
            player_id,
            source_card_id,
        } => (13, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellSecondaryCreatureChoice {
            player_id,
            source_card_id,
            ..
        } => (14, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellModalChoice {
            player_id,
            source_card_id,
            ..
        } => (15, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::AbilityTarget {
            player_id,
            source_card_id,
            ..
        } => (16, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::CleanupDiscard { player_id, .. } => (17, player_id.as_str(), ""),
    }
}

const fn phase_sort_key(phase: Phase) -> &'static str {
    match phase {
        Phase::Setup => "Setup",
        Phase::Untap => "Untap",
        Phase::Upkeep => "Upkeep",
        Phase::Draw => "Draw",
        Phase::FirstMain => "FirstMain",
        Phase::BeginningOfCombat => "BeginningOfCombat",
        Phase::DeclareAttackers => "DeclareAttackers",
        Phase::DeclareBlockers => "DeclareBlockers",
        Phase::CombatDamage => "CombatDamage",
        Phase::EndOfCombat => "EndOfCombat",
        Phase::SecondMain => "SecondMain",
        Phase::EndStep => "EndStep",
    }
}

fn sort_choice_candidates(candidates: &mut [PublicChoiceCandidate]) {
    candidates.sort_by(|left, right| {
        choice_candidate_sort_key(left).cmp(&choice_candidate_sort_key(right))
    });
}

fn choice_candidate_sort_key(candidate: &PublicChoiceCandidate) -> (u8, &str) {
    match candidate {
        PublicChoiceCandidate::Player(player_id) => (0, player_id.as_str()),
        PublicChoiceCandidate::Card(card_id) => (1, card_id.as_str()),
        PublicChoiceCandidate::StackSpell(stack_object_id) => (2, stack_object_id.as_str()),
    }
}
