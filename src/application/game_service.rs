use crate::application::{EventBus, EventStore};
use crate::domain::{
    commands::{
        AdvanceTurnCommand, DealOpeningHandsCommand, DrawCardCommand, MulliganCommand,
        PlayLandCommand, SetLifeCommand, StartGameCommand,
    },
    errors::DomainError,
    events::{
        CardDrawn, DomainEvent, GameStarted, LandPlayed, LifeChanged, MulliganTaken,
        OpeningHandDealt, TurnAdvanced,
    },
    game::Game,
};

pub struct GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    event_store: E,
    event_bus: B,
}

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    #[must_use]
    pub const fn new(event_store: E, event_bus: B) -> Self {
        Self {
            event_store,
            event_bus,
        }
    }

    /// Starts a new game.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn start_game(&self, cmd: StartGameCommand) -> Result<(Game, GameStarted), DomainError> {
        let (game, event) = Game::start(cmd)?;
        let domain_event: DomainEvent = event.clone().into();

        let game_id = game.id().0.clone();
        let _ = self
            .event_store
            .append(&game_id, std::slice::from_ref(&domain_event));
        self.event_bus.publish(&domain_event);

        Ok((game, event))
    }

    /// Deals opening hands to all players.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn deal_opening_hands(
        &self,
        game: &mut Game,
        cmd: &DealOpeningHandsCommand,
    ) -> Result<Vec<OpeningHandDealt>, DomainError> {
        let events = game.deal_opening_hands(cmd)?;

        if !events.is_empty() {
            let game_id = game.id().0.clone();
            let domain_events: Vec<DomainEvent> = events.iter().cloned().map(Into::into).collect();
            let _ = self.event_store.append(&game_id, &domain_events);
            for event in &domain_events {
                self.event_bus.publish(event);
            }
        }

        Ok(events)
    }

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
        let domain_event: DomainEvent = event.clone().into();

        let game_id = game.id().0.clone();
        let _ = self
            .event_store
            .append(&game_id, std::slice::from_ref(&domain_event));
        self.event_bus.publish(&domain_event);

        Ok(event)
    }

    /// Advances the turn to the next player.
    ///
    /// # Errors
    ///
    /// Returns an error if the active player cannot be found.
    pub fn advance_turn(
        &self,
        game: &mut Game,
        cmd: AdvanceTurnCommand,
    ) -> Result<TurnAdvanced, DomainError> {
        let event = game.advance_turn(cmd)?;
        let domain_event: DomainEvent = event.clone().into();

        let game_id = game.id().0.clone();
        let _ = self
            .event_store
            .append(&game_id, std::slice::from_ref(&domain_event));
        self.event_bus.publish(&domain_event);

        Ok(event)
    }

    /// Draws a card from the player's library.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn draw_card(
        &self,
        game: &mut Game,
        cmd: DrawCardCommand,
    ) -> Result<CardDrawn, DomainError> {
        let event = game.draw_card(cmd)?;
        let domain_event: DomainEvent = event.clone().into();

        let game_id = game.id().0.clone();
        let _ = self
            .event_store
            .append(&game_id, std::slice::from_ref(&domain_event));
        self.event_bus.publish(&domain_event);

        Ok(event)
    }

    /// Performs a mulligan for a player.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn mulligan(
        &self,
        game: &mut Game,
        cmd: MulliganCommand,
    ) -> Result<MulliganTaken, DomainError> {
        let event = game.mulligan(cmd)?;
        let domain_event: DomainEvent = event.clone().into();

        let game_id = game.id().0.clone();
        let _ = self
            .event_store
            .append(&game_id, std::slice::from_ref(&domain_event));
        self.event_bus.publish(&domain_event);

        Ok(event)
    }

    /// Sets a player's life total.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn set_life(
        &self,
        game: &mut Game,
        cmd: SetLifeCommand,
    ) -> Result<LifeChanged, DomainError> {
        let event = game.set_life(cmd)?;
        let domain_event: DomainEvent = event.clone().into();

        let game_id = game.id().0.clone();
        let _ = self
            .event_store
            .append(&game_id, std::slice::from_ref(&domain_event));
        self.event_bus.publish(&domain_event);

        Ok(event)
    }
}
