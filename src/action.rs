//! Possible actions in the Prisoner's Dilemma
//!
//! Each player can either cooperate or defect.

use std::fmt;

/// Represents an action in the Prisoner's Dilemma
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// Cooperate with the other player
    Cooperate,
    /// Defect against the other player
    Defect,
}

impl Action {
    /// Returns the opposite action
    pub fn opposite(&self) -> Action {
        match self {
            Action::Cooperate => Action::Defect,
            Action::Defect => Action::Cooperate,
        }
    }

    /// Checks if the action is cooperation
    pub fn is_cooperate(&self) -> bool {
        matches!(self, Action::Cooperate)
    }

    /// Checks if the action is defection
    pub fn is_defect(&self) -> bool {
        matches!(self, Action::Defect)
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Cooperate => write!(f, "C"),
            Action::Defect => write!(f, "D"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opposite() {
        assert_eq!(Action::Cooperate.opposite(), Action::Defect);
        assert_eq!(Action::Defect.opposite(), Action::Cooperate);
    }

    #[test]
    fn test_is_cooperate() {
        assert!(Action::Cooperate.is_cooperate());
        assert!(!Action::Defect.is_cooperate());
    }

    #[test]
    fn test_is_defect() {
        assert!(Action::Defect.is_defect());
        assert!(!Action::Cooperate.is_defect());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Action::Cooperate), "C");
        assert_eq!(format!("{}", Action::Defect), "D");
    }
}
