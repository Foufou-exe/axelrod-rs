//! Tullock Strategy
//!
//! Submitted by Gordon Tullock to Axelrod's first tournament.
//! Cooperates for the first 11 rounds, then plays randomly
//! with only 10% cooperation probability.
//!
//! This strategy tries to establish early cooperation but then
//! switches to mostly defection, hoping to exploit the relationship.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;
use rand::RngExt;

/// Tullock strategy
#[derive(Debug, Clone)]
pub struct Tullock {
    /// Number of initial cooperation rounds (default: 11)
    initial_coop_rounds: usize,
    /// Cooperation probability after initial phase (default: 0.1)
    coop_probability: f64,
}

impl Tullock {
    pub fn new() -> Self {
        Self {
            initial_coop_rounds: 11,
            coop_probability: 0.1,
        }
    }

    pub fn with_params(initial_rounds: usize, coop_prob: f64) -> Self {
        Self {
            initial_coop_rounds: initial_rounds,
            coop_probability: coop_prob.clamp(0.0, 1.0),
        }
    }
}

impl Default for Tullock {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Tullock {
    fn name(&self) -> &'static str {
        "Tullock"
    }

    fn description(&self) -> &'static str {
        "Cooperates 11 rounds, then mostly defects (10% cooperation)"
    }

    fn decide(&mut self, history: &History) -> Action {
        let round = history.len();

        // Cooperate during initial phase
        if round < self.initial_coop_rounds {
            return Action::Cooperate;
        }

        // After initial phase: mostly defect
        let mut rng = rand::rng();
        if rng.random_range(0.0..1.0) < self.coop_probability {
            Action::Cooperate
        } else {
            Action::Defect
        }
    }

    fn is_nice(&self) -> bool {
        false // Switches to mostly defection after initial phase
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooperates_initially() {
        let mut strategy = Tullock::new();
        let mut history = History::new();

        // First 11 rounds should all cooperate
        for _ in 0..11 {
            assert_eq!(strategy.decide(&history), Action::Cooperate);
            history.push(Action::Cooperate, Action::Cooperate);
        }
    }

    #[test]
    fn test_mostly_defects_after_initial_phase() {
        let mut strategy = Tullock::with_params(2, 0.0); // 0% coop after initial
        let mut history = History::new();

        // Fill initial phase
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);

        // After initial phase: always defect with 0% coop
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_always_cooperates_with_probability_1() {
        let mut strategy = Tullock::with_params(0, 1.0); // 100% coop
        let history = History::new();

        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!Tullock::new().is_nice());
    }
}
