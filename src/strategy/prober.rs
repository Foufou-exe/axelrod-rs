//! Stratégie Prober (Sondeur)
//!
//! Teste l'adversaire pour voir s'il est exploitable.
//! - Tours 1-3: Joue D, C, C (séquence de sondage)
//! - Si l'adversaire a coopéré aux tours 2 et 3: exploite (trahit toujours)
//! - Sinon: joue Tit for Tat
//!
//! Stratégie "astucieuse" qui tente d'exploiter les adversaires trop gentils.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Stratégie Prober
#[derive(Debug, Clone)]
pub struct Prober {
    /// Indique si l'adversaire est exploitable (coopère toujours)
    is_exploiting: bool,
    /// Indique si la phase de sondage est terminée
    probe_complete: bool,
}

impl Prober {
    pub fn new() -> Self {
        Self {
            is_exploiting: false,
            probe_complete: false,
        }
    }
}

impl Default for Prober {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for Prober {
    fn name(&self) -> &'static str {
        "Prober"
    }

    fn description(&self) -> &'static str {
        "Sonde avec D,C,C puis exploite si possible, sinon TFT"
    }

    fn decide(&mut self, history: &History) -> Action {
        let round_num = history.len();

        match round_num {
            0 => Action::Defect,    // Tour 1: Trahir (sonde)
            1 => Action::Cooperate, // Tour 2: Coopérer
            2 => Action::Cooperate, // Tour 3: Coopérer
            3 => {
                // Fin de la phase de sondage
                // Vérifie si l'adversaire a coopéré aux tours 2 et 3
                self.probe_complete = true;
                if let (Some(r2), Some(r3)) = (history.get(1), history.get(2)) {
                    if r2.opponent_action == Action::Cooperate
                        && r3.opponent_action == Action::Cooperate
                    {
                        self.is_exploiting = true;
                    }
                }

                if self.is_exploiting {
                    Action::Defect
                } else {
                    // TFT: copier le dernier coup
                    history.last_opponent_action().unwrap_or(Action::Cooperate)
                }
            }
            _ => {
                if self.is_exploiting {
                    Action::Defect // Exploite l'adversaire
                } else {
                    // TFT standard
                    history.last_opponent_action().unwrap_or(Action::Cooperate)
                }
            }
        }
    }

    fn reset(&mut self) {
        self.is_exploiting = false;
        self.probe_complete = false;
    }

    fn is_nice(&self) -> bool {
        false // Commence par trahir
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe_sequence() {
        let mut strategy = Prober::new();
        let mut history = History::new();

        // Tour 1: Defect
        assert_eq!(strategy.decide(&history), Action::Defect);
        history.push(Action::Defect, Action::Cooperate);

        // Tour 2: Cooperate
        assert_eq!(strategy.decide(&history), Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);

        // Tour 3: Cooperate
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_exploits_cooperator() {
        let mut strategy = Prober::new();
        let mut history = History::new();

        // Séquence de sondage contre Always Cooperate
        history.push(Action::Defect, Action::Cooperate);
        strategy.decide(&history);
        history.push(Action::Cooperate, Action::Cooperate);
        strategy.decide(&history);
        history.push(Action::Cooperate, Action::Cooperate);

        // Tour 4: devrait exploiter (trahir)
        assert_eq!(strategy.decide(&history), Action::Defect);

        history.push(Action::Defect, Action::Cooperate);
        // Tour 5: continue d'exploiter
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_plays_tft_against_retaliator() {
        let mut strategy = Prober::new();
        let mut history = History::new();

        // L'adversaire riposte à la trahison initiale
        history.push(Action::Defect, Action::Cooperate);
        strategy.decide(&history);
        history.push(Action::Cooperate, Action::Defect); // Riposte!
        strategy.decide(&history);
        history.push(Action::Cooperate, Action::Cooperate);

        // Tour 4: joue TFT (pas d'exploitation)
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!Prober::new().is_nice());
    }
}
