//! Supports game rules stack priority.

mod activation;
mod casting;
mod deferred_resolution;
mod hand_choice_effect;
mod optional_effect;
mod passing;
mod resolution;
mod scry_effect;
pub(crate) mod spell_effects;
mod surveil_effect;
pub(crate) mod triggers;

use super::super::{
    model::{AggregateCardLocationIndex, PriorityState},
    Player, TerminalState,
};
use crate::domain::play::{
    events::{
        ActivatedAbilityPutOnStack, CardDiscarded, CardDrawn, CardMovedZone, CreatureDied,
        GameEnded, LifeChanged, PriorityPassed, SpellCast, SpellPutOnStack, StackTopResolved,
        TriggeredAbilityPutOnStack,
    },
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};

pub use activation::activate_ability;
pub(crate) use activation::is_activatable_candidate;
pub use casting::cast_spell;
pub(crate) use casting::is_castable_candidate;
pub use hand_choice_effect::resolve_pending_hand_choice;
pub use optional_effect::resolve_optional_effect;
pub use passing::pass_priority;
pub use scry_effect::resolve_pending_scry;
pub use surveil_effect::resolve_pending_surveil;

pub struct StackPriorityContext<'a> {
    pub game_id: &'a GameId,
    pub players: &'a mut [Player],
    pub card_locations: &'a AggregateCardLocationIndex,
    pub active_player: &'a PlayerId,
    pub phase: &'a Phase,
    pub stack: &'a mut super::super::model::StackZone,
    pub priority: &'a mut Option<PriorityState>,
    pub pending_decision: &'a mut Option<super::super::PendingDecision>,
    pub terminal_state: &'a mut TerminalState,
}

#[derive(Debug, Clone)]
pub struct CastSpellOutcome {
    pub spell_put_on_stack: SpellPutOnStack,
}

#[derive(Debug, Clone)]
pub struct ActivateAbilityOutcome {
    pub activated_ability_put_on_stack: ActivatedAbilityPutOnStack,
    pub creatures_died: Vec<CreatureDied>,
    pub zone_changes: Vec<CardMovedZone>,
    pub moved_cards: Vec<CardInstanceId>,
}

#[derive(Debug, Clone)]
pub struct PassPriorityOutcome {
    pub priority_passed: PriorityPassed,
    pub triggered_abilities_put_on_stack: Vec<TriggeredAbilityPutOnStack>,
    pub stack_top_resolved: Option<StackTopResolved>,
    pub spell_cast: Option<SpellCast>,
    pub card_drawn: Vec<CardDrawn>,
    pub card_discarded: Option<CardDiscarded>,
    pub zone_changes: Vec<CardMovedZone>,
    pub life_changed: Option<LifeChanged>,
    pub creatures_died: Vec<CreatureDied>,
    pub moved_cards: Vec<CardInstanceId>,
    pub game_ended: Option<GameEnded>,
    pub priority_still_open: bool,
}

#[derive(Debug, Clone)]
pub struct ResolveOptionalEffectOutcome {
    pub stack_top_resolved: Option<StackTopResolved>,
    pub triggered_abilities_put_on_stack: Vec<TriggeredAbilityPutOnStack>,
    pub spell_cast: Option<SpellCast>,
    pub card_discarded: Option<CardDiscarded>,
    pub zone_changes: Vec<CardMovedZone>,
    pub life_changed: Option<LifeChanged>,
    pub creatures_died: Vec<CreatureDied>,
    pub moved_cards: Vec<CardInstanceId>,
    pub game_ended: Option<GameEnded>,
    pub priority_still_open: bool,
}

#[derive(Debug, Clone)]
pub struct ResolvePendingHandChoiceOutcome {
    pub stack_top_resolved: Option<StackTopResolved>,
    pub spell_cast: Option<SpellCast>,
    pub card_drawn: Vec<CardDrawn>,
    pub card_discarded: Option<CardDiscarded>,
    pub zone_changes: Vec<CardMovedZone>,
    pub moved_cards: Vec<CardInstanceId>,
    pub game_ended: Option<GameEnded>,
    pub priority_still_open: bool,
}

#[derive(Debug, Clone)]
pub struct ResolvePendingScryOutcome {
    pub stack_top_resolved: Option<StackTopResolved>,
    pub spell_cast: Option<SpellCast>,
    pub zone_changes: Vec<CardMovedZone>,
    pub moved_cards: Vec<CardInstanceId>,
    pub game_ended: Option<GameEnded>,
    pub priority_still_open: bool,
}

#[derive(Debug, Clone)]
pub struct ResolvePendingSurveilOutcome {
    pub stack_top_resolved: Option<StackTopResolved>,
    pub spell_cast: Option<SpellCast>,
    pub zone_changes: Vec<CardMovedZone>,
    pub moved_cards: Vec<CardInstanceId>,
    pub game_ended: Option<GameEnded>,
    pub priority_still_open: bool,
}
