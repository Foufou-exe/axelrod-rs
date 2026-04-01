//! Stratégie Grudger (Rancunier) - aussi appelée Grim Trigger ou Friedman
//!
//! - Coopère jusqu'à ce que l'adversaire trahisse
//! - Une fois trahi, trahit pour TOUJOURS
//!
//! Gentille mais impitoyable. Ne pardonne jamais.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Stratégie Grudger (Rancunier)
#[derive(Debug, Clone)]
pub struct Grudger {
    /// Si l'adversaire a déjà trahi
    has_been_betrayed: bool,
}

impl Grudger {
    pub fn new() -> Self {
        Self {
            has_been_betrayed: false,
        }
    }
}

impl Default for Grudger {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Grudger {
    fn name(&self) -> &'static str {
        "Grudger"
    }

    fn description(&self) -> &'static str {
        "Coopère jusqu'à une trahison, puis trahit pour toujours (Grim Trigger)"
    }

    fn decide(&mut self, history: &History) -> Action {
        // Vérifie si l'adversaire a trahi dans l'historique
        if !self.has_been_betrayed && history.opponent_has_defected() {
            self.has_been_betrayed = true;
        }

        if self.has_been_betrayed {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }

    fn reset(&mut self) {
        self.has_been_betrayed = false;
    }

    fn is_nice(&self) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooperates_first() {
        let mut strategy = Grudger::new();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_cooperates_while_opponent_cooperates() {
        let mut strategy = Grudger::new();
        let mut history = History::new();

        for _ in 0..10 {
            assert_eq!(strategy.decide(&history), Action::Cooperate);
            history.push(Action::Cooperate, Action::Cooperate);
        }
    }

    #[test]
    fn test_defects_forever_after_betrayal() {
        let mut strategy = Grudger::new();
        let mut history = History::new();

        // Coopère d'abord
        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        // L'adversaire trahit
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);

        // Trahit pour toujours, même si l'adversaire coopère à nouveau
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect);

        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_reset() {
        let mut strategy = Grudger::new();
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);

        strategy.reset();
        let new_history = History::new();
        assert_eq!(strategy.decide(&new_history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(Grudger::new().is_nice());
    }
}
