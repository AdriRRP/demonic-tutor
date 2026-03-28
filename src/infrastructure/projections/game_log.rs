//! Supports infrastructure projections game log.

use {
    crate::domain::play::events::DomainEvent,
    std::sync::{Arc, RwLock},
};

#[derive(Debug, Default)]
pub struct GameLogProjection {
    logs: RwLock<GameLogState>,
}

#[derive(Debug, Default)]
struct GameLogState {
    entries: Vec<String>,
    snapshot: Option<Arc<[String]>>,
}

impl GameLogProjection {
    #[must_use]
    pub fn new() -> Self {
        Self {
            logs: RwLock::new(GameLogState::default()),
        }
    }

    #[must_use]
    pub fn logs(&self) -> Arc<[String]> {
        if let Ok(state) = self.logs.read() {
            if let Some(snapshot) = &state.snapshot {
                return Arc::clone(snapshot);
            }
        }

        self.logs
            .write()
            .map(|mut state| {
                if let Some(snapshot) = &state.snapshot {
                    return Arc::clone(snapshot);
                }

                let snapshot = Arc::<[String]>::from(state.entries.clone());
                state.snapshot = Some(Arc::clone(&snapshot));
                snapshot
            })
            .unwrap_or_default()
    }

    fn describe_event(event: &DomainEvent) -> String {
        match event {
            DomainEvent::GameStarted(e) => Self::log_game_started(e),
            DomainEvent::OpeningHandDealt(e) => Self::log_opening_hand_dealt(e),
            DomainEvent::GameEnded(e) => Self::log_game_ended(e),
            DomainEvent::LandPlayed(e) => {
                format!("Player {} played land {}", e.player_id, e.card_id)
            }
            DomainEvent::TurnProgressed(e) => Self::log_turn_progressed(e),
            DomainEvent::CardDrawn(e) => {
                format!("Player {} drew a card via {:?}", e.player_id, e.draw_kind)
            }
            DomainEvent::CardDiscarded(e) => {
                format!(
                    "Player {} discarded card {} via {:?}",
                    e.player_id, e.card_id, e.discard_kind
                )
            }
            DomainEvent::MulliganTaken(e) => format!("Player {} took a mulligan", e.player_id),
            DomainEvent::LifeChanged(e) => {
                format!(
                    "Player {} life changed from {} to {}",
                    e.player_id, e.from_life, e.to_life
                )
            }
            DomainEvent::LandTapped(e) => {
                format!("Player {} tapped land {}", e.player_id, e.card_id)
            }
            DomainEvent::ManaAdded(e) => {
                format!(
                    "Player {} added {} mana (total: {})",
                    e.player_id, e.amount, e.new_mana_total
                )
            }
            DomainEvent::ActivatedAbilityPutOnStack(e) => {
                format!(
                    "Player {} activated ability from {} with effect {:?}",
                    e.player_id, e.source_card_id, e.effect
                )
            }
            DomainEvent::TriggeredAbilityPutOnStack(e) => {
                format!(
                    "Player {} put triggered ability from {} on the stack ({:?}, {:?})",
                    e.player_id, e.source_card_id, e.trigger, e.effect
                )
            }
            DomainEvent::SpellPutOnStack(e) => Self::log_spell_put_on_stack(e),
            DomainEvent::PriorityPassed(e) => format!("Player {} passed priority", e.player_id),
            DomainEvent::StackTopResolved(e) => Self::log_stack_top_resolved(e),
            DomainEvent::SpellCast(e) => Self::log_spell_cast(e),
            DomainEvent::AttackersDeclared(e) => {
                format!(
                    "Player {} declared {:?} as attackers",
                    e.player_id, e.attackers
                )
            }
            DomainEvent::BlockersDeclared(e) => {
                format!(
                    "Player {} declared {:?} as blockers",
                    e.player_id, e.assignments
                )
            }
            DomainEvent::CombatDamageResolved(e) => {
                format!("Combat damage resolved: {:?}", e.damage_events)
            }
            DomainEvent::CreatureDied(e) => {
                format!("Creature {} controlled by {} died", e.card_id, e.player_id)
            }
            DomainEvent::CardMovedZone(e) => {
                format!(
                    "Card {} moved in {} from {:?} to {:?}",
                    e.card_id, e.zone_owner_id, e.origin_zone, e.destination_zone
                )
            }
        }
    }

    fn log_game_started(event: &crate::domain::play::events::GameStarted) -> String {
        format!(
            "Game {} started with players: {:?}",
            event.game_id,
            event
                .players
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
        )
    }

    fn log_opening_hand_dealt(event: &crate::domain::play::events::OpeningHandDealt) -> String {
        format!(
            "Player {} received opening hand with {} cards",
            event.player_id,
            event.cards.len()
        )
    }

    fn log_game_ended(event: &crate::domain::play::events::GameEnded) -> String {
        match (&event.loser_id, &event.winner_id) {
            (Some(loser_id), Some(winner_id)) => {
                format!("Game ended: {loser_id} lost to {winner_id} via {:?}", event.reason)
            }
            _ => format!("Game ended in a draw via {:?}", event.reason),
        }
    }

    fn log_turn_progressed(event: &crate::domain::play::events::TurnProgressed) -> String {
        format!(
            "Turn progressed: {} {}->{}, {:?}->{:?}",
            event.active_player, event.from_turn, event.to_turn, event.from_phase, event.to_phase
        )
    }

    fn log_spell_put_on_stack(event: &crate::domain::play::events::SpellPutOnStack) -> String {
        let target_suffix = event
            .target
            .as_ref()
            .map_or_else(String::new, |target| format!(" targeting {target:?}"));
        format!(
            "Player {} put {:?} spell {} on the stack for {} mana{}",
            event.player_id, event.card_type, event.card_id, event.mana_cost_paid, target_suffix
        )
    }

    fn log_stack_top_resolved(event: &crate::domain::play::events::StackTopResolved) -> String {
        format!(
            "Stack object {} from card {} resolved for player {}",
            event.stack_object_id, event.source_card_id, event.player_id
        )
    }

    fn log_spell_cast(event: &crate::domain::play::events::SpellCast) -> String {
        format!(
            "Player {} resolved {:?} spell {} for {} mana ({:?})",
            event.player_id, event.card_type, event.card_id, event.mana_cost_paid, event.outcome
        )
    }

    pub fn handle(&self, event: &DomainEvent) {
        let log_entry = Self::describe_event(event);

        if let Ok(mut state) = self.logs.write() {
            state.entries.push(log_entry);
            state.snapshot = None;
        }
    }
}
