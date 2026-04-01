//! Pavlov Strategy (Win-Stay, Lose-Shift)
//!
//! Also called "Win-Stay, Lose-Switch" or "Simpleton".
//! - If the last outcome was "good" (R or T), repeat the action
//! - If the last outcome was "bad" (S or P), switch action
//!
//! Discovered by Nowak & Sigmund (1992) as more robust than TFT against noise.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Pavlov strategy (Win-Stay, Lose-Shift)
#[derive(Debug, Clone, Copy, Default)]
pub struct Pavlov;

impl Strategy for Pavlov {
    fn name(&self) -> &'static str {
        "Pavlov"
    }

    fn description(&self) -> &'static str {
        "Win-Stay, Lose-Shift: repeats if good outcome (R/T), switches otherwise"
    }

    fn decide(&mut self, history: &History) -> Action {
        match history.last() {
            None => Action::Cooperate, // First round: cooperate
            Some(round) => {
                // Good outcome = we cooperated and they did too (R)
                //              OR we defected and they cooperated (T)
                // Bad outcome = we cooperated and they defected (S)
                //             OR we defected and they did too (P)

                let good_outcome = matches!(
                    (round.my_action, round.opponent_action),
                    (Action::Cooperate, Action::Cooperate) | (Action::Defect, Action::Cooperate)
                );

                if good_outcome {
                    // Win-Stay: repeat last action
                    round.my_action
                } else {
                    // Lose-Shift: switch action
                    round.my_action.opposite()
                }
            }
        }
    }

    fn is_nice(&self) -> bool {
        true // Starts by cooperating
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

        // Mutual cooperation (R) -> Win-Stay -> Cooperate
        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_win_stay_after_temptation() {
        let mut strategy = Pavlov;
        let mut history = History::new();

        // I defected, they cooperated (T) -> Win-Stay -> Defect
        history.push(Action::Defect, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_lose_shift_after_sucker() {
        let mut strategy = Pavlov;
        let mut history = History::new();

        // I cooperated, they defected (S) -> Lose-Shift -> Defect
        history.push(Action::Cooperate, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_lose_shift_after_mutual_defection() {
        let mut strategy = Pavlov;
        let mut history = History::new();

        // Mutual defection (P) -> Lose-Shift -> Cooperate
        history.push(Action::Defect, Action::Defect);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_nice() {
        assert!(Pavlov.is_nice());
    }
}
