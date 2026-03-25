//! Supports rules turn flow phase behavior.

use {
    crate::domain::play::game::model::{Player, MAX_HAND_SIZE},
    crate::domain::play::{errors::DomainError, phase::Phase},
};

pub(super) trait PhaseBehavior {
    fn next_phase(&self) -> Phase;
    fn requires_player_change(&self) -> bool;
    fn triggers_auto_draw(&self) -> bool;

    fn on_enter(
        &self,
        _players: &mut [Player],
        _active_player_index: usize,
    ) -> Result<(), DomainError> {
        Ok(())
    }

    fn on_exit(
        &self,
        _players: &mut [Player],
        _active_player_index: usize,
    ) -> Result<(), DomainError> {
        Ok(())
    }
}

struct SetupPhase;
struct UntapPhase;
struct UpkeepPhase;
struct DrawPhase;
struct FirstMainPhase;
struct BeginningOfCombatPhase;
struct DeclareAttackersPhase;
struct DeclareBlockersPhase;
struct CombatDamagePhase;
struct EndOfCombatPhase;
struct SecondMainPhase;
struct EndStepPhase;

impl PhaseBehavior for SetupPhase {
    fn next_phase(&self) -> Phase {
        Phase::Untap
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for UntapPhase {
    fn next_phase(&self) -> Phase {
        Phase::Upkeep
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
        active_player_index: usize,
    ) -> Result<(), DomainError> {
        let player = players.get_mut(active_player_index).ok_or_else(|| {
            DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(
                    "active player index should point to an existing player".to_string(),
                ),
            )
        })?;
        player.for_each_battlefield_card_mut(|card| {
            card.untap();
            card.remove_summoning_sickness();
            card.reset_loyalty_activation_for_new_turn();
        });
        player.reset_lands_played();
        Ok(())
    }
}

impl PhaseBehavior for UpkeepPhase {
    fn next_phase(&self) -> Phase {
        Phase::Draw
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        true
    }
}

impl PhaseBehavior for DrawPhase {
    fn next_phase(&self) -> Phase {
        Phase::FirstMain
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for FirstMainPhase {
    fn next_phase(&self) -> Phase {
        Phase::BeginningOfCombat
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for BeginningOfCombatPhase {
    fn next_phase(&self) -> Phase {
        Phase::DeclareAttackers
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for DeclareAttackersPhase {
    fn next_phase(&self) -> Phase {
        Phase::DeclareBlockers
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for DeclareBlockersPhase {
    fn next_phase(&self) -> Phase {
        Phase::CombatDamage
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for CombatDamagePhase {
    fn next_phase(&self) -> Phase {
        Phase::EndOfCombat
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for EndOfCombatPhase {
    fn next_phase(&self) -> Phase {
        Phase::SecondMain
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for SecondMainPhase {
    fn next_phase(&self) -> Phase {
        Phase::EndStep
    }
    fn requires_player_change(&self) -> bool {
        false
    }
    fn triggers_auto_draw(&self) -> bool {
        false
    }
}

impl PhaseBehavior for EndStepPhase {
    fn next_phase(&self) -> Phase {
        Phase::Untap
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
        _active_player_index: usize,
    ) -> Result<(), DomainError> {
        for player in players.iter_mut() {
            player.for_each_battlefield_card_mut(|card| {
                card.clear_damage();
                card.clear_temporary_stat_bonuses();
            });
        }
        Ok(())
    }
}

pub(super) fn active_player_hand_size(
    players: &[Player],
    active_player_index: usize,
) -> Result<usize, DomainError> {
    players
        .get(active_player_index)
        .map(Player::hand_size)
        .ok_or_else(|| {
            DomainError::Game(
                crate::domain::play::errors::GameError::InternalInvariantViolation(
                    "active player index should point to an existing player".to_string(),
                ),
            )
        })
}

pub(super) fn clear_all_mana(players: &mut [Player]) {
    for player in players {
        player.clear_mana();
    }
}

pub(super) const fn max_hand_size() -> usize {
    MAX_HAND_SIZE
}

pub(super) fn get_phase_behavior(phase: Phase) -> &'static dyn PhaseBehavior {
    match phase {
        Phase::Setup => &SetupPhase,
        Phase::Untap => &UntapPhase,
        Phase::Upkeep => &UpkeepPhase,
        Phase::Draw => &DrawPhase,
        Phase::FirstMain => &FirstMainPhase,
        Phase::BeginningOfCombat => &BeginningOfCombatPhase,
        Phase::DeclareAttackers => &DeclareAttackersPhase,
        Phase::DeclareBlockers => &DeclareBlockersPhase,
        Phase::CombatDamage => &CombatDamagePhase,
        Phase::EndOfCombat => &EndOfCombatPhase,
        Phase::SecondMain => &SecondMainPhase,
        Phase::EndStep => &EndStepPhase,
    }
}
