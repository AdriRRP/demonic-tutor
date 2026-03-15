use crate::domain::events::DomainEvent;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct GameLogProjection {
    logs: RwLock<Vec<String>>,
}

impl GameLogProjection {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            logs: RwLock::new(Vec::new()),
        }
    }

    #[must_use]
    pub fn logs(&self) -> Vec<String> {
        self.logs
            .read()
            .map(|logs| logs.clone())
            .unwrap_or_default()
    }

    pub fn handle(&self, event: &DomainEvent) {
        let log_entry = match event {
            DomainEvent::GameStarted(e) => {
                format!(
                    "Game {} started with players: {:?}",
                    e.game_id,
                    e.players
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                )
            }
            DomainEvent::OpeningHandDealt(e) => {
                format!(
                    "Player {} received opening hand with {} cards",
                    e.player_id,
                    e.cards.len()
                )
            }
            DomainEvent::LandPlayed(e) => {
                format!("Player {} played land {}", e.player_id, e.card_id)
            }
            DomainEvent::TurnAdvanced(e) => {
                format!("Turn advanced to {}", e.new_active_player)
            }
            DomainEvent::CardDrawn(e) => {
                format!("Player {} drew a card", e.player_id)
            }
            DomainEvent::MulliganTaken(e) => {
                format!("Player {} took a mulligan", e.player_id)
            }
            DomainEvent::LifeChanged(e) => {
                format!(
                    "Player {} life changed from {} to {}",
                    e.player_id, e.from_life, e.to_life
                )
            }
            DomainEvent::TurnNumberChanged(e) => {
                format!("Turn changed from {} to {}", e.from_turn, e.to_turn)
            }
            DomainEvent::PhaseChanged(e) => {
                format!("Phase changed from {} to {}", e.from_phase, e.to_phase)
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
            DomainEvent::SpellCast(e) => {
                format!("Player {} cast spell {}", e.player_id, e.card_id)
            }
            DomainEvent::CreatureEnteredBattlefield(e) => {
                format!(
                    "Player {} played creature {} ({}/{})",
                    e.player_id, e.card_id, e.power, e.toughness
                )
            }
        };

        if let Ok(mut logs) = self.logs.write() {
            logs.push(log_entry);
        }
    }
}
