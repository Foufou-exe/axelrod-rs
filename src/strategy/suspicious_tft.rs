//! Stratégie Suspicious Tit for Tat (TFT méfiant)
//!
//! Variante de Tit for Tat qui commence par trahir.
//! - Trahit au premier tour
//! - Ensuite, copie l'action précédente de l'adversaire
//!
//! Non-gentille car elle trahit en premier.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Stratégie Suspicious Tit for Tat
#[derive(Debug, Clone, Copy, Default)]
pub struct SuspiciousTitForTat;

impl Strategy for SuspiciousTitForTat {
    fn name(&self) -> &'static str {
        "Suspicious Tit for Tat"
    }

    fn description(&self) -> &'static str {
        "Comme TFT mais commence par trahir"
    }

    fn decide(&mut self, history: &History) -> Action {
        // Premier tour: trahir (méfiance)
        // Ensuite: copier l'adversaire
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

        // Premier tour: trahit
        history.push(Action::Defect, Action::Cooperate);
        // L'adversaire a coopéré -> on coopère
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        history.push(Action::Cooperate, Action::Defect);
        // L'adversaire a trahi -> on trahit
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!SuspiciousTitForTat.is_nice());
    }
}
