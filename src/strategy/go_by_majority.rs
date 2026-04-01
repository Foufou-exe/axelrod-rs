//! Go By Majority Strategies
//!
//! These strategies play the action that the opponent has played most often.
//! - Hard Go By Majority: starts by defecting
//! - Soft Go By Majority: starts by cooperating
//!
//! In case of a tie, plays the default action.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Hard Go By Majority strategy
/// Starts by defecting, then plays the majority of opponent's moves
#[derive(Debug, Clone, Copy, Default)]
pub struct HardGoByMajority;

impl Strategy for HardGoByMajority {
    fn name(&self) -> &'static str {
        "Hard Go By Majority"
    }

    fn description(&self) -> &'static str {
        "Plays opponent's majority action, starts by defecting"
    }

    fn decide(&mut self, history: &History) -> Action {
        if history.is_empty() {
            return Action::Defect;
        }

        let cooperations = history.count_opponent_cooperations();
        let defections = history.count_opponent_defections();

        if cooperations > defections {
            Action::Cooperate
        } else {
            // In case of tie or more defections: defect
            Action::Defect
        }
    }

    fn is_nice(&self) -> bool {
        false
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(*self)
    }
}

/// Soft Go By Majority strategy
/// Starts by cooperating, then plays the majority of opponent's moves
#[derive(Debug, Clone, Copy, Default)]
pub struct SoftGoByMajority;

impl Strategy for SoftGoByMajority {
    fn name(&self) -> &'static str {
        "Soft Go By Majority"
    }

    fn description(&self) -> &'static str {
        "Plays opponent's majority action, starts by cooperating"
    }

    fn decide(&mut self, history: &History) -> Action {
        if history.is_empty() {
            return Action::Cooperate;
        }

        let cooperations = history.count_opponent_cooperations();
        let defections = history.count_opponent_defections();

        if defections > cooperations {
            Action::Defect
        } else {
            // In case of tie or more cooperations: cooperate
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

    mod hard_go_by_majority {
        use super::*;

        #[test]
        fn test_defects_first() {
            let mut strategy = HardGoByMajority;
            let history = History::new();
            assert_eq!(strategy.decide(&history), Action::Defect);
        }

        #[test]
        fn test_follows_majority() {
            let mut strategy = HardGoByMajority;
            let mut history = History::new();

            // 2 cooperations, 1 defection -> cooperates
            history.push(Action::Defect, Action::Cooperate);
            history.push(Action::Cooperate, Action::Cooperate);
            history.push(Action::Cooperate, Action::Defect);
            assert_eq!(strategy.decide(&history), Action::Cooperate);

            // Add a defection -> tie -> defects
            history.push(Action::Cooperate, Action::Defect);
            assert_eq!(strategy.decide(&history), Action::Defect);
        }

        #[test]
        fn test_is_not_nice() {
            assert!(!HardGoByMajority.is_nice());
        }
    }

    mod soft_go_by_majority {
        use super::*;

        #[test]
        fn test_cooperates_first() {
            let mut strategy = SoftGoByMajority;
            let history = History::new();
            assert_eq!(strategy.decide(&history), Action::Cooperate);
        }

        #[test]
        fn test_follows_majority() {
            let mut strategy = SoftGoByMajority;
            let mut history = History::new();

            // 1 cooperation, 2 defections -> defects
            history.push(Action::Cooperate, Action::Cooperate);
            history.push(Action::Cooperate, Action::Defect);
            history.push(Action::Defect, Action::Defect);
            assert_eq!(strategy.decide(&history), Action::Defect);

            // Add a cooperation -> tie -> cooperates
            history.push(Action::Defect, Action::Cooperate);
            assert_eq!(strategy.decide(&history), Action::Cooperate);
        }

        #[test]
        fn test_is_nice() {
            assert!(SoftGoByMajority.is_nice());
        }
    }
}
