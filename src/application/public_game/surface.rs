//! Projects the aggregate into the public gameplay read contract.

use std::{borrow::Borrow, collections::HashMap, sync::Arc};

use crate::domain::play::{
    cards::{CardInstance, KeywordAbility},
    events::DomainEvent,
    game::{Game, Player, StackObjectKind},
    ids::{CardInstanceId, PlayerId, StackObjectId},
    phase::Phase,
};

use super::{
    PublicActivatableCard, PublicBattlefieldCardView, PublicBinaryChoice, PublicBlockerOption,
    PublicCardDrawn, PublicCardView, PublicCastableCard, PublicChoiceCandidate,
    PublicChoiceRequest, PublicCombatStateView, PublicCommandApplication, PublicCommandResult,
    PublicEvent, PublicEventLogEntry, PublicGameView, PublicLegalAction, PublicModalSpellChoice,
    PublicOpeningHandDealt, PublicPendingDecisionKind, PublicPermanentStateView,
    PublicPlayableSubsetVersion, PublicPlayerView, PublicPriorityView, PublicScryChoice,
    PublicStackObjectView, PublicStackTargetView, PublicSurveilChoice,
};

#[derive(Debug, Default)]
pub(super) struct PublicSurfaceState {
    legal_actions: Vec<PublicLegalAction>,
    choice_requests: Vec<PublicChoiceRequest>,
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

    pub(super) fn into_parts(self) -> (Vec<PublicLegalAction>, Vec<PublicChoiceRequest>) {
        (self.legal_actions, self.choice_requests)
    }
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
        .filter_map(|object| stack_object_view(game, object))
        .collect();

