//! Supports domain play events.

mod combat;
mod lifecycle;
mod resource_actions;
mod stack_priority;
mod turn_flow;
mod zones;

pub use combat::{
    AttackersDeclared, BlockersDeclared, CombatDamageResolved, CreatureDied, DamageEvent,
    DamageTarget,
};
pub use lifecycle::{GameEndReason, GameEnded, GameStarted, OpeningHandDealt};
pub use resource_actions::{LandPlayed, LandTapped, LifeChanged, ManaAdded};
pub use stack_priority::{
    ActivatedAbilityPutOnStack, PriorityPassed, SpellCast, SpellCastOutcome, SpellPutOnStack,
    StackTopResolved, TriggeredAbilityPutOnStack,
};
pub use turn_flow::{
    CardDiscarded, CardDrawn, DiscardKind, DrawKind, MulliganTaken, TurnProgressed,
};
pub use zones::{CardExiled, CardMovedZone, ZoneType};

macro_rules! impl_domain_event_from {
    ($event_type:ident, $variant:ident) => {
        impl From<$event_type> for DomainEvent {
            fn from(event: $event_type) -> Self {
                Self::$variant(event)
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum DomainEvent {
    GameStarted(GameStarted),
    OpeningHandDealt(OpeningHandDealt),
    GameEnded(GameEnded),
    LandPlayed(LandPlayed),
    TurnProgressed(TurnProgressed),
    CardDrawn(CardDrawn),
    CardDiscarded(CardDiscarded),
    MulliganTaken(MulliganTaken),
    LifeChanged(LifeChanged),
    LandTapped(LandTapped),
    ManaAdded(ManaAdded),
    ActivatedAbilityPutOnStack(ActivatedAbilityPutOnStack),
    TriggeredAbilityPutOnStack(TriggeredAbilityPutOnStack),
    SpellPutOnStack(SpellPutOnStack),
    PriorityPassed(PriorityPassed),
    StackTopResolved(StackTopResolved),
    SpellCast(SpellCast),
    AttackersDeclared(AttackersDeclared),
    BlockersDeclared(BlockersDeclared),
    CombatDamageResolved(CombatDamageResolved),
    CreatureDied(CreatureDied),
    CardMovedZone(CardMovedZone),
    CardExiled(CardExiled),
}

impl_domain_event_from!(GameStarted, GameStarted);
impl_domain_event_from!(OpeningHandDealt, OpeningHandDealt);
impl_domain_event_from!(GameEnded, GameEnded);
impl_domain_event_from!(LandPlayed, LandPlayed);
impl_domain_event_from!(TurnProgressed, TurnProgressed);
impl_domain_event_from!(CardDrawn, CardDrawn);
impl_domain_event_from!(CardDiscarded, CardDiscarded);
impl_domain_event_from!(MulliganTaken, MulliganTaken);
impl_domain_event_from!(LifeChanged, LifeChanged);
impl_domain_event_from!(LandTapped, LandTapped);
impl_domain_event_from!(ManaAdded, ManaAdded);
impl_domain_event_from!(ActivatedAbilityPutOnStack, ActivatedAbilityPutOnStack);
impl_domain_event_from!(TriggeredAbilityPutOnStack, TriggeredAbilityPutOnStack);
impl_domain_event_from!(SpellPutOnStack, SpellPutOnStack);
impl_domain_event_from!(PriorityPassed, PriorityPassed);
impl_domain_event_from!(StackTopResolved, StackTopResolved);
impl_domain_event_from!(SpellCast, SpellCast);
impl_domain_event_from!(AttackersDeclared, AttackersDeclared);
impl_domain_event_from!(BlockersDeclared, BlockersDeclared);
impl_domain_event_from!(CombatDamageResolved, CombatDamageResolved);
impl_domain_event_from!(CreatureDied, CreatureDied);
impl_domain_event_from!(CardMovedZone, CardMovedZone);
impl_domain_event_from!(CardExiled, CardExiled);
