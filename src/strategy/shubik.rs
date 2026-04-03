//! Shubik Strategy (Escalating Retaliation)
//!
//! Submitted by Martin Shubik to Axelrod's first tournament.
//! Cooperates until betrayed, then retaliates with increasing severity.
//! After the N-th betrayal, defects N times before returning to cooperation.
//!
//! This creates an escalating punishment that discourages repeated defections.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Shubik strategy - escalating retaliation
#[derive(Debug, Clone)]
pub struct Shubik {
    /// Number of betrayals received
    betrayal_count: u32,
    /// Remaining retaliation rounds
    retaliation_remaining: u32,
}

impl Shubik {
    pub fn new() -> Self {
        Self {
            betrayal_count: 0,
            retaliation_remaining: 0,
        }
    }
}

impl Default for Shubik {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Shubik {
    fn name(&self) -> &'static str {
        "Shubik"
    }

    fn description(&self) -> &'static str {
        "Escalating retaliation: defects N times after N-th betrayal"
    }

    fn decide(&mut self, history: &History) -> Action {
        // Check if opponent defected last round
        if let Some(last) = history.last()
            && last.opponent_action == Action::Defect
            && self.retaliation_remaining == 0
        {
            // New betrayal detected (not during retaliation)
            self.betrayal_count += 1;
            self.retaliation_remaining = self.betrayal_count;
        }

        // If in retaliation mode, defect
        if self.retaliation_remaining > 0 {
            self.retaliation_remaining -= 1;
            return Action::Defect;
        }

        // Otherwise cooperate
        Action::Cooperate
    }

    fn reset(&mut self) {
        self.betrayal_count = 0;
        self.retaliation_remaining = 0;
    }

    fn is_nice(&self) -> bool {
        true // Cooperates first, only defects in retaliation
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
        let mut strategy = Shubik::new();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_cooperates_with_cooperator() {
        let mut strategy = Shubik::new();
        let mut history = History::new();

        for _ in 0..5 {
            assert_eq!(strategy.decide(&history), Action::Cooperate);
            history.push(Action::Cooperate, Action::Cooperate);
        }
    }

    #[test]
    fn test_retaliates_once_after_first_betrayal() {
        let mut strategy = Shubik::new();
        let mut history = History::new();

        // Opponent defects
        history.push(Action::Cooperate, Action::Defect);

        // Should defect once (1st betrayal = 1 retaliation)
        assert_eq!(strategy.decide(&history), Action::Defect);
        history.push(Action::Defect, Action::Cooperate);

        // Should cooperate again
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_retaliates_twice_after_second_betrayal() {
        let mut strategy = Shubik::new();
        let mut history = History::new();

        // First betrayal
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect); // Retaliate 1
        history.push(Action::Defect, Action::Cooperate);

        // Back to cooperation
        assert_eq!(strategy.decide(&history), Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);

        // Second betrayal
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect); // Retaliate 1
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect); // Retaliate 2
        history.push(Action::Defect, Action::Cooperate);

        // Back to cooperation
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_reset() {
        let mut strategy = Shubik::new();
        let mut history = History::new();

        // Trigger betrayal
        history.push(Action::Cooperate, Action::Defect);
        strategy.decide(&history);

        // Reset
        strategy.reset();
        assert_eq!(strategy.betrayal_count, 0);
        assert_eq!(strategy.retaliation_remaining, 0);
    }

    #[test]
    fn test_is_nice() {
        assert!(Shubik::new().is_nice());
    }
}
