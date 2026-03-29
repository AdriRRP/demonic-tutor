//! Projects persisted domain events into the public replay timeline.

use std::{borrow::Borrow, mem::size_of, sync::Arc};

use crate::domain::play::{
    events::DomainEvent,
    ids::{CardInstanceId, PlayerId, StackObjectId},
};

use super::super::{PublicCardDrawn, PublicEvent, PublicEventLogEntry, PublicOpeningHandDealt};

pub(in crate::application::public_game) struct PublicEventLogProjection {
    pub entries: Arc<[PublicEventLogEntry]>,
    pub estimated_bytes: usize,
}

#[must_use]
pub fn public_event_log<I>(events: I) -> Arc<[PublicEventLogEntry]>
where
    I: IntoIterator,
    I::Item: Borrow<DomainEvent>,
{
    public_event_log_projection(events).entries
}

pub(in crate::application::public_game) fn public_event_log_projection<I>(
    events: I,
) -> PublicEventLogProjection
where
    I: IntoIterator,
    I::Item: Borrow<DomainEvent>,
{
    let mut estimated_bytes = 2 * size_of::<usize>();
    let entries = events
        .into_iter()
        .zip(1_u64..)
        .map(|(event, sequence)| {
            let entry = PublicEventLogEntry {
                sequence,
                event: public_event(event.borrow()),
            };
            estimated_bytes += approximate_public_event_log_entry_bytes(&entry);
            entry
        })
        .collect::<Vec<_>>();

    PublicEventLogProjection {
        entries: Arc::from(entries),
        estimated_bytes,
    }
}

fn approximate_public_event_log_entry_bytes(entry: &PublicEventLogEntry) -> usize {
    size_of::<PublicEventLogEntry>() + approximate_public_event_heap_bytes(&entry.event)
}

#[allow(clippy::too_many_lines)]
fn approximate_public_event_heap_bytes(event: &PublicEvent) -> usize {
    match event {
        PublicEvent::GameStarted(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_vec_bytes(&event.players)
                + event.players.capacity() * size_of::<PlayerId>()
        }
        PublicEvent::OpeningHandDealt(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.player_id)
        }
        PublicEvent::GameEnded(event) => {
            approximate_game_id_bytes(&event.game_id)
                + event
                    .winner_id
                    .as_ref()
                    .map_or(0, approximate_player_id_bytes)
                + event
                    .loser_id
                    .as_ref()
                    .map_or(0, approximate_player_id_bytes)
        }
        PublicEvent::LandPlayed(event) => {
            approximate_game_player_card_bytes(&event.game_id, &event.player_id, &event.card_id)
        }
        PublicEvent::CardDiscarded(event) => {
            approximate_game_player_card_bytes(&event.game_id, &event.player_id, &event.card_id)
        }
        PublicEvent::LandTapped(event) => {
            approximate_game_player_card_bytes(&event.game_id, &event.player_id, &event.card_id)
        }
        PublicEvent::CreatureDied(event) => {
            approximate_game_player_card_bytes(&event.game_id, &event.player_id, &event.card_id)
        }
        PublicEvent::TurnProgressed(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.active_player)
        }
        PublicEvent::CardDrawn(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.player_id)
        }
        PublicEvent::MulliganTaken(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.player_id)
        }
        PublicEvent::LifeChanged(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.player_id)
        }
        PublicEvent::ManaAdded(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.player_id)
        }
        PublicEvent::PriorityPassed(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.player_id)
        }
        PublicEvent::ActivatedAbilityPutOnStack(event) => approximate_stack_source_event_bytes(
            &event.game_id,
            &event.player_id,
            &event.source_card_id,
            &event.stack_object_id,
        ),
        PublicEvent::TriggeredAbilityPutOnStack(event) => approximate_stack_source_event_bytes(
            &event.game_id,
            &event.player_id,
            &event.source_card_id,
            &event.stack_object_id,
        ),
        PublicEvent::StackTopResolved(event) => approximate_stack_source_event_bytes(
            &event.game_id,
            &event.player_id,
            &event.source_card_id,
            &event.stack_object_id,
        ),
        PublicEvent::SpellPutOnStack(event) => {
            approximate_stack_source_event_bytes(
                &event.game_id,
                &event.player_id,
                &event.card_id,
                &event.stack_object_id,
            ) + event
                .target
                .as_ref()
                .map_or(0, approximate_spell_target_bytes)
        }
        PublicEvent::SpellCast(event) => {
            approximate_game_player_card_bytes(&event.game_id, &event.player_id, &event.card_id)
        }
        PublicEvent::AttackersDeclared(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.player_id)
                + approximate_card_vec_bytes(&event.attackers)
                + event.attackers.capacity() * size_of::<CardInstanceId>()
        }
        PublicEvent::BlockersDeclared(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.player_id)
                + event
                    .assignments
                    .iter()
                    .fold(0, |bytes, (attacker_id, blocker_id)| {
                        bytes
                            + approximate_card_id_bytes(attacker_id)
                            + approximate_card_id_bytes(blocker_id)
                    })
                + event.assignments.capacity() * size_of::<(CardInstanceId, CardInstanceId)>()
        }
        PublicEvent::CombatDamageResolved(event) => {
            approximate_game_id_bytes(&event.game_id)
                + approximate_player_id_bytes(&event.player_id)
                + event
                    .damage_events
                    .iter()
                    .map(|damage_event| {
                        approximate_card_id_bytes(&damage_event.source)
                            + match &damage_event.target {
                                crate::domain::play::events::DamageTarget::Creature(card_id) => {
                                    approximate_card_id_bytes(card_id)
                                }
                                crate::domain::play::events::DamageTarget::Player(player_id) => {
                                    approximate_player_id_bytes(player_id)
                                }
                            }
                    })
                    .sum::<usize>()
                + event.damage_events.capacity()
                    * size_of::<crate::domain::play::events::DamageEvent>()
        }
        PublicEvent::CardMovedZone(event) => {
            approximate_game_player_card_bytes(&event.game_id, &event.zone_owner_id, &event.card_id)
        }
    }
}

