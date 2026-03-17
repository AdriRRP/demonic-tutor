#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Setup,
    Untap,
    Upkeep,
    Draw,
    FirstMain,
    Combat,
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
            Self::Combat => write!(f, "Combat"),
            Self::SecondMain => write!(f, "SecondMain"),
            Self::EndStep => write!(f, "EndStep"),
        }
    }
}
