//! Stratégie Tit for Two Tats (TF2T)
//!
//! Variante plus tolérante de Tit for Tat.
//! - Coopère au premier tour
//! - Ne trahit qu'après DEUX trahisons consécutives de l'adversaire
//!
//! Plus pardonnante que TFT, résiste mieux au bruit.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Stratégie Tit for Two Tats
#[derive(Debug, Clone, Copy, Default)]
pub struct TitForTwoTats;

impl Strategy for TitForTwoTats {
    fn name(&self) -> &'static str {
        "Tit for Two Tats"
    }

    fn description(&self) -> &'static str {
        "Coopère sauf si l'adversaire a trahi deux fois de suite"
    }

    fn decide(&mut self, history: &History) -> Action {
        let last_two = history.last_n_opponent_actions(2);

        // Trahit seulement si les deux dernières actions adverses sont des trahisons
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
        // Une seule trahison -> on coopère encore
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_defects_after_two_defections() {
        let mut strategy = TitForTwoTats;
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        history.push(Action::Cooperate, Action::Defect);
        // Deux trahisons consécutives -> on trahit
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_forgives_if_not_consecutive() {
        let mut strategy = TitForTwoTats;
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        history.push(Action::Cooperate, Action::Cooperate);
        history.push(Action::Cooperate, Action::Defect);
        // Trahisons non consécutives -> on coopère
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(TitForTwoTats.is_nice());
    }
}
