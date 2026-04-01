//! Prober Strategy
//!
//! Tests the opponent to see if they are exploitable.
//! - Rounds 1-3: Plays D, C, C (probe sequence)
//! - If opponent cooperated on rounds 2 and 3: exploits (always defects)
//! - Otherwise: plays Tit for Tat
//!
//! A "clever" strategy that tries to exploit overly nice opponents.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;

/// Prober strategy
#[derive(Debug, Clone)]
pub struct Prober {
    /// Whether the opponent is exploitable (always cooperates)
    is_exploiting: bool,
    /// Whether the probe phase is complete
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
        "Probes with D,C,C then exploits if possible, otherwise TFT"
    }

    fn decide(&mut self, history: &History) -> Action {
        let round_num = history.len();

        match round_num {
            0 => Action::Defect,    // Round 1: Defect (probe)
            1 => Action::Cooperate, // Round 2: Cooperate
            2 => Action::Cooperate, // Round 3: Cooperate
            3 => {
                // End of probe phase
                // Check if opponent cooperated on rounds 2 and 3
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
                    // TFT: copy last move
                    history.last_opponent_action().unwrap_or(Action::Cooperate)
                }
            }
            _ => {
                if self.is_exploiting {
                    Action::Defect // Exploit the opponent
                } else {
                    // Standard TFT
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
        false // Starts by defecting
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

        // Round 1: Defect
        assert_eq!(strategy.decide(&history), Action::Defect);
        history.push(Action::Defect, Action::Cooperate);

        // Round 2: Cooperate
        assert_eq!(strategy.decide(&history), Action::Cooperate);
        history.push(Action::Cooperate, Action::Cooperate);

        // Round 3: Cooperate
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_exploits_cooperator() {
        let mut strategy = Prober::new();
        let mut history = History::new();

        // Probe sequence against Always Cooperate
        history.push(Action::Defect, Action::Cooperate);
        strategy.decide(&history);
        history.push(Action::Cooperate, Action::Cooperate);
        strategy.decide(&history);
        history.push(Action::Cooperate, Action::Cooperate);

        // Round 4: should exploit (defect)
        assert_eq!(strategy.decide(&history), Action::Defect);

        history.push(Action::Defect, Action::Cooperate);
        // Round 5: continues exploiting
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_plays_tft_against_retaliator() {
        let mut strategy = Prober::new();
        let mut history = History::new();

        // Opponent retaliates to initial defection
        history.push(Action::Defect, Action::Cooperate);
        strategy.decide(&history);
        history.push(Action::Cooperate, Action::Defect); // Retaliation!
        strategy.decide(&history);
        history.push(Action::Cooperate, Action::Cooperate);

        // Round 4: plays TFT (no exploitation)
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_is_not_nice() {
        assert!(!Prober::new().is_nice());
    }
}
