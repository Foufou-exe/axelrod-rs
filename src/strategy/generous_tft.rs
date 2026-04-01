//! Stratégie Generous Tit for Tat (GTFT)
//!
//! Variante de TFT qui pardonne occasionnellement les trahisons.
//! - Coopère au premier tour
//! - Copie l'adversaire, MAIS pardonne les trahisons avec une probabilité p
//!
//! Permet de briser les cycles de représailles mutuelles.

use crate::action::Action;
use crate::history::History;
use crate::strategy::Strategy;
use rand::Rng;

/// Stratégie Generous Tit for Tat
#[derive(Debug, Clone)]
pub struct GenerousTitForTat {
    /// Probabilité de pardonner une trahison (typiquement 0.05 à 0.10)
    forgiveness_probability: f64,
}

impl GenerousTitForTat {
    /// Crée une nouvelle instance avec une probabilité de pardon personnalisée
    pub fn new(forgiveness_probability: f64) -> Self {
        Self {
            forgiveness_probability: forgiveness_probability.clamp(0.0, 1.0),
        }
    }

    /// Crée une instance avec la probabilité de pardon par défaut (5%)
    pub fn default() -> Self {
        Self::new(0.05)
    }
}

impl Strategy for GenerousTitForTat {
    fn name(&self) -> &'static str {
        "Generous Tit for Tat"
    }

    fn description(&self) -> &'static str {
        "Comme TFT mais pardonne parfois les trahisons (~5%)"
    }

    fn decide(&mut self, history: &History) -> Action {
        match history.last_opponent_action() {
            None => Action::Cooperate,
            Some(Action::Cooperate) => Action::Cooperate,
            Some(Action::Defect) => {
                // Pardonner avec une certaine probabilité
                let mut rng = rand::thread_rng();
                if Rng::r#gen::<f64>(&mut rng) < self.forgiveness_probability {
                    Action::Cooperate
                } else {
                    Action::Defect
                }
            }
        }
    }

    fn is_nice(&self) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cooperates_first() {
        let mut strategy = GenerousTitForTat::default();
        let history = History::new();
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_cooperates_after_cooperation() {
        let mut strategy = GenerousTitForTat::default();
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Cooperate);
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_always_forgives_with_probability_1() {
        let mut strategy = GenerousTitForTat::new(1.0);
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        // Avec probabilité 1, pardonne toujours
        assert_eq!(strategy.decide(&history), Action::Cooperate);
    }

    #[test]
    fn test_never_forgives_with_probability_0() {
        let mut strategy = GenerousTitForTat::new(0.0);
        let mut history = History::new();

        history.push(Action::Cooperate, Action::Defect);
        // Avec probabilité 0, ne pardonne jamais
        assert_eq!(strategy.decide(&history), Action::Defect);
    }

    #[test]
    fn test_is_nice() {
        assert!(GenerousTitForTat::default().is_nice());
    }
}
