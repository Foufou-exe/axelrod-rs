//! Suspicious Tit for Tat Strategy
//!
//! A variant of Tit for Tat that starts by defecting.
//! - Defects on the first move
//! - Then copies the opponent's previous action
//!
//! Not nice because it defects first.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Suspicious Tit for Tat strategy
#[derive(Debug, Clone, Copy, Default)]
pub struct SuspiciousTitForTat;

impl Strategy for SuspiciousTitForTat {
    fn name(&self) -> &'static str {
        "Suspicious Tit for Tat"
    }

    fn description(&self) -> &'static str {
        "Like TFT but starts by defecting"
    }

    fn decide(&mut self, history: &History) -> Action {
        // First round: defect (suspicious)
        // Then: copy opponent
        history.last_opponent_action().unwrap_or(Action::Defect)
    }

    fn is_nice(&self) -> bool {
        false
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defects_first() {
        let mut strategy = SuspiciousTitForTat;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_copies_opponent() {
        let mut strategy = SuspiciousTitForTat;
        let mut history = History::new();

        // First round: defects
        history.push(Action::Defect, Action::Cooperate);
        // Opponent cooperated -> we cooperate
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        history.push(Action::Cooperate, Action::Defect);
        // Opponent defected -> we defect
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!SuspiciousTitForTat.is_nice());
    }
}
