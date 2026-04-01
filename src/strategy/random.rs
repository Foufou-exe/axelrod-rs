//! Random Strategy
//!
//! Cooperates or defects randomly with a 50% probability.
//! Useful as a baseline and for testing the robustness of other strategies.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;
use rand::RngExt;

/// Random strategy
#[derive(Debug, Clone)]
pub struct Random {
    /// Probability of cooperating (default 0.5)
    cooperation_probability: f64,
}

impl Random {
    /// Creates a new instance with custom cooperation probability
    pub fn with_probability(cooperation_probability: f64) -> Self {
        Self {
            cooperation_probability: cooperation_probability.clamp(0.0, 1.0),
        }
    }

    /// Creates an instance with 50/50 probability
    pub fn new() -> Self {
        Self::with_probability(0.5)
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Random {
    fn name(&self) -> &'static str {
        "Random"
    }

    fn description(&self) -> &'static str {
        "Cooperates or defects randomly (50/50)"
    }

    fn decide(&mut self, _history: &History) -> Action {
        let mut rng = rand::rng();
        if rng.random_range(0.0..1.0) < self.cooperation_probability {
            Action::Cooperate
        } else {
            Action::Defect
        }
    }

    fn is_nice(&self) -> bool {
        false // May defect first
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_cooperates_with_probability_1() {
        let mut strategy = Random::with_probability(1.0);
        let history = History::new();

        for _ in 0..100 {
            assert_eq!(strategy.decide(&history), Action::Cooperate);
        }
    }

    #[test]
    fn test_always_defects_with_probability_0() {
        let mut strategy = Random::with_probability(0.0);
        let history = History::new();

        for _ in 0..100 {
            assert_eq!(strategy.decide(&history), Action::Defect);
        }
    }

    #[test]
    fn test_produces_both_actions() {
        let mut strategy = Random::new();
        let history = History::new();

        let mut cooperations = 0;
        let mut defections = 0;

        for _ in 0..1000 {
            match strategy.decide(&history) {
                Action::Cooperate => cooperations += 1,
                Action::Defect => defections += 1,
            }
        }

        // With 1000 trials and p=0.5, we should have both
        assert!(cooperations > 0);
        assert!(defections > 0);
        // And approximately 50% each (with margin)
        assert!(cooperations > 400 && cooperations < 600);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!Random::new().is_nice());
    }
}
