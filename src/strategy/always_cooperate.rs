//! Stratégie Always Cooperate (Coopération totale)
//!
//! Coopère toujours, quoi que fasse l'adversaire.
//! C'est une stratégie "gentille" mais facilement exploitable.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Stratégie qui coopère toujours
#[derive(Debug, Clone, Copy, Default)]
pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    fn name(&self) -> &'static str {
        "Always Cooperate"
    }

    fn description(&self) -> &'static str {
        "Coopère toujours, peu importe les actions de l'adversaire"
    }

    fn decide(&mut self, _history: &History) -> Action {
        Action::Cooperate
    }

    fn is_nice(&self) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_cooperates() {
        let mut strategy = AlwaysCooperate;
        let mut history = History::new();

        // Premier tour
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        // Après des coopérations
        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        // Après des trahisons de l'adversaire
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(AlwaysCooperate.is_nice());
    }
}
