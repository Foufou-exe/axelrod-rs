//! Grofman Strategy
//!
//! Submitted by Bernard Grofman to Axelrod's first tournament.
//! Cooperates if both players did the same thing in the previous round
//! (both cooperated or both defected), otherwise defects.
//!
//! The idea is to reward coordination and punish asymmetry.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Grofman strategy - cooperates on coordination
#[derive(Debug, Clone, Copy)]
pub struct Grofman;

impl Grofman {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Grofman {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Grofman {
    fn name(&self) -> &'static str {
        "Grofman"
    }

    fn description(&self) -> &'static str {
        "Cooperates if both players did the same thing last round"
    }

    fn decide(&mut self, history: &History) -> Action {
        // First round: cooperate
        if history.is_empty() {
            return Action::Cooperate;
        }

        // Get last round
        if let Some(last) = history.last() {
            // Cooperate if same action (coordination)
            if last.my_action == last.opponent_action {
                Action::Cooperate
            } else {
                Action::Defect
            }
        } else {
            Action::Cooperate
        }
    }

    fn is_nice(&self) -> bool {
        true // Cooperates first
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooperates_first() {
        let mut strategy = Grofman::new();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_cooperates_after_mutual_cooperation() {
        let mut strategy = Grofman::new();
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_cooperates_after_mutual_defection() {
        let mut strategy = Grofman::new();
        let mut history = History::new();

        history.push(Action::Defect, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_defects_after_asymmetry() {
        let mut strategy = Grofman::new();
        let mut history = History::new();

        // I cooperated, opponent defected
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);

        // I defected, opponent cooperated
        let mut history2 = History::new();
        history2.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history2), Action::Defect);
    }

    #[test]
    fn test_is_nice() {
        assert!(Grofman::new().is_nice());
    }
}
