//! Stratégie Pavlov (Win-Stay, Lose-Shift)
//!
//! Aussi appelée "Win-Stay, Lose-Switch" ou "Simpleton".
//! - Si le dernier résultat était "bon" (R ou T), répète l'action
//! - Si le dernier résultat était "mauvais" (S ou P), change d'action
//!
//! Découverte par Nowak & Sigmund (1992) comme plus robuste que TFT face au bruit.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Stratégie Pavlov (Win-Stay, Lose-Shift)
#[derive(Debug, Clone, Copy, Default)]
pub struct Pavlov;

impl Strategy for Pavlov {
    fn name(&self) -> &'static str {
        "Pavlov"
    }

    fn description(&self) -> &'static str {
        "Win-Stay, Lose-Shift: répète si bon résultat (R/T), change sinon"
    }

    fn decide(&mut self, history: &History) -> Action {
        match history.last() {
            None => Action::Cooperate, // Premier tour: coopérer
            Some(round) => {
                // Bon résultat = on a coopéré et l'autre aussi (R)
                //              OU on a trahi et l'autre a coopéré (T)
                // Mauvais résultat = on a coopéré et l'autre a trahi (S)
                //                  OU on a trahi et l'autre aussi (P)

                let good_outcome = matches!(
                    (round.my_action, round.opponent_action),
                    (Action::Cooperate, Action::Cooperate) | (Action::Defect, Action::Cooperate)
                );

                if good_outcome {
                    // Win-Stay: répéter la dernière action
                    round.my_action
                } else {
                    // Lose-Shift: changer d'action
                    round.my_action.opposite()
                }
            }
        }
    }

    fn is_nice(&self) -> bool {
        true // Commence par coopérer
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
        let mut strategy = Pavlov;
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_win_stay_after_mutual_cooperation() {
        let mut strategy = Pavlov;
        let mut history = History::new();

        // Coopération mutuelle (R) -> Win-Stay -> Coopérer
        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_win_stay_after_temptation() {
        let mut strategy = Pavlov;
        let mut history = History::new();

        // J'ai trahi, l'autre a coopéré (T) -> Win-Stay -> Trahir
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_lose_shift_after_sucker() {
        let mut strategy = Pavlov;
        let mut history = History::new();

        // J'ai coopéré, l'autre a trahi (S) -> Lose-Shift -> Trahir
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_lose_shift_after_mutual_defection() {
        let mut strategy = Pavlov;
        let mut history = History::new();

        // Trahison mutuelle (P) -> Lose-Shift -> Coopérer
        history.push(Action::Defect, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(Pavlov.is_nice());
    }
}
