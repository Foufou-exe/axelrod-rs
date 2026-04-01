//! Stratégies Go By Majority (Jouer la majorité)
//!
//! Ces stratégies jouent l'action que l'adversaire a le plus jouée.
//! - Hard Go By Majority: commence par trahir
//! - Soft Go By Majority: commence par coopérer
//!
//! En cas d'égalité, joue l'action par défaut.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Stratégie Hard Go By Majority
/// Commence par trahir, puis joue la majorité des coups adverses
#[derive(Debug, Clone, Copy, Default)]
pub struct HardGoByMajority;

impl Strategy for HardGoByMajority {
    fn name(&self) -> &'static str {
        "Hard Go By Majority"
    }

    fn description(&self) -> &'static str {
        "Joue l'action majoritaire de l'adversaire, commence par trahir"
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
            // En cas d'égalité ou plus de trahisons: trahir
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

/// Stratégie Soft Go By Majority
/// Commence par coopérer, puis joue la majorité des coups adverses
#[derive(Debug, Clone, Copy, Default)]
pub struct SoftGoByMajority;

impl Strategy for SoftGoByMajority {
    fn name(&self) -> &'static str {
        "Soft Go By Majority"
    }

    fn description(&self) -> &'static str {
        "Joue l'action majoritaire de l'adversaire, commence par coopérer"
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
            // En cas d'égalité ou plus de coopérations: coopérer
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

            // 2 coopérations, 1 trahison -> coopère
            history.push(Action::Defect, Action::Cooperate);
            history.push(Action::Cooperate, Action::Cooperate);
            history.push(Action::Cooperate, Action::Defect);
            assert_eq!(strategy.decide(&history), Action::Cooperate);

            // Ajoute une trahison -> égalité -> trahit
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

            // 1 coopération, 2 trahisons -> trahit
            history.push(Action::Cooperate, Action::Cooperate);
            history.push(Action::Cooperate, Action::Defect);
            history.push(Action::Defect, Action::Defect);
            assert_eq!(strategy.decide(&history), Action::Defect);

            // Ajoute une coopération -> égalité -> coopère
            history.push(Action::Defect, Action::Cooperate);
            assert_eq!(strategy.decide(&history), Action::Cooperate);
        }

        #[test]
        fn test_is_nice() {
            assert!(SoftGoByMajority.is_nice());
        }
    }
}
