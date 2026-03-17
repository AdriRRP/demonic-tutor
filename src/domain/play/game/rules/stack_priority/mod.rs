mod casting;
mod passing;
mod resolution;

use super::super::{model::PriorityState, Player, TerminalState};
use crate::domain::play::{
    events::{
        CreatureDied, GameEnded, PriorityPassed, SpellCast, SpellPutOnStack, StackTopResolved,
    },
    ids::{GameId, PlayerId},
    phase::Phase,
};

pub use casting::cast_spell;
pub use passing::pass_priority;

pub struct StackPriorityContext<'a> {
    pub game_id: &'a GameId,
    pub players: &'a mut [Player],
    pub active_player: &'a PlayerId,
    pub phase: &'a Phase,
    pub stack: &'a mut super::super::model::StackZone,
    pub priority: &'a mut Option<PriorityState>,
    pub next_stack_object_number: &'a mut u32,
    pub terminal_state: &'a mut TerminalState,
}

#[derive(Debug, Clone)]
pub struct CastSpellOutcome {
    pub spell_put_on_stack: SpellPutOnStack,
}

#[derive(Debug, Clone)]
pub struct PassPriorityOutcome {
    pub priority_passed: PriorityPassed,
    pub stack_top_resolved: Option<StackTopResolved>,
    pub spell_cast: Option<SpellCast>,
    pub creatures_died: Vec<CreatureDied>,
    pub game_ended: Option<GameEnded>,
    pub priority_still_open: bool,
}
