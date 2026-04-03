//! Nydegger Strategy
//!
//! Submitted by Rudy Nydegger to Axelrod's first tournament.
//! Uses a lookup table based on the first 3 rounds to decide
//! subsequent moves. The strategy encodes the history as a number
//! and uses it to look up whether to cooperate or defect.
//!
//! One of the more complex strategies in the original tournament.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Nydegger strategy - uses a lookup table based on recent history
#[derive(Debug, Clone)]
pub struct Nydegger;

impl Nydegger {
    pub fn new() -> Self {
        Self
    }

    /// Calculate the "A" value based on the last 3 rounds
    /// A = 16*a1 + 4*a2 + a3 where ai = 2 if we defected, 1 if opponent defected
    fn calculate_a(history: &History) -> u32 {
        let len = history.len();
        if len < 3 {
            return 0;
        }

        let mut a: u32 = 0;

        // Last 3 rounds (most recent first)
        for (i, multiplier) in [(0, 16), (1, 4), (2, 1)] {
            if let Some(round) = history.get(len - 1 - i) {
                let val = match (round.my_action, round.opponent_action) {
                    (Action::Defect, Action::Defect) => 3,
                    (Action::Defect, Action::Cooperate) => 2,
                    (Action::Cooperate, Action::Defect) => 1,
                    (Action::Cooperate, Action::Cooperate) => 0,
                };
                a += multiplier * val;
            }
        }

        a
    }

    /// The lookup table for defection
    /// These are the A values where Nydegger defects
    const DEFECT_VALUES: [u32; 9] = [1, 6, 7, 17, 22, 23, 26, 29, 30];

    fn should_defect(a: u32) -> bool {
        Self::DEFECT_VALUES.contains(&a)
    }
}

impl Default for Nydegger {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Nydegger {
    fn name(&self) -> &'static str {
        "Nydegger"
    }

    fn description(&self) -> &'static str {
        "Uses lookup table based on last 3 rounds to decide"
    }

    fn decide(&mut self, history: &History) -> Action {
        match history.len() {
            // First 3 rounds: TFT-like behavior
            0 => Action::Cooperate,
            1 => history.last_opponent_action().unwrap_or(Action::Cooperate),
            2 => history.last_opponent_action().unwrap_or(Action::Cooperate),
            _ => {
                // Use lookup table
                let a = Self::calculate_a(history);
                if Self::should_defect(a) {
                    Action::Defect
                } else {
                    Action::Cooperate
                }
            }
        }
    }

    fn is_nice(&self) -> bool {
        true // Cooperates first and follows TFT-like rules initially
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
        let mut strategy = Nydegger::new();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_copies_opponent_early() {
        let mut strategy = Nydegger::new();
        let mut history = History::new();

        // Round 1
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);

        // Round 2
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_calculate_a() {
        let mut history = History::new();

        // All cooperation: A = 0
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(Nydegger::calculate_a(&history), 0);

        // All defection: A = 16*3 + 4*3 + 3 = 48 + 12 + 3 = 63
        let mut history2 = History::new();
        history2.push(Action::Defect, Action::Defect);
        history2.push(Action::Defect, Action::Defect);
        history2.push(Action::Defect, Action::Defect);
        assert_eq!(Nydegger::calculate_a(&history2), 63);
    }

    #[test]
    fn test_uses_lookup_after_round_3() {
        let mut strategy = Nydegger::new();
        let mut history = History::new();

        // Setup: 3 rounds of mutual cooperation
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);

        // A = 0, not in defect list, so cooperate
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(Nydegger::new().is_nice());
    }
}
