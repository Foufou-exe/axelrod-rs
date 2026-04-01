//! Tit for Two Tats Strategy (TF2T)
//!
//! A more forgiving variant of Tit for Tat.
//! - Cooperates on the first move
//! - Only defects after TWO consecutive opponent defections
//!
//! More forgiving than TFT, better resistance to noise.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Tit for Two Tats strategy
#[derive(Debug, Clone, Copy, Default)]
pub struct TitForTwoTats;

impl Strategy for TitForTwoTats {
    fn name(&self) -> &'static str {
        "Tit for Two Tats"
    }

    fn description(&self) -> &'static str {
        "Cooperates unless opponent defected twice in a row"
    }

    fn decide(&mut self, history: &History) -> Action {
        let last_two = history.last_n_opponent_actions(2);

        // Defects only if the last two opponent actions were defections
        if last_two.len() >= 2 && last_two[0] == Action::Defect && last_two[1] == Action::Defect {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }

    fn is_nice(&self) -> bool {
        true
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
        let mut strategy = TitForTwoTats;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_cooperates_after_single_defection() {
        let mut strategy = TitForTwoTats;
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        // Single defection -> still cooperate
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_defects_after_two_defections() {
        let mut strategy = TitForTwoTats;
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        history.push(Action::Cooperate, Action::Defect);
        // Two consecutive defections -> defect
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_forgives_if_not_consecutive() {
        let mut strategy = TitForTwoTats;
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Defect);
        // Non-consecutive defections -> cooperate
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(TitForTwoTats.is_nice());
    }
}
