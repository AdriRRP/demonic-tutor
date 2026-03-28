//! Supports application game service lifecycle.

use {
    super::{rollback::GameRollback, GameService},
    crate::{
        application::{EventBus, EventStore},
        domain::play::{
            commands::{
                ConcedeCommand, DealOpeningHandsCommand, MulliganCommand, StartGameCommand,
            },
            errors::DomainError,
            events::{GameEnded, GameStarted, MulliganTaken, OpeningHandDealt},
            game::Game,
        },
    },
};

impl<E, B> GameService<E, B>
where
    E: EventStore,
    B: EventBus,
{
    /// Starts a new game.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn start_game(&self, cmd: StartGameCommand) -> Result<(Game, GameStarted), DomainError> {
        let (game, event) = Game::start(cmd)?;
        self.persist_and_publish_event(game.id().as_str(), &event)?;

        Ok((game, event))
    }

    /// Starts a new game and deals opening hands as one persisted lifecycle batch.
    ///
    /// # Errors
    ///
    /// Returns an error if either lifecycle step is invalid.
    pub(crate) fn start_game_with_opening_hands(
        &self,
        start_cmd: StartGameCommand,
        opening_cmd: &DealOpeningHandsCommand,
    ) -> Result<(Game, GameStarted, Vec<OpeningHandDealt>), DomainError> {
        let (mut game, game_started) = Game::start(start_cmd)?;
        let opening_hands = game.deal_opening_hands(opening_cmd)?;

        let mut domain_events = Vec::with_capacity(1 + opening_hands.len());
        domain_events.push(game_started.clone().into());
        domain_events.extend(opening_hands.iter().cloned().map(Into::into));
        self.persist_and_publish_events(game.id().as_str(), &domain_events)?;

        Ok((game, game_started, opening_hands))
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
        let rollback = GameRollback::default()
            .capture_all_players(game)?
            .capture_card_locations(game);
        self.apply_persisted(
            game,
            rollback,
            |game| game.deal_opening_hands(cmd),
            |events| events.iter().cloned().map(Into::into).collect(),
        )
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
        let rollback = GameRollback::default()
            .capture_player(game, &cmd.player_id)?
            .capture_card_locations(game);
        self.apply_persisted_event(game, rollback, |game| game.mulligan(cmd))
    }

    /// Concedes an active game for one player.
    ///
    /// # Errors
    ///
    /// Returns an error if the command is invalid.
    pub fn concede(&self, game: &mut Game, cmd: ConcedeCommand) -> Result<GameEnded, DomainError> {
        let rollback = GameRollback::default().capture_terminal_state(game);
        self.apply_persisted_event(game, rollback, |game| game.concede(cmd))
    }
}

#[cfg(test)]
mod tests {
    //! Verifies lifecycle rollback restores aggregate state after persistence failures.

    #![allow(clippy::expect_used)]

    use std::{error::Error, io, sync::Arc};

    use super::GameService;
    use crate::{
        application::EventStore,
        domain::play::{
            cards::ManaColor,
            commands::{
                DealOpeningHandsCommand, LibraryCard, MulliganCommand, PlayerDeck, PlayerLibrary,
                StartGameCommand,
            },
            ids::{CardDefinitionId, DeckId, GameId, PlayerId},
        },
        infrastructure::{InMemoryEventBus, InMemoryEventStore},
    };

    struct FailingEventStore;

    impl EventStore for FailingEventStore {
        fn append(
            &self,
            _aggregate_id: &str,
            _events: &[crate::domain::play::events::DomainEvent],
        ) -> Result<(), Box<dyn Error + Send + Sync>> {
            Err(Box::new(io::Error::other("simulated append failure")))
        }

        fn get_events(
            &self,
            _aggregate_id: &str,
        ) -> Result<Arc<[crate::domain::play::events::DomainEvent]>, Box<dyn Error + Send + Sync>>
        {
            Ok(Arc::from(
                Vec::<crate::domain::play::events::DomainEvent>::new(),
            ))
        }
    }

    fn build_game_setup(game_id: &str) -> StartGameCommand {
        StartGameCommand::new(
            GameId::new(game_id),
            vec![
                PlayerDeck::new(PlayerId::new("player-1"), DeckId::new("deck-1")),
                PlayerDeck::new(PlayerId::new("player-2"), DeckId::new("deck-2")),
            ],
        )
    }

    fn build_library(player_prefix: &str, color: ManaColor, count: usize) -> Vec<LibraryCard> {
        (0..count)
            .map(|index| {
                LibraryCard::land(
                    CardDefinitionId::new(format!("{player_prefix}-land-{index}")),
                    color,
                )
            })
            .collect()
    }

    fn opening_hands_command(card_count: usize) -> DealOpeningHandsCommand {
        DealOpeningHandsCommand::new(vec![
            PlayerLibrary::new(
                PlayerId::new("player-1"),
                build_library("p1", ManaColor::Green, card_count),
            ),
            PlayerLibrary::new(
                PlayerId::new("player-2"),
                build_library("p2", ManaColor::Blue, card_count),
            ),
        ])
    }

    #[test]
    fn deal_opening_hands_restores_card_locations_when_persistence_fails() {
        let setup_service = GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new());
        let (mut game, _) = setup_service
            .start_game(build_game_setup("game-opening-hands-rollback"))
            .expect("game should start");
        let before_locations = game.cloned_card_locations();

        let failing_service = GameService::new(FailingEventStore, InMemoryEventBus::new());
        let result = failing_service.deal_opening_hands(&mut game, &opening_hands_command(7));

        assert!(
            result.is_err(),
            "persistence failure should reject the command"
        );
        assert_eq!(game.cloned_card_locations(), before_locations);
        assert_eq!(game.players()[0].hand_size(), 0);
        assert_eq!(game.players()[0].library_size(), 0);
    }

    #[test]
    fn mulligan_restores_card_locations_when_persistence_fails() {
        let setup_service = GameService::new(InMemoryEventStore::new(), InMemoryEventBus::new());
        let (mut game, _) = setup_service
            .start_game(build_game_setup("game-mulligan-rollback"))
            .expect("game should start");
        setup_service
            .deal_opening_hands(&mut game, &opening_hands_command(14))
            .expect("opening hands should succeed");
        let before_locations = game.cloned_card_locations();
        let before_hand = game.players()[0].hand_card_ids();
        let before_library = game.players()[0].library_size();

        let failing_service = GameService::new(FailingEventStore, InMemoryEventBus::new());
        let result =
            failing_service.mulligan(&mut game, MulliganCommand::new(PlayerId::new("player-1")));

        assert!(
            result.is_err(),
            "persistence failure should reject the mulligan"
        );
        assert_eq!(game.cloned_card_locations(), before_locations);
        assert_eq!(game.players()[0].hand_card_ids(), before_hand);
        assert_eq!(game.players()[0].library_size(), before_library);
    }
}
