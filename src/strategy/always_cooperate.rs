//! Always Cooperate Strategy
//!
//! Always cooperates, regardless of what the opponent does.
//! This is a "nice" strategy but easily exploitable.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Strategy that always cooperates
#[derive(Debug, Clone, Copy, Default)]
pub struct AlwaysCooperate;

impl Strategy for AlwaysCooperate {
    fn name(&self) -> &'static str {
        "Always Cooperate"
    }

    fn description(&self) -> &'static str {
        "Always cooperates, regardless of opponent's actions"
    }

    fn decide(&mut self, _history: &History) -> Action {
        Action::Cooperate
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
    fn test_always_cooperates() {
        let mut strategy = AlwaysCooperate;
        let mut history = History::new();

        // First round
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        // After cooperations
        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        // After opponent defections
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(AlwaysCooperate.is_nice());
    }
}
