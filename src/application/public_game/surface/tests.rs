//! Verifies the public surface keeps degraded projections explicit.

#![allow(clippy::expect_used)]

use super::interaction::spell_choice_request;
use super::{game_view, public_surface_state};
use crate::{
    domain::play::{
        cards::ActivatedAbilityProfile,
        cards::{CardType, ManaColor, SupportedSpellRules},
        commands::{
            DealOpeningHandsCommand, LibraryCard, PlayerDeck, PlayerLibrary, StartGameCommand,
        },
        game::{
            model::{
                ActivatedAbilityOnStack, StackCardRef, StackObject, StackObjectKind,
                StackTargetRef, StackZone,
            },
            PendingDecision, PriorityState,
        },
        ids::{CardDefinitionId, CardInstanceId, DeckId, GameId, PlayerCardHandle, PlayerId},
        phase::Phase,
    },
    PublicChoiceRequest, PublicLegalAction, PublicPendingDecisionKind, PublicStackObjectView,
    PublicStackTargetView,
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

#[test]
fn game_view_keeps_stack_object_visible_when_controller_index_is_stale() {
    let start = crate::domain::play::game::Game::start(StartGameCommand::new(
        GameId::new("game-stale-stack-controller"),
        vec![
            PlayerDeck::new(PlayerId::new("p1"), DeckId::new("d1")),
            PlayerDeck::new(PlayerId::new("p2"), DeckId::new("d2")),
        ],
    ));
    assert!(start.is_ok(), "game should start");
    let Some((mut game, _)) = start.ok() else {
        return;
    };

    let source_card_id = CardInstanceId::new("stale-ability-source");
    let mut stack = StackZone::empty();
    stack.push(StackObject::new(
        1,
        99,
        StackObjectKind::ActivatedAbility(ActivatedAbilityOnStack::new(
            StackCardRef::new(0, PlayerCardHandle::new(0)),
            source_card_id,
            ActivatedAbilityProfile::tap_to_gain_life_to_target_player(1),
            Some(StackTargetRef::Player(1)),
        )),
    ));
    game.replace_stack(stack);

    let view = game_view(&game);

    assert!(matches!(
        view.stack.as_slice(),
        [PublicStackObjectView::Unavailable { number: 1 }]
    ));
}

#[test]
fn game_view_marks_stack_targets_unavailable_when_target_refs_are_stale() {
    let start = crate::domain::play::game::Game::start(StartGameCommand::new(
        GameId::new("game-stale-stack-target"),
        vec![
            PlayerDeck::new(PlayerId::new("p1"), DeckId::new("d1")),
            PlayerDeck::new(PlayerId::new("p2"), DeckId::new("d2")),
        ],
    ));
    assert!(start.is_ok(), "game should start");
    let Some((mut game, _)) = start.ok() else {
        return;
    };

    let source_card_id = CardInstanceId::new("stale-target-ability");
    let mut stack = StackZone::empty();
    stack.push(StackObject::new(
        1,
        0,
        StackObjectKind::ActivatedAbility(ActivatedAbilityOnStack::new(
            StackCardRef::new(0, PlayerCardHandle::new(0)),
            source_card_id,
            ActivatedAbilityProfile::tap_to_gain_life_to_target_player(1),
            Some(StackTargetRef::Player(99)),
        )),
    ));
    game.replace_stack(stack);

    let view = game_view(&game);

    assert!(matches!(
        view.stack.as_slice(),
        [PublicStackObjectView::ActivatedAbility {
            target: Some(PublicStackTargetView::Unavailable),
            ..
        }]
    ));
}

#[test]
fn phase_surface_stays_explicit_when_active_player_index_is_stale() {
    let start = crate::domain::play::game::Game::start(StartGameCommand::new(
        GameId::new("game-unavailable-phase"),
        vec![
            PlayerDeck::new(PlayerId::new("p1"), DeckId::new("d1")),
            PlayerDeck::new(PlayerId::new("p2"), DeckId::new("d2")),
        ],
    ));
    assert!(start.is_ok(), "game should start");
    let Some((mut game, _)) = start.ok() else {
        return;
    };
    game.replace_active_player_index(99);
    game.replace_phase(Phase::FirstMain);

    let surface = public_surface_state(&game, &PlayerId::new("p1"));

    assert!(surface.choice_requests.iter().any(|request| matches!(
        request,
        PublicChoiceRequest::PhaseUnavailable { player_id, phase }
            if player_id.as_str() == "p1" && *phase == Phase::FirstMain
    )));
}

#[test]
fn game_view_surfaces_missing_active_player_without_inventing_public_ids() {
    let start = crate::domain::play::game::Game::start(StartGameCommand::new(
        GameId::new("game-unavailable-active-player"),
        vec![
            PlayerDeck::new(PlayerId::new("p1"), DeckId::new("d1")),
            PlayerDeck::new(PlayerId::new("p2"), DeckId::new("d2")),
        ],
    ));
    assert!(start.is_ok(), "game should start");
    let Some((mut game, _)) = start.ok() else {
        return;
    };
    game.replace_active_player_index(99);

    let view = game_view(&game);

    assert_eq!(view.active_player_id, None);
    assert!(view.players.iter().all(|player| !player.is_active));
}

#[test]
fn spell_choice_request_stays_explicit_when_secondary_target_candidates_are_missing() {
    let start = crate::domain::play::game::Game::start(StartGameCommand::new(
        GameId::new("game-missing-secondary-choice-candidates"),
        vec![
            PlayerDeck::new(PlayerId::new("p1"), DeckId::new("d1")),
            PlayerDeck::new(PlayerId::new("p2"), DeckId::new("d2")),
        ],
    ));
    assert!(start.is_ok(), "game should start");
    let Some((mut game, _)) = start.ok() else {
        return;
    };

    let libraries = vec![
        PlayerLibrary::new(
            PlayerId::new("p1"),
            vec![
                LibraryCard::new(CardDefinitionId::new("distribute-counters"), CardType::Sorcery, 0)
                    .with_supported_spell_rules(
                        SupportedSpellRules::distribute_two_plus_one_plus_one_counters_among_up_to_two_target_creatures(),
                    ),
                LibraryCard::land(CardDefinitionId::new("p1-forest-a"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-b"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-c"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-d"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-e"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-f"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-g"), ManaColor::Green),
            ],
        ),
        PlayerLibrary::new(
            PlayerId::new("p2"),
            vec![
                LibraryCard::land(CardDefinitionId::new("p2-forest-a"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-b"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-c"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-d"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-e"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-f"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-g"), ManaColor::Green),
            ],
        ),
    ];
    let dealt = game.deal_opening_hands(&DealOpeningHandsCommand::new(libraries));
    assert!(dealt.is_ok(), "opening hands should be dealt");

    let player = game.players().first().expect("p1 should exist");
    let Some(spell_id) = player
        .hand_card_by_definition(&CardDefinitionId::new("distribute-counters"))
        .map(|card| card.id().clone())
    else {
        return;
    };

    let request = spell_choice_request(player, None, &spell_id, None);

    assert!(matches!(
        request,
        Some(PublicChoiceRequest::SpellSecondaryCreatureChoiceUnavailable {
            player_id,
            source_card_id,
        }) if player_id.as_str() == "p1" && source_card_id == spell_id
    ));
}

#[test]
fn spell_choice_request_surfaces_opponent_lookup_as_invariant_violation() {
    let start = crate::domain::play::game::Game::start(StartGameCommand::new(
        GameId::new("game-discard-choice-source"),
        vec![
            PlayerDeck::new(PlayerId::new("p1"), DeckId::new("d1")),
            PlayerDeck::new(PlayerId::new("p2"), DeckId::new("d2")),
        ],
    ));
    assert!(start.is_ok(), "game should start");
    let Some((mut game, _)) = start.ok() else {
        return;
    };

    let libraries = vec![
        PlayerLibrary::new(
            PlayerId::new("p1"),
            vec![
                LibraryCard::new(
                    CardDefinitionId::new("discard-choice"),
                    CardType::Sorcery,
                    0,
                )
                .with_supported_spell_rules(
                    SupportedSpellRules::target_player_discards_chosen_card(),
                ),
                LibraryCard::land(CardDefinitionId::new("p1-forest-a"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-b"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-c"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-d"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-e"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-f"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p1-forest-g"), ManaColor::Green),
            ],
        ),
        PlayerLibrary::new(
            PlayerId::new("p2"),
            vec![
                LibraryCard::land(CardDefinitionId::new("p2-forest-a"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-b"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-c"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-d"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-e"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-f"), ManaColor::Green),
                LibraryCard::land(CardDefinitionId::new("p2-forest-g"), ManaColor::Green),
            ],
        ),
    ];
    let dealt = game.deal_opening_hands(&DealOpeningHandsCommand::new(libraries));
    assert!(dealt.is_ok(), "opening hands should be dealt");

    let player = game.players().first().expect("p1 should exist");
    let Some(spell_id) = player
        .hand_card_by_definition(&CardDefinitionId::new("discard-choice"))
        .map(|card| card.id().clone())
    else {
        return;
    };

    let request = spell_choice_request(player, None, &spell_id, Some(&[]));

    assert!(matches!(
        request,
        Some(PublicChoiceRequest::SpellChoiceInvariantViolation {
            player_id,
            source_card_id,
        }) if player_id.as_str() == "p1" && source_card_id == spell_id
    ));
}
