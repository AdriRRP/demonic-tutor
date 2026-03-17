#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Setup,
    Untap,
    Upkeep,
    Draw,
    FirstMain,
    BeginningOfCombat,
    DeclareAttackers,
    DeclareBlockers,
    CombatDamage,
    EndOfCombat,
    SecondMain,
    EndStep,
}

impl std::fmt::Display for Phase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Setup => write!(f, "Setup"),
            Self::Untap => write!(f, "Untap"),
            Self::Upkeep => write!(f, "Upkeep"),
            Self::Draw => write!(f, "Draw"),
            Self::FirstMain => write!(f, "FirstMain"),
            Self::BeginningOfCombat => write!(f, "BeginningOfCombat"),
            Self::DeclareAttackers => write!(f, "DeclareAttackers"),
            Self::DeclareBlockers => write!(f, "DeclareBlockers"),
            Self::CombatDamage => write!(f, "CombatDamage"),
            Self::EndOfCombat => write!(f, "EndOfCombat"),
            Self::SecondMain => write!(f, "SecondMain"),
            Self::EndStep => write!(f, "EndStep"),
        }
    }
}
