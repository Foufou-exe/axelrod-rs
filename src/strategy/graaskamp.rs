//! Graaskamp Strategy
//!
//! Submitted by James Graaskamp to Axelrod's first tournament.
//! Plays Tit for Tat but defects on round 50 to test the opponent.
//! If the opponent retaliates, continues with TFT.
//! If the opponent doesn't retaliate, may continue to exploit.
//!
//! This probing behavior tests whether opponents are "pushovers".

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Graaskamp strategy - TFT with a probe at round 50
#[derive(Debug, Clone)]
pub struct Graaskamp {
    /// The round on which to probe (default: 50)
    probe_round: usize,
}

impl Graaskamp {
    pub fn new() -> Self {
        Self { probe_round: 50 }
    }

    pub fn with_probe_round(round: usize) -> Self {
        Self { probe_round: round }
    }
}

impl Default for Graaskamp {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Graaskamp {
    fn name(&self) -> &'static str {
        "Graaskamp"
    }

    fn description(&self) -> &'static str {
        "TFT but defects on round 50 to test the opponent"
    }

    fn decide(&mut self, history: &History) -> Action {
        let round = history.len();

        // Probe on the specified round
        if round == self.probe_round - 1 {
            return Action::Defect;
        }

        // First round: cooperate
        if history.is_empty() {
            return Action::Cooperate;
        }

        // Standard TFT: copy opponent's last move
        history.last_opponent_action().unwrap_or(Action::Cooperate)
    }

    fn is_nice(&self) -> bool {
        false // Defects first on round 50
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
        let mut strategy = Graaskamp::new();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_copies_opponent() {
        let mut strategy = Graaskamp::new();
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);

        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_defects_on_probe_round() {
        let mut strategy = Graaskamp::with_probe_round(5);
        let mut history = History::new();

        // Fill history up to round 4
        for _ in 0..4 {
            history.push(Action::Cooperate, Action::Cooperate);
        }

        // Round 5 (index 4): should defect
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_resumes_tft_after_probe() {
        let mut strategy = Graaskamp::with_probe_round(3);
        let mut history = History::new();

        // Rounds 1-2: cooperate
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);

        // Round 3: probe (defect)
        assert_eq!(strategy.decide(&history), Action::Defect);
        history.push(Action::Defect, Action::Cooperate);

        // Round 4: back to TFT (opponent cooperated)
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!Graaskamp::new().is_nice());
    }
}
