use super::GameplayWorld;
use demonictutor::{
    CardDefinitionId, CardInstance, CardInstanceId, Game, Phase, PlayerId,
};

impl GameplayWorld {
    pub fn is_initialized(&self) -> bool {
        self.game.is_some()
    }

    pub fn game(&self) -> &Game {
        self.game
            .as_ref()
            .expect("world game should be initialized")
    }

    pub fn game_mut(&mut self) -> &mut Game {
        self.game
            .as_mut()
            .expect("world game should be initialized")
    }

    pub fn player_id(alias: &str) -> PlayerId {
        match alias {
            "Alice" => PlayerId::new("player-1"),
            "Bob" => PlayerId::new("player-2"),
            _ => panic!("unknown player alias: {alias}"),
        }
    }

    pub fn phase_from_name(name: &str) -> Phase {
        match name {
            "Untap" => Phase::Untap,
            "Upkeep" => Phase::Upkeep,
            "Draw" => Phase::Draw,
            "FirstMain" => Phase::FirstMain,
            "Combat" | "BeginningOfCombat" => Phase::BeginningOfCombat,
            "DeclareAttackers" => Phase::DeclareAttackers,
            "DeclareBlockers" => Phase::DeclareBlockers,
            "CombatDamage" => Phase::CombatDamage,
            "EndOfCombat" => Phase::EndOfCombat,
            "SecondMain" => Phase::SecondMain,
            "EndStep" => Phase::EndStep,
            other => panic!("unsupported phase in BDD suite: {other}"),
        }
    }

    pub fn player(&self, alias: &str) -> &demonictutor::domain::play::game::Player {
        let player_id = Self::player_id(alias);
        self.game()
            .players()
            .iter()
            .find(|player| player.id() == &player_id)
            .unwrap_or_else(|| panic!("player should exist: {player_id}"))
    }

    pub fn hand_contains(&self, alias: &str, card_id: &CardInstanceId) -> bool {
        self.player(alias)
            .hand()
            .cards()
            .iter()
            .any(|card: &CardInstance| card.id() == card_id)
    }

    pub fn battlefield_contains(&self, alias: &str, card_id: &CardInstanceId) -> bool {
        self.player(alias)
            .battlefield()
            .cards()
            .iter()
            .any(|card: &CardInstance| card.id() == card_id)
    }

    pub fn graveyard_contains(&self, alias: &str, card_id: &CardInstanceId) -> bool {
        self.player(alias)
            .graveyard()
            .cards()
            .iter()
            .any(|card: &CardInstance| card.id() == card_id)
    }

    pub fn battlefield_card(&self, alias: &str, card_id: &CardInstanceId) -> &CardInstance {
        self.player(alias)
            .battlefield()
            .cards()
            .iter()
            .find(|card: &&CardInstance| card.id() == card_id)
            .unwrap_or_else(|| panic!("battlefield card should exist: {card_id}"))
    }

    pub fn hand_card_by_definition(&self, alias: &str, definition_id: &str) -> CardInstanceId {
        let definition_id = CardDefinitionId::new(definition_id);
        self.player(alias)
            .hand()
            .cards()
            .iter()
            .find(|card: &&CardInstance| card.definition_id() == &definition_id)
            .unwrap_or_else(|| panic!("hand card should exist: {definition_id}"))
            .id()
            .clone()
    }

    pub fn player_hand_size(&self, alias: &str) -> usize {
        self.player(alias).hand().cards().len()
    }

    pub fn player_library_size(&self, alias: &str) -> usize {
        self.player(alias).library().len()
    }

    pub fn player_life(&self, alias: &str) -> u32 {
        self.player(alias).life()
    }

    pub fn tracked_card(&self, alias: &str) -> &CardInstance {
        let card_id = self
            .tracked_card_id
            .as_ref()
            .expect("tracked card should exist");
        self.battlefield_card(alias, card_id)
    }

    pub fn tracked_attacker(&self) -> &CardInstance {
        let attacker_id = self
            .tracked_attacker_id
            .as_ref()
            .expect("tracked attacker should exist");
        self.battlefield_card("Alice", attacker_id)
    }

    pub fn tracked_blocker(&self) -> &CardInstance {
        let blocker_id = self
            .tracked_blocker_id
            .as_ref()
            .expect("tracked blocker should exist");
        self.battlefield_card("Bob", blocker_id)
    }
}
