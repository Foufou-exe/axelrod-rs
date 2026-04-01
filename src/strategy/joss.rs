//! Joss Strategy (Sneaky Tit for Tat)
//!
//! Submitted by Johann Joss to Axelrod's first tournament.
//! Plays like Tit for Tat but occasionally defects (~10% of the time)
//! when it would otherwise cooperate.
//!
//! This "sneaky" behavior aims to exploit cooperative opponents,
//! but often backfires by triggering retaliation cycles.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;
use rand::{Rng, RngExt};

/// Joss strategy - Sneaky Tit for Tat
#[derive(Debug, Clone)]
pub struct Joss {
    /// Probability of random defection (default: 0.1)
    sneaky_probability: f64,
}

impl Joss {
    pub fn new() -> Self {
        Self {
            sneaky_probability: 0.1,
        }
    }

    pub fn with_probability(probability: f64) -> Self {
        Self {
            sneaky_probability: probability.clamp(0.0, 1.0),
        }
    }
}

impl Default for Joss {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Joss {
    fn name(&self) -> &'static str {
        "Joss"
    }

    fn description(&self) -> &'static str {
        "Sneaky TFT: like TFT but randomly defects ~10% of the time"
    }

    fn decide(&mut self, history: &History) -> Action {
        // Base TFT behavior
        let tft_action = if history.is_empty() {
            Action::Cooperate
        } else {
            history.last_opponent_action().unwrap_or(Action::Cooperate)
        };

        // If TFT would cooperate, sometimes defect instead
        if tft_action == Action::Cooperate {
            let mut rng = rand::rng();
            if rng.random_range(0.0..1.0) < self.sneaky_probability {
                return Action::Defect;
            }
        }

        tft_action
    }

    fn is_nice(&self) -> bool {
        false // Can defect first due to random sneaky defections
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooperates_first_usually() {
        // With 0% sneaky probability, always cooperates first
        let mut strategy = Joss::with_probability(0.0);
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_copies_opponent_defection() {
        let mut strategy = Joss::with_probability(0.0);
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_always_defects_with_probability_1() {
        let mut strategy = Joss::with_probability(1.0);
        let mut history = History::new();

        // First move: would cooperate but sneaky defects
        assert_eq!(strategy.decide(&history), Action::Defect);

        // After opponent cooperates: would cooperate but sneaky defects
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!Joss::new().is_nice());
    }
}
