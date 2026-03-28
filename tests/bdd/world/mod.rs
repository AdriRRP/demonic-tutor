//! BDD coverage for bdd world.

#[path = "../../unit/support/mod.rs"]
pub mod support;

mod actions;
mod queries;
mod setup;
mod setup_combat_windows;
mod setup_priority_windows;

use demonictutor::{
    ActivatedAbilityPutOnStack, CardDiscarded, CardDrawn, CardInstanceId, CardMovedZone,
    CombatDamageResolved, CreatureDied, Game, GameEnded, LifeChanged, PriorityPassed, SpellCast,
    SpellPutOnStack, StackTopResolved, TurnProgressed,
};

#[derive(Debug, Default, cucumber::World)]
pub struct GameplayWorld {
    pub(super) game: Option<Game>,
    pub last_turn_progressed: Option<TurnProgressed>,
    pub last_game_ended: Option<GameEnded>,
    pub last_card_drawn: Option<CardDrawn>,
    pub last_cards_drawn: Vec<CardDrawn>,
    pub last_card_discarded: Option<CardDiscarded>,
    pub last_zone_change: Option<CardMovedZone>,
    pub last_activated_ability_put_on_stack: Option<ActivatedAbilityPutOnStack>,
    pub last_spell_put_on_stack: Option<SpellPutOnStack>,
    pub last_spell_cast: Option<SpellCast>,
    pub last_priority_passed: Option<PriorityPassed>,
    pub last_stack_top_resolved: Option<StackTopResolved>,
    pub last_combat_damage: Option<CombatDamageResolved>,
    pub last_life_changed: Option<LifeChanged>,
    pub last_creature_died: Vec<CreatureDied>,
    pub last_error: Option<String>,
    pub pre_advance_hand_size: Option<usize>,
    pub post_advance_hand_size: Option<usize>,
    pub tracked_card_id: Option<CardInstanceId>,
    pub tracked_response_card_id: Option<CardInstanceId>,
    pub tracked_second_response_card_id: Option<CardInstanceId>,
    pub tracked_attacker_id: Option<CardInstanceId>,
    pub tracked_blocker_id: Option<CardInstanceId>,
    pub blocker_assignments: Vec<(CardInstanceId, CardInstanceId)>,
}
