use super::common::DomainEvents;
use crate::domain::play::{
    events::DomainEvent,
    game::{CastSpellOutcome, PassPriorityOutcome},
};

pub(super) fn domain_events_for_cast_spell(outcome: &CastSpellOutcome) -> Vec<DomainEvent> {
    vec![outcome.spell_put_on_stack.clone().into()]
}

pub(super) fn domain_events_for_pass_priority(outcome: &PassPriorityOutcome) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.priority_passed.clone());
    domain_events.push_optional(outcome.stack_top_resolved.clone());
    domain_events.push_optional(outcome.spell_cast.clone());
    domain_events.push_optional(outcome.life_changed.clone());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}
