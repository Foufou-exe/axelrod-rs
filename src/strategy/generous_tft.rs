//! Generous Tit for Tat (GTFT) Strategy
//!
//! A variant of TFT that occasionally forgives defections.
//! - Cooperates on the first move
//! - Copies opponent, BUT forgives defections with probability p
//!
//! Helps break cycles of mutual retaliation.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;
use rand::RngExt;

/// Generous Tit for Tat strategy
#[derive(Debug, Clone)]
pub struct GenerousTitForTat {
    /// Probability of forgiving a defection (typically 0.05 to 0.10)
    forgiveness_probability: f64,
}

impl GenerousTitForTat {
    /// Creates a new instance with a custom forgiveness probability
    pub fn new(forgiveness_probability: f64) -> Self {
        Self {
            forgiveness_probability: forgiveness_probability.clamp(0.0, 1.0),
        }
    }

    /// Creates an instance with the default forgiveness probability (5%)
    pub fn default() -> Self {
        Self::new(0.05)
    }
}

impl Strategy for GenerousTitForTat {
    fn name(&self) -> &'static str {
        "Generous Tit for Tat"
    }

    fn description(&self) -> &'static str {
        "Like TFT but sometimes forgives defections (~5%)"
    }

    fn decide(&mut self, history: &History) -> Action {
        match history.last_opponent_action() {
            None => Action::Cooperate,
            Some(Action::Cooperate) => Action::Cooperate,
            Some(Action::Defect) => {
                // Forgive with a certain probability
                let mut rng = rand::rng();
                if rng.random_range(0.0..1.0) < self.forgiveness_probability {
                    Action::Cooperate
                } else {
                    Action::Defect
                }
            }
        }
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
        let mut strategy = GenerousTitForTat::default();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_cooperates_after_cooperation() {
        let mut strategy = GenerousTitForTat::default();
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_always_forgives_with_probability_1() {
        let mut strategy = GenerousTitForTat::new(1.0);
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        // With probability 1, always forgives
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_never_forgives_with_probability_0() {
        let mut strategy = GenerousTitForTat::new(0.0);
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        // With probability 0, never forgives
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_is_nice() {
        assert!(GenerousTitForTat::default().is_nice());
    }
}