fn approximate_stack_source_event_bytes(
    game_id: &crate::domain::play::ids::GameId,
    player_id: &PlayerId,
    source_card_id: &CardInstanceId,
    stack_object_id: &StackObjectId,
) -> usize {
    approximate_game_player_card_bytes(game_id, player_id, source_card_id)
        + approximate_stack_object_id_bytes(stack_object_id)
}

fn approximate_game_player_card_bytes(
    game_id: &crate::domain::play::ids::GameId,
    player_id: &PlayerId,
    card_id: &CardInstanceId,
) -> usize {
    approximate_game_id_bytes(game_id)
        + approximate_player_id_bytes(player_id)
        + approximate_card_id_bytes(card_id)
}

fn approximate_player_vec_bytes(players: &[PlayerId]) -> usize {
    players.iter().map(approximate_player_id_bytes).sum()
}

fn approximate_card_vec_bytes(cards: &[CardInstanceId]) -> usize {
    cards.iter().map(approximate_card_id_bytes).sum()
}

fn approximate_game_id_bytes(id: &crate::domain::play::ids::GameId) -> usize {
    id.estimated_heap_bytes()
}

fn approximate_player_id_bytes(id: &PlayerId) -> usize {
    id.estimated_heap_bytes()
}

fn approximate_card_id_bytes(id: &CardInstanceId) -> usize {
    id.estimated_heap_bytes()
}

fn approximate_stack_object_id_bytes(id: &StackObjectId) -> usize {
    id.estimated_heap_bytes()
}

fn approximate_spell_target_bytes(target: &crate::domain::play::game::SpellTarget) -> usize {
    match target {
        crate::domain::play::game::SpellTarget::Player(player_id) => {
            approximate_player_id_bytes(player_id)
        }
        crate::domain::play::game::SpellTarget::Creature(card_id)
        | crate::domain::play::game::SpellTarget::Permanent(card_id)
        | crate::domain::play::game::SpellTarget::GraveyardCard(card_id) => {
            approximate_card_id_bytes(card_id)
        }
        crate::domain::play::game::SpellTarget::StackObject(stack_object_id) => {
            approximate_stack_object_id_bytes(stack_object_id)
        }
    }
}

pub(in crate::application::public_game) fn public_events<I>(events: I) -> Vec<PublicEvent>
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