    PublicGameView {
        game_id: game.id().clone(),
        playable_subset_version: PublicPlayableSubsetVersion::V1,
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

fn priority_surface_state(game: &Game, player: &Player) -> PublicSurfaceState {
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
                game,
                player,
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

fn phase_surface_state(game: &Game, viewer_id: &PlayerId) -> PublicSurfaceState {
    let mut actions = Vec::new();
    let mut choice_requests = Vec::new();
    let Some(active_player) = active_player(game) else {
        return PublicSurfaceState::default();
    };

    match game.phase() {
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
            let Some(defending_player) = defending_player(game, active_player) else {
                return PublicSurfaceState::default();
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
                actions.push(PublicLegalAction::DiscardForCleanup {
                    player_id: active_player.id().clone(),
                    card_ids: active_player.hand_card_ids(),
                });
                choice_requests.push(PublicChoiceRequest::CleanupDiscard {
                    player_id: active_player.id().clone(),
                    hand_card_ids: active_player.hand_card_ids(),
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

pub(super) fn public_surface_state(game: &Game, viewer_id: &PlayerId) -> PublicSurfaceState {
    if game.is_over() {
        return PublicSurfaceState::default();
    }

    let mut state = game.pending_decision().map_or_else(
        || {
            game.priority().map_or_else(
                || phase_surface_state(game, viewer_id),
                |priority| {
                    let current_holder = priority.current_holder();
                    if current_holder == viewer_id {
                        player_by_id(game, current_holder).map_or_else(
                            || unavailable_priority_surface(viewer_id),
                            |player| priority_surface_state(game, player),
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

#[must_use]
pub fn public_event_log<I>(events: I) -> Arc<[PublicEventLogEntry]>
where
    I: IntoIterator,
    I::Item: Borrow<DomainEvent>,
{
    Arc::from(
        events
            .into_iter()
            .zip(1_u64..)
            .map(|(event, sequence)| PublicEventLogEntry {
                sequence,
                event: public_event(event.borrow()),
            })
            .collect::<Vec<_>>(),
    )
}

pub(super) fn public_events<I>(events: I) -> Vec<PublicEvent>
where
    I: IntoIterator,
    I::Item: Borrow<DomainEvent>,
{
    events
        .into_iter()
        .map(|event| public_event(event.borrow()))
        .collect()
}

fn public_event(event: &DomainEvent) -> PublicEvent {
    match event {
        DomainEvent::GameStarted(event) => PublicEvent::GameStarted(event.clone()),
        DomainEvent::OpeningHandDealt(event) => {
            PublicEvent::OpeningHandDealt(PublicOpeningHandDealt {
                game_id: event.game_id.clone(),
                player_id: event.player_id.clone(),
                card_count: event.cards.len(),
            })
        }
        DomainEvent::GameEnded(event) => PublicEvent::GameEnded(event.clone()),
        DomainEvent::LandPlayed(event) => PublicEvent::LandPlayed(event.clone()),
        DomainEvent::TurnProgressed(event) => PublicEvent::TurnProgressed(event.clone()),
        DomainEvent::CardDrawn(event) => PublicEvent::CardDrawn(PublicCardDrawn {
            game_id: event.game_id.clone(),
            player_id: event.player_id.clone(),
            draw_kind: event.draw_kind,
        }),
        DomainEvent::CardDiscarded(event) => PublicEvent::CardDiscarded(event.clone()),
        DomainEvent::MulliganTaken(event) => PublicEvent::MulliganTaken(event.clone()),
        DomainEvent::LifeChanged(event) => PublicEvent::LifeChanged(event.clone()),
        DomainEvent::LandTapped(event) => PublicEvent::LandTapped(event.clone()),
        DomainEvent::ManaAdded(event) => PublicEvent::ManaAdded(event.clone()),
        DomainEvent::ActivatedAbilityPutOnStack(event) => {
            PublicEvent::ActivatedAbilityPutOnStack(event.clone())
        }
        DomainEvent::TriggeredAbilityPutOnStack(event) => {
            PublicEvent::TriggeredAbilityPutOnStack(event.clone())
        }
        DomainEvent::SpellPutOnStack(event) => PublicEvent::SpellPutOnStack(event.clone()),
        DomainEvent::PriorityPassed(event) => PublicEvent::PriorityPassed(event.clone()),
        DomainEvent::StackTopResolved(event) => PublicEvent::StackTopResolved(event.clone()),
        DomainEvent::SpellCast(event) => PublicEvent::SpellCast(event.clone()),
        DomainEvent::AttackersDeclared(event) => PublicEvent::AttackersDeclared(event.clone()),
        DomainEvent::BlockersDeclared(event) => PublicEvent::BlockersDeclared(event.clone()),
        DomainEvent::CombatDamageResolved(event) => {
            PublicEvent::CombatDamageResolved(event.clone())
        }
        DomainEvent::CreatureDied(event) => PublicEvent::CreatureDied(event.clone()),
        DomainEvent::CardMovedZone(event) => PublicEvent::CardMovedZone(event.clone()),
    }
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
) -> Option<PublicStackObjectView> {
    let controller_id = game.players().get(object.controller_index())?.id().clone();
    Some(match object.kind() {
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
    })
}

fn stack_target_view(
    game: &Game,
    target: crate::domain::play::game::model::StackTargetRef,
) -> Option<PublicStackTargetView> {
    match target {
        crate::domain::play::game::model::StackTargetRef::Player(index) => game
            .players()
            .get(index)
            .map(|player| PublicStackTargetView::Player(player.id().clone())),
        crate::domain::play::game::model::StackTargetRef::Creature(card_ref)
        | crate::domain::play::game::model::StackTargetRef::Permanent(card_ref)
        | crate::domain::play::game::model::StackTargetRef::GraveyardCard(card_ref) => {
            let card = game
                .players()
                .get(card_ref.player_index())?
                .card_by_handle(card_ref.handle())?;
            Some(PublicStackTargetView::Card(card.id().clone()))
        }
        crate::domain::play::game::model::StackTargetRef::StackSpell(number) => Some(
            PublicStackTargetView::StackSpell(StackObjectId::for_stack_object(game.id(), number)),
        ),
    }
}

fn player_by_id<'a>(game: &'a Game, player_id: &PlayerId) -> Option<&'a Player> {
    let players = game.players();
    match players {
        [first, second] => {
            if first.id() == player_id {
                Some(first)
            } else if second.id() == player_id {
                Some(second)
            } else {
                None
            }
        }
        _ => players.iter().find(|player| player.id() == player_id),
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
        .filter_map(|card_id| castable_card(game, player, &card_id))
        .collect()
}

fn castable_card(
    game: &Game,
    player: &Player,
    card_id: &CardInstanceId,
) -> Option<PublicCastableCard> {
    let card = player
        .hand_card(card_id)
        .or_else(|| player.graveyard_card(card_id))?;
    if game.castable_card(player.id(), card_id) {
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
    player: &Player,
    source_card_id: &CardInstanceId,
    cached_target_candidates: Option<&[PublicChoiceCandidate]>,
) -> Option<PublicChoiceRequest> {
    let card = player
        .hand_card(source_card_id)
        .or_else(|| player.graveyard_card(source_card_id))?;
    let rules = card.supported_spell_rules();

    if rules.requires_explicit_hand_card_choice() {
        return Some(
            opponent_hand_choice_candidates(game.players(), player.id()).map_or_else(
                || PublicChoiceRequest::SpellChoiceUnavailable {
                    player_id: player.id().clone(),
                    source_card_id: source_card_id.clone(),
                },
                |hand_card_ids| PublicChoiceRequest::SpellChoice {
                    player_id: player.id().clone(),
                    source_card_id: source_card_id.clone(),
                    hand_card_ids,
                },
            ),
        );
    }

    if rules.requires_explicit_secondary_creature_choice() {
        return Some(PublicChoiceRequest::SpellSecondaryCreatureChoice {
            player_id: player.id().clone(),
            source_card_id: source_card_id.clone(),
            creature_ids: cached_target_candidates
                .unwrap_or(&[])
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
        .battlefield_card_ids()
        .filter_map(|card_id| activatable_card(game, player, card_id))
        .collect()
}

fn activatable_card(
    game: &Game,
    player: &Player,
    card_id: &CardInstanceId,
) -> Option<PublicActivatableCard> {
    let card = player.battlefield_card(card_id)?;
    let ability = card.activated_ability()?;
    if game.activatable_card(player.id(), card_id) {
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

fn opponent_hand_choice_candidates(
    players: &[Player],
    actor_id: &PlayerId,
) -> Option<Vec<CardInstanceId>> {
    match players {
        [first, second] => {
            if first.id() == actor_id {
                Some(second.hand_card_ids())
            } else if second.id() == actor_id {
                Some(first.hand_card_ids())
            } else {
                None
            }
        }
        _ => players
            .iter()
            .find(|player| player.id() != actor_id)
            .map(Player::hand_card_ids),
    }
}

fn active_player(game: &Game) -> Option<&Player> {
    player_by_id(game, game.active_player())
}

fn defending_player<'a>(game: &'a Game, active_player: &Player) -> Option<&'a Player> {
    let players = game.players();
    match players {
        [first, second] => {
            if first.id() == active_player.id() {
                Some(second)
            } else if second.id() == active_player.id() {
                Some(first)
            } else {
                None
            }
        }
        _ => players
            .iter()
            .find(|player| player.id() != active_player.id()),
    }
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
        PublicChoiceRequest::PendingDecisionUnavailable {
            player_id,
            decision,
        } => (
            match decision {
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
        } => (5, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::PendingSurveil {
            player_id,
            source_card_id,
            ..
        } => (6, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::PendingHandChoice {
            player_id,
            source_card_id,
            ..
        } => (7, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::OptionalEffectDecision {
            player_id,
            source_card_id,
            ..
        } => (8, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellTarget {
            player_id,
            source_card_id,
            ..
        } => (9, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellChoiceUnavailable {
            player_id,
            source_card_id,
        } => (10, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellChoice {
            player_id,
            source_card_id,
            ..
        } => (11, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellSecondaryCreatureChoice {
            player_id,
            source_card_id,
            ..
        } => (12, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::SpellModalChoice {
            player_id,
            source_card_id,
            ..
        } => (13, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::AbilityTarget {
            player_id,
            source_card_id,
            ..
        } => (14, player_id.as_str(), source_card_id.as_str()),
        PublicChoiceRequest::CleanupDiscard { player_id, .. } => (15, player_id.as_str(), ""),
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

#[cfg(test)]
mod tests {
    //! Verifies the public surface keeps pending-decision failures explicit.

    use super::public_surface_state;
    use crate::{
        domain::play::{
            commands::{PlayerDeck, StartGameCommand},
            game::{PendingDecision, PriorityState},
            ids::{DeckId, GameId, PlayerId},
        },
        PublicChoiceRequest, PublicLegalAction, PublicPendingDecisionKind,
    };

    #[test]
    fn pending_decision_surface_stays_explicit_when_request_payload_cannot_be_built() {
        let start = crate::domain::play::game::Game::start(StartGameCommand::new(
            GameId::new("game-unavailable-pending-decision"),
            vec![
                PlayerDeck::new(PlayerId::new("p1"), DeckId::new("d1")),
                PlayerDeck::new(PlayerId::new("p2"), DeckId::new("d2")),
            ],
        ));
        assert!(start.is_ok(), "game should start");
        let Some((mut game, _)) = start.ok() else {
            return;
        };
        game.replace_pending_decision(Some(PendingDecision::scry(0, 999, 1)));

        let surface = public_surface_state(&game, &PlayerId::new("p1"));

        assert!(surface.legal_actions.iter().any(|action| matches!(
            action,
            PublicLegalAction::ResolvePendingScry { player_id }
                if player_id.as_str() == "p1"
        )));
        assert!(surface.choice_requests.iter().any(|request| matches!(
            request,
            PublicChoiceRequest::PendingDecisionUnavailable { player_id, decision }
                if player_id.as_str() == "p1" && *decision == PublicPendingDecisionKind::Scry
        )));
    }

    #[test]
    fn pending_decision_surface_does_not_panic_when_controller_index_is_stale() {
        let start = crate::domain::play::game::Game::start(StartGameCommand::new(
            GameId::new("game-stale-pending-controller"),
            vec![
                PlayerDeck::new(PlayerId::new("p1"), DeckId::new("d1")),
                PlayerDeck::new(PlayerId::new("p2"), DeckId::new("d2")),
            ],
        ));
        assert!(start.is_ok(), "game should start");
        let Some((mut game, _)) = start.ok() else {
            return;
        };
        game.replace_pending_decision(Some(PendingDecision::scry(99, 999, 1)));

        let surface = public_surface_state(&game, &PlayerId::new("p1"));

        assert!(surface.legal_actions.iter().any(|action| matches!(
            action,
            PublicLegalAction::ResolvePendingScry { player_id }
                if player_id.as_str() == "p1"
        )));
        assert!(surface.choice_requests.iter().any(|request| matches!(
            request,
            PublicChoiceRequest::PendingDecisionUnavailable { player_id, decision }
                if player_id.as_str() == "p1" && *decision == PublicPendingDecisionKind::Scry
        )));
    }

    #[test]
    fn priority_surface_stays_explicit_when_priority_holder_is_missing() {
        let start = crate::domain::play::game::Game::start(StartGameCommand::new(
            GameId::new("game-unavailable-priority-holder"),
            vec![
                PlayerDeck::new(PlayerId::new("p1"), DeckId::new("d1")),
                PlayerDeck::new(PlayerId::new("p2"), DeckId::new("d2")),
            ],
        ));
        assert!(start.is_ok(), "game should start");
        let Some((mut game, _)) = start.ok() else {
            return;
        };
        game.replace_priority(Some(PriorityState::opened(PlayerId::new("ghost"))));

        let surface = public_surface_state(&game, &PlayerId::new("ghost"));

        assert!(surface.choice_requests.iter().any(|request| matches!(
            request,
            PublicChoiceRequest::PriorityUnavailable { player_id }
                if player_id.as_str() == "ghost"
        )));
    }
}
