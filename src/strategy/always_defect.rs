//! Always Defect Strategy - also called ALL D
//!
//! Always defects, regardless of what the opponent does.
//! This is the Nash-optimal strategy for a single game, but suboptimal in iterated play.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Strategy that always defects
#[derive(Debug, Clone, Copy, Default)]
pub struct AlwaysDefect;

impl Strategy for AlwaysDefect {
    fn name(&self) -> &'static str {
        "Always Defect"
    }

    fn description(&self) -> &'static str {
        "Always defects, regardless of opponent's actions (ALL D)"
    }

    fn decide(&mut self, _history: &History) -> Action {
        Action::Defect
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
    fn test_always_defects() {
        let mut strategy = AlwaysDefect;
        let mut history = History::new();

        // First round
        assert_eq!(strategy.decide(&history), Action::Defect);

        // After opponent cooperations
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect);

        // After opponent defections
        history.push(Action::Defect, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!AlwaysDefect.is_nice());
    }
}
