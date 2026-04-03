//! Feld Strategy
//!
//! Submitted by Scott Feld to Axelrod's first tournament.
//! Plays Tit for Tat but with a cooperation probability that
//! decreases over time. Starts at 100% and decreases to 50%
//! by round 200.
//!
//! The idea is to gradually become less forgiving as the game progresses.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;
use rand::RngExt;

/// Feld strategy - TFT with decreasing cooperation
#[derive(Debug, Clone)]
pub struct Feld {
    /// Starting cooperation probability (default: 1.0)
    start_coop_prob: f64,
    /// Ending cooperation probability (default: 0.5)
    end_coop_prob: f64,
    /// Number of rounds for the transition (default: 200)
    transition_rounds: usize,
}

impl Feld {
    pub fn new() -> Self {
        Self {
            start_coop_prob: 1.0,
            end_coop_prob: 0.5,
            transition_rounds: 200,
        }
    }

    pub fn with_params(start: f64, end: f64, rounds: usize) -> Self {
        Self {
            start_coop_prob: start.clamp(0.0, 1.0),
            end_coop_prob: end.clamp(0.0, 1.0),
            transition_rounds: rounds.max(1),
        }
    }

    /// Calculate cooperation probability at a given round
    fn coop_probability(&self, round: usize) -> f64 {
        if round >= self.transition_rounds {
            return self.end_coop_prob;
        }

        let progress = round as f64 / self.transition_rounds as f64;
        self.start_coop_prob + (self.end_coop_prob - self.start_coop_prob) * progress
    }
}

impl Default for Feld {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Feld {
    fn name(&self) -> &'static str {
        "Feld"
    }

    fn description(&self) -> &'static str {
        "TFT with decreasing cooperation probability (100% -> 50%)"
    }

    fn decide(&mut self, history: &History) -> Action {
        let round = history.len();

        // First round: cooperate
        if history.is_empty() {
            return Action::Cooperate;
        }

        // Get opponent's last action (TFT base)
        let opponent_last = history.last_opponent_action().unwrap_or(Action::Cooperate);

        // If opponent defected, defect back (standard TFT)
        if opponent_last == Action::Defect {
            return Action::Defect;
        }

        // If opponent cooperated, cooperate with decreasing probability
        let coop_prob = self.coop_probability(round);
        let mut rng = rand::rng();

        if rng.random_range(0.0..1.0) < coop_prob {
            Action::Cooperate
        } else {
            Action::Defect
        }
    }

    fn is_nice(&self) -> bool {
        false // Can randomly defect even when opponent cooperates
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
        let mut strategy = Feld::new();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_retaliates_against_defection() {
        let mut strategy = Feld::new();
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_coop_probability_decreases() {
        let strategy = Feld::new();

        // At round 0: 100%
        assert!((strategy.coop_probability(0) - 1.0).abs() < 0.001);

        // At round 100: 75%
        assert!((strategy.coop_probability(100) - 0.75).abs() < 0.001);

        // At round 200: 50%
        assert!((strategy.coop_probability(200) - 0.5).abs() < 0.001);

        // After round 200: stays at 50%
        assert!((strategy.coop_probability(300) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_always_cooperates_with_high_prob() {
        // 100% cooperation probability throughout
        let mut strategy = Feld::with_params(1.0, 1.0, 200);
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!Feld::new().is_nice());
    }
}
