//! Supports application game service resource actions.

use {
    super::{common::DomainEvents, GameService},
    crate::{
        application::{EventBus, EventStore},
        domain::play::{
            commands::{
                AdjustPlayerLifeEffectCommand, ExileCardCommand, PlayLandCommand, TapLandCommand,
            },
            errors::DomainError,
            events::{CardMovedZone, DomainEvent, LandPlayed, LandTapped, ManaAdded},
            game::{AdjustPlayerLifeEffectOutcome, Game},
        },
    },
};

pub fn domain_events_for_adjust_player_life_effect(
    outcome: &AdjustPlayerLifeEffectOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.life_changed.clone());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.push_optional(outcome.game_ended.clone());
    domain_events.into_vec()
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Plays a land card.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn play_land(
        &self,
        game: &mut Game,
        cmd: PlayLandCommand,
    ) -> Result<LandPlayed, DomainError> {
        let event = game.play_land(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok(event)
    }

    /// Exiles a card.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn exile_card(
        &self,
        game: &mut Game,
        cmd: &ExileCardCommand,
    ) -> Result<CardMovedZone, DomainError> {
        let zone_change = game.exile_card(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &zone_change)?;

        Ok(zone_change)
    }

    /// Resolves an explicit life effect from a caster onto a target player.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn adjust_player_life_effect(
        &self,
        game: &mut Game,
        cmd: AdjustPlayerLifeEffectCommand,
    ) -> Result<AdjustPlayerLifeEffectOutcome, DomainError> {
        let outcome = game.adjust_player_life_effect(cmd)?;
        let domain_events = domain_events_for_adjust_player_life_effect(&outcome);
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok(outcome)
    }

    /// Taps a land to add mana.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn tap_land(
        &self,
        game: &mut Game,
        cmd: TapLandCommand,
    ) -> Result<(LandTapped, ManaAdded), DomainError> {
        let (land_event, mana_event) = game.tap_land(cmd)?;
        let mut domain_events = DomainEvents::with(land_event.clone());
        domain_events.push(mana_event.clone());
        let domain_events = domain_events.into_vec();
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok((land_event, mana_event))
    }
}
