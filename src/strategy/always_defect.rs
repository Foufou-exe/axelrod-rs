//! Stratégie Always Defect (Trahison totale) - aussi appelée ALL D
//!
//! Trahit toujours, quoi que fasse l'adversaire.
//! C'est la stratégie Nash-optimale pour un jeu unique, mais sous-optimale en itéré.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Stratégie qui trahit toujours
#[derive(Debug, Clone, Copy, Default)]
pub struct AlwaysDefect;

impl Strategy for AlwaysDefect {
    fn name(&self) -> &'static str {
        "Always Defect"
    }

    fn description(&self) -> &'static str {
        "Trahit toujours, peu importe les actions de l'adversaire (ALL D)"
    }

    fn decide(&mut self, _history: &History) -> Action {
        Action::Defect
    }

    fn is_nice(&self) -> bool {
        false
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_defects() {
        let mut strategy = AlwaysDefect;
        let mut history = History::new();

        // Premier tour
        assert_eq!(strategy.decide(&history), Action::Defect);

        // Après des coopérations de l'adversaire
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect);

        // Après des trahisons de l'adversaire
        history.push(Action::Defect, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!AlwaysDefect.is_nice());
    }
}
