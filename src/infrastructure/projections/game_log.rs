use {crate::domain::play::events::DomainEvent, std::sync::RwLock};

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
            DomainEvent::TurnProgressed(e) => {
                format!(
                    "Turn progressed: {} {}->{}, {:?}->{:?}",
                    e.active_player, e.from_turn, e.to_turn, e.from_phase, e.to_phase
                )
            }
            DomainEvent::CardDrawn(e) => {
                format!("Player {} drew a card via {:?}", e.player_id, e.draw_kind)
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
                format!(
                    "Player {} cast {:?} spell {} for {} mana ({:?})",
                    e.player_id, e.card_type, e.card_id, e.mana_cost_paid, e.outcome
                )
            }
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
        };

        if let Ok(mut logs) = self.logs.write() {
            logs.push(log_entry);
        }
    }
}
