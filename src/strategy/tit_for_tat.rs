//! Tit for Tat Strategy - by Anatol Rapoport
//!
//! Winner of both Axelrod tournaments (1980, 1984).
//! - Cooperates on the first move
//! - Then copies the opponent's previous action
//!
//! Characteristics: Nice, Provocable, Forgiving, Clear

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Tit for Tat strategy
#[derive(Debug, Clone, Copy, Default)]
pub struct TitForTat;

impl Strategy for TitForTat {
    fn name(&self) -> &'static str {
        "Tit for Tat"
    }

    fn description(&self) -> &'static str {
        "Cooperates first, then copies opponent's last move (Rapoport)"
    }

    fn decide(&mut self, history: &History) -> Action {
        // First round: cooperate
        // Then: copy opponent
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

        // Opponent cooperates -> we cooperate
        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);

        // Opponent defects -> we defect
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);

        // Opponent cooperates again -> we forgive
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(TitForTat.is_nice());
    }
}
