//! Actions possibles dans le Dilemme du Prisonnier
//!
//! Chaque joueur peut soit coopérer, soit trahir (défection).

use std::fmt;

/// Représente une action dans le Dilemme du Prisonnier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// Coopérer avec l'autre joueur
    Cooperate,
    /// Trahir l'autre joueur (défection)
    Defect,
}

impl Action {
    /// Retourne l'action opposée
    pub fn opposite(&self) -> Action {
        match self {
            Action::Cooperate => Action::Defect,
            Action::Defect => Action::Cooperate,
        }
    }

    /// Vérifie si l'action est une coopération
    pub fn is_cooperate(&self) -> bool {
        matches!(self, Action::Cooperate)
    }

    /// Vérifie si l'action est une trahison
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
