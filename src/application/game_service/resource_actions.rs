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
            game::{AdjustPlayerLifeEffectOutcome, Game, GameCheckpointSpec},
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
        self.apply_persisted_event(game, GameCheckpointSpec::PLAY_LAND, |game| {
            game.play_land(cmd)
        })
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
        self.apply_persisted_event(game, GameCheckpointSpec::EXILE_CARD, |game| {
            game.exile_card(cmd)
        })
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
        self.apply_persisted(
            game,
            GameCheckpointSpec::ADJUST_PLAYER_LIFE_EFFECT,
            |game| game.adjust_player_life_effect(cmd),
            domain_events_for_adjust_player_life_effect,
        )
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
        self.apply_persisted(
            game,
            GameCheckpointSpec::TAP_LAND,
            |game| game.tap_land(cmd),
            |(land_event, mana_event)| {
                let mut domain_events = DomainEvents::with(land_event.clone());
                domain_events.push(mana_event.clone());
                domain_events.into_vec()
            },
        )
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
