//! Stratégie Tit for Tat (Œil pour œil) - par Anatol Rapoport
//!
//! Gagnante des deux tournois d'Axelrod (1980, 1984).
//! - Coopère au premier tour
//! - Ensuite, copie l'action précédente de l'adversaire
//!
//! Caractéristiques: Gentille, Provocable, Pardonnante, Claire

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Stratégie Tit for Tat (œil pour œil)
#[derive(Debug, Clone, Copy, Default)]
pub struct TitForTat;

impl Strategy for TitForTat {
    fn name(&self) -> &'static str {
        "Tit for Tat"
    }

    fn description(&self) -> &'static str {
        "Coopère d'abord, puis copie le dernier coup de l'adversaire (Rapoport)"
    }

    fn decide(&mut self, history: &History) -> Action {
        // Premier tour: coopérer
        // Ensuite: copier l'adversaire
        history.last_opponent_action().unwrap_or(Action::Cooperate)
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
        let mut strategy = TitForTat;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_copies_opponent() {
        let mut strategy = TitForTat;
        let mut history = History::new();

        // L'adversaire coopère -> on coopère
        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        // L'adversaire trahit -> on trahit
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);

        // L'adversaire coopère à nouveau -> on pardonne
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(TitForTat.is_nice());
    }
}
