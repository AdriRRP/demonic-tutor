use crate::domain::{errors::DomainError, ids::PlayerId};

use super::player::Player;

/// Trait defining behavior specific to each game phase.
pub trait PhaseBehavior {
    /// Returns the next phase in the turn sequence.
    fn next_phase(&self) -> crate::domain::game::Phase;

    /// Returns whether this phase triggers a player change.
    fn requires_player_change(&self) -> bool;

    /// Returns whether this phase triggers an automatic card draw.
    fn triggers_auto_draw(&self) -> bool;

    /// Called when entering this phase.
    /// Can modify game state as needed.
    ///
    /// # Errors
    /// Returns an error if phase-specific logic fails.
    fn on_enter(
        &self,
        _players: &mut [Player],
        _active_player: &PlayerId,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    /// Called when exiting this phase.
    /// Can perform cleanup or state updates.
    ///
    /// # Errors
    /// Returns an error if phase-specific cleanup fails.
    fn on_exit(
        &self,
        _players: &mut [Player],
        _active_player: &PlayerId,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    /// Returns a human-readable description of the phase.
    fn description(&self) -> &'static str;
}

/// Implementation for Setup phase.
pub struct SetupPhase;

impl PhaseBehavior for SetupPhase {
    fn next_phase(&self) -> crate::domain::game::Phase {
        crate::domain::game::Phase::Untap
    }

    fn requires_player_change(&self) -> bool {
        false
    }

    fn triggers_auto_draw(&self) -> bool {
        false
    }

    fn description(&self) -> &'static str {
        "Setup - Initial game preparation"
    }
}

/// Implementation for Untap phase.
pub struct UntapPhase;

impl PhaseBehavior for UntapPhase {
    fn next_phase(&self) -> crate::domain::game::Phase {
        crate::domain::game::Phase::Upkeep
    }

    fn requires_player_change(&self) -> bool {
        false
    }

    fn triggers_auto_draw(&self) -> bool {
        false
    }

    fn on_enter(
        &self,
        players: &mut [Player],
        _active_player: &PlayerId,
    ) -> Result<(), DomainError> {
        // Untap all permanents, remove summoning sickness, and reset lands played for all players
        for player in players.iter_mut() {
            player
                .battlefield_mut()
                .cards_mut()
                .iter_mut()
                .for_each(|card| {
                    card.untap();
                    card.remove_summoning_sickness();
                });
            *player.lands_played_this_turn_mut() = 0;
        }

        Ok(())
    }

    fn description(&self) -> &'static str {
        "Untap - Untap permanents and remove summoning sickness"
    }
}

/// Implementation for Upkeep phase.
pub struct UpkeepPhase;

impl PhaseBehavior for UpkeepPhase {
    fn next_phase(&self) -> crate::domain::game::Phase {
        crate::domain::game::Phase::Draw
    }

    fn requires_player_change(&self) -> bool {
        false
    }

    fn triggers_auto_draw(&self) -> bool {
        false
    }

    fn description(&self) -> &'static str {
        "Upkeep - Beginning of turn effects"
    }
}

/// Implementation for Draw phase.
pub struct DrawPhase;

impl PhaseBehavior for DrawPhase {
    fn next_phase(&self) -> crate::domain::game::Phase {
        crate::domain::game::Phase::FirstMain
    }

    fn requires_player_change(&self) -> bool {
        false
    }

    fn triggers_auto_draw(&self) -> bool {
        true
    }

    fn description(&self) -> &'static str {
        "Draw - Draw a card"
    }
}

/// Implementation for First Main phase.
pub struct FirstMainPhase;

impl PhaseBehavior for FirstMainPhase {
    fn next_phase(&self) -> crate::domain::game::Phase {
        crate::domain::game::Phase::Combat
    }

    fn requires_player_change(&self) -> bool {
        false
    }

    fn triggers_auto_draw(&self) -> bool {
        false
    }

    fn description(&self) -> &'static str {
        "First Main - Play lands and spells"
    }
}

/// Implementation for Combat phase.
pub struct CombatPhase;

impl PhaseBehavior for CombatPhase {
    fn next_phase(&self) -> crate::domain::game::Phase {
        crate::domain::game::Phase::SecondMain
    }

    fn requires_player_change(&self) -> bool {
        false
    }

    fn triggers_auto_draw(&self) -> bool {
        false
    }

    fn description(&self) -> &'static str {
        "Combat - Declare attackers and blockers"
    }
}

/// Implementation for Second Main phase.
pub struct SecondMainPhase;

impl PhaseBehavior for SecondMainPhase {
    fn next_phase(&self) -> crate::domain::game::Phase {
        crate::domain::game::Phase::EndStep
    }

    fn requires_player_change(&self) -> bool {
        false
    }

    fn triggers_auto_draw(&self) -> bool {
        false
    }

    fn description(&self) -> &'static str {
        "Second Main - Play lands and spells after combat"
    }
}

/// Implementation for End Step phase.
pub struct EndStepPhase;

impl PhaseBehavior for EndStepPhase {
    fn next_phase(&self) -> crate::domain::game::Phase {
        crate::domain::game::Phase::Untap
    }

    fn requires_player_change(&self) -> bool {
        true
    }

    fn triggers_auto_draw(&self) -> bool {
        false
    }

    fn on_exit(
        &self,
        players: &mut [Player],
        _active_player: &PlayerId,
    ) -> Result<(), DomainError> {
        // Clear damage from all creatures at end of turn
        for player in players.iter_mut() {
            player
                .battlefield_mut()
                .cards_mut()
                .iter_mut()
                .for_each(super::super::cards::CardInstance::clear_damage);
        }
        Ok(())
    }

    fn description(&self) -> &'static str {
        "End Step - Cleanup and end of turn"
    }
}

/// Converts a Phase enum to its corresponding `PhaseBehavior` implementation.
#[must_use]
pub fn get_phase_behavior(phase: &crate::domain::game::Phase) -> Box<dyn PhaseBehavior> {
    match phase {
        crate::domain::game::Phase::Setup => Box::new(SetupPhase),
        crate::domain::game::Phase::Untap => Box::new(UntapPhase),
        crate::domain::game::Phase::Upkeep => Box::new(UpkeepPhase),
        crate::domain::game::Phase::Draw => Box::new(DrawPhase),
        crate::domain::game::Phase::FirstMain => Box::new(FirstMainPhase),
        crate::domain::game::Phase::Combat => Box::new(CombatPhase),
        crate::domain::game::Phase::SecondMain => Box::new(SecondMainPhase),
        crate::domain::game::Phase::EndStep => Box::new(EndStepPhase),
    }
}
