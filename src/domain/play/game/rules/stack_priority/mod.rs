//! Supports game rules stack priority.

mod activation;
mod casting;
mod passing;
mod resolution;
mod spell_effects;
pub(crate) mod triggers;

use super::super::{
    model::{AggregateCardLocationIndex, PriorityState},
    Player, TerminalState,
};
use crate::domain::play::{
    events::{
        ActivatedAbilityPutOnStack, CardDiscarded, CardExiled, CreatureDied, GameEnded,
        LifeChanged, PriorityPassed, SpellCast, SpellPutOnStack, StackTopResolved,
        TriggeredAbilityPutOnStack,
    },
    ids::{CardInstanceId, GameId, PlayerId},
    phase::Phase,
};

pub use activation::activate_ability;
pub use casting::cast_spell;
pub use passing::pass_priority;

pub struct StackPriorityContext<'a> {
    pub game_id: &'a GameId,
    pub players: &'a mut [Player],
    pub card_locations: &'a AggregateCardLocationIndex,
    pub active_player: &'a PlayerId,
    pub phase: &'a Phase,
    pub stack: &'a mut super::super::model::StackZone,
    pub priority: &'a mut Option<PriorityState>,
    pub terminal_state: &'a mut TerminalState,
}

#[derive(Debug, Clone)]
pub struct CastSpellOutcome {
    pub spell_put_on_stack: SpellPutOnStack,
}

#[derive(Debug, Clone)]
pub struct ActivateAbilityOutcome {
    pub activated_ability_put_on_stack: ActivatedAbilityPutOnStack,
}

#[derive(Debug, Clone)]
pub struct PassPriorityOutcome {
    pub priority_passed: PriorityPassed,
    pub triggered_abilities_put_on_stack: Vec<TriggeredAbilityPutOnStack>,
    pub stack_top_resolved: Option<StackTopResolved>,
    pub spell_cast: Option<SpellCast>,
    pub card_exiled: Option<CardExiled>,
    pub card_discarded: Option<CardDiscarded>,
    pub life_changed: Option<LifeChanged>,
    pub creatures_died: Vec<CreatureDied>,
    pub moved_cards: Vec<CardInstanceId>,
    pub game_ended: Option<GameEnded>,
    pub priority_still_open: bool,
}
