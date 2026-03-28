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
            events::{CardMovedZone, DomainEvent, LandPlayed, LandTapped, ManaAdded, ZoneType},
            game::{AdjustPlayerLifeEffectOutcome, Game},
        },
    },
};

fn zone_change_for_creature_died(
    event: &crate::domain::play::events::CreatureDied,
) -> CardMovedZone {
    CardMovedZone::new(
        event.game_id.clone(),
        event.player_id.clone(),
        event.card_id.clone(),
        ZoneType::Battlefield,
        ZoneType::Graveyard,
    )
}

pub fn domain_events_for_adjust_player_life_effect(
    outcome: &AdjustPlayerLifeEffectOutcome,
) -> Vec<DomainEvent> {
    let mut domain_events = DomainEvents::with(outcome.life_changed.clone());
    domain_events.extend(outcome.creatures_died.iter().cloned());
    domain_events.extend(
        outcome
            .creatures_died
            .iter()
            .map(zone_change_for_creature_died),
    );
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

#[cfg(test)]
mod tests {
    //! Tests application resource action event mapping.

    use super::domain_events_for_adjust_player_life_effect;
    use crate::domain::play::{
        events::{CreatureDied, DomainEvent, LifeChanged},
        game::AdjustPlayerLifeEffectOutcome,
        ids::{CardInstanceId, GameId, PlayerId},
    };

    #[test]
    fn adjust_player_life_effect_surfaces_zone_moves_for_creatures_that_die() {
        let outcome = AdjustPlayerLifeEffectOutcome::new(
            LifeChanged::new(GameId::new("game"), PlayerId::new("p1"), 20, 18),
            vec![CreatureDied::new(
                GameId::new("game"),
                PlayerId::new("p1"),
                CardInstanceId::new("creature-1"),
            )],
            None,
        );

        let events = domain_events_for_adjust_player_life_effect(&outcome);

        assert!(matches!(
            events.as_slice(),
            [
                DomainEvent::LifeChanged(_),
                DomainEvent::CreatureDied(creature_died),
                DomainEvent::CardMovedZone(zone_change),
            ] if creature_died.card_id == CardInstanceId::new("creature-1")
                && zone_change.card_id == CardInstanceId::new("creature-1")
                && zone_change.origin_zone.as_str() == "battlefield"
                && zone_change.destination_zone.as_str() == "graveyard"
        ));
    }
}
