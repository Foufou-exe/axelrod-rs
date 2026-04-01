//! Davis Strategy
//!
//! Submitted by Morton Davis to Axelrod's first tournament.
//! Cooperates for the first 10 rounds, then plays Grudger.
//! If the opponent ever defects after round 10, defects forever.
//!
//! This gives the opponent a "grace period" to establish trust.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Davis strategy - 10 rounds of cooperation, then Grudger
#[derive(Debug, Clone)]
pub struct Davis {
    /// Number of initial cooperation rounds (default: 10)
    grace_period: usize,
    /// Whether the opponent has betrayed (after grace period)
    betrayed: bool,
}

impl Davis {
    pub fn new() -> Self {
        Self {
            grace_period: 10,
            betrayed: false,
        }
    }

    pub fn with_grace_period(rounds: usize) -> Self {
        Self {
            grace_period: rounds,
            betrayed: false,
        }
    }
}

impl Default for Davis {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Davis {
    fn name(&self) -> &'static str {
        "Davis"
    }

    fn description(&self) -> &'static str {
        "Cooperates 10 rounds, then plays Grudger"
    }

    fn decide(&mut self, history: &History) -> Action {
        let round = history.len();

        // During grace period: always cooperate
        if round < self.grace_period {
            return Action::Cooperate;
        }

        // After grace period: check for betrayal
        if !self.betrayed {
            // Check if opponent defected
            if let Some(last) = history.last() {
                if last.opponent_action == Action::Defect {
                    self.betrayed = true;
                }
            }
        }

        // Grudger behavior after grace period
        if self.betrayed {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }

    fn reset(&mut self) {
        self.betrayed = false;
    }

    fn is_nice(&self) -> bool {
        true // Cooperates first and during grace period
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
        let mut strategy = Davis::new();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_cooperates_during_grace_period() {
        let mut strategy = Davis::with_grace_period(5);
        let mut history = History::new();

        // Even if opponent defects during grace period, keep cooperating
        for _ in 0..5 {
            assert_eq!(strategy.decide(&history), Action::Cooperate);
            history.push(Action::Cooperate, Action::Defect);
        }
    }

    #[test]
    fn test_cooperates_after_grace_if_no_betrayal() {
        let mut strategy = Davis::with_grace_period(3);
        let mut history = History::new();

        // Grace period with cooperation
        for _ in 0..3 {
            history.push(Action::Cooperate, Action::Cooperate);
        }

        // After grace period: still cooperate if no betrayal
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_defects_forever_after_post_grace_betrayal() {
        let mut strategy = Davis::with_grace_period(2);
        let mut history = History::new();

        // Grace period
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);

        // Betrayal after grace period
        history.push(Action::Cooperate, Action::Defect);

        // Should defect forever
        assert_eq!(strategy.decide(&history), Action::Defect);
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect);
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_reset() {
        let mut strategy = Davis::new();
        strategy.betrayed = true;

        strategy.reset();
        assert!(!strategy.betrayed);
    }

    #[test]
    fn test_is_nice() {
        assert!(Davis::new().is_nice());
    }
}
