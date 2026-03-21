use super::{common::DomainEvents, GameService};
use crate::{
    application::{EventBus, EventStore},
    domain::play::{
        commands::{CastSpellCommand, PassPriorityCommand},
        errors::DomainError,
        events::DomainEvent,
        game::{CastSpellOutcome, Game, PassPriorityOutcome},
    },
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

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Casts a spell.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn cast_spell(
        &self,
        game: &mut Game,
        cmd: CastSpellCommand,
    ) -> Result<CastSpellOutcome, DomainError> {
        let outcome = game.cast_spell(cmd)?;
        let domain_events = domain_events_for_cast_spell(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Passes priority in an open priority window.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn pass_priority(
        &self,
        game: &mut Game,
        cmd: PassPriorityCommand,
    ) -> Result<PassPriorityOutcome, DomainError> {
        let outcome = game.pass_priority(cmd)?;
        let domain_events = domain_events_for_pass_priority(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }
}
