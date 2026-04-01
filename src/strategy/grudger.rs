//! Grudger Strategy - also called Grim Trigger or Friedman
//!
//! - Cooperates until the opponent defects
//! - Once betrayed, defects FOREVER
//!
//! Nice but unforgiving. Never forgives.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Grudger strategy
#[derive(Debug, Clone)]
pub struct Grudger {
    /// Whether the opponent has ever defected
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
        "Cooperates until betrayed, then defects forever (Grim Trigger)"
    }

    fn decide(&mut self, history: &History) -> Action {
        // Check if opponent has defected in history
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

        // Cooperates first
        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        // Opponent defects
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);

        // Defects forever, even if opponent cooperates again
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
